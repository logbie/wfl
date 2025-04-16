# WFL (WebFirst Language) - TODO List

## Recent Progress
- [x] Created implementation_progress.md to track overall implementation status
- [x] Enhanced parser to support parenthesized expressions 
- [x] Improved check statement parsing
- [x] Added comment handling in parser's synchronize function
- [x] Fixed bytecode compiler for basic expressions and statements
- [x] Created simplified test.wfl file for validating basic functionality
- [x] Complete bytecode generation for all parser features
- [x] Started comprehensive test suite implementation
- [x] Created test runner

## Current Focus

### Parser Improvements
- [x] Implement container definition parsing
- [ ] Implement action definition parsing
- [ ] Add support for collection literals (lists, maps)
- [ ] Implement collection access expressions
- [ ] Add support for advanced function calls with named parameters
- [ ] Improve error handling and recovery

### Bytecode Compiler Enhancements
- [x] Complete bytecode generation for all parser features
- [ ] Implement type checking
- [ ] Add optimization passes

### Virtual Machine Implementation
- [ ] Design bytecode instruction set
- [ ] Implement VM execution loop
- [ ] Create runtime value representation
- [ ] Add runtime error handling
- [ ] Implement standard library functions

### Testing and Documentation
- [x] Create comprehensive test suite
  - [x] Set up test directory structure 
  - [x] Create test utility functions
  - [x] Implement example unit tests for lexer, parser, and bytecode compiler
  - [x] Implement example integration tests
  - [x] Set up end-to-end test infrastructure
  - [x] Complete test implementation for all components
  - [x] Create test runner
- [ ] Write language specification document
- [ ] Add code comments and developer documentation
- [ ] Create user documentation and tutorials

# WFL Implementation Todo List

This document outlines the remaining tasks for implementing the WFL (WebFirst Language) interpreter. Tasks are categorized by component and priority.

## High Priority Tasks

### Parser

- [ ] Implement container definition parsing
  - [ ] Support for container property definitions
  - [ ] Handle container method declarations
  
- [ ] Complete action (function) definition parsing
  - [ ] Parameter declaration with types
  - [ ] Function body parsing
  - [ ] Return value handling
  
- [ ] Function call parsing with named parameters
  - [ ] Support for positional and named arguments
  - [ ] Method call syntax on objects

- [ ] Collection initialization and access
  - [ ] List literal syntax `[1, 2, 3]`
  - [ ] Map literal syntax `{key: value}`
  - [ ] Index access for collections

### Bytecode Compiler

- [x] Implement compilation for container definitions
  - [x] Property storage and access
  - [x] Method compilation
  
- [x] Action definition compilation
  - [x] Parameter handling
  - [x] Scoping and variable environment
  - [x] Return value handling
  
- [x] Function call compilation
  - [x] Named parameter resolution
  - [x] Method calls on objects
  
- [x] Collection handling
  - [x] List and map creation
  - [x] Element access and modification

### Virtual Machine

- [ ] Create VM infrastructure
  - [ ] Instruction execution loop
  - [ ] Operand stack implementation
  
- [ ] Value representation
  - [ ] Basic types (numbers, strings, booleans)
  - [ ] Container objects
  - [ ] Functions as first-class values
  
- [ ] Runtime environment
  - [ ] Variable environment and scoping
  - [ ] Call stack management
  - [ ] Error handling during execution

## Medium Priority Tasks

- [ ] Implement proper error reporting throughout the compiler pipeline
- [ ] Add support for standard library functions
- [ ] Implement string interpolation
- [ ] Add type checking for basic operations

## Low Priority Tasks

- [ ] Performance optimizations
- [ ] REPL (Read-Eval-Print Loop) for interactive development
- [ ] Source maps for debugging
- [ ] Documentation generation from code comments

## Next Steps

1. Begin developing the virtual machine infrastructure
2. Implement the virtual machine execution loop
3. Create runtime value representations
4. Add support for standard library functions

## Recent Progress

- [x] Documentation created for project status tracking
  - [x] Created implementation_progress.md
  - [x] Created missing_parser_functions.md
  - [x] Created missing_bytecode_features.md
  - [x] Created wfl_compatibility_issues.md
  - [x] Updated README.md with documentation links
  
- [x] Parser improvements
  - [x] Added support for parenthesized expressions
  - [x] Improved error handling in the parser
  - [x] Enhanced synchronization to handle comments

- [x] Bytecode compiler foundation
  - [x] Basic expression compilation
  - [x] Variable declaration support
  - [x] Control flow statements (if/else, loops)
  - [x] Simple function calls
  - [x] Advanced function calls with named parameters
  - [x] Container and action definition compilation
  - [x] Collection operations (lists, maps)

## Current Focus

- Developing the virtual machine to execute the generated bytecode
- Implementing runtime value representation
- Adding support for standard library functions
- Improving error handling and debugging capabilities

See also:
- [missing_parser_functions.md](missing_parser_functions.md) for detailed parser implementation tasks
- [missing_bytecode_features.md](missing_bytecode_features.md) for bytecode compiler requirements
- [wfl_compatibility_issues.md](wfl_compatibility_issues.md) for integration challenges

# WFL Compiler TODO List

## High Priority
- [x] Bytecode Compiler: Add support for parenthesized expressions
- [x] Bytecode Compiler: Implement function definition compilation
- [x] Bytecode Compiler: Implement container definition compilation
- [x] Bytecode Compiler: Add support for function calls with named parameters
- [ ] Virtual Machine: Create basic VM for executing bytecode
- [ ] Parser: Fix handling of action (function) definitions
- [x] Parser: Improve container creation syntax parsing

## Medium Priority
- [x] Bytecode Compiler: Add support for list and map operations
- [x] Bytecode Compiler: Add support for element access (indexing)
- [ ] Bytecode Compiler: Optimize bytecode generation
- [ ] Parser: Add support for import statements
- [ ] Parser: Implement full error recovery in complex expressions
- [ ] Virtual Machine: Add proper error handling and stack traces
- [ ] Virtual Machine: Implement standard library functions
- [ ] Tooling: Add a REPL (Read-Eval-Print Loop) for interactive testing

## Low Priority
- [ ] Bytecode Compiler: Generate debug information
- [ ] Bytecode Compiler: Add bytecode verification step
- [ ] Virtual Machine: Implement garbage collection
- [ ] Parser: Support for method chaining
- [ ] Documentation: Create language specification
- [ ] Documentation: Generate API documentation
- [ ] Tooling: Add a debugger for WFL programs
- [ ] Tooling: Create visualization tools for bytecode structure

## Completed
- [x] Parser: Implement basic expression parsing
- [x] Parser: Add support for variable declarations
- [x] Parser: Implement if/else statement parsing
- [x] Parser: Add support for loop constructs
- [x] Parser: Implement check statement parsing
- [x] Parser: Add support for parenthesized expressions
- [x] Bytecode Compiler: Implement basic expression compilation
- [x] Bytecode Compiler: Add support for variable declarations
- [x] Bytecode Compiler: Implement control flow (if/else) compilation
- [x] Bytecode Compiler: Add support for loops 
- [x] Bytecode Compiler: Complete bytecode generation for all parser features

# WFL Bytecode Compiler Implementation TODO

## High Priority Tasks

- [x] Update `compile_statement` to handle all AST node types
  - [x] `ActionDefinition` (function definitions)
  - [x] `ContainerDefinition` (container/class definitions)
  - [x] `ExpressionStatement` with function calls and named parameters
  - [x] Named parameter support in function calls

- [x] Implement collection operations
  - [x] List initialization
  - [x] Map initialization
  - [x] List/Map indexing

- [x] Fix type compatibility between AST and bytecode compiler
  - [x] Update parameter handling
  - [x] Support return type annotations
  - [x] Support container fields and methods

## Medium Priority Tasks

- [ ] Enhance error handling
  - [ ] Line information for errors
  - [ ] Better error messages
  - [ ] Runtime error handling

- [ ] Add optimization passes
  - [ ] Constant folding
  - [ ] Dead code elimination

## Low Priority Tasks

- [ ] Documentation
  - [ ] Bytecode format
  - [ ] VM instruction set
  - [ ] Debugging information

- [ ] Testing
  - [ ] Unit tests for compiler
  - [ ] Integration tests with parser
  - [ ] Benchmark tests

## Completed Tasks

- [x] Basic expression compilation
- [x] Variable declarations
- [x] Control flow (if statements, loops)
- [x] Basic function calls 
- [x] Container definition compilation
- [x] Action definition compilation
- [x] Collection operations (lists, maps)
- [x] Function calls with named parameters 

# Parser Implementation TODOs

## Member Access Implementation

- [x] Investigate the current implementation for member access in the parser
- [ ] Implement dot notation handling in `parse_primary` to support container member access
- [ ] Test container member access syntax with examples
- [ ] Update parser to allow method calls on member access expressions

## Progress:
- Identified issue: The parser doesn't properly handle dot notation for member access
- Found that `TokenType::Dot` is already defined in the lexer
- The bytecode compiler already has MemberAccess expression compilation, but the parser doesn't create these nodes 

# WFL Compiler Todo List

## Parser Issues to Fix
- [ ] Fix container field access in container methods (issue in `container_test.wfl`)
- [ ] Implement proper collection handling in parser
- [ ] Address general parsing errors in more complex WFL syntax

## Tests to Fix
- [ ] `container_test.wfl` - Fix "Undefined variable: name" error inside container method
- [ ] `collection_test.wfl` - Fix "Undefined variable" errors for collections
- [ ] `test.wfl` and `hello.wfl` - Fix parsing errors

## Implementation Steps
1. Examine parser implementation to identify issues
2. Fix container field scope resolution in container methods
3. Implement collection parsing and handling
4. Test fixes against failing test files
5. Address any remaining parser issues 

# WFL hello.wfl Program Implementation Tasks

This section outlines the specific features needed to make the hello.wfl example program run successfully.

## Parsing Features Needed

- [~] Multi-word identifiers for variable names
  - [x] Support for `current language` and `display count` style field names
  - [ ] Update parser to handle spaces in identifiers for specific contexts

- [~] Object creation and instantiation
  - [x] Implement `create X as new "ContainerName"` syntax
  - [ ] Add instance creation operator

- [~] Container constructor implementation
  - [x] `when created:` block parsing
  - [x] Constructor body compilation

- [ ] Multi-property validation syntax
  - [ ] Support for `check language and greeting:` syntax
  - [ ] Implement validators like `must not be empty` and `must be at most X characters`

- [ ] String manipulation functions
  - [ ] Implement `join` keyword for string concatenation
  - [ ] Support for the `and` operator in string joining operations

- [~] Complex container field access
  - [x] Support for implicit `self` reference in container methods
  - [~] Fix field references within containers

- [ ] Collection membership testing
  - [ ] Implement `is in` operator for collections
  - [ ] Support for `check if X is in Y:` syntax

- [ ] Try-catch exception handling
  - [ ] Parse `try:` / `catch any error:` blocks
  - [ ] Support nested error handling

- [ ] Array literal syntax
  - [ ] Support for inline array literals like `["Spanish", "French", "Japanese"]`

- [ ] Variable manipulation operations
  - [ ] Support for `increase X by Y` syntax
  - [ ] Implement counter increment operations

## Bytecode Features Needed

- [ ] Add display/print functionality
  - [ ] Implement `display` keyword for console output
  - [ ] Support for string interpolation in display commands

- [ ] Collection operation bytecode
  - [ ] Support for storing values into collections at specific keys
  - [ ] Implement `store X in Y at Z` operations

- [ ] Exception handling
  - [ ] Bytecode support for try-catch blocks
  - [ ] Error propagation mechanics

- [ ] Method invocation on objects
  - [ ] Support for `perform object "method name"` syntax
  - [ ] Proper method resolution and binding

- [ ] Validation bytecode
  - [ ] Implement validation checks (empty strings, length limits)
  - [ ] Validation error generation

## VM Runtime Features

- [ ] Object instantiation runtime
  - [ ] Constructor invocation
  - [ ] Instance creation mechanics

- [ ] String concatenation operations
  - [ ] Optimize string joining operations

- [ ] Advanced collection operations
  - [ ] Membership testing
  - [ ] Collection iteration

## Testing Strategy

1. Create simplified versions of each feature
2. Test individual features in isolation
3. Gradually combine features to build up to the full hello.wfl program
4. Create regression tests for each implemented feature 