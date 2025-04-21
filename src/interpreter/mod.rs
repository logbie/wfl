#![allow(clippy::await_holding_refcell_ref)]
pub mod environment;
pub mod error;
#[cfg(test)]
mod tests;
pub mod value;

use self::environment::Environment;
use self::error::{ErrorKind, RuntimeError};
use self::value::{FunctionValue, Value};
use crate::debug_report::CallFrame;
use crate::parser::ast::{Expression, Literal, Operator, Program, Statement, UnaryOperator};
use crate::stdlib;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::{Duration, Instant};

use std::collections::HashMap;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncSeekExt;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
// use self::value::FutureValue;

#[derive(Debug, Clone, PartialEq)]
pub enum LoopControl {
    None,
    Continue,
    Break,
    Exit {
        line: usize,
        column: usize,
    },
}

pub struct Interpreter {
    global_env: Rc<RefCell<Environment>>,
    current_count: RefCell<Option<f64>>,
    in_count_loop: RefCell<bool>,
    started: Instant,
    max_duration: Duration,
    call_stack: RefCell<Vec<CallFrame>>,
    #[allow(dead_code)]
    io_client: Rc<IoClient>,
}

#[allow(dead_code)]
pub struct IoClient {
    http_client: reqwest::Client,
    file_handles: Mutex<HashMap<String, (PathBuf, tokio::fs::File)>>,
    next_file_id: Mutex<usize>,
}

impl IoClient {
    fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            file_handles: Mutex::new(HashMap::new()),
            next_file_id: Mutex::new(1),
        }
    }

    #[allow(dead_code)]
    async fn http_get(&self, url: &str) -> Result<String, String> {
        match self.http_client.get(url).send().await {
            Ok(response) => match response.text().await {
                Ok(text) => Ok(text),
                Err(e) => Err(format!("Failed to read response body: {}", e)),
            },
            Err(e) => Err(format!("Failed to send HTTP GET request: {}", e)),
        }
    }

    #[allow(dead_code)]
    async fn http_post(&self, url: &str, data: &str) -> Result<String, String> {
        match self
            .http_client
            .post(url)
            .body(data.to_string())
            .send()
            .await
        {
            Ok(response) => match response.text().await {
                Ok(text) => Ok(text),
                Err(e) => Err(format!("Failed to read response body: {}", e)),
            },
            Err(e) => Err(format!("Failed to send HTTP POST request: {}", e)),
        }
    }

    #[allow(dead_code)]
    async fn open_file(&self, path: &str) -> Result<String, String> {
        let handle_id = {
            let mut next_id = self.next_file_id.lock().await;
            let id = format!("file{}", *next_id);
            *next_id += 1;
            id
        };

        let path_buf = PathBuf::from(path);

        match tokio::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false) // Explicitly preserve file content on open
            .open(path)
            .await
        {
            Ok(file) => {
                let mut file_handles = self.file_handles.lock().await;

                for (id, (existing_path, _)) in file_handles.iter() {
                    if existing_path == &path_buf {
                        return Err(format!("File already open with handle {}", id));
                    }
                }

                file_handles.insert(handle_id.clone(), (path_buf, file));
                Ok(handle_id)
            }
            Err(e) => Err(format!("Failed to open file {}: {}", path, e)),
        }
    }

    #[allow(dead_code)]
    async fn read_file(&self, handle_id: &str) -> Result<String, String> {
        let mut file_handles = self.file_handles.lock().await;

        if !file_handles.contains_key(handle_id) {
            return Err(format!("Invalid file handle: {}", handle_id));
        }

        let mut file_clone = match file_handles.get_mut(handle_id).unwrap().1.try_clone().await {
            Ok(clone) => clone,
            Err(e) => return Err(format!("Failed to clone file handle: {}", e)),
        };

        drop(file_handles);

        let mut contents = String::new();
        match AsyncReadExt::read_to_string(&mut file_clone, &mut contents).await {
            Ok(_) => Ok(contents),
            Err(e) => Err(format!("Failed to read file: {}", e)),
        }
    }

    #[allow(dead_code)]
    async fn write_file(&self, handle_id: &str, content: &str) -> Result<(), String> {
        let mut file_handles = self.file_handles.lock().await;

        if !file_handles.contains_key(handle_id) {
            return Err(format!("Invalid file handle: {}", handle_id));
        }

        let mut file_clone = match file_handles.get_mut(handle_id).unwrap().1.try_clone().await {
            Ok(clone) => clone,
            Err(e) => return Err(format!("Failed to clone file handle: {}", e)),
        };

        drop(file_handles);

        match AsyncSeekExt::seek(&mut file_clone, std::io::SeekFrom::Start(0)).await {
            Ok(_) => match file_clone.set_len(0).await {
                Ok(_) => {
                    match AsyncWriteExt::write_all(&mut file_clone, content.as_bytes()).await {
                        Ok(_) => Ok(()),
                        Err(e) => Err(format!("Failed to write to file: {}", e)),
                    }
                }
                Err(e) => Err(format!("Failed to truncate file: {}", e)),
            },
            Err(e) => Err(format!("Failed to seek in file: {}", e)),
        }
    }

    #[allow(dead_code)]
    async fn close_file(&self, handle_id: &str) -> Result<(), String> {
        let mut file_handles = self.file_handles.lock().await;

        if !file_handles.contains_key(handle_id) {
            return Err(format!("Invalid file handle: {}", handle_id));
        }

        file_handles.remove(handle_id);
        Ok(())
    }
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
            started: Instant::now(),
            max_duration: Duration::from_secs(u64::MAX), // Effectively no timeout by default
            call_stack: RefCell::new(Vec::new()),
            io_client: Rc::new(IoClient::new()),
        }
    }

    pub fn with_timeout(seconds: u64) -> Self {
        let mut interpreter = Self::new();
        interpreter.started = Instant::now();
        interpreter.max_duration = Duration::from_secs(seconds);
        interpreter
    }

    pub fn get_call_stack(&self) -> Vec<CallFrame> {
        self.call_stack.borrow().clone()
    }

    pub fn clear_call_stack(&self) {
        self.call_stack.borrow_mut().clear();
    }
    pub fn global_env(&self) -> &Rc<RefCell<Environment>> {
        &self.global_env
    }

    fn check_time(&self) -> Result<(), RuntimeError> {
        if self.started.elapsed() > self.max_duration {
            if *self.in_count_loop.borrow() {
                *self.in_count_loop.borrow_mut() = false;
                *self.current_count.borrow_mut() = None;
            }

            Err(RuntimeError::new(
                format!(
                    "Execution exceeded timeout ({}s)",
                    self.max_duration.as_secs()
                ),
                0,
                0,
            ))
        } else {
            Ok(())
        }
    }

    fn assert_invariants(&self) {
        debug_assert_eq!(
            *self.in_count_loop.borrow(),
            self.current_count.borrow().is_some()
        );

        debug_assert!(self.call_stack.borrow().len() < 10_000);
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

    pub async fn interpret(&mut self, program: &Program) -> Result<Value, Vec<RuntimeError>> {
        self.assert_invariants();
        self.call_stack.borrow_mut().clear();

        let mut last_value = Value::Null;
        let mut errors = Vec::new();

        for statement in &program.statements {
            if let Err(err) = self.check_time() {
                errors.push(err);
                return Err(errors);
            }

            match self
                .execute_statement(statement, Rc::clone(&self.global_env))
                .await
            {
                Ok((value, control)) => {
                    last_value = value;
                    if let LoopControl::Exit { line, column } = control {
                        // Convert unhandled Exit to runtime error
                        let err = RuntimeError::new(
                            "`exit loop` used outside of any loop".to_string(),
                            line,
                            column,
                        );
                        errors.push(err);
                        break;
                    } else if matches!(control, LoopControl::Break | LoopControl::Continue) {
                        // Convert unhandled Break/Continue to runtime error
                        let msg = if matches!(control, LoopControl::Break) {
                            "`break` used outside of any loop".to_string()
                        } else {
                            "`continue` used outside of any loop".to_string()
                        };
                        
                        let (line, column) = match control {
                            LoopControl::Break => (0, 0), // These should be stored in the enum
                            LoopControl::Continue => (0, 0), // These should be stored in the enum
                            LoopControl::Exit { line, column } => (line, column),
                            _ => (0, 0), // Fallback
                        };
                        
                        let err = RuntimeError::new(
                            msg,
                            line,
                            column,
                        );
                        errors.push(err);
                        break;
                    }
                },
                Err(err) => {
                    errors.push(err);
                    break; // Stop on first runtime error
                }
            }
        }

        if errors.is_empty() {
            let main_func_opt = {
                if let Some(Value::Function(main_func)) = self.global_env.borrow().get("main") {
                    Some(main_func.clone())
                } else {
                    None
                }
            };

            if let Some(main_func) = main_func_opt {
                match self.call_function(&main_func, vec![], 0, 0).await {
                    Ok(value) => last_value = value,
                    Err(err) => {
                        errors.push(err);
                    }
                }
            }

            self.assert_invariants();
            Ok(last_value)
        } else {
            self.assert_invariants();
            Err(errors)
        }
    }

    async fn execute_statement(
        &self,
        stmt: &Statement,
        env: Rc<RefCell<Environment>>,
    ) -> Result<(Value, LoopControl), RuntimeError> {
        Box::pin(self._execute_statement(stmt, env)).await
    }

    async fn _execute_statement(
        &self,
        stmt: &Statement,
        env: Rc<RefCell<Environment>>,
    ) -> Result<(Value, LoopControl), RuntimeError> {
        self.check_time()?;

        match stmt {
            Statement::VariableDeclaration {
                name,
                value,
                line: _,
                column: _,
            } => {
                let (value, control) = self.evaluate_expression(value, Rc::clone(&env)).await?;
                if control != LoopControl::None {
                    return Ok((Value::Null, control));
                }
                env.borrow_mut().define(name, value);
                Ok((Value::Null, LoopControl::None))
            }

            Statement::Assignment {
                name,
                value,
                line,
                column,
            } => {
                let (value, control) = self.evaluate_expression(value, Rc::clone(&env)).await?;
                
                if control != LoopControl::None {
                    return Ok((Value::Null, control));
                }
                
                match env.borrow_mut().assign(name, value) {
                    Ok(_) => Ok((Value::Null, LoopControl::None)),
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
                let (condition_value, control) = self.evaluate_expression(condition, Rc::clone(&env)).await?;
                
                if control != LoopControl::None {
                    return Ok((Value::Null, control));
                }

                if condition_value.is_truthy() {
                    self.execute_block(then_block, Rc::clone(&env)).await
                } else if let Some(else_stmts) = else_block {
                    self.execute_block(else_stmts, Rc::clone(&env)).await
                } else {
                    Ok((Value::Null, LoopControl::None))
                }
            }

            Statement::SingleLineIf {
                condition,
                then_stmt,
                else_stmt,
                line: _line,
                column: _column,
            } => {
                let (condition_value, control) = self.evaluate_expression(condition, Rc::clone(&env)).await?;
                
                if control != LoopControl::None {
                    return Ok((Value::Null, control));
                }

                if condition_value.is_truthy() {
                    self.execute_statement(then_stmt, Rc::clone(&env)).await
                } else if let Some(else_stmt) = else_stmt {
                    self.execute_statement(else_stmt, Rc::clone(&env)).await
                } else {
                    Ok((Value::Null, LoopControl::None))
                }
            }

            Statement::DisplayStatement {
                value,
                line: _line,
                column: _column,
            } => {
                let (value, control) = self.evaluate_expression(value, Rc::clone(&env)).await?;
                if control != LoopControl::None {
                    return Ok((Value::Null, control));
                }
                println!("{}", value);
                Ok((Value::Null, LoopControl::None))
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
                    env: Rc::downgrade(&env),
                    line: *line,
                    column: *column,
                };

                let function_value = Value::Function(Rc::new(function));
                env.borrow_mut().define(name, function_value.clone());

                Ok((function_value, LoopControl::None))
            }

            Statement::ReturnStatement {
                value,
                line: _line,
                column: _column,
            } => {
                if let Some(expr) = value {
                    let (val, _) = self.evaluate_expression(expr, Rc::clone(&env)).await?;
                    Ok((val, LoopControl::None))
                } else {
                    Ok((Value::Null, LoopControl::None))
                }
            }

            Statement::ExpressionStatement {
                expression,
                line: _line,
                column: _column,
            } => {
                let (val, control) = self.evaluate_expression(expression, Rc::clone(&env)).await?;
                Ok((val, control))
            },

            Statement::CountLoop {
                start,
                end,
                step,
                downward,
                body,
                line,
                column,
            } => {
                let (start_val, control1) = self.evaluate_expression(start, Rc::clone(&env)).await?;
                if control1 != LoopControl::None {
                    return Ok((Value::Null, control1));
                }
                
                let (end_val, control2) = self.evaluate_expression(end, Rc::clone(&env)).await?;
                if control2 != LoopControl::None {
                    return Ok((Value::Null, control2));
                }

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
                    let (step_val, control3) = self.evaluate_expression(step_expr, Rc::clone(&env)).await?;
                    if control3 != LoopControl::None {
                        return Ok((Value::Null, control3));
                    }
                    
                    match step_val {
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
                let loop_env = Environment::new_child_env(&env);

                let should_continue: Box<dyn Fn(f64, f64) -> bool> = if *downward {
                    Box::new(|count, end_num| count >= end_num)
                } else {
                    Box::new(|count, end_num| count <= end_num)
                };

                let max_iterations = if end_num > 1000000.0 {
                    u64::MAX // Effectively no limit for large end values, rely on timeout instead
                } else {
                    10000 // Reasonable limit for normal loops
                };
                let mut iterations = 0;

                *self.in_count_loop.borrow_mut() = true;

                while should_continue(count, end_num) && iterations < max_iterations {
                    self.check_time()?;

                    *self.current_count.borrow_mut() = Some(count);

                    match self.execute_block(body, Rc::clone(&loop_env)).await {
                        Ok((_, control)) => {
                            if matches!(control, LoopControl::Break) {
                                break;
                            } else if let LoopControl::Exit { line, column } = control {
                                *self.in_count_loop.borrow_mut() = false;
                                *self.current_count.borrow_mut() = None;
                                return Ok((Value::Null, LoopControl::Exit { line, column }));
                            }else if matches!(control, LoopControl::Continue) {
                            }
                        }
                        Err(e) => {
                            *self.in_count_loop.borrow_mut() = false;
                            *self.current_count.borrow_mut() = None;
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

                Ok((Value::Null, LoopControl::None))
            }

            Statement::ForEachLoop {
                item_name,
                collection,
                reversed,
                body,
                line,
                column,
            } => {
                let (collection_val, control) = self
                    .evaluate_expression(collection, Rc::clone(&env))
                    .await?;
                
                if control != LoopControl::None {
                    return Ok((Value::Null, control));
                }

                let loop_env = Environment::new_child_env(&env);

                match &collection_val {
                    Value::List(list_rc) => {
                        let items: Vec<Value> = {
                            let list = list_rc.borrow();
                            let indices: Vec<usize> = if *reversed {
                                (0..list.len()).rev().collect()
                            } else {
                                (0..list.len()).collect()
                            };
                            indices.iter().map(|&i| list[i].clone()).collect()
                        };

                        for item in items {
                            loop_env.borrow_mut().define(item_name, item);
                            match self.execute_block(body, Rc::clone(&loop_env)).await {
                                Ok((_, control)) => {
                                    if matches!(control, LoopControl::Break) {
                                        break;
                                    } else if let LoopControl::Exit { line, column } = control {
                                        return Ok((Value::Null, LoopControl::Exit { line, column }));
                                    }else if matches!(control, LoopControl::Continue) {
                                        continue;
                                    }
                                }
                                Err(e) => return Err(e),
                            }
                        }
                    }
                    Value::Object(obj_rc) => {
                        let items: Vec<(String, Value)> = {
                            let obj = obj_rc.borrow();
                            obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
                        };

                        for (_, value) in items {
                            loop_env.borrow_mut().define(item_name, value);
                            match self.execute_block(body, Rc::clone(&loop_env)).await {
                                Ok((_, control)) => {
                                    if matches!(control, LoopControl::Break) {
                                        break;
                                    } else if let LoopControl::Exit { line, column } = control {
                                        return Ok((Value::Null, LoopControl::Exit { line, column }));
                                    }else if matches!(control, LoopControl::Continue) {
                                        continue;
                                    }
                                }
                                Err(e) => return Err(e),
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

                Ok((Value::Null, LoopControl::None))
            }

            Statement::WhileLoop {
                condition,
                body,
                line: _line,
                column: _column,
            } => {
                loop {
                    let (condition_value, control) = self.evaluate_expression(condition, Rc::clone(&env)).await?;
                    
                    if control != LoopControl::None {
                        return Ok((Value::Null, control));
                    }
                    
                    if !condition_value.is_truthy() {
                        break;
                    }
                    self.check_time()?;
                    match self.execute_block(body, Rc::clone(&env)).await {
                        Ok((_, control)) => {
                            if matches!(control, LoopControl::Break) {
                                break;
                            } else if let LoopControl::Exit { line, column } = control {
                                return Ok((Value::Null, LoopControl::Exit { line, column }));
                            } else if matches!(control, LoopControl::Continue) {
                                continue;
                            }
                        }
                        Err(e) => return Err(e),
                    }
                }
                Ok((Value::Null, LoopControl::None))
            }

            Statement::RepeatUntilLoop {
                condition,
                body,
                line: _line,
                column: _column,
            } => {
                loop {
                    self.check_time()?;
                    match self.execute_block(body, Rc::clone(&env)).await {
                        Ok((_, control)) => {
                            if matches!(control, LoopControl::Break) {
                                break;
                            } else if let LoopControl::Exit { line, column } = control {
                                return Ok((Value::Null, LoopControl::Exit { line, column }));
                            } else if matches!(control, LoopControl::Continue) {
                                continue;
                            }
                        }
                        Err(e) => return Err(e),
                    }
                    let (condition_value, control) = self.evaluate_expression(condition, Rc::clone(&env)).await?;
                    
                    if control != LoopControl::None {
                        return Ok((Value::Null, control));
                    }
                    
                    if condition_value.is_truthy() {
                        break;
                    }
                }
                Ok((Value::Null, LoopControl::None))
            }

            Statement::ForeverLoop {
                body,
                line: _line,
                column: _column,
            } => {
                loop {
                    self.check_time()?;
                    match self.execute_block(body, Rc::clone(&env)).await {
                        Ok((_, control)) => {
                            if matches!(control, LoopControl::Break) {
                                break;
                            } else if let LoopControl::Exit { line, column } = control {
                                return Ok((Value::Null, LoopControl::Exit { line, column }));
                            } else if matches!(control, LoopControl::Continue) {
                                continue;
                            }
                        }
                        Err(e) => return Err(e),
                    }
                }
                #[allow(unreachable_code)]
                Ok((Value::Null, LoopControl::None))
            }

            Statement::BreakStatement { .. } => {
                Ok((Value::Null, LoopControl::Break))
            }
            Statement::ContinueStatement { .. } => {
                Ok((Value::Null, LoopControl::Continue))
            }
            Statement::ExitLoopStatement { line, column } => {
                Ok((Value::Null, LoopControl::Exit {
                    line: *line,
                    column: *column,
                }))
            }

            Statement::OpenFileStatement {
                path,
                variable_name,
                line,
                column,
            } => {
                let (path_value, path_control) = self.evaluate_expression(path, Rc::clone(&env)).await?;
                
                if path_control != LoopControl::None {
                    return Ok((Value::Null, path_control));
                }
                
                let path_str = match &path_value {
                    Value::Text(s) => s.clone(),
                    _ => {
                        return Err(RuntimeError::new(
                            format!("Expected string for file path, got {:?}", path_value),
                            *line,
                            *column,
                        ));
                    }
                };

                match self.io_client.open_file(&path_str).await {
                    Ok(handle) => {
                        env.borrow_mut()
                            .define(variable_name, Value::Text(handle.into()));
                        Ok((Value::Null, LoopControl::None))
                    }
                    Err(e) => Err(RuntimeError::new(e, *line, *column)),
                }
            }
            Statement::ReadFileStatement {
                path,
                variable_name,
                line,
                column,
            } => {
                let (path_value, path_control) = self.evaluate_expression(path, Rc::clone(&env)).await?;
                
                if path_control != LoopControl::None {
                    return Ok((Value::Null, path_control));
                }
                
                let handle = match &path_value {
                    Value::Text(s) => s.clone(),
                    _ => {
                        return Err(RuntimeError::new(
                            format!("Expected string for file handle, got {:?}", path_value),
                            *line,
                            *column,
                        ));
                    }
                };

                match self.io_client.read_file(&handle).await {
                    Ok(content) => {
                        env.borrow_mut()
                            .define(variable_name, Value::Text(content.into()));
                        Ok((Value::Null, LoopControl::None))
                    }
                    Err(e) => Err(RuntimeError::new(e, *line, *column)),
                }
            }
            Statement::WriteFileStatement {
                file,
                content,
                line,
                column,
            } => {
                let (file_value, file_control) = self.evaluate_expression(file, Rc::clone(&env)).await?;
                
                if file_control != LoopControl::None {
                    return Ok((Value::Null, file_control));
                }
                
                let (content_value, content_control) = self.evaluate_expression(content, Rc::clone(&env)).await?;
                
                if content_control != LoopControl::None {
                    return Ok((Value::Null, content_control));
                }

                let file_str = match &file_value {
                    Value::Text(s) => s.clone(),
                    _ => {
                        return Err(RuntimeError::new(
                            format!("Expected string for file handle, got {:?}", file_value),
                            *line,
                            *column,
                        ));
                    }
                };

                let content_str = match &content_value {
                    Value::Text(s) => s.clone(),
                    _ => {
                        return Err(RuntimeError::new(
                            format!("Expected string for file content, got {:?}", content_value),
                            *line,
                            *column,
                        ));
                    }
                };

                match self.io_client.write_file(&file_str, &content_str).await {
                    Ok(_) => Ok((Value::Null, LoopControl::None)),
                    Err(e) => Err(RuntimeError::new(e, *line, *column)),
                }
            }
            Statement::CloseFileStatement { file, line, column } => {
                let (file_value, control) = self.evaluate_expression(file, Rc::clone(&env)).await?;
                if control != LoopControl::None {
                    return Ok((Value::Null, control));
                }

                let file_str = match &file_value {
                    Value::Text(s) => s.clone(),
                    _ => {
                        return Err(RuntimeError::new(
                            format!("Expected string for file handle, got {:?}", file_value),
                            *line,
                            *column,
                        ));
                    }
                };

                match self.io_client.close_file(&file_str).await {
                    Ok(_) => Ok((Value::Null, LoopControl::None)),
                    Err(e) => Err(RuntimeError::new(e, *line, *column)),
                }
            }
            Statement::WaitForStatement {
                inner,
                line: _,
                column: _,
            } => self.execute_statement(inner, Rc::clone(&env)).await,
            Statement::TryStatement {
                body,
                error_name,
                when_block,
                otherwise_block,
                line: _line,
                column: _column,
            } => {
                let child_env = Environment::new_child_env(&env);

                match self.exec_block_ignore_control(body, Rc::clone(&child_env)).await {
                    Ok(val) => Ok((val, LoopControl::None)), // Success path: just bubble result
                    Err(err) => {
                        child_env
                            .borrow_mut()
                            .define(error_name, Value::Text(err.message.into()));

                        let result = self.exec_block_ignore_control(when_block, Rc::clone(&child_env)).await;

                        if result.is_ok() || otherwise_block.is_none() {
                            Ok((result?, LoopControl::None))
                        } else {
                            let val = self.exec_block_ignore_control(otherwise_block.as_ref().unwrap(), child_env).await?;
                            Ok((val, LoopControl::None))
                        }
                    }
                }
            }
            Statement::HttpGetStatement {
                url,
                variable_name,
                line,
                column,
            } => {
                let (url_val, control) = self.evaluate_expression(url, Rc::clone(&env)).await?;
                if control != LoopControl::None {
                    return Ok((Value::Null, control));
                }
                
                let url_str = match &url_val {
                    Value::Text(s) => s.clone(),
                    _ => {
                        return Err(RuntimeError::new(
                            format!("Expected string for URL, got {:?}", url_val),
                            *line,
                            *column,
                        ));
                    }
                };

                match self.io_client.http_get(&url_str).await {
                    Ok(body) => {
                        env.borrow_mut()
                            .define(variable_name, Value::Text(body.into()));
                        Ok((Value::Null, LoopControl::None))
                    }
                    Err(e) => Err(RuntimeError::new(e, *line, *column)),
                }
            }
            Statement::HttpPostStatement {
                url,
                data,
                variable_name,
                line,
                column,
            } => {
                let (url_val, control1) = self.evaluate_expression(url, Rc::clone(&env)).await?;
                if control1 != LoopControl::None {
                    return Ok((Value::Null, control1));
                }
                
                let (data_val, control2) = self.evaluate_expression(data, Rc::clone(&env)).await?;
                if control2 != LoopControl::None {
                    return Ok((Value::Null, control2));
                }

                let url_str = match &url_val {
                    Value::Text(s) => s.clone(),
                    _ => {
                        return Err(RuntimeError::new(
                            format!("Expected string for URL, got {:?}", url_val),
                            *line,
                            *column,
                        ));
                    }
                };

                let data_str = match &data_val {
                    Value::Text(s) => s.clone(),
                    _ => {
                        return Err(RuntimeError::new(
                            format!("Expected string for data, got {:?}", data_val),
                            *line,
                            *column,
                        ));
                    }
                };

                match self.io_client.http_post(&url_str, &data_str).await {
                    Ok(body) => {
                        env.borrow_mut()
                            .define(variable_name, Value::Text(body.into()));
                        Ok((Value::Null, LoopControl::None))
                    }
                    Err(e) => Err(RuntimeError::new(e, *line, *column)),
                }
            }
        }
    }

    async fn execute_block(
        &self,
        statements: &[Statement],
        env: Rc<RefCell<Environment>>,
    ) -> Result<(Value, LoopControl), RuntimeError> {
        Box::pin(self._execute_block(statements, env)).await
    }
    
    async fn exec_block_ignore_control(
        &self,
        statements: &[Statement],
        env: Rc<RefCell<Environment>>,
    ) -> Result<Value, RuntimeError> {
        match self.execute_block(statements, env).await {
            Ok((value, _)) => Ok(value), // Discard the LoopControl
            Err(e) => Err(e),
        }
    }

    async fn _execute_block(
        &self,
        statements: &[Statement],
        env: Rc<RefCell<Environment>>,
    ) -> Result<(Value, LoopControl), RuntimeError> {
        self.assert_invariants();
        let mut last_value = Value::Null;

        for statement in statements {
            match self.execute_statement(statement, Rc::clone(&env)).await {
                Ok((value, control)) => {
                    last_value = value;
                    if matches!(control, LoopControl::Break | LoopControl::Continue | LoopControl::Exit { line: _, column: _ }) {
                        return Ok((last_value, control));
                    }
                }
                Err(e) => return Err(e),
            }
        }

        self.assert_invariants();
        Ok((last_value, LoopControl::None))
    }

    async fn evaluate_expression(
        &self,
        expr: &Expression,
        env: Rc<RefCell<Environment>>,
    ) -> Result<(Value, LoopControl), RuntimeError> {
        Box::pin(self._evaluate_expression(expr, env)).await
    }

    async fn _evaluate_expression(
        &self,
        expr: &Expression,
        env: Rc<RefCell<Environment>>,
    ) -> Result<(Value, LoopControl), RuntimeError> {
        self.assert_invariants();
        self.check_time()?;

        let result = match expr {
            &Expression::AwaitExpression {
                ref expression,
                line: _line,
                column: _column,
            } => {
                let (value, control) = self
                    .evaluate_expression(expression, Rc::clone(&env))
                    .await?;
                if control != LoopControl::None {
                    return Ok((value, control));
                }
                Ok((value, LoopControl::None))
            }
            Expression::Literal(literal, _line, _column) => match literal {
                Literal::String(s) => Ok((Value::Text(Rc::from(s.as_str())), LoopControl::None)),
                Literal::Integer(i) => Ok((Value::Number(*i as f64), LoopControl::None)),
                Literal::Float(f) => Ok((Value::Number(*f), LoopControl::None)),
                Literal::Boolean(b) => Ok((Value::Bool(*b), LoopControl::None)),
                Literal::Nothing => Ok((Value::Null, LoopControl::None)),
                Literal::Pattern(s) => Ok((Value::Text(Rc::from(s.as_str())), LoopControl::None)),
            },

            Expression::Variable(name, line, column) => {
                if name == "count" {
                    if let Some(count_value) = *self.current_count.borrow() {
                        return Ok((Value::Number(count_value), LoopControl::None));
                    } else {
                        println!(
                            "Warning: Using 'count' outside of a count loop context at line {}, column {}",
                            line, column
                        );
                        return Ok((Value::Number(0.0), LoopControl::None));
                    }
                }

                if let Some(value) = env.borrow().get(name) {
                    Ok((value, LoopControl::None))
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
                let left_val = self.evaluate_expression(left, Rc::clone(&env)).await?;
                let right_val = self.evaluate_expression(right, Rc::clone(&env)).await?;

                match operator {
                    Operator::Plus => self.add(left_val, right_val, *line, *column),
                    Operator::Minus => self.subtract(left_val, right_val, *line, *column),
                    Operator::Multiply => self.multiply(left_val, right_val, *line, *column),
                    Operator::Divide => self.divide(left_val, right_val, *line, *column),
                    Operator::Equals => Ok((Value::Bool(self.is_equal(&left_val.0, &right_val.0)), LoopControl::None)),
                    Operator::NotEquals => Ok((Value::Bool(!self.is_equal(&left_val.0, &right_val.0)), LoopControl::None)),
                    Operator::GreaterThan => self.greater_than(left_val, right_val, *line, *column),
                    Operator::LessThan => self.less_than(left_val, right_val, *line, *column),
                    Operator::GreaterThanOrEqual => {
                        self.greater_than_equal(left_val, right_val, *line, *column)
                    }
                    Operator::LessThanOrEqual => {
                        self.less_than_equal(left_val, right_val, *line, *column)
                    }
                    Operator::And => {
                        let (left, left_control) = left_val;
                        let (right, right_control) = right_val;
                        
                        if left_control != LoopControl::None {
                            return Ok((Value::Null, left_control));
                        }
                        if right_control != LoopControl::None {
                            return Ok((Value::Null, right_control));
                        }
                        
                        Ok((Value::Bool(left.is_truthy() && right.is_truthy()), LoopControl::None))
                    },
                    Operator::Or => {
                        let (left, left_control) = left_val;
                        let (right, right_control) = right_val;
                        
                        if left_control != LoopControl::None {
                            return Ok((Value::Null, left_control));
                        }
                        if right_control != LoopControl::None {
                            return Ok((Value::Null, right_control));
                        }
                        
                        Ok((Value::Bool(left.is_truthy() || right.is_truthy()), LoopControl::None))
                    },
                    Operator::Contains => self.contains(left_val, right_val, *line, *column),
                }
            }

            Expression::UnaryOperation {
                operator,
                expression,
                line,
                column,
            } => {
                let (value, control) = self
                    .evaluate_expression(expression, Rc::clone(&env))
                    .await?;
                
                if control != LoopControl::None {
                    return Ok((Value::Null, control));
                }

                match operator {
                    UnaryOperator::Not => Ok((Value::Bool(!value.is_truthy()), LoopControl::None)),
                    UnaryOperator::Minus => match value {
                        Value::Number(n) => Ok((Value::Number(-n), LoopControl::None)),
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
                let (function_val, function_control) = self.evaluate_expression(function, Rc::clone(&env)).await?;
                
                if function_control != LoopControl::None {
                    return Ok((Value::Null, function_control));
                }

                let mut arg_values = Vec::new();
                let mut has_control_flow = false;
                let mut control_to_propagate = LoopControl::None;
                
                for arg in arguments {
                    let (arg_val, arg_control) = self.evaluate_expression(&arg.value, Rc::clone(&env)).await?;
                    
                    if arg_control != LoopControl::None {
                        has_control_flow = true;
                        control_to_propagate = arg_control;
                        break;
                    }
                    
                    arg_values.push(arg_val);
                }
                
                if has_control_flow {
                    return Ok((Value::Null, control_to_propagate));
                }

                match function_val {
                    Value::Function(func) => {
                        let result = self.call_function(&func, arg_values, *line, *column).await?;
                        Ok((result, LoopControl::None))
                    }
                    Value::NativeFunction(native_fn) => {
                        let result = native_fn(arg_values).map_err(|e| {
                            RuntimeError::new(
                                format!("Error in native function: {}", e),
                                *line,
                                *column,
                            )
                        })?;
                        Ok((result, LoopControl::None))
                    }
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
                let object_val = self.evaluate_expression(object, Rc::clone(&env)).await?;

                match object_val {
                    (Value::Object(obj_rc), _) => {
                        let obj = obj_rc.borrow();
                        if let Some(value) = obj.get(property) {
                            Ok((value.clone(), LoopControl::None))
                        } else {
                            Err(RuntimeError::new(
                                format!("Object has no property '{}'", property),
                                *line,
                                *column,
                            ))
                        }
                    }
                    (other_val, _) => Err(RuntimeError::new(
                        format!("Cannot access property of {}", other_val.type_name()),
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
                let collection_val = self
                    .evaluate_expression(collection, Rc::clone(&env))
                    .await?;
                let index_val = self.evaluate_expression(index, Rc::clone(&env)).await?;

                match (collection_val, index_val) {
                    ((Value::List(list_rc), _), (Value::Number(idx), _)) => {
                        let list = list_rc.borrow();
                        let idx = idx as usize;

                        if idx < list.len() {
                            Ok((list[idx].clone(), LoopControl::None))
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
                    ((Value::Object(obj_rc), _), (Value::Text(key), _)) => {
                        let obj = obj_rc.borrow();
                        let key_str = key.to_string();

                        if let Some(value) = obj.get(&key_str) {
                            Ok((value.clone(), LoopControl::None))
                        } else {
                            Err(RuntimeError::new(
                                format!("Object has no key '{}'", key_str),
                                *line,
                                *column,
                            ))
                        }
                    }
                    ((collection, _), (index, _)) => Err(RuntimeError::new(
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
                let (left_val, _) = self.evaluate_expression(left, Rc::clone(&env)).await?;

                let (right_val, _) = match right.as_ref() {
                    Expression::Variable(name, _, _) if name == "count" => {
                        if let Some(count_value) = *self.current_count.borrow() {
                            (Value::Number(count_value), LoopControl::None)
                        } else {
                            self.evaluate_expression(right, Rc::clone(&env)).await?
                        }
                    }
                    _ => self.evaluate_expression(right, Rc::clone(&env)).await?,
                };

                let result = format!("{}{}", left_val, right_val);
                Ok((Value::Text(Rc::from(result.as_str())), LoopControl::None))
            }

            Expression::PatternMatch {
                text,
                pattern,
                line: _line,
                column: _column,
            } => {
                let (text_val, text_control) = self.evaluate_expression(text, Rc::clone(&env)).await?;
                let (pattern_val, pattern_control) = self.evaluate_expression(pattern, Rc::clone(&env)).await?;
                
                if text_control != LoopControl::None {
                    return Ok((Value::Null, text_control));
                }
                if pattern_control != LoopControl::None {
                    return Ok((Value::Null, pattern_control));
                }

                let args = vec![text_val, pattern_val];
                let result = crate::stdlib::pattern::native_pattern_matches(args)?;
                Ok((result, LoopControl::None))
            }

            Expression::PatternFind {
                text,
                pattern,
                line: _line,
                column: _column,
            } => {
                let (text_val, text_control) = self.evaluate_expression(text, Rc::clone(&env)).await?;
                let (pattern_val, pattern_control) = self.evaluate_expression(pattern, Rc::clone(&env)).await?;
                
                if text_control != LoopControl::None {
                    return Ok((Value::Null, text_control));
                }
                if pattern_control != LoopControl::None {
                    return Ok((Value::Null, pattern_control));
                }

                let args = vec![pattern_val, text_val]; // Note: pattern first, then text
                let result = crate::stdlib::pattern::native_pattern_find(args)?;
                Ok((result, LoopControl::None))
            }

            Expression::PatternReplace {
                text,
                pattern,
                replacement,
                line: _line,
                column: _column,
            } => {
                let (text_val, text_control) = self.evaluate_expression(text, Rc::clone(&env)).await?;
                let (pattern_val, pattern_control) = self.evaluate_expression(pattern, Rc::clone(&env)).await?;
                let (replacement_val, replacement_control) = self
                    .evaluate_expression(replacement, Rc::clone(&env))
                    .await?;
                
                if text_control != LoopControl::None {
                    return Ok((Value::Null, text_control));
                }
                if pattern_control != LoopControl::None {
                    return Ok((Value::Null, pattern_control));
                }
                if replacement_control != LoopControl::None {
                    return Ok((Value::Null, replacement_control));
                }

                let args = vec![pattern_val, replacement_val, text_val]; // Note: pattern, replacement, then text
                let result = crate::stdlib::pattern::native_pattern_replace(args)?;
                Ok((result, LoopControl::None))
            }

            Expression::PatternSplit {
                text,
                pattern,
                line: _line,
                column: _column,
            } => {
                let (text_val, text_control) = self.evaluate_expression(text, Rc::clone(&env)).await?;
                let (pattern_val, pattern_control) = self.evaluate_expression(pattern, Rc::clone(&env)).await?;
                
                if text_control != LoopControl::None {
                    return Ok((Value::Null, text_control));
                }
                if pattern_control != LoopControl::None {
                    return Ok((Value::Null, pattern_control));
                }

                let args = vec![text_val, pattern_val];
                let result = crate::stdlib::pattern::native_pattern_split(args)?;
                Ok((result, LoopControl::None))
            }
        };
        self.assert_invariants();
        result
    }

    async fn call_function(
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

        let func_env = match func.env.upgrade() {
            Some(env) => env,
            None => {
                return Err(RuntimeError::with_kind(
                    "Environment no longer exists".to_string(),
                    line,
                    column,
                    ErrorKind::EnvDropped,
                ));
            }
        };

        let call_env = Environment::new_child_env(&func_env);

        for (param, arg) in func.params.iter().zip(args) {
            call_env.borrow_mut().define(param, arg);
        }

        let frame = CallFrame::new(
            func.name
                .clone()
                .unwrap_or_else(|| "<anonymous>".to_string()),
            line,
            column,
        );
        self.call_stack.borrow_mut().push(frame);

        let result = self.execute_block(&func.body, call_env.clone()).await;

        match result {
            Ok((value, _control)) => {
                self.call_stack.borrow_mut().pop();
                Ok(value)
            }
            Err(err) => {
                if let Some(last_frame) = self.call_stack.borrow_mut().last_mut() {
                    last_frame.capture_locals(&call_env);
                }

                let error_with_stack = err.clone();

                self.call_stack.borrow_mut().pop();

                Err(error_with_stack)
            }
        }
    }

    fn add(
        &self,
        left: (Value, LoopControl),
        right: (Value, LoopControl),
        line: usize,
        column: usize,
    ) -> Result<(Value, LoopControl), RuntimeError> {
        match (left, right) {
            ((Value::Number(a), _), (Value::Number(b), _)) => Ok((Value::Number(a + b), LoopControl::None)),
            ((Value::Text(a), _), (Value::Text(b), _)) => {
                let result = format!("{}{}", a, b);
                Ok((Value::Text(Rc::from(result.as_str())), LoopControl::None))
            }
            ((Value::Text(a), _), (b, _)) => {
                let result = format!("{}{}", a, b);
                Ok((Value::Text(Rc::from(result.as_str())), LoopControl::None))
            }
            ((a, _), (Value::Text(b), _)) => {
                let result = format!("{}{}", a, b);
                Ok((Value::Text(Rc::from(result.as_str())), LoopControl::None))
            }
            ((a, _), (b, _)) => Err(RuntimeError::new(
                format!("Cannot add {} and {}", a.type_name(), b.type_name()),
                line,
                column,
            )),
        }
    }

    fn subtract(
        &self,
        left: (Value, LoopControl),
        right: (Value, LoopControl),
        line: usize,
        column: usize,
    ) -> Result<(Value, LoopControl), RuntimeError> {
        match (left, right) {
            ((Value::Number(a), _), (Value::Number(b), _)) => Ok((Value::Number(a - b), LoopControl::None)),
            ((a, _), (b, _)) => Err(RuntimeError::new(
                format!("Cannot subtract {} from {}", b.type_name(), a.type_name()),
                line,
                column,
            )),
        }
    }

    fn multiply(
        &self,
        left: (Value, LoopControl),
        right: (Value, LoopControl),
        line: usize,
        column: usize,
    ) -> Result<(Value, LoopControl), RuntimeError> {
        match (left, right) {
            ((Value::Number(a), _), (Value::Number(b), _)) => Ok((Value::Number(a * b), LoopControl::None)),
            ((a, _), (b, _)) => Err(RuntimeError::new(
                format!("Cannot multiply {} and {}", a.type_name(), b.type_name()),
                line,
                column,
            )),
        }
    }

    fn divide(
        &self,
        left: (Value, LoopControl),
        right: (Value, LoopControl),
        line: usize,
        column: usize,
    ) -> Result<(Value, LoopControl), RuntimeError> {
        match (left, right) {
            ((Value::Number(a), _), (Value::Number(b), _)) => {
                if b == 0.0 {
                    Err(RuntimeError::new(
                        "Division by zero".to_string(),
                        line,
                        column,
                    ))
                } else {
                    Ok((Value::Number(a / b), LoopControl::None))
                }
            }
            ((a, _), (b, _)) => Err(RuntimeError::new(
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
        left: (Value, LoopControl),
        right: (Value, LoopControl),
        line: usize,
        column: usize,
    ) -> Result<(Value, LoopControl), RuntimeError> {
        match (left, right) {
            ((Value::Number(a), _), (Value::Number(b), _)) => Ok((Value::Bool(a > b), LoopControl::None)),
            ((Value::Text(a), _), (Value::Text(b), _)) => Ok((Value::Bool(a > b), LoopControl::None)),
            ((a, _), (b, _)) => Err(RuntimeError::new(
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
        left: (Value, LoopControl),
        right: (Value, LoopControl),
        line: usize,
        column: usize,
    ) -> Result<(Value, LoopControl), RuntimeError> {
        match (left, right) {
            ((Value::Number(a), _), (Value::Number(b), _)) => Ok((Value::Bool(a < b), LoopControl::None)),
            ((Value::Text(a), _), (Value::Text(b), _)) => Ok((Value::Bool(a < b), LoopControl::None)),
            ((a, _), (b, _)) => Err(RuntimeError::new(
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
        left: (Value, LoopControl),
        right: (Value, LoopControl),
        line: usize,
        column: usize,
    ) -> Result<(Value, LoopControl), RuntimeError> {
        match (left, right) {
            ((Value::Number(a), _), (Value::Number(b), _)) => Ok((Value::Bool(a >= b), LoopControl::None)),
            ((Value::Text(a), _), (Value::Text(b), _)) => Ok((Value::Bool(a >= b), LoopControl::None)),
            ((a, _), (b, _)) => Err(RuntimeError::new(
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
        left: (Value, LoopControl),
        right: (Value, LoopControl),
        line: usize,
        column: usize,
    ) -> Result<(Value, LoopControl), RuntimeError> {
        match (left, right) {
            ((Value::Number(a), _), (Value::Number(b), _)) => Ok((Value::Bool(a <= b), LoopControl::None)),
            ((Value::Text(a), _), (Value::Text(b), _)) => Ok((Value::Bool(a <= b), LoopControl::None)),
            ((a, _), (b, _)) => Err(RuntimeError::new(
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
        left: (Value, LoopControl),
        right: (Value, LoopControl),
        line: usize,
        column: usize,
    ) -> Result<(Value, LoopControl), RuntimeError> {
        match (left, right) {
            ((Value::List(list_rc), _), (item, _)) => {
                let list = list_rc.borrow();
                for value in list.iter() {
                    if self.is_equal(value, &item) {
                        return Ok((Value::Bool(true), LoopControl::None));
                    }
                }
                Ok((Value::Bool(false), LoopControl::None))
            }
            ((Value::Object(obj_rc), _), (Value::Text(key), _)) => {
                let obj = obj_rc.borrow();
                Ok((Value::Bool(obj.contains_key(&key.to_string())), LoopControl::None))
            }
            ((Value::Text(text), _), (Value::Text(substring), _)) => {
                Ok((Value::Bool(text.contains(&*substring)), LoopControl::None))
            }
            ((a, _), (b, _)) => Err(RuntimeError::new(
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
