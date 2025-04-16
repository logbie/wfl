/// Opcodes for the WFL virtual machine
#[derive(Debug, Clone)]
pub enum OpCode {
    // Stack operations
    Constant(usize),         // Push a constant from the constant pool onto the stack
    Pop,                     // Pop the top value from the stack
    
    // Local variables
    GetLocal(usize),         // Get a local variable and push its value onto the stack
    SetLocal(usize),         // Set a local variable to the value on top of the stack
    DefineLocal(usize),      // Define a new local variable with the value on top of the stack
    
    // Arithmetic operations
    Add,                     // Add the top two values on the stack
    Subtract,                // Subtract the top value from the second-to-top value
    Multiply,                // Multiply the top two values
    Divide,                  // Divide the second-to-top value by the top value
    Modulo,                  // Compute the remainder of the second-to-top value divided by the top value
    Negate,                  // Negate the top value
    
    // Comparison operations
    Equal,                   // Check if the top two values are equal
    Greater,                 // Check if the second-to-top value is greater than the top value
    Less,                    // Check if the second-to-top value is less than the top value
    
    // Logical operations
    Not,                     // Logical NOT of the top value
    And,                     // Logical AND of the top two values
    Or,                      // Logical OR of the top two values
    
    // Control flow
    Jump(usize),             // Jump to an absolute position in the bytecode
    JumpIfFalse(usize),      // Jump if the top value is falsey
    JumpIfTrue(usize),       // Jump if the top value is truthy
    
    // Function operations
    Call(String, usize),     // Call a function by name with a specific number of arguments
    Return,                  // Return from the current function
    
    // Collection operations
    NewList(usize),          // Create a new list with the top n values from the stack
    NewMap(usize),           // Create a new map with the top n key-value pairs from the stack
    GetItem,                 // Get an item from a collection using the top value as the index
    SetItem,                 // Set an item in a collection using the second value as the index and the top value as the value
    
    // String operations
    Join,                    // Join (concatenate) two strings
    
    // Other
    Print,                   // Print the top value (for debugging)
    Assert,                  // Assert that the top value is truthy (for debugging)
} 