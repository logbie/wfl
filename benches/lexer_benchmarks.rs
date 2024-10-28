use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wfl::lexer::Lexer;

fn benchmark_lexer(c: &mut Criterion) {
    let input = r#"
    define action process_data:
        needs:
            data as text
            count as number
        do:
            store result as nothing
            check if count > 0:
                process data
                give back result
            end check
    end action
    "#;

    c.bench_function("lexer_basic", |b| {
        b.iter(|| {
            let lexer = Lexer::new(black_box(input));
            let _ = lexer.collect_tokens();
        });
    });

    // Benchmark with a larger input
    let large_input = input.repeat(100);
    c.bench_function("lexer_large", |b| {
        b.iter(|| {
            let lexer = Lexer::new(black_box(&large_input));
            let _ = lexer.collect_tokens();
        });
    });

    // Benchmark complex tokens
    let complex_input = r#"
    greeting's text
    number1 + number2 >= 100
    "Hello \"escaped\" World"
    // This is a comment
    yes and no or maybe
    "#;

    c.bench_function("lexer_complex", |b| {
        b.iter(|| {
            let lexer = Lexer::new(black_box(complex_input));
            let _ = lexer.collect_tokens();
        });
    });
}

criterion_group!(benches, benchmark_lexer);
criterion_main!(benches);