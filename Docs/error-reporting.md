# WFL Error Reporting System
   
WFL's error reporting system is designed to provide clear, actionable, and user-friendly error messages. The system is inspired by languages like Elm, which are known for their helpful and informative error messages.
   
## Features
   
- **Structured Error Data**: Each error has a structured representation with:
  - Severity (error, warning, note, help)
  - Message (clear description of the issue)
  - Line and column information
  - Additional context specific to the error type (e.g., expected vs. found types)
   
- **Diagnostic Formatting**: Errors are displayed with:
  - Source code snippets showing the exact location of the error
  - Colorized output (where supported)
  - Additional notes and suggestions for fixing the error
   
- **Actionable Messages**: Error messages are designed to be:
  - Clear and understandable to beginners
  - Specific about what went wrong
  - Helpful in suggesting how to fix the issue
   
## Error Types
   
The compiler can produce several types of errors:
   
1. **Parse Errors**: Syntax errors in the code
   - Example: Missing closing parenthesis, unexpected token
   
2. **Semantic Errors**: Issues with the meaning of the code
   - Example: Using a variable that hasn't been declared
   
3. **Type Errors**: Type mismatches or invalid operations
   - Example: Adding a number to a string without conversion
   
4. **Runtime Errors**: Issues that occur during program execution
   - Example: Division by zero, accessing a non-existent file
   
## Example Error Messages
   
### Parse Error
   
```
error: Expected expression after 'as'
  --> example.wfl:3:11
   |
 3 | store z as
   |           ^ Error occurred here
```

### Missing Keyword Error

```
error: Expected 'as' after identifier(s), but found IntLiteral(42)
  --> example.wfl:3:14
   |
 3 | store greeting 42
   |              ^ Error occurred here
   |
   = Note: Did you forget to use 'as' before assigning a value? For example: `store greeting as 42`
```

```
error: Expected 'to' after identifier(s), but found IntLiteral(10)
  --> example.wfl:5:16
   |
 5 | change counter 10
   |                ^ Error occurred here
   |
   = Note: Did you forget to use 'to' before assigning a value? For example: `change counter to 10`
```
   
### Type Error
   
```
error: Cannot add number and text - Expected Number but found Text
  --> example.wfl:3:12
   |
 3 | display x plus y
   |            ^ Type error occurred here
   |
   = Note: Try converting the text to a number using 'convert to number'
```
   
### Semantic Error
   
```
error: Variable 'countt' is not defined
  --> example.wfl:5:9
   |
 5 | display countt
   |         ^^^^^^ Semantic error occurred here
   |
   = Note: Did you misspell the variable name? Did you mean 'count'?
```
   
### Runtime Error
   
```
error: Division by zero
  --> example.wfl:7:14
   |
 7 | display 10 divided by x
   |              ^ Runtime error occurred here
   |
   = Note: Check your divisor to ensure it's never zero
```
   
## Implementation Details
   
The error reporting system uses the `codespan-reporting` library to format and display errors with source code snippets. The system is extensible, allowing for new error types and customized error messages.
   
When an error occurs, the compiler:
   
1. Creates a structured error representation
2. Converts it to a diagnostic with appropriate labels and notes
3. Displays the diagnostic with source code context
   
This approach ensures that errors are both machine-readable (for IDE integration) and human-friendly (for developers).
