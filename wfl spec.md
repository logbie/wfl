Below is a single, unified document that merges all the core concepts of WebFirst Language (WFL). It is organized to be both **comprehensive** and **easy to understand**. Use it as a reference guide to learn and apply WFL’s natural-language programming approach.

---

# WebFirst Language (WFL) – Complete Specification

## Table of Contents

1. [Introduction and Guiding Principles](#1-introduction-and-guiding-principles)  
2. [Language Basics](#2-language-basics)  
3. [Variables and Data Types](#3-variables-and-data-types)  
4. [Flow Control](#4-flow-control)  
    - [Check Construct](#41-check-construct)  
    - [Loop Constructs](#42-loop-constructs)  
5. [Functions (Actions)](#5-functions-actions)  
6. [Containers (Classes)](#6-containers-classes)  
7. [File Operations](#7-file-operations)  
8. [Additional Features](#8-additional-features)  
    - [Pattern Matching](#81-pattern-matching)  
    - [Asynchronous Operations](#82-asynchronous-operations)  
    - [Error Handling](#83-error-handling)  
    - [Security Features](#84-security-features)  
9. [Best Practices](#9-best-practices)  
10. [Conclusion](#10-conclusion)

---

## 1. Introduction and Guiding Principles

### 1.1 What is WFL?
The **WebFirst Language (WFL)** is a programming language designed to be **readable** and **intuitive**, using **natural language constructs** in place of special characters and symbols. It aims to lower the barriers to entry for beginners, while also providing powerful features for experienced developers.

### 1.2 Guiding Principles

1. **Natural-Language Syntax**  
   - Use English-like statements to make code more accessible.

2. **Minimize Special Characters**  
   - Avoid symbols like `<, >, @, ^, %` unless absolutely necessary.

3. **Readability and Clarity**  
   - Write code that clearly communicates its intent.

4. **Explicit Over Implicit**  
   - No hidden behaviors—every action is clearly stated.

5. **Consistency**  
   - Uniform use of keywords to reduce confusion.

6. **Predictable Evaluation Order**  
   - Prevent unexpected execution flows by specifying order clearly.

7. **Clear and Actionable Errors**  
   - Provide user-friendly messages and guidance on fixes.

8. **Strict Type Safety**  
   - Encourage reliability and reduce runtime errors.

9. **Support for Modern Features**  
   - Includes asynchronous operations, pattern matching, concurrency, and more.

10. **Accessibility for Beginners**  
    - Simple enough for novices; rich enough for experts.

---

## 2. Language Basics

In WFL, code is often structured with **words** and **phrases** rather than symbols. Blocks are opened with statements like `check`, `for each`, `define action`, etc., and closed with an `end` statement.

**Example**:

```wfl
// Simple block structure
check something:
    // Code inside
otherwise:
    // Alternate code
end check
```

- **Indentation**: WFL encourages indentation for clarity but is not strictly whitespace-sensitive.  
- **Comments**: Use `//` for single-line comments.

---

## 3. Variables and Data Types

WFL supports a range of data types and provides **natural-language** ways to define and manipulate variables.

### 3.1 Declaring Variables

```wfl
store count as 0
store message as "Hello, World!"
store is active as yes
store unknown value as nothing
```

- **Numbers**: `42`, `10.99`, `1 million`, `-5`  
- **Text**: `"Some text"`, can be empty `""`  
- **Truth Values**: `yes`, `no`, `true`, `false`  
- **Special Values**: `nothing`, `missing`, `undefined`

### 3.2 Basic Operations

```wfl
add 5 to count
subtract 2 from count
join "Hello" and "World" into greeting  // greeting becomes "HelloWorld"
convert "123" to number
```

### 3.3 Collections

- **Lists**:

  ```wfl
  create list shopping:
      add "milk"
      add "bread"
      add "eggs"
  end list
  
  // Modifying the list
  add "cheese" to shopping
  remove "bread" from shopping
  ```

- **Sets**:

  ```wfl
  create set unique colors:
      add "red"
      add "blue"
      add "red"      // Duplicate ignored
  end set
  ```

- **Maps**:

  ```wfl
  create map settings:
      theme is "dark"
      volume is 75
      notifications are on
  end map
  ```

- **Records**:

  ```wfl
  create record person:
      name is "Alice"
      age is 30
      email is "alice@example.com"
      is member is yes
  end record
  ```

### 3.4 Type Validation

```wfl
check age:
    must be number
    must be at least 0
    must be less than 120
end check
```

---

## 4. Flow Control

Flow control in WFL is designed to be expressive and readable, reflecting **if/else** constructs, loops, and specialized checks.

### 4.1 Check Construct

The **check** block is central to WFL:

```wfl
check condition:
    // Code if condition is true
otherwise:
    // Code if condition is false
end check
```

- **Multi-condition checks**:

  ```wfl
  check value:
      if value is above 100 then
          // High value logic
      if value is between 50 and 100 then
          // Medium value logic
      otherwise:
          // Low value logic
  end check
  ```

- **Pattern Matching**:

  ```wfl
  check data matches pattern {
      type: one of ["user", "admin"],
      id: number where > 0,
      email: string where contains "@"
  }:
      // Process valid data
  otherwise:
      // Handle invalid data
  end check
  ```

- **Check with loops**:

  ```wfl
  foreach item in collection:
      check item:
          if item is valid then
              process item
          if item is broken then
              skip item
              continue foreach
      end check
  end foreach
  ```

- **Asynchronous check** (with timeouts, waiting, etc.):

  ```wfl
  check async operation with timeout 30s:
      while processing:
          if operation is pending then
              await delay 100ms
              continue check
          if operation is complete then
              process result
              break check
      end while
  otherwise:
      handle timeout
  end check
  ```

### 4.2 Loop Constructs

WFL’s loop constructs include **count**, **for each**, and **conditional** loops. All can use **natural language** flow controls like `stop`, `skip`, `pause`, and `resume`.

1. **Count Loop**  

   ```wfl
   count from 0 to 10:
       display "Iteration: " with count
   end count
   ```

2. **For Each (collection loop)**  

   ```wfl
   for each item in collection:
       process item
   end for
   ```

   - With index:

     ```wfl
     for each index and item in collection:
         display "Index: " with index
     end for
     ```

3. **Repeat While/Until**  

   ```wfl
   repeat while condition is true:
       // do something
   end repeat
   
   repeat until condition is true:
       // do something
   end repeat
   ```

4. **Control Keywords**  
   - `stop`: Exits the loop immediately  
   - `skip`: Skips the current iteration and moves to the next  
   - `pause` and `resume`: Temporarily halt or continue execution  
   - **Named loops**: 

     ```wfl
     count from 0 to 10 as outer:
         count from 0 to 5 as inner:
             check if inner is 3:
                 stop inner count
             end check
         end count
     end count
     ```

---

## 5. Functions (Actions)

In WFL, **functions** are referred to as **actions**. They can be simple one-liners or complex operations with parameters, return values, and modifiers.

### 5.1 Defining Actions

```wfl
define action say hello:
    display "Hello, World!"
end action
```

#### Parameters and Return Values

```wfl
define action add numbers:
    needs:
        x as number
        y as number
    gives back:
        sum as number
    do:
        give back x plus y
end action
```

#### Modifiers

- **Async**:

  ```wfl
  define async action fetch data:
      needs:
          url as text
      gives back:
          data as any
      do:
          store response as await fetch from url
          give back response data
  end action
  ```

- **Private** (action only accessible within the current container or file).
- **Cleanup block** with `try ... finally`.

### 5.2 Generic Actions

WFL supports **generics** via **Of Type T** or multiple type parameters:

```wfl
define action transform Of Type T:
    needs:
        value as T
        transformer as action taking T giving T
    gives back:
        result as T
    do:
        give back perform transformer with value
end action
```

### 5.3 Action Overloading

```wfl
define action format value:
    needs:
        number as number
    gives back:
        text
    do:
        give back format number with 2 decimals
end action

define action format value:
    needs:
        date as date
    gives back:
        text
    do:
        give back format date as "YYYY-MM-DD"
end action
```

---

## 6. Containers (Classes)

Containers in WFL are analogous to classes or objects in other languages. They group related **fields (stores)** and **actions (methods)**.

### 6.1 Basic Container

```wfl
create container User:
    private:
        store id as text
        store email as text

    protected:
        store role as text
        store permissions as list

    public:
        when created:
            needs:
                name as text
                email as text
            do:
                set id to generate unique id
                set email to email
                set role to "user"
                set permissions to default user permissions
        end when

        define action get profile:
            give back:
                id is id
                email is email
                role is role
            end back
        end action
end container
```

### 6.2 Inheritance

```wfl
create container Admin from User:
    private:
        store admin level as number

    public when created:
        needs:
            name as text
            email as text
            level as number
        do:
            parent when created with:
                name as name
                email as email
            end with
            set admin level to level
            set role to "admin"
    end when

    public define action get profile:
        store base as parent perform get profile
        give back base with:
            admin level is admin level
        end back
    end action
end container
```

### 6.3 Interfaces

```wfl
define interface Data Store:
    action save item:
        needs:
            item as any
        gives back:
            success as truth
    
    action load item:
        needs:
            id as text
        gives back:
            item as any
    
    action delete item:
        needs:
            id as text
        gives back:
            success as truth
end interface

// Implementation
create container File Store implements Data Store:
    private:
        store base path as text

    public:
        when created:
            needs:
                path as text
            do:
                set base path to path
        end when

        define action save item:
            needs:
                item as any
            gives back:
                success as truth
            do:
                try:
                    write item to file at base path
                    give back yes
                catch any error:
                    give back no
                end try
        end action
        // ... load item, delete item, etc.
end container
```

---

## 7. File Operations

WFL includes a powerful set of **file operation** constructs, maintaining the language’s natural syntax while ensuring robust functionality.

### 7.1 Core File Operations

```wfl
read "config.txt"
write "Hello" to "greeting.txt"
append "log entry" to "system.log"
delete "temp.txt"
```

Or more explicitly:

```wfl
read file "data.csv":
    mode is read only
    encoding is "utf8"
    line endings are "lf"
    buffer size is 8KB
end read

write to file "output.json":
    mode is write only
    create if missing
    encoding is "utf8"
    format is "pretty"
    indent with 2 spaces
end write
```

### 7.2 File Information and Manipulation

```wfl
get info about "document.txt":
    retrieve:
        size
        creation date
        last modified
        permissions
end get

check file "data.txt":
    exists
    is readable
    is writable
    is not empty
end check

copy "source.txt" to "destination.txt":
    create path if missing
    overwrite if exists
    verify after copy
end copy

move "old.txt" to "new/location.txt":
    create folders if needed
    overwrite if exists
    cleanup source after
end move
```

### 7.3 File Access and Security

```wfl
set permissions for "secure.txt":
    owner can:
        read
        write
        execute
    group "developers" can:
        read
        write
    others can:
        read only
    audit:
        log access attempts
end set

encrypt file "secret.txt":
    algorithm is "AES-256-GCM"
    key from secure storage "main-key"
    verify after encryption
end encrypt
```

---

## 8. Additional Features

### 8.1 Pattern Matching

Pattern matching in WFL allows you to **describe a structure** and ensure your data fits that structure before you process it. You can do so inside a `check ... matches pattern` block, which is especially useful for validation, security checks, or structured data parsing.

```wfl
check credentials match pattern {
    username: string where length > 0 and safe,
    password: string where length >= 8 and safe,
    mfa_code: optional string where length is 6 and numeric
}:
    proceed with authentication
otherwise:
    reject with "Invalid format"
end check
```

### 8.2 Asynchronous Operations

Asynchronous constructs let you **wait** for events, handle **timeouts**, and do **parallel** checks:

```wfl
check async operation with timeout 30s:
    while processing:
        if operation is pending then
            await delay 100ms
            continue check
        if operation is complete then
            process result
            break check
    end while
otherwise:
    handle timeout
end check
```

### 8.3 Error Handling

WFL encourages clear **try/catch**-style blocks with natural language:

```wfl
try process data:
    when error is temporary:
        retry up to 3 times
    when error is permanent:
        log error
        notify admin
    otherwise:
        continue
end try
```

You can also define **error zones** or **operation-specific handling**:

```wfl
create error zone "file processing":
    handle errors:
        file missing:
            try create file
        permission denied:
            try elevate permissions
            maximum retries is 3
    end handle
end zone
```

### 8.4 Security Features

- **Rate Limiting**:

  ```wfl
  check rate limit:
      monitor attempts:
          if exceed 3 in 5m then
              lock for 15m
          if exceed 5 in 1h then
              require account_review
      end monitor
  end check
  ```

- **Resource Locking** to prevent race conditions:

  ```wfl
  check operation:
      unless in_progress:
          lock resource
          try process:
              perform operation
          finally:
              unlock resource
          end try
      end unless
  end check
  ```

---

## 9. Best Practices

### 9.1 Readability
1. Use **descriptive condition names** and clear variables.  
2. Keep blocks **focused**.  
3. **Comment** complex logic.  

### 9.2 Performance
1. Use **timeouts** for long-running checks.  
2. Consider **caching**.  
3. Use **parallel** processing where appropriate.  
4. **Batch** similar operations for efficiency.  

### 9.3 Security
1. Always **validate inputs** (paths, credentials, user inputs).  
2. Use **encryption** for sensitive data.  
3. Keep proper **access controls**.  
4. Maintain **audit logs**.  

### 9.4 Error Handling
1. Define **clear** error messages.  
2. Offer **recovery** steps.  
3. Clean up resources in `finally` blocks.

### 9.5 Scalability and Maintenance
1. Keep containers (classes) and actions (functions) well-structured.  
2. Maintain a **module** system for large projects.  
3. Provide **testing** and **validation** for critical features.

---

## 10. Conclusion

The **WebFirst Language (WFL)** merges **readability**, **natural syntax**, and **powerful features** to streamline development. By adhering to its guiding principles—natural-language constructs, minimized special characters, explicit behavior, and strong type safety—you can write code that is not only easier to create and maintain but also robust and secure.

Use this specification as a comprehensive reference for:
- **Variable management**  
- **Data types**  
- **Check constructs**  
- **Loops**  
- **Functions (Actions)**  
- **Containers (Classes)**  
- **File operations**  
- **Asynchronous flows**  
- **Security and error handling**  

With WFL, your code can become **more expressive**, **understandable**, and **inclusive**—empowering both beginners and experienced developers to work confidently and collaboratively.

---

### Looking for More?

- **Tutorials**: Step-by-step guides and examples for WFL’s most common tasks.  
- **Advanced Patterns**: Delve into concurrency, custom pattern matching, and specialized containers.  
- **Community Resources**: Join forums and user groups to share code and get help.  

WFL is continuously evolving, and the community thrives on collaboration. Happy coding in WebFirst Language!