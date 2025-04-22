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
use crate::stdlib::{core, list, math, pattern, text};
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

pub struct Interpreter {
    global_env: Rc<RefCell<Environment>>,
    current_count: RefCell<Option<f64>>,
    in_count_loop: RefCell<bool>,
    started: Instant,
    max_duration: Duration,
    call_stack: RefCell<Vec<CallFrame>>,
    #[allow(dead_code)]
    io_client: Rc<IoClient>,
    bytes_allocated: RefCell<usize>,
    max_memory_bytes: usize,
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

            // Register stdlib without pattern functions that need interpreter
            core::register_core(&mut env);
            math::register_math(&mut env);
            text::register_text(&mut env);
            list::register_list(&mut env);
        }

        let interpreter = Interpreter {
            global_env,
            current_count: RefCell::new(None),
            in_count_loop: RefCell::new(false),
            started: Instant::now(),
            max_duration: Duration::from_secs(u64::MAX), // Effectively no timeout by default
            call_stack: RefCell::new(Vec::new()),
            io_client: Rc::new(IoClient::new()),
            bytes_allocated: RefCell::new(0),
            max_memory_bytes: 512 * 1024 * 1024, // Default 512 MB
        };

        // Register pattern functions that need interpreter reference
        {
            let mut env = interpreter.global_env.borrow_mut();
            pattern::register(&mut env, &interpreter);
        }

        interpreter
    }

    pub fn with_timeout(seconds: u64) -> Self {
        let mut interpreter = Self::new();
        interpreter.started = Instant::now();
        interpreter.max_duration = Duration::from_secs(seconds);
        interpreter
    }

    pub fn with_config(config: &crate::config::WflConfig) -> Self {
        let mut interpreter = Self::new();
        interpreter.started = Instant::now();
        interpreter.max_duration = Duration::from_secs(config.timeout_seconds);
        interpreter.max_memory_bytes = config.max_memory_mb * 1024 * 1024;
        interpreter
    }

    pub fn track_allocation(&self, bytes: usize) -> Result<(), RuntimeError> {
        let mut alloc = self.bytes_allocated.borrow_mut();
        *alloc += bytes;

        if *alloc > self.max_memory_bytes {
            return Err(RuntimeError::with_kind(
                format!(
                    "Out of memory: Used {}MB exceeds limit of {}MB",
                    *alloc / (1024 * 1024),
                    self.max_memory_bytes / (1024 * 1024)
                ),
                0,
                0,
                ErrorKind::OutOfMemory,
            ));
        }

        Ok(())
    }

    pub fn track_deallocation(&self, bytes: usize) {
        let mut alloc = self.bytes_allocated.borrow_mut();
        *alloc = alloc.saturating_sub(bytes);
    }

    pub fn check_memory(&self) -> Result<(), RuntimeError> {
        let bytes = *self.bytes_allocated.borrow();
        if bytes > self.max_memory_bytes {
            Err(RuntimeError::with_kind(
                format!(
                    "Out of memory: Used {}MB exceeds limit of {}MB",
                    bytes / (1024 * 1024),
                    self.max_memory_bytes / (1024 * 1024)
                ),
                0,
                0,
                ErrorKind::OutOfMemory,
            ))
        } else {
            Ok(())
        }
    }

    pub fn get_call_stack(&self) -> Vec<CallFrame> {
        self.call_stack.borrow().clone()
    }

    pub fn global_env(&self) -> &Rc<RefCell<Environment>> {
        &self.global_env
    }

    pub fn clear_call_stack(&mut self) {
        self.call_stack.borrow_mut().clear();
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

        let mut errors = Vec::new();
        if let Err(e) = self.check_memory() {
            errors.push(e);
            self.call_stack.borrow_mut().clear();
            return Err(errors);
        }

        println!(
            "Starting script execution with {} statements...",
            program.statements.len()
        );

        let mut last_value = Value::Null;
        let mut errors = Vec::new();

        for (i, statement) in program.statements.iter().enumerate() {
            println!(
                "Executing statement {}/{}...",
                i + 1,
                program.statements.len()
            );

            if let Err(err) = self.check_time() {
                println!(
                    "Timeout reached at statement {}/{}",
                    i + 1,
                    program.statements.len()
                );
                errors.push(err);
                self.call_stack.borrow_mut().clear();
                return Err(errors);
            }

            match self
                .execute_statement(statement, Rc::clone(&self.global_env))
                .await
            {
                Ok(value) => {
                    last_value = value;
                    println!(
                        "Statement {}/{} completed successfully",
                        i + 1,
                        program.statements.len()
                    );
                }
                Err(err) => {
                    println!(
                        "Error at statement {}/{}: {:?}",
                        i + 1,
                        program.statements.len(),
                        err
                    );
                    errors.push(err);

                    if matches!(err.kind, ErrorKind::OutOfMemory) {
                        self.call_stack.borrow_mut().clear();
                    }

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

                        if matches!(err.kind, ErrorKind::OutOfMemory) {
                            self.call_stack.borrow_mut().clear();
                        }
                    }
                }
            }

            self.call_stack.borrow_mut().clear();
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
    ) -> Result<Value, RuntimeError> {
        Box::pin(self._execute_statement(stmt, env)).await
    }

    async fn _execute_statement(
        &self,
        stmt: &Statement,
        env: Rc<RefCell<Environment>>,
    ) -> Result<Value, RuntimeError> {
        self.check_time()?;

        match stmt {
            Statement::VariableDeclaration {
                name,
                value,
                line: _,
                column: _,
            } => {
                let value = self.evaluate_expression(value, Rc::clone(&env)).await?;
                env.borrow_mut().define(name, value);
                Ok(Value::Null)
            }

            Statement::Assignment {
                name,
                value,
                line,
                column,
            } => {
                let value = self.evaluate_expression(value, Rc::clone(&env)).await?;
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
                let condition_value = self.evaluate_expression(condition, Rc::clone(&env)).await?;

                if condition_value.is_truthy() {
                    self.execute_block(then_block, Rc::clone(&env)).await
                } else if let Some(else_stmts) = else_block {
                    self.execute_block(else_stmts, Rc::clone(&env)).await
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
                let condition_value = self.evaluate_expression(condition, Rc::clone(&env)).await?;

                if condition_value.is_truthy() {
                    self.execute_statement(then_stmt, Rc::clone(&env)).await
                } else if let Some(else_stmt) = else_stmt {
                    self.execute_statement(else_stmt, Rc::clone(&env)).await
                } else {
                    Ok(Value::Null)
                }
            }

            Statement::DisplayStatement {
                value,
                line: _line,
                column: _column,
            } => {
                let value = self.evaluate_expression(value, Rc::clone(&env)).await?;
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
                    env: Rc::downgrade(&env),
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
                    self.evaluate_expression(expr, Rc::clone(&env)).await
                } else {
                    Ok(Value::Null)
                }
            }

            Statement::ExpressionStatement {
                expression,
                line: _line,
                column: _column,
            } => self.evaluate_expression(expression, Rc::clone(&env)).await,

            Statement::CountLoop {
                start,
                end,
                step,
                downward,
                body,
                line,
                column,
            } => {
                let previous_count = *self.current_count.borrow();
                let was_in_count_loop = *self.in_count_loop.borrow();
                let start_val = self.evaluate_expression(start, Rc::clone(&env)).await?;
                let end_val = self.evaluate_expression(end, Rc::clone(&env)).await?;

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
                    match self.evaluate_expression(step_expr, Rc::clone(&env)).await? {
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
                        Ok(_) => {}
                        Err(e) => {
                            *self.current_count.borrow_mut() = previous_count;
                            *self.in_count_loop.borrow_mut() = was_in_count_loop;
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

                *self.current_count.borrow_mut() = previous_count;
                *self.in_count_loop.borrow_mut() = was_in_count_loop;

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
                let collection_val = self
                    .evaluate_expression(collection, Rc::clone(&env))
                    .await?;

                let loop_env = Environment::new_child_env(&env);

                match collection_val {
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
                            self.execute_block(body, Rc::clone(&loop_env)).await?;
                        }
                    }
                    Value::Object(obj_rc) => {
                        let items: Vec<(String, Value)> = {
                            let obj = obj_rc.borrow();
                            obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
                        };

                        for (_, value) in items {
                            loop_env.borrow_mut().define(item_name, value);
                            self.execute_block(body, Rc::clone(&loop_env)).await?;
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
                    .evaluate_expression(condition, Rc::clone(&env))
                    .await?
                    .is_truthy()
                {
                    self.check_time()?;
                    self.execute_block(body, Rc::clone(&env)).await?;
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
                    self.check_time()?;
                    self.execute_block(body, Rc::clone(&env)).await?;
                    if self
                        .evaluate_expression(condition, Rc::clone(&env))
                        .await?
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
                    self.check_time()?;
                    self.execute_block(body, Rc::clone(&env)).await?;
                }
                #[allow(unreachable_code)]
                Ok(Value::Null)
            }

            Statement::BreakStatement { .. } | Statement::ContinueStatement { .. } => {
                Ok(Value::Null)
            }

            Statement::OpenFileStatement {
                path,
                variable_name,
                line,
                column,
            } => {
                let path_value = self.evaluate_expression(path, Rc::clone(&env)).await?;
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
                        let text_value = Value::new_text(handle, self)?;
                        env.borrow_mut().define(variable_name, text_value);
                        Ok(Value::Null)
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
                let path_value = self.evaluate_expression(path, Rc::clone(&env)).await?;
                let path_str = match &path_value {
                    Value::Text(s) => s.clone(),
                    _ => {
                        return Err(RuntimeError::new(
                            format!(
                                "Expected string for file path or handle, got {:?}",
                                path_value
                            ),
                            *line,
                            *column,
                        ));
                    }
                };

                let is_file_path = matches!(path, Expression::Literal(Literal::String(_), _, _));

                if is_file_path {
                    match self.io_client.open_file(&path_str).await {
                        Ok(handle) => match self.io_client.read_file(&handle).await {
                            Ok(content) => {
                                let text_value = Value::new_text(content, self)?;
                                env.borrow_mut().define(variable_name, text_value);
                                let _ = self.io_client.close_file(&handle).await;
                                Ok(Value::Null)
                            }
                            Err(e) => {
                                let _ = self.io_client.close_file(&handle).await;
                                Err(RuntimeError::new(e, *line, *column))
                            }
                        },
                        Err(e) => Err(RuntimeError::new(e, *line, *column)),
                    }
                } else {
                    match self.io_client.read_file(&path_str).await {
                        Ok(content) => {
                            let text_value = Value::new_text(content, self)?;
                            env.borrow_mut().define(variable_name, text_value);
                            Ok(Value::Null)
                        }
                        Err(e) => Err(RuntimeError::new(e, *line, *column)),
                    }
                }
            }
            Statement::WriteFileStatement {
                file,
                content,
                line,
                column,
            } => {
                let file_value = self.evaluate_expression(file, Rc::clone(&env)).await?;
                let content_value = self.evaluate_expression(content, Rc::clone(&env)).await?;

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
                    Ok(_) => Ok(Value::Null),
                    Err(e) => Err(RuntimeError::new(e, *line, *column)),
                }
            }
            Statement::CloseFileStatement { file, line, column } => {
                let file_value = self.evaluate_expression(file, Rc::clone(&env)).await?;

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
                    Ok(_) => Ok(Value::Null),
                    Err(e) => Err(RuntimeError::new(e, *line, *column)),
                }
            }
            Statement::WaitForStatement {
                inner,
                line: _,
                column: _,
            } => {
                match inner.as_ref() {
                    Statement::ExpressionStatement {
                        expression: Expression::Variable(var_name, _, _),
                        line: _,
                        column: _,
                    } => {
                        let max_attempts = 1000; // Prevent infinite waiting
                        for _ in 0..max_attempts {
                            if let Some(value) = env.borrow().get(var_name) {
                                if !matches!(value, Value::Null) {
                                    return Ok(Value::Null);
                                }
                            }

                            tokio::time::sleep(std::time::Duration::from_millis(10)).await;

                            self.check_time()?;
                        }

                        Err(RuntimeError::new(
                            format!("Timeout waiting for variable '{}'", var_name),
                            0,
                            0,
                        ))
                    }
                    Statement::ReadFileStatement {
                        path,
                        variable_name,
                        line,
                        column,
                    } => {
                        let path_value = self.evaluate_expression(path, Rc::clone(&env)).await?;
                        let path_str = match &path_value {
                            Value::Text(s) => s.clone(),
                            _ => {
                                return Err(RuntimeError::new(
                                    format!(
                                        "Expected string for file path or handle, got {:?}",
                                        path_value
                                    ),
                                    *line,
                                    *column,
                                ));
                            }
                        };

                        let is_file_path =
                            matches!(path, Expression::Literal(Literal::String(_), _, _));

                        if is_file_path {
                            match self.io_client.open_file(&path_str).await {
                                Ok(handle) => match self.io_client.read_file(&handle).await {
                                    Ok(content) => {
                                        env.borrow_mut()
                                            .define(variable_name, Value::Text(content.into()));
                                        let _ = self.io_client.close_file(&handle).await;
                                        Ok(Value::Null)
                                    }
                                    Err(e) => {
                                        let _ = self.io_client.close_file(&handle).await;
                                        Err(RuntimeError::new(e, *line, *column))
                                    }
                                },
                                Err(e) => Err(RuntimeError::new(e, *line, *column)),
                            }
                        } else {
                            match self.io_client.read_file(&path_str).await {
                                Ok(content) => {
                                    env.borrow_mut()
                                        .define(variable_name, Value::Text(content.into()));
                                    Ok(Value::Null)
                                }
                                Err(e) => Err(RuntimeError::new(e, *line, *column)),
                            }
                        }
                    }
                    _ => self.execute_statement(inner, Rc::clone(&env)).await,
                }
            }
            Statement::TryStatement {
                body,
                error_name,
                when_block,
                otherwise_block,
                line: _line,
                column: _column,
            } => {
                let child_env = Environment::new_child_env(&env);

                match self.execute_block(body, Rc::clone(&child_env)).await {
                    Ok(val) => Ok(val), // Success path: just bubble result
                    Err(err) => {
                        child_env
                            .borrow_mut()
                            .define(error_name, Value::Text(err.message.into()));

                        let result = self.execute_block(when_block, Rc::clone(&child_env)).await;

                        if result.is_ok() || otherwise_block.is_none() {
                            result
                        } else {
                            self.execute_block(otherwise_block.as_ref().unwrap(), child_env)
                                .await
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
                let url_val = self.evaluate_expression(url, Rc::clone(&env)).await?;
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
                        let text_value = Value::new_text(body, self)?;
                        env.borrow_mut().define(variable_name, text_value);
                        Ok(Value::Null)
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
                let url_val = self.evaluate_expression(url, Rc::clone(&env)).await?;
                let data_val = self.evaluate_expression(data, Rc::clone(&env)).await?;

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
                        let text_value = Value::new_text(body, self)?;
                        env.borrow_mut().define(variable_name, text_value);
                        Ok(Value::Null)
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
    ) -> Result<Value, RuntimeError> {
        Box::pin(self._execute_block(statements, env)).await
    }

    async fn _execute_block(
        &self,
        statements: &[Statement],
        env: Rc<RefCell<Environment>>,
    ) -> Result<Value, RuntimeError> {
        self.assert_invariants();
        let mut last_value = Value::Null;

        for statement in statements {
            last_value = self.execute_statement(statement, Rc::clone(&env)).await?;
        }

        self.assert_invariants();
        Ok(last_value)
    }

    async fn evaluate_expression(
        &self,
        expr: &Expression,
        env: Rc<RefCell<Environment>>,
    ) -> Result<Value, RuntimeError> {
        Box::pin(self._evaluate_expression(expr, env)).await
    }

    async fn _evaluate_expression(
        &self,
        expr: &Expression,
        env: Rc<RefCell<Environment>>,
    ) -> Result<Value, RuntimeError> {
        self.assert_invariants();
        self.check_time()?;

        let result = match expr {
            &Expression::AwaitExpression {
                ref expression,
                line: _line,
                column: _column,
            } => {
                let value = self
                    .evaluate_expression(expression, Rc::clone(&env))
                    .await?;
                Ok(value)
            }
            Expression::Literal(literal, _line, _column) => match literal {
                Literal::String(s) => Value::new_text(s.clone(), self),
                Literal::Integer(i) => Ok(Value::Number(*i as f64)),
                Literal::Float(f) => Ok(Value::Number(*f)),
                Literal::Boolean(b) => Ok(Value::Bool(*b)),
                Literal::Nothing => Ok(Value::Null),
                Literal::Pattern(s) => Value::new_text(s.clone(), self),
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
                let left_val = match left.as_ref() {
                    Expression::Variable(name, _, _) if name == "count" => {
                        if let Some(count_value) = *self.current_count.borrow() {
                            Value::Number(count_value)
                        } else {
                            self.evaluate_expression(left, Rc::clone(&env)).await?
                        }
                    }
                    _ => self.evaluate_expression(left, Rc::clone(&env)).await?,
                };

                let right_val = match right.as_ref() {
                    Expression::Variable(name, _, _) if name == "count" => {
                        if let Some(count_value) = *self.current_count.borrow() {
                            Value::Number(count_value)
                        } else {
                            self.evaluate_expression(right, Rc::clone(&env)).await?
                        }
                    }
                    _ => self.evaluate_expression(right, Rc::clone(&env)).await?,
                };

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
                let value = self
                    .evaluate_expression(expression, Rc::clone(&env))
                    .await?;

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
                let function_val = self.evaluate_expression(function, Rc::clone(&env)).await?;

                let mut arg_values = Vec::new();
                for arg in arguments {
                    arg_values.push(
                        self.evaluate_expression(&arg.value, Rc::clone(&env))
                            .await?,
                    );
                }

                match function_val {
                    Value::Function(func) => {
                        self.call_function(&func, arg_values, *line, *column).await
                    }
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
                let object_val = self.evaluate_expression(object, Rc::clone(&env)).await?;

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
                let collection_val = self
                    .evaluate_expression(collection, Rc::clone(&env))
                    .await?;
                let index_val = self.evaluate_expression(index, Rc::clone(&env)).await?;

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
                let left_val = self.evaluate_expression(left, Rc::clone(&env)).await?;

                let right_val = match right.as_ref() {
                    Expression::Variable(name, _, _) if name == "count" => {
                        if let Some(count_value) = *self.current_count.borrow() {
                            Value::Number(count_value)
                        } else {
                            self.evaluate_expression(right, Rc::clone(&env)).await?
                        }
                    }
                    _ => self.evaluate_expression(right, Rc::clone(&env)).await?,
                };

                let result = format!("{}{}", left_val, right_val);
                Value::new_text(result, self)
            }

            Expression::PatternMatch {
                text,
                pattern,
                line: _line,
                column: _column,
            } => {
                let text_val = self.evaluate_expression(text, Rc::clone(&env)).await?;
                let pattern_val = self.evaluate_expression(pattern, Rc::clone(&env)).await?;

                let args = vec![text_val, pattern_val];
                crate::stdlib::pattern::native_pattern_matches(args, self)
            }

            Expression::PatternFind {
                text,
                pattern,
                line: _line,
                column: _column,
            } => {
                let text_val = self.evaluate_expression(text, Rc::clone(&env)).await?;
                let pattern_val = self.evaluate_expression(pattern, Rc::clone(&env)).await?;

                let args = vec![pattern_val, text_val]; // Note: pattern first, then text
                crate::stdlib::pattern::native_pattern_find(args, self)
            }

            Expression::PatternReplace {
                text,
                pattern,
                replacement,
                line: _line,
                column: _column,
            } => {
                let text_val = self.evaluate_expression(text, Rc::clone(&env)).await?;
                let pattern_val = self.evaluate_expression(pattern, Rc::clone(&env)).await?;
                let replacement_val = self
                    .evaluate_expression(replacement, Rc::clone(&env))
                    .await?;

                let args = vec![pattern_val, replacement_val, text_val]; // Note: pattern, replacement, then text
                crate::stdlib::pattern::native_pattern_replace(args, self)
            }

            Expression::PatternSplit {
                text,
                pattern,
                line: _line,
                column: _column,
            } => {
                let text_val = self.evaluate_expression(text, Rc::clone(&env)).await?;
                let pattern_val = self.evaluate_expression(pattern, Rc::clone(&env)).await?;

                let args = vec![text_val, pattern_val];
                crate::stdlib::pattern::native_pattern_split(args, self)
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
            Ok(value) => {
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
        left: Value,
        right: Value,
        line: usize,
        column: usize,
    ) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::Text(a), Value::Text(b)) => {
                let result = format!("{}{}", a, b);
                Value::new_text(result, self)
            }
            (Value::Text(a), b) => {
                let result = format!("{}{}", a, b);
                Value::new_text(result, self)
            }
            (a, Value::Text(b)) => {
                let result = format!("{}{}", a, b);
                Value::new_text(result, self)
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
