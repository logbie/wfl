use std::cell::RefCell;
use std::rc::Rc;
use wfl::interpreter::environment::Environment;
use wfl::interpreter::value::{FunctionValue, Value};
use wfl::parser::ast::Statement;

#[test]
fn closure_cycle() {
    let global_env = Environment::new_global();

    let mut env = Rc::clone(&global_env);
    for i in 0..10_000 {
        let func = FunctionValue {
            name: Some(format!("func_{}", i)),
            params: vec![],
            body: vec![],
            env: Rc::downgrade(&env), // This should use Weak reference
            line: 0,
            column: 0,
        };

        let func_value = Value::Function(Rc::new(func));
        env.borrow_mut().define(&format!("func_{}", i), func_value);

        let new_env = Environment::new_child_env(&env);
        env = new_env;
    }

    println!(
        "Strong count before drop: {}",
        Rc::strong_count(&global_env)
    );

    drop(env);

    assert_eq!(
        Rc::strong_count(&global_env),
        1,
        "Memory leak detected: Environment reference count should be 1 after dropping all closures"
    );
}

#[test]
fn parser_stability() {
    use std::process;
    use wfl::parser::Parser;

    fn current_rss() -> usize {
        let mut out = std::process::Command::new("ps")
            .args(&["-o", "rss=", &process::id().to_string()])
            .output()
            .expect("Failed to execute ps command");

        let output = String::from_utf8_lossy(&out.stdout);
        output.trim().parse::<usize>().unwrap_or(0)
    }

    let mut script = String::with_capacity(500_000);
    for i in 0..5_000 {
        script.push_str(&format!("store var_{} as {}\n", i, i));
    }

    let initial_rss = current_rss();
    println!("Initial RSS: {} KB", initial_rss);

    let mut parser = Parser::new(&script);
    let _ = parser.parse();

    let after_first_parse = current_rss();
    println!("RSS after first parse: {} KB", after_first_parse);

    let mut parser = Parser::new(&script);
    let _ = parser.parse();

    let after_second_parse = current_rss();
    println!("RSS after second parse: {} KB", after_second_parse);

    let delta = after_second_parse as i64 - after_first_parse as i64;
    println!("Delta: {} KB", delta);

    assert!(
        delta.abs() < 10_240,
        "Memory usage grew by {} KB between parses, which exceeds the 10 MB limit",
        delta
    );
}
