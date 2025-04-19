# WFL Error Catalog

This document catalogs common error messages in WFL and provides explanations and solutions for each.

## Parse Errors

### Missing 'as' Keyword in Variable Declarations

**Error Message:**
```
Expected 'as' after identifier(s), but found IntLiteral(42)
```

**Example Code:**
```wfl
store greeting 42
```

**Explanation:**
The `store` and `create` statements require the `as` keyword between the variable name and its value.

**Solution:**
Add the `as` keyword before the value:
```wfl
store greeting as 42
```

### Missing 'to' Keyword in Assignments

**Error Message:**
```
Expected 'to' after identifier(s), but found IntLiteral(10)
```

**Example Code:**
```wfl
change counter 10
```

**Explanation:**
The `change` statement requires the `to` keyword between the variable name and its new value.

**Solution:**
Add the `to` keyword before the value:
```wfl
change counter to 10
```

## Type Errors

### Type Mismatch in Operations

**Error Message:**
```
Cannot add number and text - Expected Number but found Text
```

**Example Code:**
```wfl
display x plus y  // where x is a number and y is text
```

**Explanation:**
WFL requires matching types for binary operations like addition.

**Solution:**
Convert the text to a number before the operation:
```wfl
display x plus (convert y to number)
```

## Semantic Errors

### Undefined Variable

**Error Message:**
```
Variable 'countt' is not defined
```

**Example Code:**
```wfl
display countt  // Misspelled variable name
```

**Explanation:**
The variable name used has not been declared in the current scope.

**Solution:**
Check for typos in the variable name or declare the variable before using it:
```wfl
store count as 0
display count
```

## Runtime Errors

### Division by Zero

**Error Message:**
```
Division by zero
```

**Example Code:**
```wfl
display 10 divided by x  // where x is 0
```

**Explanation:**
Division by zero is mathematically undefined and causes a runtime error.

**Solution:**
Add a check to prevent division by zero:
```wfl
check if x is not equal to 0:
  display 10 divided by x
otherwise:
  display "Cannot divide by zero"
end check
```
