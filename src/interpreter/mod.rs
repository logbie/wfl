pub mod environment;
pub mod error;
#[cfg(test)]
mod tests;
pub mod value;

use self::environment::Environment;
use self::error::RuntimeError;
use self::value::{FunctionValue, Value};
use crate::parser::ast::{Expression, Literal, Operator, Program, Statement, UnaryOperator};
use crate::stdlib;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    global_env: Rc<RefCell<Environment>>,
    current_count: RefCell<Option<f64>>,
    in_count_loop: RefCell<bool>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let global_env = Environment::new_global();

        {
            let mut env = global_env.borrow_mut();
            env.define("display", Value::NativeFunction(Self::native_display));

            stdlib::register_stdlib(&mut env);
        }

        Interpreter {
            global_env,
            current_count: RefCell::new(None),
            in_count_loop: RefCell::new(false),
        }
    }

    fn native_display(args: Vec<Value>) -> Result<Value, RuntimeError> {
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{}", arg);
        }
        println!();
        Ok(Value::Null)
    }

    pub fn interpret(&mut self, program: &Program) -> Result<Value, Vec<RuntimeError>> {
        let mut last_value = Value::Null;
        let mut errors = Vec::new();

        for statement in &program.statements {
            match self.execute_statement(statement, Rc::clone(&self.global_env)) {
                Ok(value) => last_value = value,
                Err(err) => errors.push(err),
            }
        }

        if errors.is_empty() {
            if let Some(Value::Function(main_func)) = self.global_env.borrow().get("main") {
                match self.call_function(&main_func, vec![], 0, 0) {
                    Ok(value) => last_value = value,
                    Err(err) => errors.push(err),
                }
            }

            Ok(last_value)
        } else {
            Err(errors)
        }
    }

    fn execute_statement(
        &self,
        stmt: &Statement,
        env: Rc<RefCell<Environment>>,
    ) -> Result<Value, RuntimeError> {
        match stmt {
            Statement::VariableDeclaration {
                name,
                value,
                line: _,
                column: _,
            } => {
                let value = self.evaluate_expression(value, Rc::clone(&env))?;
                env.borrow_mut().define(name, value);
                Ok(Value::Null)
            }

            Statement::Assignment {
                name,
                value,
                line,
                column,
            } => {
                let value = self.evaluate_expression(value, Rc::clone(&env))?;
                match env.borrow_mut().assign(name, value) {
                    Ok(_) => Ok(Value::Null),
                    Err(msg) => Err(RuntimeError::new(msg, *line, *column)),
                }
            }

            Statement::IfStatement {
                condition,
                then_block,
                else_block,
                line: _line,
                column: _column,
            } => {
                let condition_value = self.evaluate_expression(condition, Rc::clone(&env))?;

                if condition_value.is_truthy() {
                    self.execute_block(then_block, Rc::clone(&env))
                } else if let Some(else_stmts) = else_block {
                    self.execute_block(else_stmts, Rc::clone(&env))
                } else {
                    Ok(Value::Null)
                }
            }

            Statement::SingleLineIf {
                condition,
                then_stmt,
                else_stmt,
                line: _line,
                column: _column,
            } => {
                let condition_value = self.evaluate_expression(condition, Rc::clone(&env))?;

                if condition_value.is_truthy() {
                    self.execute_statement(then_stmt, Rc::clone(&env))
                } else if let Some(else_stmt) = else_stmt {
                    self.execute_statement(else_stmt, Rc::clone(&env))
                } else {
                    Ok(Value::Null)
                }
            }

            Statement::DisplayStatement {
                value,
                line: _line,
                column: _column,
            } => {
                let value = self.evaluate_expression(value, Rc::clone(&env))?;
                println!("{}", value);
                Ok(Value::Null)
            }

            Statement::ActionDefinition {
                name,
                parameters,
                body,
                return_type: _return_type,
                line,
                column,
            } => {
                let param_names: Vec<String> = parameters.iter().map(|p| p.name.clone()).collect();

                let function = FunctionValue {
                    name: Some(name.clone()),
                    params: param_names,
                    body: body.clone(),
                    env: Rc::clone(&env),
                    line: *line,
                    column: *column,
                };

                let function_value = Value::Function(Rc::new(function));
                env.borrow_mut().define(name, function_value.clone());

                Ok(function_value)
            }

            Statement::ReturnStatement {
                value,
                line: _line,
                column: _column,
            } => {
                if let Some(expr) = value {
                    self.evaluate_expression(expr, Rc::clone(&env))
                } else {
                    Ok(Value::Null)
                }
            }

            Statement::ExpressionStatement {
                expression,
                line: _line,
                column: _column,
            } => self.evaluate_expression(expression, Rc::clone(&env)),

            Statement::CountLoop {
                start,
                end,
                step,
                downward,
                body,
                line,
                column,
            } => {
                let start_val = self.evaluate_expression(start, Rc::clone(&env))?;
                let end_val = self.evaluate_expression(end, Rc::clone(&env))?;

                let (start_num, end_num) = match (start_val, end_val) {
                    (Value::Number(s), Value::Number(e)) => (s, e),
                    _ => {
                        return Err(RuntimeError::new(
                            "Count loop requires numeric start and end values".to_string(),
                            *line,
                            *column,
                        ));
                    }
                };

                let step_num = if let Some(step_expr) = step {
                    match self.evaluate_expression(step_expr, Rc::clone(&env))? {
                        Value::Number(n) => n,
                        _ => {
                            return Err(RuntimeError::new(
                                "Count loop step must be a number".to_string(),
                                *line,
                                *column,
                            ));
                        }
                    }
                } else {
                    1.0
                };

                let mut count = start_num;
                let loop_env = Environment::new(&env);

                let should_continue: Box<dyn Fn(f64, f64) -> bool> = if *downward {
                    Box::new(|count, end_num| count >= end_num)
                } else {
                    Box::new(|count, end_num| count <= end_num)
                };

                let max_iterations = 10000; // Reasonable limit for most loops
                let mut iterations = 0;

                *self.in_count_loop.borrow_mut() = true;

                while should_continue(count, end_num) && iterations < max_iterations {
                    *self.current_count.borrow_mut() = Some(count);

                    match self.execute_block(body, Rc::clone(&loop_env)) {
                        Ok(_) => {}
                        Err(e) => {
                            *self.in_count_loop.borrow_mut() = false;
                            return Err(e);
                        }
                    }

                    if *downward {
                        count -= step_num;
                    } else {
                        count += step_num;
                    }

                    iterations += 1;
                }

                *self.current_count.borrow_mut() = None;
                *self.in_count_loop.borrow_mut() = false;

                if iterations >= max_iterations {
                    return Err(RuntimeError::new(
                        format!(
                            "Count loop exceeded maximum iterations ({})",
                            max_iterations
                        ),
                        *line,
                        *column,
                    ));
                }

                Ok(Value::Null)
            }

            Statement::ForEachLoop {
                item_name,
                collection,
                reversed,
                body,
                line,
                column,
            } => {
                let collection_val = self.evaluate_expression(collection, Rc::clone(&env))?;

                let loop_env = Environment::new(&env);

                match collection_val {
                    Value::List(list_rc) => {
                        let list = list_rc.borrow();
                        let indices: Vec<usize> = if *reversed {
                            (0..list.len()).rev().collect()
                        } else {
                            (0..list.len()).collect()
                        };

                        for i in indices {
                            loop_env.borrow_mut().define(item_name, list[i].clone());

                            self.execute_block(body, Rc::clone(&loop_env))?;
                        }
                    }
                    Value::Object(obj_rc) => {
                        let obj = obj_rc.borrow();
                        let keys: Vec<String> = obj.keys().cloned().collect();

                        for key in keys {
                            if let Some(value) = obj.get(&key) {
                                loop_env.borrow_mut().define(item_name, value.clone());

                                self.execute_block(body, Rc::clone(&loop_env))?;
                            }
                        }
                    }
                    _ => {
                        return Err(RuntimeError::new(
                            format!("Cannot iterate over {}", collection_val.type_name()),
                            *line,
                            *column,
                        ));
                    }
                }

                Ok(Value::Null)
            }

            Statement::WhileLoop {
                condition,
                body,
                line: _line,
                column: _column,
            } => {
                while self
                    .evaluate_expression(condition, Rc::clone(&env))?
                    .is_truthy()
                {
                    self.execute_block(body, Rc::clone(&env))?;
                }
                Ok(Value::Null)
            }

            Statement::RepeatUntilLoop {
                condition,
                body,
                line: _line,
                column: _column,
            } => {
                loop {
                    self.execute_block(body, Rc::clone(&env))?;
                    if self
                        .evaluate_expression(condition, Rc::clone(&env))?
                        .is_truthy()
                    {
                        break;
                    }
                }
                Ok(Value::Null)
            }

            Statement::ForeverLoop {
                body,
                line: _line,
                column: _column,
            } => {
                loop {
                    self.execute_block(body, Rc::clone(&env))?;
                }
                #[allow(unreachable_code)]
                Ok(Value::Null)
            }

            Statement::BreakStatement { .. } | Statement::ContinueStatement { .. } => {
                Ok(Value::Null)
            }

            Statement::OpenFileStatement { .. }
            | Statement::ReadFileStatement { .. }
            | Statement::WriteFileStatement { .. }
            | Statement::CloseFileStatement { .. } => Ok(Value::Null),
        }
    }

    fn execute_block(
        &self,
        statements: &[Statement],
        env: Rc<RefCell<Environment>>,
    ) -> Result<Value, RuntimeError> {
        let mut last_value = Value::Null;

        for statement in statements {
            last_value = self.execute_statement(statement, Rc::clone(&env))?;
        }

        Ok(last_value)
    }

    fn evaluate_expression(
        &self,
        expr: &Expression,
        env: Rc<RefCell<Environment>>,
    ) -> Result<Value, RuntimeError> {
        match expr {
            Expression::Literal(literal, _line, _column) => match literal {
                Literal::String(s) => Ok(Value::Text(Rc::from(s.as_str()))),
                Literal::Integer(i) => Ok(Value::Number(*i as f64)),
                Literal::Float(f) => Ok(Value::Number(*f)),
                Literal::Boolean(b) => Ok(Value::Bool(*b)),
                Literal::Nothing => Ok(Value::Null),
                Literal::Pattern(s) => Ok(Value::Text(Rc::from(s.as_str()))),
            },

            Expression::Variable(name, line, column) => {
                if name == "count" {
                    if let Some(count_value) = *self.current_count.borrow() {
                        return Ok(Value::Number(count_value));
                    } else {
                        println!(
                            "Warning: Using 'count' outside of a count loop context at line {}, column {}",
                            line, column
                        );
                        return Ok(Value::Number(0.0));
                    }
                }

                if let Some(value) = env.borrow().get(name) {
                    Ok(value)
                } else {
                    Err(RuntimeError::new(
                        format!("Undefined variable '{}'", name),
                        *line,
                        *column,
                    ))
                }
            }

            Expression::BinaryOperation {
                left,
                operator,
                right,
                line,
                column,
            } => {
                let left_val = self.evaluate_expression(left, Rc::clone(&env))?;
                let right_val = self.evaluate_expression(right, Rc::clone(&env))?;

                match operator {
                    Operator::Plus => self.add(left_val, right_val, *line, *column),
                    Operator::Minus => self.subtract(left_val, right_val, *line, *column),
                    Operator::Multiply => self.multiply(left_val, right_val, *line, *column),
                    Operator::Divide => self.divide(left_val, right_val, *line, *column),
                    Operator::Equals => Ok(Value::Bool(self.is_equal(&left_val, &right_val))),
                    Operator::NotEquals => Ok(Value::Bool(!self.is_equal(&left_val, &right_val))),
                    Operator::GreaterThan => self.greater_than(left_val, right_val, *line, *column),
                    Operator::LessThan => self.less_than(left_val, right_val, *line, *column),
                    Operator::GreaterThanOrEqual => {
                        self.greater_than_equal(left_val, right_val, *line, *column)
                    }
                    Operator::LessThanOrEqual => {
                        self.less_than_equal(left_val, right_val, *line, *column)
                    }
                    Operator::And => Ok(Value::Bool(left_val.is_truthy() && right_val.is_truthy())),
                    Operator::Or => Ok(Value::Bool(left_val.is_truthy() || right_val.is_truthy())),
                    Operator::Contains => self.contains(left_val, right_val, *line, *column),
                }
            }

            Expression::UnaryOperation {
                operator,
                expression,
                line,
                column,
            } => {
                let value = self.evaluate_expression(expression, Rc::clone(&env))?;

                match operator {
                    UnaryOperator::Not => Ok(Value::Bool(!value.is_truthy())),
                    UnaryOperator::Minus => match value {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => Err(RuntimeError::new(
                            format!("Cannot negate {}", value.type_name()),
                            *line,
                            *column,
                        )),
                    },
                }
            }

            Expression::FunctionCall {
                function,
                arguments,
                line,
                column,
            } => {
                let function_val = self.evaluate_expression(function, Rc::clone(&env))?;

                let mut arg_values = Vec::new();
                for arg in arguments {
                    arg_values.push(self.evaluate_expression(&arg.value, Rc::clone(&env))?);
                }

                match function_val {
                    Value::Function(func) => self.call_function(&func, arg_values, *line, *column),
                    Value::NativeFunction(native_fn) => native_fn(arg_values).map_err(|e| {
                        RuntimeError::new(
                            format!("Error in native function: {}", e),
                            *line,
                            *column,
                        )
                    }),
                    _ => Err(RuntimeError::new(
                        format!("Cannot call {}", function_val.type_name()),
                        *line,
                        *column,
                    )),
                }
            }

            Expression::MemberAccess {
                object,
                property,
                line,
                column,
            } => {
                let object_val = self.evaluate_expression(object, Rc::clone(&env))?;

                match object_val {
                    Value::Object(obj_rc) => {
                        let obj = obj_rc.borrow();
                        if let Some(value) = obj.get(property) {
                            Ok(value.clone())
                        } else {
                            Err(RuntimeError::new(
                                format!("Object has no property '{}'", property),
                                *line,
                                *column,
                            ))
                        }
                    }
                    _ => Err(RuntimeError::new(
                        format!("Cannot access property of {}", object_val.type_name()),
                        *line,
                        *column,
                    )),
                }
            }

            Expression::IndexAccess {
                collection,
                index,
                line,
                column,
            } => {
                let collection_val = self.evaluate_expression(collection, Rc::clone(&env))?;
                let index_val = self.evaluate_expression(index, Rc::clone(&env))?;

                match (collection_val, index_val) {
                    (Value::List(list_rc), Value::Number(idx)) => {
                        let list = list_rc.borrow();
                        let idx = idx as usize;

                        if idx < list.len() {
                            Ok(list[idx].clone())
                        } else {
                            Err(RuntimeError::new(
                                format!(
                                    "Index {} out of bounds for list of length {}",
                                    idx,
                                    list.len()
                                ),
                                *line,
                                *column,
                            ))
                        }
                    }
                    (Value::Object(obj_rc), Value::Text(key)) => {
                        let obj = obj_rc.borrow();
                        let key_str = key.to_string();

                        if let Some(value) = obj.get(&key_str) {
                            Ok(value.clone())
                        } else {
                            Err(RuntimeError::new(
                                format!("Object has no key '{}'", key_str),
                                *line,
                                *column,
                            ))
                        }
                    }
                    (collection, index) => Err(RuntimeError::new(
                        format!(
                            "Cannot index {} with {}",
                            collection.type_name(),
                            index.type_name()
                        ),
                        *line,
                        *column,
                    )),
                }
            }

            Expression::Concatenation {
                left,
                right,
                line: _line,
                column: _column,
            } => {
                let left_val = self.evaluate_expression(left, Rc::clone(&env))?;

                let right_val = match right.as_ref() {
                    Expression::Variable(name, _, _) if name == "count" => {
                        if let Some(count_value) = *self.current_count.borrow() {
                            Value::Number(count_value)
                        } else {
                            self.evaluate_expression(right, Rc::clone(&env))?
                        }
                    }
                    _ => self.evaluate_expression(right, Rc::clone(&env))?,
                };

                let result = format!("{}{}", left_val, right_val);
                Ok(Value::Text(Rc::from(result.as_str())))
            }
            
            Expression::PatternMatch {
                text,
                pattern,
                line: _line,
                column: _column,
            } => {
                let text_val = self.evaluate_expression(text, Rc::clone(&env))?;
                let pattern_val = self.evaluate_expression(pattern, Rc::clone(&env))?;
                
                let args = vec![text_val, pattern_val];
                crate::stdlib::pattern::native_pattern_matches(args)
            }
            
            Expression::PatternFind {
                text,
                pattern,
                line: _line,
                column: _column,
            } => {
                let text_val = self.evaluate_expression(text, Rc::clone(&env))?;
                let pattern_val = self.evaluate_expression(pattern, Rc::clone(&env))?;
                
                let args = vec![pattern_val, text_val]; // Note: pattern first, then text
                crate::stdlib::pattern::native_pattern_find(args)
            }
            
            Expression::PatternReplace {
                text,
                pattern,
                replacement,
                line: _line,
                column: _column,
            } => {
                let text_val = self.evaluate_expression(text, Rc::clone(&env))?;
                let pattern_val = self.evaluate_expression(pattern, Rc::clone(&env))?;
                let replacement_val = self.evaluate_expression(replacement, Rc::clone(&env))?;
                
                let args = vec![pattern_val, replacement_val, text_val]; // Note: pattern, replacement, then text
                crate::stdlib::pattern::native_pattern_replace(args)
            }
            
            Expression::PatternSplit {
                text,
                pattern,
                line: _line,
                column: _column,
            } => {
                let text_val = self.evaluate_expression(text, Rc::clone(&env))?;
                let pattern_val = self.evaluate_expression(pattern, Rc::clone(&env))?;
                
                let args = vec![text_val, pattern_val];
                crate::stdlib::pattern::native_pattern_split(args)
            }
        }
    }

    fn call_function(
        &self,
        func: &FunctionValue,
        args: Vec<Value>,
        line: usize,
        column: usize,
    ) -> Result<Value, RuntimeError> {
        if args.len() != func.params.len() {
            return Err(RuntimeError::new(
                format!(
                    "Expected {} arguments but got {}",
                    func.params.len(),
                    args.len()
                ),
                line,
                column,
            ));
        }

        let call_env = Environment::new(&func.env);

        for (param, arg) in func.params.iter().zip(args) {
            call_env.borrow_mut().define(param, arg);
        }

        self.execute_block(&func.body, call_env)
    }

    fn add(
        &self,
        left: Value,
        right: Value,
        line: usize,
        column: usize,
    ) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::Text(a), Value::Text(b)) => {
                let result = format!("{}{}", a, b);
                Ok(Value::Text(Rc::from(result.as_str())))
            }
            (Value::Text(a), b) => {
                let result = format!("{}{}", a, b);
                Ok(Value::Text(Rc::from(result.as_str())))
            }
            (a, Value::Text(b)) => {
                let result = format!("{}{}", a, b);
                Ok(Value::Text(Rc::from(result.as_str())))
            }
            (a, b) => Err(RuntimeError::new(
                format!("Cannot add {} and {}", a.type_name(), b.type_name()),
                line,
                column,
            )),
        }
    }

    fn subtract(
        &self,
        left: Value,
        right: Value,
        line: usize,
        column: usize,
    ) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            (a, b) => Err(RuntimeError::new(
                format!("Cannot subtract {} from {}", b.type_name(), a.type_name()),
                line,
                column,
            )),
        }
    }

    fn multiply(
        &self,
        left: Value,
        right: Value,
        line: usize,
        column: usize,
    ) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            (a, b) => Err(RuntimeError::new(
                format!("Cannot multiply {} and {}", a.type_name(), b.type_name()),
                line,
                column,
            )),
        }
    }

    fn divide(
        &self,
        left: Value,
        right: Value,
        line: usize,
        column: usize,
    ) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if b == 0.0 {
                    Err(RuntimeError::new(
                        "Division by zero".to_string(),
                        line,
                        column,
                    ))
                } else {
                    Ok(Value::Number(a / b))
                }
            }
            (a, b) => Err(RuntimeError::new(
                format!("Cannot divide {} by {}", a.type_name(), b.type_name()),
                line,
                column,
            )),
        }
    }

    fn is_equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => (a - b).abs() < f64::EPSILON,
            (Value::Text(a), Value::Text(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }

    fn greater_than(
        &self,
        left: Value,
        right: Value,
        line: usize,
        column: usize,
    ) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
            (Value::Text(a), Value::Text(b)) => Ok(Value::Bool(a > b)),
            (a, b) => Err(RuntimeError::new(
                format!(
                    "Cannot compare {} and {} with >",
                    a.type_name(),
                    b.type_name()
                ),
                line,
                column,
            )),
        }
    }

    fn less_than(
        &self,
        left: Value,
        right: Value,
        line: usize,
        column: usize,
    ) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
            (Value::Text(a), Value::Text(b)) => Ok(Value::Bool(a < b)),
            (a, b) => Err(RuntimeError::new(
                format!(
                    "Cannot compare {} and {} with <",
                    a.type_name(),
                    b.type_name()
                ),
                line,
                column,
            )),
        }
    }

    fn greater_than_equal(
        &self,
        left: Value,
        right: Value,
        line: usize,
        column: usize,
    ) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),
            (Value::Text(a), Value::Text(b)) => Ok(Value::Bool(a >= b)),
            (a, b) => Err(RuntimeError::new(
                format!(
                    "Cannot compare {} and {} with >=",
                    a.type_name(),
                    b.type_name()
                ),
                line,
                column,
            )),
        }
    }

    fn less_than_equal(
        &self,
        left: Value,
        right: Value,
        line: usize,
        column: usize,
    ) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
            (Value::Text(a), Value::Text(b)) => Ok(Value::Bool(a <= b)),
            (a, b) => Err(RuntimeError::new(
                format!(
                    "Cannot compare {} and {} with <=",
                    a.type_name(),
                    b.type_name()
                ),
                line,
                column,
            )),
        }
    }

    fn contains(
        &self,
        left: Value,
        right: Value,
        line: usize,
        column: usize,
    ) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::List(list_rc), item) => {
                let list = list_rc.borrow();
                for value in list.iter() {
                    if self.is_equal(value, &item) {
                        return Ok(Value::Bool(true));
                    }
                }
                Ok(Value::Bool(false))
            }
            (Value::Object(obj_rc), Value::Text(key)) => {
                let obj = obj_rc.borrow();
                Ok(Value::Bool(obj.contains_key(&key.to_string())))
            }
            (Value::Text(text), Value::Text(substring)) => {
                Ok(Value::Bool(text.contains(&*substring)))
            }
            (a, b) => Err(RuntimeError::new(
                format!(
                    "Cannot check if {} contains {}",
                    a.type_name(),
                    b.type_name()
                ),
                line,
                column,
            )),
        }
    }
}
