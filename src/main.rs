use wfl::Lexer;
fn main() {
    let source = r#"
define action called greet does
    give back "Hello, World!"
end
"#;

    let mut lexer = Lexer::new(source.to_string());
    let tokens = lexer.tokenize();

    for token in tokens {
        println!("{:?}", token);
    }
}
