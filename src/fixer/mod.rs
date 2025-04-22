use crate::analyzer::Analyzer;
use crate::lexer::lex_wfl_with_positions;
use crate::parser::Parser;
use crate::parser::ast::{Expression, Literal, Operator, Program, Statement, UnaryOperator};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub struct CodeFixer {
    indent_size: usize,
}

pub enum FixerOutputMode {
    Stdout,  // Print fixed code to stdout
    InPlace, // Overwrite the input file
    Diff,    // Generate a unified diff
}

pub struct FixerSummary {
    pub lines_reformatted: usize,
    pub vars_renamed: usize,
    pub dead_code_removed: usize,
}

impl FixerSummary {
    pub fn total(&self) -> usize {
        self.lines_reformatted + self.vars_renamed + self.dead_code_removed
    }
}

impl CodeFixer {
    pub fn new() -> Self {
        Self { indent_size: 4 }
    }

    pub fn set_indent_size(&mut self, size: usize) {
        self.indent_size = size;
    }

    pub fn fix(&self, program: &Program, _source: &str) -> (String, FixerSummary) {
        let _analyzer = Analyzer::new();
        let dead_code = Vec::new();

        let simplified_program = self.simplify_program(program, &dead_code);

        let mut output = String::new();
        let mut summary = FixerSummary {
            lines_reformatted: 0,
            vars_renamed: 0,
            dead_code_removed: dead_code.len(),
        };

        self.pretty_print(&simplified_program, &mut output, 0, &mut summary);

        let tokens = lex_wfl_with_positions(&output);
        let mut parser = Parser::new(&tokens);
        match parser.parse() {
            Ok(_new_program) => {}
            Err(_) => {
                eprintln!(
                    "Warning: Re-parsing the fixed code resulted in errors. This is a bug in the code fixer."
                );
            }
        }

        (output, summary)
    }

    pub fn fix_file(&self, path: &Path, mode: FixerOutputMode) -> io::Result<FixerSummary> {
        let source = fs::read_to_string(path)?;

        let tokens = lex_wfl_with_positions(&source);
        let mut parser = Parser::new(&tokens);
        let program = match parser.parse() {
            Ok(program) => program,
            Err(err) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Failed to parse file: {:?}", err),
                ));
            }
        };

        let (fixed_code, summary) = self.fix(&program, &source);

        match mode {
            FixerOutputMode::Stdout => {
                io::stdout().write_all(fixed_code.as_bytes())?;
            }
            FixerOutputMode::InPlace => {
                fs::write(path, fixed_code)?;
            }
            FixerOutputMode::Diff => {
                let diff = self.generate_diff(&source, &fixed_code);
                io::stdout().write_all(diff.as_bytes())?;
            }
        }

        Ok(summary)
    }

    fn simplify_program(&self, program: &Program, dead_code: &[usize]) -> Program {
        let estimated_size = program.statements.len() - dead_code.len();
        let mut simplified_statements = Vec::with_capacity(estimated_size);

        let max_statements = 1000; // Arbitrary limit
        let mut processed = 0;

        for (i, statement) in program.statements.iter().enumerate() {
            if dead_code.contains(&i) {
                continue;
            }

            if processed >= max_statements {
                break;
            }

            let simplified = self.simplify_statement(statement);
            simplified_statements.push(simplified);
            processed += 1;
        }

        Program {
            statements: simplified_statements,
        }
    }

    fn simplify_statement(&self, statement: &Statement) -> Statement {
        thread_local! {
            static RECURSION_DEPTH: std::cell::RefCell<usize> = std::cell::RefCell::new(0);
        }
        
        let should_return_early = RECURSION_DEPTH.with(|depth| {
            let mut d = depth.borrow_mut();
            *d += 1;
            let too_deep = *d > 100; // Arbitrary limit
            too_deep
        });
        
        if should_return_early {
            RECURSION_DEPTH.with(|depth| {
                let mut d = depth.borrow_mut();
                *d -= 1;
            });
            return statement.clone();
        }
        
        let result = match statement.clone() {
            Statement::IfStatement {
                condition,
                then_block,
                else_block,
                line,
                column,
            } => {
                let simplified_condition = self.simplify_boolean_expression(&condition);

                let mut simplified_then = Vec::with_capacity(then_block.len());
                
                let max_statements = 100; // Arbitrary limit
                for (_i, stmt) in then_block.iter().enumerate().take(max_statements) {
                    simplified_then.push(self.simplify_statement(stmt));
                }

                let simplified_else = if let Some(else_stmts) = else_block {
                    let mut simplified = Vec::with_capacity(else_stmts.len());
                    for (_i, stmt) in else_stmts.iter().enumerate().take(max_statements) {
                        simplified.push(self.simplify_statement(stmt));
                    }
                    Some(simplified)
                } else {
                    None
                };
                
                RECURSION_DEPTH.with(|depth| {
                    let mut d = depth.borrow_mut();
                    *d -= 1;
                });

                Statement::IfStatement {
                    condition: simplified_condition,
                    then_block: simplified_then,
                    else_block: simplified_else,
                    line: line,
                    column: column,
                }
            }
            _ => statement.clone(),
        };
        result
    }
    #[allow(clippy::only_used_in_recursion)]
    fn simplify_boolean_expression(&self, expression: &Expression) -> Expression {
        match expression {
            Expression::BinaryOperation {
                left,
                operator,
                right,
                line,
                column,
            } => {
                let simplified_left = self.simplify_boolean_expression(left);
                let simplified_right = self.simplify_boolean_expression(right);

                Expression::BinaryOperation {
                    left: Box::new(simplified_left),
                    operator: operator.clone(),
                    right: Box::new(simplified_right),
                    line: *line,
                    column: *column,
                }
            }
            _ => expression.clone(),
        }
    }

    fn pretty_print(
        &self,
        program: &Program,
        output: &mut String,
        indent_level: usize,
        summary: &mut FixerSummary,
    ) {
        output.reserve(program.statements.len() * 64);
        for statement in &program.statements {
            self.pretty_print_statement(statement, output, indent_level, summary);
        }
    }

    fn pretty_print_statement(
        &self,
        statement: &Statement,
        output: &mut String,
        indent_level: usize,
        summary: &mut FixerSummary,
    ) {
        let reserve_size = match statement {
            Statement::VariableDeclaration { name, .. } => name.len() + 20,
            Statement::Assignment { name, .. } => name.len() + 20,
            Statement::ActionDefinition { name, parameters, body, .. } => {
                name.len() + parameters.len() * 10 + body.len() * 5 + 50
            },
            Statement::IfStatement { condition: _condition, then_block, else_block, .. } => {
                100 + then_block.len() * 5 + else_block.as_ref().map_or(0, |e| e.len() * 5)
            },
            Statement::ForEachLoop { item_name, collection: _collection, body, .. } => {
                item_name.len() + 50 + body.len() * 5
            },
            Statement::CountLoop { start: _start, end: _end, step: _step, body, .. } => {
                50 + body.len() * 5 // No variable name to measure
            },
            Statement::ReturnStatement { .. } => 20,
            Statement::ExpressionStatement { .. } => 30,
            _ => 50,
        };
        
        let current_len = output.len();
        if output.capacity() < current_len + reserve_size {
            output.reserve(reserve_size);
        }
        
        let indent = " ".repeat(indent_level * self.indent_size);

        match statement {
            Statement::VariableDeclaration { name, value, .. } => {
                let fixed_name = self.fix_identifier_name(name, summary);
                output.push_str(&indent);
                output.push_str("store ");
                output.push_str(&fixed_name);
                output.push_str(" as ");
                self.pretty_print_expression(value, output, indent_level, summary);
                output.push('\n');
                summary.lines_reformatted += 1;
            }
            Statement::Assignment { name, value, .. } => {
                let fixed_name = self.fix_identifier_name(name, summary);
                output.push_str(&indent);
                output.push_str("change ");
                output.push_str(&fixed_name);
                output.push_str(" to ");
                self.pretty_print_expression(value, output, indent_level, summary);
                output.push('\n');
                summary.lines_reformatted += 1;
            }
            Statement::ActionDefinition {
                name,
                parameters,
                body,
                return_type,
                ..
            } => {
                let fixed_name = self.fix_identifier_name(name, summary);
                output.push_str(&indent);
                output.push_str("define action called ");
                output.push_str(&fixed_name);

                if !parameters.is_empty() {
                    output.push_str(" with parameters ");
                    for (i, param) in parameters.iter().enumerate() {
                        if i > 0 {
                            output.push_str(", ");
                        }
                        let fixed_param_name = self.fix_identifier_name(&param.name, summary);
                        output.push_str(&fixed_param_name);

                        if let Some(param_type) = &param.param_type {
                            output.push_str(" as ");
                            output.push_str(&format!("{:?}", param_type));
                        }

                        if let Some(default_value) = &param.default_value {
                            output.push_str(" default ");
                            self.pretty_print_expression(
                                default_value,
                                output,
                                indent_level,
                                summary,
                            );
                        }
                    }
                }

                if let Some(ret_type) = return_type {
                    output.push_str(" returning ");
                    output.push_str(&format!("{:?}", ret_type));
                }

                output.push_str(":\n");

                let max_statements = 100; // Arbitrary limit to prevent stack overflow
                for (_i, stmt) in body.iter().enumerate().take(max_statements) {
                    self.pretty_print_statement(stmt, output, indent_level + 1, summary);
                }
                
                if body.len() > max_statements {
                    let truncated_indent = " ".repeat((indent_level + 1) * self.indent_size);
                    output.push_str(&truncated_indent);
                    output.push_str("// ... truncated for memory safety\n");
                }

                output.push_str(&indent);
                output.push_str("end action\n");
                summary.lines_reformatted += 1;
            }
            Statement::IfStatement {
                condition,
                then_block,
                else_block,
                ..
            } => {
                output.push_str(&indent);
                output.push_str("check if ");
                self.pretty_print_expression(condition, output, indent_level, summary);
                output.push_str(":\n");

                let max_statements = 100; // Arbitrary limit to prevent stack overflow
                for (_i, stmt) in then_block.iter().enumerate().take(max_statements) {
                    self.pretty_print_statement(stmt, output, indent_level + 1, summary);
                }
                
                if then_block.len() > max_statements {
                    let truncated_indent = " ".repeat((indent_level + 1) * self.indent_size);
                    output.push_str(&truncated_indent);
                    output.push_str("// ... truncated for memory safety\n");
                }

                if let Some(else_stmts) = else_block {
                    output.push_str(&indent);
                    output.push_str("otherwise:\n");

                    for (_i, stmt) in else_stmts.iter().enumerate().take(max_statements) {
                        self.pretty_print_statement(stmt, output, indent_level + 1, summary);
                    }
                    
                    if else_stmts.len() > max_statements {
                        let truncated_indent = " ".repeat((indent_level + 1) * self.indent_size);
                        output.push_str(&truncated_indent);
                        output.push_str("// ... truncated for memory safety\n");
                    }
                }

                output.push_str(&indent);
                output.push_str("end check\n");
                summary.lines_reformatted += 1;
            }
            Statement::SingleLineIf {
                condition,
                then_stmt,
                else_stmt,
                ..
            } => {
                output.push_str(&indent);
                output.push_str("if ");
                self.pretty_print_expression(condition, output, indent_level, summary);
                output.push_str(" then ");

                thread_local! {
                    static SINGLE_LINE_DEPTH: std::cell::RefCell<usize> = std::cell::RefCell::new(0);
                }
                
                let should_truncate = SINGLE_LINE_DEPTH.with(|depth| {
                    let mut d = depth.borrow_mut();
                    *d += 1;
                    let too_deep = *d > 3; // Lower limit for single line statements
                    too_deep
                });
                
                if should_truncate {
                    output.push_str("<truncated>");
                    SINGLE_LINE_DEPTH.with(|depth| {
                        let mut d = depth.borrow_mut();
                        *d -= 1;
                    });
                } else {
                    let mut then_output = String::with_capacity(100); // Pre-allocate
                    self.pretty_print_statement(then_stmt, &mut then_output, 0, summary);
                    let then_str = then_output.trim();
                    output.push_str(then_str);
    
                    if let Some(else_stmt) = else_stmt {
                        output.push_str(" otherwise ");
    
                        let mut else_output = String::with_capacity(100); // Pre-allocate
                        self.pretty_print_statement(else_stmt, &mut else_output, 0, summary);
                        let else_str = else_output.trim();
                        output.push_str(else_str);
                    }
                    
                    SINGLE_LINE_DEPTH.with(|depth| {
                        let mut d = depth.borrow_mut();
                        *d -= 1;
                    });
                }

                output.push('\n');
                summary.lines_reformatted += 1;
            }
            Statement::ForEachLoop {
                item_name,
                collection,
                body,
                ..
            } => {
                let fixed_item_name = self.fix_identifier_name(item_name, summary);
                output.push_str(&indent);
                output.push_str("for each ");
                output.push_str(&fixed_item_name);
                output.push_str(" in ");
                self.pretty_print_expression(collection, output, indent_level, summary);
                output.push_str(":\n");

                let max_statements = 100; // Arbitrary limit to prevent stack overflow
                for (_i, stmt) in body.iter().enumerate().take(max_statements) {
                    self.pretty_print_statement(stmt, output, indent_level + 1, summary);
                }
                
                if body.len() > max_statements {
                    let truncated_indent = " ".repeat((indent_level + 1) * self.indent_size);
                    output.push_str(&truncated_indent);
                    output.push_str("// ... truncated for memory safety\n");
                }

                output.push_str(&indent);
                output.push_str("end for each\n");
                summary.lines_reformatted += 1;
            }
            Statement::CountLoop {
                start,
                end,
                step,
                body,
                ..
            } => {
                output.push_str(&indent);
                output.push_str("count from ");
                self.pretty_print_expression(start, output, indent_level, summary);
                output.push_str(" to ");
                self.pretty_print_expression(end, output, indent_level, summary);

                if let Some(step_expr) = step {
                    output.push_str(" by ");
                    self.pretty_print_expression(step_expr, output, indent_level, summary);
                }

                output.push_str(":\n");

                let max_statements = 100; // Arbitrary limit to prevent stack overflow
                for (_i, stmt) in body.iter().enumerate().take(max_statements) {
                    self.pretty_print_statement(stmt, output, indent_level + 1, summary);
                }
                
                if body.len() > max_statements {
                    let truncated_indent = " ".repeat((indent_level + 1) * self.indent_size);
                    output.push_str(&truncated_indent);
                    output.push_str("// ... truncated for memory safety\n");
                }

                output.push_str(&indent);
                output.push_str("end count\n");
                summary.lines_reformatted += 1;
            }
            Statement::WhileLoop {
                condition, body, ..
            } => {
                output.push_str(&indent);
                output.push_str("while ");
                self.pretty_print_expression(condition, output, indent_level, summary);
                output.push_str(":\n");

                let max_statements = 100; // Arbitrary limit to prevent stack overflow
                for (_i, stmt) in body.iter().enumerate().take(max_statements) {
                    self.pretty_print_statement(stmt, output, indent_level + 1, summary);
                }
                
                if body.len() > max_statements {
                    let truncated_indent = " ".repeat((indent_level + 1) * self.indent_size);
                    output.push_str(&truncated_indent);
                    output.push_str("// ... truncated for memory safety\n");
                }

                output.push_str(&indent);
                output.push_str("end while\n");
                summary.lines_reformatted += 1;
            }
            Statement::DisplayStatement { value, .. } => {
                output.push_str(&indent);
                output.push_str("display ");
                self.pretty_print_expression(value, output, indent_level, summary);
                output.push('\n');
                summary.lines_reformatted += 1;
            }
            Statement::ReturnStatement { value, .. } => {
                output.push_str(&indent);
                output.push_str("return");

                if let Some(expr) = value {
                    output.push(' ');
                    self.pretty_print_expression(expr, output, indent_level, summary);
                }

                output.push('\n');
                summary.lines_reformatted += 1;
            }
            Statement::ExpressionStatement { expression, .. } => {
                output.push_str(&indent);
                self.pretty_print_expression(expression, output, indent_level, summary);
                output.push('\n');
                summary.lines_reformatted += 1;
            }
            _ => {
                output.push_str(&indent);
                output.push_str(&format!("{:?}\n", statement));
                summary.lines_reformatted += 1;
            }
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    fn pretty_print_expression(
        &self,
        expression: &Expression,
        output: &mut String,
        indent_level: usize,
        summary: &mut FixerSummary,
    ) {
        let reserve_size = match expression {
            Expression::Literal(Literal::String(s), ..) => s.len() + 2, // quotes
            Expression::Literal(Literal::Pattern(p), ..) => p.len() + 2, // slashes
            Expression::Literal(Literal::Integer(_), ..) => 20,         // typical int size
            Expression::Literal(Literal::Float(_), ..) => 20,           // typical float size
            Expression::Variable(name, ..) => name.len(),
            Expression::BinaryOperation { .. } => 100, // rough estimate
            Expression::UnaryOperation { .. } => 50,   // rough estimate
            Expression::FunctionCall { .. } => 100,    // rough estimate
            _ => 50,                                   // default size
        };

        let current_len = output.len();
        if output.capacity() < current_len + reserve_size {
            output.reserve(reserve_size);
        }

        match expression {
            Expression::Literal(literal, ..) => match literal {
                Literal::String(s) => {
                    output.push('"');
                    output.push_str(s);
                    output.push('"');
                }
                Literal::Integer(n) => {
                    output.push_str(&n.to_string());
                }
                Literal::Float(f) => {
                    output.push_str(&f.to_string());
                }
                Literal::Boolean(b) => {
                    output.push_str(if *b { "yes" } else { "no" });
                }
                Literal::Nothing => {
                    output.push_str("nothing");
                }
                Literal::Pattern(p) => {
                    output.push('/');
                    output.push_str(p);
                    output.push('/');
                }
            },
            Expression::Variable(name, ..) => {
                let fixed_name = self.fix_identifier_name(name, summary);
                output.push_str(&fixed_name);
            }
            Expression::BinaryOperation {
                left,
                operator,
                right,
                ..
            } => {
                output.push('(');
                self.pretty_print_expression(left, output, indent_level, summary);

                match operator {
                    Operator::Plus => output.push_str(" + "),
                    Operator::Minus => output.push_str(" - "),
                    Operator::Multiply => output.push_str(" * "),
                    Operator::Divide => output.push_str(" / "),
                    Operator::Equals => output.push_str(" == "),
                    Operator::NotEquals => output.push_str(" != "),
                    Operator::LessThan => output.push_str(" < "),
                    Operator::LessThanOrEqual => output.push_str(" <= "),
                    Operator::GreaterThan => output.push_str(" > "),
                    Operator::GreaterThanOrEqual => output.push_str(" >= "),
                    Operator::And => output.push_str(" and "),
                    Operator::Or => output.push_str(" or "),
                    Operator::Contains => output.push_str(" contains "),
                }

                self.pretty_print_expression(right, output, indent_level, summary);
                output.push(')');
            }
            Expression::UnaryOperation {
                operator,
                expression: expr,
                ..
            } => {
                match operator {
                    UnaryOperator::Minus => output.push('-'),
                    UnaryOperator::Not => output.push_str("not "),
                }

                self.pretty_print_expression(expr, output, indent_level, summary);
            }
            Expression::FunctionCall {
                function,
                arguments,
                ..
            } => {
                self.pretty_print_expression(function, output, indent_level, summary);
                output.push('(');

                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        output.push_str(", ");
                    }

                    if let Some(name) = &arg.name {
                        let fixed_name = self.fix_identifier_name(name, summary);
                        output.push_str(&fixed_name);
                        output.push_str(": ");
                    }

                    self.pretty_print_expression(&arg.value, output, indent_level, summary);
                }

                output.push(')');
            }
            Expression::MemberAccess {
                object, property, ..
            } => {
                self.pretty_print_expression(object, output, indent_level, summary);
                output.push('.');
                output.push_str(property);
            }
            Expression::IndexAccess {
                collection, index, ..
            } => {
                self.pretty_print_expression(collection, output, indent_level, summary);
                output.push('[');
                self.pretty_print_expression(index, output, indent_level, summary);
                output.push(']');
            }
            Expression::Concatenation { left, right, .. } => {
                self.pretty_print_expression(left, output, indent_level, summary);
                output.push_str(" & ");
                self.pretty_print_expression(right, output, indent_level, summary);
            }
            Expression::PatternMatch { text, pattern, .. } => {
                self.pretty_print_expression(text, output, indent_level, summary);
                output.push_str(" matches ");
                self.pretty_print_expression(pattern, output, indent_level, summary);
            }
            Expression::PatternFind { text, pattern, .. } => {
                output.push_str("find ");
                self.pretty_print_expression(pattern, output, indent_level, summary);
                output.push_str(" in ");
                self.pretty_print_expression(text, output, indent_level, summary);
            }
            Expression::PatternReplace {
                text,
                pattern,
                replacement,
                ..
            } => {
                output.push_str("replace ");
                self.pretty_print_expression(pattern, output, indent_level, summary);
                output.push_str(" with ");
                self.pretty_print_expression(replacement, output, indent_level, summary);
                output.push_str(" in ");
                self.pretty_print_expression(text, output, indent_level, summary);
            }
            Expression::PatternSplit { text, pattern, .. } => {
                output.push_str("split ");
                self.pretty_print_expression(text, output, indent_level, summary);
                output.push_str(" by ");
                self.pretty_print_expression(pattern, output, indent_level, summary);
            }
            Expression::AwaitExpression {
                expression: expr, ..
            } => {
                output.push_str("await ");
                self.pretty_print_expression(expr, output, indent_level, summary);
            }
            #[allow(unreachable_patterns)]
            _ => {
                output.push_str(&format!("{:?}", expression));
            }
        }
    }

    fn fix_identifier_name(&self, name: &str, summary: &mut FixerSummary) -> String {
        if !self.is_snake_case(name) {
            summary.vars_renamed += 1;
            self.to_snake_case(name)
        } else {
            name.to_string()
        }
    }

    fn is_snake_case(&self, s: &str) -> bool {
        !s.contains(char::is_uppercase) && !s.contains(' ')
    }

    fn to_snake_case(&self, s: &str) -> String {
        let mut result = String::new();
        let mut previous_char_is_lowercase = false;

        for (i, c) in s.char_indices() {
            if c.is_uppercase() {
                if i > 0 && previous_char_is_lowercase {
                    result.push('_');
                }
                result.push(c.to_lowercase().next().unwrap());
            } else if c == ' ' {
                result.push('_');
            } else {
                result.push(c);
            }

            previous_char_is_lowercase = c.is_lowercase();
        }

        result
    }

    pub fn diff(&self, original: &str, fixed: &str) -> String {
        self.generate_diff(original, fixed)
    }

    pub fn generate_diff(&self, original: &str, fixed: &str) -> String {
        let mut diff = String::new();

        let original_lines: Vec<&str> = original.lines().collect();
        let fixed_lines: Vec<&str> = fixed.lines().collect();

        for i in 0..original_lines.len().max(fixed_lines.len()) {
            if i < original_lines.len() && i < fixed_lines.len() {
                if original_lines[i] != fixed_lines[i] {
                    diff.push_str(&format!("-{}\n", original_lines[i]));
                    diff.push_str(&format!("+{}\n", fixed_lines[i]));
                }
            } else if i < original_lines.len() {
                diff.push_str(&format!("-{}\n", original_lines[i]));
            } else if i < fixed_lines.len() {
                diff.push_str(&format!("+{}\n", fixed_lines[i]));
            }
        }

        diff
    }

    pub fn load_config(&mut self, dir: &Path) {
        let config = crate::config::load_config(dir);
        self.indent_size = config.indent_size;
    }
}

#[cfg(test)]
mod tests;

impl Default for CodeFixer {
    fn default() -> Self {
        Self::new()
    }
}
