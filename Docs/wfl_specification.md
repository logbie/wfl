# WFL Language Specification

## 1. Introduction

WebFirst Language (WFL) is designed to be a readable and intuitive programming language, using natural language constructs instead of special characters and symbols. It aims to lower the barriers to entry for beginners while providing powerful features for experienced developers.

### 1.1 Design Goals

- **Readability**: Code should be easily understood by both programmers and non-programmers.
- **Minimal Syntax**: Reduce the use of special characters and complex symbols.
- **Type Safety**: Provide strong typing to catch errors early.
- **Modern Features**: Support modern programming paradigms including object-oriented and functional programming.
- **Clear Error Messages**: Provide actionable and understandable error messages.

### 1.2 Language Overview

WFL is a general-purpose programming language with the following key features:
- C-like syntax with a focus on natural language keywords
- Static typing with type inference
- First-class functions
- Container-based object system
- Built-in collection types
- Pattern matching
- Error handling

## 2. Lexical Structure

### 2.1 Comments

Comments in WFL begin with the `#` character and continue until the end of the line:

```wfl
# This is a comment
define variable x = 10  # This is an end-of-line comment
```

### 2.2 Identifiers

Identifiers in WFL can include letters, numbers, and underscores, and must start with a letter or underscore:

```wfl
myVariable
_privateVar
counter1
some_name
```

WFL also supports multi-word identifiers in certain contexts:

```wfl
define field user name
define field display count
```

### 2.3 Keywords

The following are reserved keywords in WFL:

```
define    variable    container    field    action    
if        else        while        end      check
and       or          not          true     false
return    when        created      new      print
```

### 2.4 Literals

#### 2.4.1 Numeric Literals

```wfl
42        # Integer
3.14      # Floating-point
```

#### 2.4.2 String Literals

Strings are enclosed in double quotes:

```wfl
"Hello, World!"
"This is a string with \"quotes\" inside"
```

#### 2.4.3 Boolean Literals

```wfl
true
false
```

#### 2.4.4 Collection Literals

Lists:
```wfl
[1, 2, 3, 4]
["a", "b", "c"]
[]  # Empty list
```

Maps:
```wfl
{"name": "John", "age": 30}
{}  # Empty map
```

## 3. Data Types

### 3.1 Basic Types

- **Number**: Represents both integer and floating-point values
- **String**: Sequence of Unicode characters
- **Boolean**: `true` or `false`

### 3.2 Collections

- **List**: Ordered collection of values
- **Map**: Key-value pairs

### 3.3 User-defined Types

- **Container**: User-defined object type with fields and methods

## 4. Expressions

### 4.1 Literals

As described in section 2.4.

### 4.2 Variables

```wfl
myVariable
```

### 4.3 Operators

#### 4.3.1 Arithmetic Operators

- `+`: Addition
- `-`: Subtraction
- `*`: Multiplication
- `/`: Division

#### 4.3.2 Comparison Operators

- `==`: Equal to
- `!=`: Not equal to
- `>`: Greater than
- `<`: Less than
- `>=`: Greater than or equal to
- `<=`: Less than or equal to

#### 4.3.3 Logical Operators

- `and`: Logical AND
- `or`: Logical OR
- `not`: Logical NOT

### 4.4 Function Calls

```wfl
print("Hello, World!")
calculateTotal(price, quantity)
```

### 4.5 Member Access

```wfl
person.name
counter.value
```

### 4.6 Collection Access

```wfl
myList[0]
users["admin"]
matrix[i][j]
```

### 4.7 Grouping

Expressions can be grouped with parentheses:

```wfl
(a + b) * c
```

## 5. Statements

### 5.1 Variable Declaration

```wfl
define variable name = "John"
define variable age = 30
define variable isActive = true
```

### 5.2 Assignment

```wfl
name = "Jane"
counter = counter + 1
```

### 5.3 Conditional Statements

#### 5.3.1 If Statement

```wfl
if x > 10
    print("x is greater than 10")
end if
```

#### 5.3.2 If-Else Statement

```wfl
if x > 10
    print("x is greater than 10")
else
    print("x is not greater than 10")
end if
```

### 5.4 Loops

#### 5.4.1 While Loop

```wfl
define variable counter = 0
while counter < 5
    print(counter)
    counter = counter + 1
end while
```

### 5.5 Check Statement

The `check` statement is used for validation and assertions:

```wfl
check x > 0:
    print("x must be positive")
    x = 0
end check
```

### 5.6 Return Statement

```wfl
return result
```

## 6. Containers

Containers are WFL's equivalent of classes or objects. They encapsulate data and behavior.

### 6.1 Container Definition

```wfl
define container Person
    define field name
    define field age
    
    define action greet
        print("Hello, my name is " + name)
    end action
    
    define action setAge(newAge)
        age = newAge
    end action
end container
```

### 6.2 Constructor

```wfl
define container Counter
    define field value
    
    when created
        value = 0
    end when
    
    define action increment
        value = value + 1
    end action
end container
```

### 6.3 Creating Container Instances

```wfl
define variable person = new Person
person.name = "John"
person.age = 30

define variable counter = new Counter
counter.increment()
```

## 7. Actions

Actions are functions or methods in WFL.

### 7.1 Defining Actions

```wfl
define action add(a, b)
    return a + b
end action
```

### 7.2 Actions in Containers

```wfl
define container Calculator
    define action add(a, b)
        return a + b
    end action
    
    define action subtract(a, b)
        return a - b
    end action
end container
```

### 7.3 Calling Actions

```wfl
define variable result = add(5, 3)

define variable calc = new Calculator
define variable sum = calc.add(5, 3)
```

### 7.4 Named Parameters

```wfl
define action createPerson(name, age)
    define variable person = new Person
    person.name = name
    person.age = age
    return person
end action

define variable john = createPerson(name: "John", age: 30)
```

## 8. Collections

### 8.1 Lists

```wfl
define variable numbers = [1, 2, 3, 4, 5]
define variable first = numbers[0]
numbers[1] = 10

define variable empty = []
```

### 8.2 Maps

```wfl
define variable person = {"name": "John", "age": 30}
define variable name = person["name"]
person["email"] = "john@example.com"

define variable empty = {}
```

### 8.3 Collection Operations

```wfl
# List operations
define variable length = numbers.length()
define variable contains = numbers.contains(3)

# Map operations
define variable keys = person.keys()
define variable hasEmail = person.hasKey("email")
```

## 9. Error Handling

### 9.1 Try-Catch Blocks

```wfl
try:
    divide(10, 0)
catch any error:
    print("An error occurred: " + error.message)
end try
```

### 9.2 Check Statements

```wfl
check x > 0:
    print("x must be positive")
    x = 0
end check
```

## 10. Standard Library

The WFL standard library includes functions for common operations:

### 10.1 Input/Output

```wfl
print(value)  # Display a value
define variable input = readLine()  # Read user input
```

### 10.2 String Functions

```wfl
define variable length = "hello".length()
define variable upper = "hello".toUpper()
define variable contains = "hello".contains("e")
define variable joined = join("Hello", " ", "World")
```

### 10.3 Math Functions

```wfl
define variable absolute = abs(-10)
define variable rounded = round(3.7)
define variable maximum = max(5, 10)
```

### 10.4 Collection Functions

```wfl
define variable length = [1, 2, 3].length()
define variable sorted = [3, 1, 2].sort()
define variable keys = {"a": 1, "b": 2}.keys()
```

## 11. Examples

### 11.1 Hello World

```wfl
print("Hello, World!")
```

### 11.2 Factorial

```wfl
define action factorial(n)
    if n <= 1
        return 1
    else
        return n * factorial(n - 1)
    end if
end action

define variable result = factorial(5)  # 120
print(result)
```

### 11.3 Container Example

```wfl
define container Counter
    define field value
    
    when created
        value = 0
    end when
    
    define action increment
        value = value + 1
        return value
    end action
    
    define action decrement
        value = value - 1
        return value
    end action
    
    define action getValue
        return value
    end action
end container

define variable counter = new Counter
print(counter.increment())  # 1
print(counter.increment())  # 2
print(counter.decrement())  # 1
print(counter.getValue())   # 1
```

### 11.4 List Processing

```wfl
define variable numbers = [1, 2, 3, 4, 5]
define variable sum = 0

define variable i = 0
while i < numbers.length()
    sum = sum + numbers[i]
    i = i + 1
end while

print("Sum: " + sum)  # Sum: 15
```

## 12. Grammar

The following is a simplified grammar for WFL in EBNF notation:

```ebnf
program = statement*

statement = var_declaration
          | assignment
          | if_statement
          | while_statement
          | check_statement
          | container_definition
          | action_definition
          | return_statement
          | expression_statement

var_declaration = "define" "variable" IDENTIFIER "=" expression

assignment = expression "=" expression

if_statement = "if" expression statement* ("else" statement*)? "end" "if"

while_statement = "while" expression statement* "end" "while"

check_statement = "check" expression ":" statement* "end" "check"

container_definition = "define" "container" IDENTIFIER 
                      (field_definition | action_definition | constructor)*
                      "end" "container"

field_definition = "define" "field" IDENTIFIER

action_definition = "define" "action" IDENTIFIER "(" parameter_list? ")"
                   statement*
                   "end" "action"
                   
constructor = "when" "created" statement* "end" "when"

return_statement = "return" expression

expression_statement = expression

expression = literal 
           | variable
           | binary_operation
           | unary_operation
           | function_call
           | member_access
           | index_access
           | grouping

literal = NUMBER | STRING | BOOLEAN | list_literal | map_literal

list_literal = "[" expression ("," expression)* "]"

map_literal = "{" (string ":" expression) ("," string ":" expression)* "}"

variable = IDENTIFIER

binary_operation = expression operator expression

unary_operation = operator expression

function_call = expression "(" argument_list? ")"

member_access = expression "." IDENTIFIER

index_access = expression "[" expression "]"

grouping = "(" expression ")"

parameter_list = IDENTIFIER ("," IDENTIFIER)*

argument_list = expression ("," expression)*
              | (IDENTIFIER ":" expression) ("," IDENTIFIER ":" expression)*

operator = "+" | "-" | "*" | "/" | "==" | "!=" | ">" | "<" | ">=" | "<=" | "and" | "or" | "not"
```

## 13. Future Language Extensions

Future versions of WFL may include:
- Pattern matching
- Asynchronous programming with async/await
- Modules and imports
- Type annotations and generics
- Exception handling improvements
- More collection operations
- Enhanced standard library

## 14. Differences from Other Languages

WFL draws inspiration from several languages while maintaining its own unique design:

- Python-like readability with significant whitespace
- JavaScript-like object model
- Ruby-like focus on natural language
- Rust-like pattern matching (planned)
- Swift-like optionals and error handling (planned)

## 15. Best Practices

### 15.1 Naming Conventions

- Use descriptive names for variables, actions, and containers
- Container names should be PascalCase (e.g., `Person`, `Counter`)
- Variable and action names should be camelCase (e.g., `userName`, `calculateTotal`)
- Constants should be in UPPER_SNAKE_CASE (e.g., `MAX_SIZE`)

### 15.2 Code Organization

- Group related containers together
- Keep actions focused on a single responsibility
- Use descriptive comments for complex logic
- Structure code with appropriate indentation

### 15.3 Error Handling

- Use check statements for validation
- Handle errors explicitly with try-catch blocks
- Provide meaningful error messages

---

This specification document describes WFL version 1.0. 