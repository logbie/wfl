# WFL (WebFirst Language)

WFL is a programming language designed to be readable and intuitive, using natural language constructs to lower the barrier to entry for new programmers while still providing powerful features for experienced developers.

## Overview

WFL features a syntax that resembles English sentences, indentation-based structure, and modern programming concepts like containers (classes), actions (functions), and collections. The language is designed to be approachable for beginners while still being powerful enough for real-world applications.

## Project Status

The WFL compiler is currently under development. Here's the current status:

- ✅ **Lexer**: Complete - Converts source code into tokens
- ✅ **Parser**: Complete - Transforms tokens into an Abstract Syntax Tree (AST)
- ✅ **Semantic Analyzer**: Complete - Analyzes the AST for semantic correctness
- ✅ **Type Checker**: Complete - Performs static type analysis on the AST
- 🔄 **Interpreter**: In Progress - Will execute the AST directly
- 🔄 **Bytecode Compiler**: Planned - Will convert the AST into bytecode instructions
- 🔄 **Virtual Machine**: Planned - Will execute bytecode instructions

## AI-Assisted Development

This project is developed with the assistance of AI:

- **Devin.ai**: Primary AI developer responsible for core implementation
- **ChatGPT (GPT-4)**: Assisted with code reviews and optimization
- **Claude (via Cline)**: Assisted with documentation and architectural design
- **Grok**: Indirectly contributed to the project through knowledge base

The combination of AI assistance with human oversight has allowed for rapid development while maintaining high code quality and documentation standards.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/logbie/wfl.git
   cd wfl
   ```

2. Build the project:
   ```
   cargo build
   ```

### Usage

To run a WFL program:

```
cargo run -- path/to/your/program.wfl
```

Or, after building:

```
./target/debug/wfl path/to/your/program.wfl
```

## Example WFL Program

```
store greeting as "Hello, World!"
display greeting

check if 5 is greater than 3:
  display "Math works!"
otherwise:
  display "Something is wrong with the universe."
end check

count from 1 to 5:
  display "Counting: " with the current count
end count
```

## Project Structure

- `src/`: Source code
  - `lexer/`: Lexical analyzer
  - `parser/`: Parser and AST
  - `analyzer/`: Semantic analyzer
  - `typechecker/`: Static type checker
  - `interpreter/`: Runtime interpreter (in progress)
  - `bytecode/`: Bytecode compiler (planned)
- `Docs/`: Documentation
  - `wfl-spec.md`: Language specification
  - `wfl-foundation.md`: Design principles
  - `wfl-error.md`: Error handling philosophy
  - `wfl-staticTypeChecker.md`: Type system design
  - `wfl-interpretor.md`: Interpreter design
  - `implementation_progress_2025-04-17.md`: Implementation status
- `Test Programs/`: Example WFL programs

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
