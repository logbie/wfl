/// Opcodes for the WFL virtual machine
#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    // Stack operations
    Constant(usize),         // Push a constant from the constant pool onto the stack
    Pop,                     // Pop the top value from the stack
    Duplicate,               // Duplicate the top value on the stack
    
    // Local variables
    GetLocal(usize),         // Get a local variable and push its value onto the stack
    SetLocal(usize),         // Set a local variable to the value on top of the stack
    DefineLocal(usize),      // Define a new local variable with the value on top of the stack
    
    // Global variables
    GetGlobal(usize),        // Get a global variable and push its value onto the stack
    SetGlobal(usize),        // Set a global variable to the value on top of the stack
    DefineGlobal(usize),     // Define a new global variable with the value on top of the stack
    
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
    GreaterEqual,            // Check if the second-to-top value is greater than or equal to the top value
    LessEqual,               // Check if the second-to-top value is less than or equal to the top value
    
    // Logical operations
    Not,                     // Logical NOT of the top value
    And,                     // Logical AND of the top two values
    Or,                      // Logical OR of the top two values
    
    // Control flow
    Jump(usize),             // Jump to an absolute position in the bytecode
    JumpIfFalse(usize),      // Jump if the top value is falsey
    JumpIfTrue(usize),       // Jump if the top value is truthy
    
    // Function operations
    Call(usize),             // Call a function with a specific number of arguments
    Return,                  // Return from the current function
    Closure(usize),          // Create a closure from a function prototype
    
    // Container operations
    NewContainer,            // Create a new container (class instance)
    GetProperty,             // Get a property from an object
    SetProperty,             // Set a property on an object
    DefineField,             // Define a field on a container
    DefineMethod,            // Define a method on a container
    
    // Collection operations
    NewList(usize),          // Create a new list with the top n values from the stack
    NewMap(usize),           // Create a new map with the top n key-value pairs from the stack
    GetIndex,                // Get an item from a collection using the top value as the index
    SetIndex,                // Set an item in a collection using the second value as the index and the top value as the value
    AddList,                 // Add an element to a list
    ListAppend,              // Append an element to a list
    
    // String operations
    Join,                    // Join (concatenate) two strings
    
    // Special values
    Null,                    // Push a null value onto the stack
    
    // Other
    Print,                   // Print the top value (for debugging)
    Assert,                  // Assert that the top value is truthy (for debugging)
} 