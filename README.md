# WFL (WebFirst Language)

WFL is a programming language designed to be readable and intuitive, using natural language constructs to lower the barrier to entry for new programmers while still providing powerful features for experienced developers.

## Overview

WFL features a syntax that resembles English sentences, indentation-based structure, and modern programming concepts like containers (classes), actions (functions), and collections. The language is designed to be approachable for beginners while still being powerful enough for real-world applications.

## Project Status

The WFL compiler is currently under development. Here's the current status:

- ✅ **Lexer**: Complete - Converts source code into tokens
- ✅ **Parser**: Complete - Transforms tokens into an Abstract Syntax Tree (AST)
- 🔄 **Bytecode Compiler**: In Progress - Will convert the AST into bytecode instructions
- 🔄 **Virtual Machine**: Planned - Will execute bytecode instructions

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
  - `bytecode/`: Bytecode compiler (in progress)
- `Docs/`: Documentation
  - `wfl-spec.md`: Language specification
  - `implementation_progress.md`: Implementation status
- `Test Programs/`: Example WFL programs

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
