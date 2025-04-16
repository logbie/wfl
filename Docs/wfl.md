# WebFirst Language (WFL)

## Introduction

### Project Statement

The **WebFirst Language (WFL)** is a programming language designed to be readable and intuitive, using natural language constructs instead of special characters and symbols. It aims to lower the barriers to entry for beginners while providing powerful features for experienced developers. WFL bridges the gap between human communication and code, enabling developers to write clear, readable, and maintainable programs.

### Guiding Principles

1. **Natural-Language Syntax**
   - Use English-like statements to make code more accessible
   - Reduce the learning curve for beginners and enhance readability

2. **Minimize Special Characters**
   - Rely on words and phrases instead of symbols
   - Make the language more accessible and less intimidating for newcomers

3. **Readability and Clarity**
   - Prioritize code that clearly communicates its intent
   - Facilitate maintenance and collaboration

4. **Explicit Over Implicit Behavior**
   - Require that actions and conditions are clearly stated
   - Minimize ambiguity and enhance predictability

5. **Consistency in Keyword Usage**
   - Use a uniform set of keywords and constructs
   - Streamline the learning process and reduce potential errors

6. **Clear Error Reporting**
   - Provide user-friendly error messages with guidance
   - Help developers quickly identify and fix issues

7. **Type Safety**
   - Enforce strict type checking and compatibility
   - Prevent runtime errors and enhance code reliability

8. **Modern Programming Features**
   - Support for containers (classes), actions (functions), and collections
   - Include features like error handling and validation

---

## Core Language Concepts

### Variables and Data Types

Based on hello.wfl, WFL uses a clear and consistent approach to variables and data types:

#### Basic Values

```wfl
// Numbers
store display count as number        // Numeric variable
store greeting text as text          // Text variable
store success as truth value         // Boolean variable

// Variable initialization
set greetings to:                    // Map initialization
    "English" is "Hello"
    "Spanish" is "Hola"
    "French" is "Bonjour"
    "Japanese" is "こんにちは"
end set
set current language to "English"    // Text assignment
set display count to 0               // Number assignment
```

#### Collections

```wfl
// Maps (Key-Value Pairs)
store greetings as map               // Map declaration
set greetings to:                    // Map initialization
    "English" is "Hello"
    "Spanish" is "Hola"
    "French" is "Bonjour"
end set

// Lists
for each language in ["Spanish", "French", "Japanese"]:  // List literal
    // Process each language
end for each
```

#### Variable Operations

```wfl
// Incrementing
increase display count by 1

// String concatenation
store message as join greeting text and ", " and name and "!"
join message and " (displayed " and display count and " times)"

// Variable access
store greeting text as perform "get greeting" with language
```

#### Validation

```wfl
// Input validation
check name:
    must not be empty
    must be at most 100 characters
end check

// Conditional checks
check if language is in greetings:
    give back greetings at language
otherwise:
    give back greetings at "English"
end check

check language and greeting:
    must not be empty
    must be at most 50 characters
end check
```

---

### Functions (Actions) and Containers

Based on hello.wfl, WFL uses a clear syntax for defining functions (called "actions") and object-oriented containers:

#### Actions (Functions)

```wfl
// Basic action definition
define action called "run hello world":
    does:
        // Action implementation
        display "Hello, World!"
    end action
end action

// Action with parameters and return value
define action called "say hello":
    needs:
        name as text with default "World"
        language as text with default "English"
    gives back:
        response as text
    does:
        // Implementation
        store message as join "Hello, " and name and "!"
        give back message
    end action
end action

// Private action
define private action called "get greeting":
    needs:
        language as text
    gives back:
        greeting as text
    does:
        // Implementation
        give back "Hello"
    end action
end action
```

#### Calling Actions

```wfl
// Simple action call
perform "run hello world"

// Action call with parameters
store result as perform greeter "say hello" with:
    name as "Programmer"
    language as "English"
end with

// Action call with single parameter
store greeting text as perform "get greeting" with language
```

#### Containers (Classes)

```wfl
// Container definition
create container called "AdvancedGreeting":
    // Private members
    private:
        store greetings as map
        store current language as text
        store display count as number
        
    // Public members
    public:
        // Constructor
        when created:
            does:
                // Initialize members
                set greetings to:
                    "English" is "Hello"
                    "Spanish" is "Hola"
                    "French" is "Bonjour"
                end set
                set current language to "English"
                set display count to 0
        end when

        // Public method
        define action called "say hello":
            needs:
                name as text with default "World"
                language as text with default "English"
            gives back:
                response as text
            does:
                // Implementation
                give back "Hello, World!"
            end action
        end action
end container
```

#### Creating and Using Container Instances

```wfl
// Creating an instance
create greeter as new "AdvancedGreeting"

// Calling a method on the instance
store result as perform greeter "say hello"
```

### Control Flow

Based on hello.wfl, WFL provides several control flow structures:

#### Conditional Statements

```wfl
// Simple if-else check
check if language is in greetings:
    give back greetings at language
otherwise:
    give back greetings at "English"
end check

// Validation checks
check name:
    must not be empty
    must be at most 100 characters
end check
```

#### Loops

```wfl
// For each loop with array literal
for each language in ["Spanish", "French", "Japanese"]:
    store result as perform greeter "say hello" with:
        name as "World"
        language as language
    end with
    display result
end for each
```

### Error Handling

WFL provides robust error handling with try-catch blocks:

```wfl
// Try-catch block
try:
    check language and greeting:
        must not be empty
        must be at most 50 characters
    end check
    
    store greeting in greetings at language
    give back yes
catch any error:
    give back no
end try

// Error handling with specific error types
try:
    perform greeter "say hello" with:
        name as ""  // Empty name should trigger validation
    end with
catch any error:
    display "Caught expected validation error!"
end try
```
## Advanced Features

Based on hello.wfl, WFL includes several advanced features that make it powerful yet approachable:

### Object-Oriented Programming

```wfl
// Container instantiation
create greeter as new "AdvancedGreeting"

// Method calls with named parameters
store result as perform greeter "say hello" with:
    name as "Programmer"
    language as "German"
end with
```

### Display and Output

```wfl
// Simple display
display result

// Display with error handling
display "Caught expected validation error!"
```

### Collections and Iteration

```wfl
// Map access
give back greetings at language

// Collection storage
store greeting in greetings at language

// Collection membership check
check if language is in greetings
```

### Best Practices

#### Readability Guidelines

1. **Use Descriptive Names**: Choose clear and meaningful names for variables, actions, and containers.
2. **Keep Code Blocks Focused**: Ensure each code block or function serves a single, clear purpose.
3. **Appropriate Indentation**: Use consistent indentation to enhance readability and structure.
4. **Comment Complex Logic**: Add comments to explain complex or non-obvious code sections.
5. **Prefer Natural Language Constructs**: Write code that reads like natural language to improve understanding.

#### Performance Considerations

1. **Use Validation**: Validate inputs early to prevent errors later in execution.
2. **Implement Error Handling**: Use try-catch blocks to gracefully handle errors.
3. **Structure Code Logically**: Organize code into containers and actions for better maintainability.
4. **Use Default Parameters**: Provide sensible defaults for action parameters.
5. **Leverage Collections**: Use appropriate collection types (maps, lists) for data storage.

#### Security Guidelines

1. **Validate All Inputs**: Always check and sanitize input data to prevent injection attacks.
2. **Handle Errors Appropriately**: Avoid exposing sensitive information in error messages.
3. **Implement Proper Access Controls**: Use private and public sections in containers.
4. **Use Encapsulation**: Keep implementation details private within containers.
5. **Validate Before Processing**: Check data validity before performing operations.

---

## Summary

The WebFirst Language (WFL) is designed to be readable and intuitive, using natural language constructs instead of special characters and symbols. It aims to lower the barriers to entry for beginners while providing powerful features for experienced developers.

Key features of WFL include:
- Natural language syntax with minimal special characters
- Container-based object-oriented programming
- Clear action (function) definitions with named parameters
- Robust error handling with try-catch blocks
- Strong validation capabilities
- Intuitive control flow structures

By adhering to these principles and guidelines, WFL aims to transform programming into an activity that is more inclusive, efficient, and aligned with the way humans naturally think and communicate. WFL seeks not only to be a tool for building software but also to be an enabler of innovation and creativity across the global developer community.
