# WFL Variables: Storing and Using Data in Plain English

**Summary:** In this section, we'll introduce **variables** in WebFirst Language (WFL) – how to create them, give them values, and use them in a very natural way. Think of variables as labeled containers for information. WFL is designed so that writing code feels like writing simple English sentences. Even if you've never coded before, you'll find WFL variables intuitive and friendly. By the end of this guide, you'll know how to store different types of data (numbers, text, yes/no values, etc.), update those values, group values into collections, and even set default values or validate data – all with easy-to-read syntax.

## What Is a Variable?

A **variable** is like a named **storage box** for a piece of data. You give the box a name and put something inside it. Later, you can use the name to get the data back or change it. In traditional programming, you might use symbols or cryptic syntax to handle variables, but in WFL we do it using plain English.

**Why use variables?** Suppose you want to remember a user's age or a price of an item. Instead of writing the number everywhere, you store it in a variable with a clear name (like "age" or "price"). This way, you can refer to that piece of information later by name, which makes your code easier to read and change.

## Creating and Naming Variables

In WFL, creating a variable reads almost like a simple sentence. You typically use the keyword **`store`** or **`create`** to introduce a new variable. For example:

```wfl
store name as "Alice"
```

This line **creates a variable** called **`name`** and **stores** the text `"Alice"` in it. It's like saying *"store name as Alice"* – very natural! Similarly, you can store numbers or other values. For instance:

```wfl
store count as 42
store temperature as -5
store message as "Hello, world!"
store is active as yes
```

Here we made a **`count`** variable with the number 42, a **`temperature`** with -5, a **`message`** with `"Hello, world!"`, and an **`is active`** flag with the value **yes** (WFL uses **yes/no** as natural equivalents to true/false). Notice that variable names can be multiple words (like **`is active`**) to clearly describe what they hold. WFL encourages descriptive names – it's perfectly fine to have spaces in the name. This makes the code read like English (e.g., *"is active is yes"*).

A couple more examples of different value types:

- **Numbers:** You can write numbers in a friendly way. `store price as 10.99` (a decimal) or even `store very large as 1 million` (WFL understands common phrases for large numbers).
- **Text:** Always put text in quotes. For example, `store greeting as "Hello"`.
- **Yes/No (Booleans):** Use **yes** or **no** (or **true/false**, which WFL also accepts). For example, `store has account as no`.
- **Special values:** WFL has words for "nothingness" – for cases when a variable has no value. You can use **nothing**, **missing**, or **undefined** depending on context. For example, `store unknown value as nothing` might indicate an absence of a value.

### Creating Constants

Sometimes you have a value that never changes (like π = 3.14159). You can mark it as a **constant**. In WFL, that's as easy as saying:

```wfl
create new constant PI as 3.14159
```

This works like a variable, except WFL knows you don't intend to change it later. Naming constants in all caps (like `PI`) is a common convention, but not required.

### A Note on "Let" Syntax

You might wonder if you can use more conversational phrasing. Indeed, one of WFL's goals is to let you write code that sounds like English. For example, you could say something like *"Let the user's name be Alice."* While the official syntax uses `store` or `create`, the idea is the same. The key is that WFL tries to be as readable as possible. So even if we use `store X as Y` in these examples, you can read it in your head as "let X be Y."

## Updating and Changing Variables

Once you have variables, you'll often want to change their values. WFL provides natural-language ways to do this, avoiding cryptic operators. Suppose we have:

```wfl
store count as 0
```

Now we want to update `count`. Here are some ways to change variables:

```wfl
change count to 1
add 5 to count
subtract 2 from count
multiply count by 3
divide count by 2
```

Let's go through those:

- **`change count to 1`:** This sets the value of `count` to 1 (basically an assignment).
- **`add 5 to count`:** Increases `count` by 5. If count was 1, it becomes 6.
- **`subtract 2 from count`:** Decreases `count` by 2.
- **`multiply count by 3`:** Multiplies the value of `count` by 3.
- **`divide count by 2`:** Divides the value of `count` by 2.

These are equivalent to `count = count + 5`, `count = count - 2`, etc., but notice how **readable** they are. You can almost read them as simple instructions in English, which is exactly WFL's goal!

You can use these operations for any numeric variable. They make the intent crystal clear (no need to remember what `+=` or `++` means, as in other languages).

### Working with Text

WFL also makes manipulating text (strings) straightforward:

```wfl
join "Hello, " and name into greeting
```

This would take `"Hello, "` and whatever is in `name` (say "Alice"), and **join** them together into a new variable `greeting`. After running that, `greeting` would contain `"Hello, Alice"`.

Other text operations include:

```wfl
take first 5 letters from message
take last 3 letters from name
convert message to lowercase
convert name to uppercase
```

What do these do? If `message` is `"Hello, World!"`, then:
- `take first 5 letters from message` would result in `"Hello"`.
- `take last 3 letters from name` (if name is "Alice") would give `"ice"`.
- `convert message to lowercase` would change `"Hello, World!"` to `"hello, world!"`.
- `convert name to uppercase` would change "Alice" to "ALICE".

Again, the syntax is self-explanatory: you read it as you would in English. *"Take the first 5 letters from message."* It's clear what's happening without needing any complex functions or slicing indices.

## Collections: Lists, Sets, and Maps

As you get more comfortable, you might want to group multiple values together. WFL offers a few natural ways to handle **collections** of items: **lists, sets,** and **maps**. These terms might sound technical, but think of it this way:
- A **list** is an ordered list of things (allowing duplicates).
- A **set** is a collection of unique things (no duplicates).
- A **map** is like a dictionary or phone book: it maps keys to values (key/value pairs).

### Lists (Ordered Collections)

A list in WFL is like a shopping list or to-do list – an ordered collection where the order matters and you can have the same item more than once. Here's how you create and use a list:

```wfl
create list shopping:
    add "milk"
    add "bread"
    add "eggs"
end list
```

This defines a list called `shopping` with three items: "milk", "bread", "eggs". The syntax is a block: you start with `create list [name]:`, then list each element with `add ...` on its own line, and finish with `end list`.

After creating a list, you can manipulate it with natural commands:
```wfl
add "butter" to shopping      // now shopping has milk, bread, eggs, butter
remove "eggs" from shopping   // now eggs is removed
clear shopping list           // now shopping is empty
```

As shown:
- `add "butter" to shopping` puts "butter" at the end of the list.
- `remove "eggs" from shopping` takes "eggs" out of the list.
- `clear shopping list` empties the list completely.

### Sets (Unique Collections)

A set is for when you need to ensure all items are unique. Creating a set is similar to a list:

```wfl
create set unique colors:
    add "red"
    add "blue"
    add "red"
end set
```

We tried to add "red" twice. WFL will automatically ignore duplicates in a set. So our `unique colors` set effectively contains just "red" and "blue". The order is not as important in a set, and you won't have repeats.

Think of a set like a collection of tags or labels – each label only listed once.

### Maps (Key-Value Pairs)

A map is like a dictionary or a simple database of key-value pairs. Each entry has a unique key and a value associated with that key. For example, imagine settings in an application:

```wfl
create map settings:
    theme is "dark"
    volume is 75
    notifications are on
end map
```

Here we created a map called `settings` with three key-value pairs:
- The key **theme** has value `"dark"`.
- **volume** is 75.
- **notifications** are on (on is synonymous with yes/true).

The syntax uses `key is value` or `key are value` inside the map block, which again reads like English descriptions. After this, you could query or change entries in `settings` in a natural way (e.g., change the volume or check the theme), though the exact syntax for that can depend on context (often similar to removing from a list, etc., but using the key).

Maps are great when you want to look up a value by a name. For instance, `settings` is like a configuration where you can ask "what is the volume?" and get 75.

## Grouping Multiple Values with Records

A **record** in WFL is a way to group several related values together under one name. You can think of it like a simple object or a structured data type. If you have information that naturally goes together (like a person's name, age, and email), you can store it in a record instead of separate variables.

For example:

```wfl
create record person:
    name is "John Smith"
    age is 30
    email is "john@example.com"
    is member is yes
end record
```

This defines a `person` record with four fields: `name`, `age`, `email`, and `is member`. It's like creating a structured variable where each field has its own value.

Using a record feels intuitive:
- To **get** a field from a record, you can use a possessive style. For example, `get person's email` would retrieve `"john@example.com"`. (It reads like *"get person's email"*. In WFL, the apostrophe in code might not actually be required – you might simply write `get person email` – but conceptually it's the same idea of ownership.)
- To **change** a field: `change person's age to 31` would update the age in the `person` record.
- To **remove** a field: `remove person's email` would remove the email information from the record (perhaps setting it to nothing or making it unavailable).

Records are helpful when you want to pass around or manage a bunch of related info as one unit. For instance, you could have a record for a user, a record for a product, etc., each bundling relevant attributes.

### Dates and Times

WFL even handles dates and times in a friendly way. You can create date or time values with simple phrases:

```wfl
create date today
create time now
create date tomorrow as today plus 1 day
create time meeting at "14:30"
```

Explanation:
- `create date today` sets `today` to the current date.
- `create time now` sets `now` to the current time.
- `create date tomorrow as today plus 1 day` is quite literal: tomorrow = today + 1 day.
- `create time meeting at "14:30"` sets a time variable `meeting` to 2:30 PM.

The syntax for adding time (like plus 1 day) or specifying a time in quotes is designed to be straightforward. You could similarly do `plus 5 days` or set dates to specific strings like `"2025-12-31"` if needed.

## Beyond the Basics: Converting and Validating Values

The features above cover the basics of storing and manipulating data. WFL also provides higher-level constructs to make sure your data is in the right format and to convert between types safely. These are more intermediate features – you might not need them when you first start – but it's nice to know they exist as you grow more confident.

### Converting Between Types

Sometimes you have a value in one form and need it in another. For example, you have a text `"123"` and you need it as a number `123` to do math, or vice versa. WFL lets you **convert** types with an easy phrase:

```wfl
convert "123" to number    // yields the number 123
convert 123 to text        // yields the text "123"
convert "yes" to truth value  // yields the boolean yes
```

These are straightforward:
- Converting `"123"` (a text) to number results in the numeric value 123.
- Converting `123` (number) to text gives `"123"` (as text).
- Converting `"yes"` to a truth value yields **yes** (which is like true).

If the conversion is impossible (for example, converting `"abc"` to a number), WFL would normally throw an error or stop. But WFL also provides a **safe conversion** mechanism to handle such cases gracefully:

```wfl
safely convert "abc" to number:
    when invalid:
        use 0 instead
    when missing:
        ask user for number
end convert
```

In this example, we attempt to convert the text `"abc"` to a number. Clearly, `"abc"` isn't a numeric string, so it would fail. The **`safely convert ...`** block catches that:
- `when invalid:` means if the conversion is invalid (e.g., not a number), then do the following indented steps. Here we say `use 0 instead`, meaning if `"abc"` can't convert, just use 0.
- `when missing:` would handle the case where the value was missing/undefined. In our example it doesn't apply, but we included it to show the pattern – maybe if the value was not there, we could prompt the user for a number.

This safe conversion block ensures your program can decide what to do if data isn't in the expected format, without crashing. It reads like a plan: *"Try to convert, if invalid use 0, if missing ask user."*

### Validating Values (Ensuring Data Quality)

When working with user input or any data, it's good to validate that it meets certain criteria. WFL has a friendly way to enforce rules on values using the **`check ... must ...`** syntax.

**Simple validation example:**

```wfl
check age:
    must be number
    must be at least 0
    must be less than 150
end check
```

This sets up requirements for the variable `age`. It reads:
- Age **must be a number** (so if `age` is text or anything not numeric, this check would fail).
- Age **must be at least 0** (no negative ages).
- Age **must be less than 150** (assuming an age should be in a reasonable range).

If any of these conditions are violated, WFL would throw a clear error message explaining which rule failed. This is great for catching mistakes early. The phrasing is so straightforward, it's like writing documentation for your data that the computer also reads.

Another example, more complex:

```wfl
check email:
    must not be empty
    must contain "@"
    must contain "." after "@"
    must not contain spaces
end check
```

This defines what a valid `email` should look like:
- It can't be empty.
- It must have an "@" symbol.
- It must have a "." that comes after the "@"
- It must not have any spaces.

These rules cover basic email format checking, and again, they are written in plain terms. If `email` doesn't meet these, WFL will report which rule was broken (e.g., "email must contain '@'").

You can also define a **named validation rule** if you plan to reuse it:

```wfl
create rule valid password:
    must be at least 8 characters
    must have uppercase letter
    must have lowercase letter
    must have number
    must have special character
end rule
```

This creates a reusable rule (kind of like a template for checking passwords). Later you might apply `valid password` to check a user's password input. The idea is to keep your code DRY (Don't Repeat Yourself) – define the checks once and reuse.

### Using Conditions in Expressions

We've seen how to enforce rules. WFL also lets you simply **check a condition** on the fly. For instance:

```wfl
check if temperature is above 0
check if name is "Alice"
```

These could be used inside other constructs (like in an if-statement, which we cover in the next section on Control Flow). The point here is that WFL uses very natural comparative phrases:
- *"is above 0"*, *"is at least 18"*, *"is at most 100"*, *"is between 1 and 10"* are all valid ways to compare values.
- You can combine conditions with **and** or **or** in a check. For example:

```wfl
check if:
    age is at least 18
    and country is "USA"
    and has consent is yes
end check
```

This checks that **all** the listed conditions are true (the person is at least 18 **and** from USA **and** consent is yes). We could similarly use **or** if only one of multiple conditions needs to be true.

(We will dive deeper into `if` statements and conditional logic in the Control Flow section, so don't worry if you want to learn more about how `check if` works in practice. The takeaway here for the Variables chapter is just that WFL's conditions read naturally and can be used to assert things about your data.)

### Default Values for Variables

When creating variables, you might want to specify a default value up front, especially if it's something that might change later or if it's optional information. WFL allows default values in declarations:

```wfl
create username with default "guest"
create score with default 0
create status with default "pending"
```

This creates `username` defaulting to `"guest"`, `score` defaulting to `0`, and `status` defaulting to `"pending"`. You can later change them, but if not changed, those are the starting values.

For more complex structures, defaults can be even more elaborate:
```wfl
create new user:
    name defaults to "New User"
    joined date defaults to today
    settings default to:
        theme is "light"
        language is "English"
        notifications are on
    end settings
end create
```

This looks like creating a new *record or object* (like a user profile) with multiple fields, each having a default. We see nested defaults: the `settings` field itself is a map with its own defaults (light theme, English language, notifications on). Such nested structure might be part of a container or module, but the syntax shows how natural it is to provide initial values.

Defaults ensure that even if you don't explicitly set something, it has a sensible value from the start.

### Variable Scope and Lifetime (Global vs Local)

As you write larger programs, the concept of **scope** becomes important. Scope determines **where** a variable can be accessed. WFL uses clear words for scope:

- **Global variables:** available everywhere in your program. You declare one by prefixing with `global`. For example:
  ```wfl
  global create server url as "http://example.com"
  ```
  Now `server url` can be used from any part of the code.

- **Local variables:** exist only inside a specific block or routine. For example, if you are inside a process or action and only need a temporary variable:
  ```wfl
  inside process user:
      local create temp id
      local create user data
  end inside
  ```
  Here, `temp id` and `user data` exist only within that `inside` block (perhaps a function or process called "process user"). Once you leave that block, those locals are gone. This is similar to local variables in other languages, but WFL makes it explicit and readable with the `inside ... local create ...` structure.

- **Shared variables:** sometimes you want a variable accessible across a module or file, but not truly global to everything. WFL uses `shared` for that. For example:
  ```wfl
  create module user handling:
      shared create user count as 0
      shared create active users as empty list
  end module
  ```
  This suggests we have a module (like a grouping of related code) named "user handling", and inside it we have `user count` and `active users` defined as shared. Code within the same module can use them, but outside code won't see them (which helps prevent accidental misuse).

For a beginner, you don't need to deeply worry about scope if you're writing small scripts – just create variables as needed. But it's good to know these words (global, local, shared) exist and do exactly what they say. It keeps your code organized as it grows.

### Monitoring Changes to Variables (Advanced)

This is a more advanced feature, but an exciting one: WFL can **watch** for changes in variables and react to them. This is beyond what most beginner programmers need, but it shows the power of WFL's natural syntax.

For example, say you want to take action whenever a variable changes value:

```wfl
track changes to user count:
    when increases:
        check if user count is above limit
    when decreases:
        update display
end track
```

This means: *"Track changes to `user count`. If it increases, then check if it went above some limit (perhaps to alert or prevent something). If it decreases, update the display."* The keywords `when increases` and `when decreases` let you respond to how the value changed.

Another example, watching a settings object for any specific field changes:

```wfl
watch settings:
    when theme changes:
        update display theme
    when volume changes:
        update volume output
    when notifications change:
        update notification preferences
end watch
```

Here, we watch the `settings` (which could be a map or record of user settings). If the `theme` inside `settings` changes, we call some code to update the display theme. If `volume` changes, we adjust the audio output, and so on. It's like saying "keep an eye on these and respond when needed."

These features can make building interactive or reactive programs easier because the code reads like event descriptions. Again, this is an intermediate-to-advanced concept; new users of WFL can safely ignore tracking/watching until they have a need for it. But it's nice to know WFL can do this with such clear syntax.

## Conclusion

In this chapter, we covered a lot of ground in how WFL handles data:

- We learned that variables in WFL are declared with everyday words like **store** and **create**, and you can name them in a descriptive, spaced-out way (making code self-documenting).
- We saw basic data types: numbers, text, yes/no booleans, and special placeholders like nothing/missing.
- We performed operations to update variables (in plain arithmetic language) and to manipulate text.
- We organized data using lists, sets, and maps, and grouped related data into records.
- We touched on converting between types safely, validating data with simple rules, setting default values, and even advanced features like scope control and change tracking.

The key theme is **clarity**. WFL's variable and data handling is designed to be **clear at a glance**. You don't see cryptic symbols or hard-to-guess abbreviations – it's all written out in a way that beginners can understand and experienced developers can appreciate for its readability.

Feel free to play around with these examples. Try creating your own variables and collections. In the next sections, we'll use these variables in control structures (like loops and conditionals) and see how WFL continues the trend of English-like syntax for coding tasks. Happy coding!
# WFL Loops: Repeating Actions the Easy Way

**Summary:** In this section, we'll explore **loops** in WebFirst Language (WFL) – a way to repeat actions multiple times without writing the same code over and over. Loops are essential for tasks like processing lists of items, counting, or repeating something until a condition is met. WFL makes loops intuitive with natural language syntax that clearly expresses your intent. By the end of this guide, you'll understand how to use different types of loops for various scenarios, control when loops stop, and work with loop variables – all using WFL's friendly, English-like approach.

## What Is a Loop?

A **loop** is a programming construct that lets you **repeat a set of instructions** multiple times. Instead of writing the same code over and over, you can wrap it in a loop and tell the computer how many times to repeat it or under what conditions to keep going.

**Why use loops?** Imagine you need to send a message to 100 users. Without loops, you'd have to write the "send message" code 100 times! With loops, you write it once and tell the computer to repeat it 100 times. Loops save time, reduce errors, and make your code more maintainable.

## Types of Loops in WFL

WFL offers several types of loops, each designed for specific scenarios. Let's explore them one by one:

### Count Loops: Repeating a Specific Number of Times

The **count loop** is perfect when you know exactly how many times you want to repeat something. It's like saying, "Do this 5 times" or "Count from 1 to 10 and do something each time."

Here's a simple example:

```wfl
count from 1 to 5:
    display "Hello, this is iteration number " with count
end count
```

This loop will run 5 times, displaying:
```
Hello, this is iteration number 1
Hello, this is iteration number 2
Hello, this is iteration number 3
Hello, this is iteration number 4
Hello, this is iteration number 5
```

Notice how we used `count` inside the loop? WFL automatically creates a special variable called `count` that keeps track of the current iteration. You don't need to declare it – it's available automatically inside the loop.

You can also count in different increments or even count backward:

```wfl
// Count by 2s
count from 0 to 10 by 2:
    display count  // Displays: 0, 2, 4, 6, 8, 10
end count

// Count backward
count from 10 down to 1:
    display "Countdown: " with count  // Counts: 10, 9, 8, 7, 6, 5, 4, 3, 2, 1
end count
```

If you need to use the count value in calculations but don't want to modify the `count` variable itself (which could affect the loop), you can store it in your own variable:

```wfl
count from 1 to 5:
    store current as count
    display "Processing item " with current
end count
```

### For-Each Loops: Processing Collections

When you want to process each item in a collection (like a list, set, or map), the **for-each loop** is your friend. It automatically iterates through all elements, giving you each one in turn.

```wfl
create list fruits:
    add "apple"
    add "banana"
    add "cherry"
end list

for each fruit in fruits:
    display "I like " with fruit
end for
```

This will display:
```
I like apple
I like banana
I like cherry
```

The syntax is very natural: `for each [item variable] in [collection]:`. Inside the loop, you use the `item variable` (in this case, `fruit`) to refer to the current item being processed.

For-each loops work with any collection type:

```wfl
// With a set
create set unique colors:
    add "red"
    add "green"
    add "blue"
end set

for each color in unique colors:
    display "Color: " with color
end for

// With a map (gives you the keys)
create map settings:
    theme is "dark"
    volume is 75
end map

for each setting in settings:
    display "Setting: " with setting
end for
```

When using for-each with a map, you get the keys by default. If you want both the key and value, you can use this pattern:

```wfl
for each key in settings:
    display key with " is set to " with settings[key]
end for
```

### While Loops: Repeating While a Condition Is True

Sometimes you don't know in advance how many times you need to repeat something. Instead, you want to keep going as long as a certain condition remains true. That's where **while loops** come in.

In WFL, while loops use the `repeat while` syntax:

```wfl
store count as 1
repeat while count is less than or equal to 5:
    display "Count is " with count
    add 1 to count
end repeat
```

This loop will also display numbers 1 through 5, but notice the difference from a count loop: we have to manually initialize the counter variable before the loop and increment it inside the loop. The loop continues as long as the condition `count is less than or equal to 5` remains true.

While loops are flexible because the condition can be anything:

```wfl
store user input as ""
repeat while user input is not "quit":
    display "Enter a command (type 'quit' to exit):"
    store user input as input from user
    display "You entered: " with user input
end repeat
```

This loop keeps asking for input until the user types "quit". The condition is checked at the beginning of each iteration, so if the condition is false from the start, the loop body won't execute at all.

### Until Loops: Repeating Until a Condition Becomes True

The **until loop** is similar to a while loop, but with an inverted condition. Instead of continuing while something is true, it continues until something becomes true.

```wfl
store count as 1
repeat until count is greater than 5:
    display "Count is " with count
    add 1 to count
end repeat
```

This produces the same result as our while loop example, but the condition is expressed differently. Sometimes one form reads more naturally than the other, depending on what you're trying to express.

### Do-Until Loops: Checking the Condition After Execution

A variation of the until loop is the **do-until loop**, which checks the condition after executing the loop body. This ensures the loop body runs at least once, even if the condition is true from the start.

```wfl
store count as 1
repeat:
    display "Count is " with count
    add 1 to count
until count is greater than 5
```

Notice the different structure: the condition comes after the loop body. This is useful when you need to perform an action at least once before checking whether to continue.

### Infinite Loops: Repeating Forever (with an Exit Strategy)

Sometimes you need a loop that keeps going indefinitely until something specific happens inside the loop. WFL provides the **forever loop** for this:

```wfl
repeat forever:
    display "Enter a command (type 'quit' to exit):"
    store user input as input from user
    
    if user input is "quit":
        exit loop
    end if
    
    display "You entered: " with user input
end repeat
```

The `exit loop` statement is crucial here – it provides a way to break out of the loop when a certain condition is met. Without it, the loop would truly run forever (or until you stop the program).

## Controlling Loop Execution

Beyond the basic loop structures, WFL provides ways to control how loops execute:

### Exiting Loops Early

We've already seen the `exit loop` statement, which immediately breaks out of the current loop:

```wfl
count from 1 to 100:
    if count is 10:
        display "Reached 10, stopping early!"
        exit loop
    end if
    display count
end count
```

This loop would normally count from 1 to 100, but the `exit loop` statement causes it to stop after reaching 10.

### Skipping Iterations

Sometimes you want to skip the rest of the current iteration and move to the next one. WFL uses the `skip iteration` statement for this:

```wfl
count from 1 to 10:
    if count is divisible by 2:  // Skip even numbers
        skip iteration
    end if
    display count  // Only displays odd numbers: 1, 3, 5, 7, 9
end count
```

This is useful when you want to process only certain items that meet specific criteria.

### Nested Loops: Loops Inside Loops

You can place loops inside other loops, creating **nested loops**. This is powerful for working with multi-dimensional data or when you need to perform complex iterations:

```wfl
count from 1 to 3:
    store row as count
    
    count from 1 to 3:
        store column as count
        display "Position: " with row with "," with column
    end count
end count
```

This creates a 3×3 grid of positions:
```
Position: 1,1
Position: 1,2
Position: 1,3
Position: 2,1
Position: 2,2
Position: 2,3
Position: 3,1
Position: 3,2
Position: 3,3
```

Notice that each loop has its own `count` variable. To avoid confusion, we store each loop's count in a separate variable (`row` and `column`).

## Common Loop Patterns

Here are some common patterns you might use with loops:

### Processing Lists with Indexes

Sometimes you need both the item and its position in the list. You can combine a count loop with list access:

```wfl
create list tasks:
    add "Buy groceries"
    add "Clean house"
    add "Pay bills"
end list

count from 1 to length of tasks:
    display count with ". " with tasks[count - 1]
end count
```

This displays:
```
1. Buy groceries
2. Clean house
3. Pay bills
```

Note that we use `count - 1` because list indexes typically start at 0, but our count starts at 1.

### Accumulating Results

Loops are often used to build up a result by processing each item:

```wfl
create list numbers:
    add 10
    add 20
    add 30
    add 40
end list

store total as 0
for each number in numbers:
    add number to total
end for

display "The sum is: " with total  // Displays: The sum is: 100
```

### Finding Items in Collections

You can use loops to search for specific items:

```wfl
create list fruits:
    add "apple"
    add "banana"
    add "cherry"
    add "date"
end list

store found as no
for each fruit in fruits:
    if fruit is "cherry":
        store found as yes
        display "Found cherry!"
        exit loop
    end if
end for

if found is no:
    display "Cherry not found in the list."
end if
```

## Loop Best Practices

To write effective and maintainable loops, consider these best practices:

1. **Choose the right loop type** for your task:
   - Use **count loops** when you know the exact number of iterations
   - Use **for-each loops** when processing collections
   - Use **while/until loops** when the number of iterations depends on a condition

2. **Avoid infinite loops** by ensuring that:
   - Your loop condition will eventually become false
   - You have an `exit loop` statement that will be reached
   - You're modifying the variables used in the condition

3. **Keep loops simple** – if a loop body becomes too complex, consider:
   - Breaking it into smaller loops
   - Moving some logic into separate actions (functions)
   - Using helper variables to clarify the logic

4. **Be careful with loop variables** – remember that:
   - The `count` variable in count loops is managed automatically
   - Variables created inside a loop are recreated each iteration
   - Variables created before a loop persist across iterations

## Conclusion

Loops are a powerful tool in your programming toolkit. They allow you to automate repetitive tasks, process collections of data, and create dynamic behaviors in your programs. WFL's natural language approach to loops makes them particularly accessible, with clear syntax that expresses your intent without cryptic symbols or complex structures.

In this section, we've covered:
- Different types of loops for various scenarios
- How to control loop execution with exit and skip statements
- Common patterns and best practices for using loops effectively

As you practice with loops, you'll find they become an essential part of your programming repertoire. They're the key to making your programs more efficient, more powerful, and more adaptable to varying amounts of data.

In the next sections, we'll explore how to combine loops with other control structures like conditionals to create even more sophisticated programs. Happy looping!

# WFL Conditionals and Control Flow: Making Decisions in Code

**Summary:** In this section, we'll explore **conditionals** in WebFirst Language (WFL) – powerful constructs that allow your programs to make decisions and follow different paths based on specific conditions. Conditionals are essential for creating dynamic, responsive programs that can adapt to different situations. WFL makes conditionals intuitive with natural language syntax that clearly expresses your intent. By the end of this guide, you'll understand how to use if/else statements, handle multiple conditions, implement switch-like behavior, and gracefully manage exceptions – all using WFL's friendly, English-like approach.

## What Are Conditionals?

**Conditionals** are programming constructs that allow your code to make decisions. They evaluate a condition (a statement that's either true or false) and execute different code depending on the result. Think of conditionals as forks in the road – they let your program choose which path to take based on certain criteria.

**Why use conditionals?** Imagine you're building a weather app. You might want to display different messages depending on the temperature: "Wear a coat" if it's cold, "Bring an umbrella" if it's raining, or "Don't forget sunscreen" if it's sunny. Conditionals make this kind of decision-making possible.

## Basic If Statements

The simplest form of conditional is the **if statement**. It checks a condition and executes a block of code only if that condition is true.

```wfl
store temperature as 28

if temperature is above 25:
    display "It's hot today!"
end if
```

In this example, the message "It's hot today!" will only be displayed if the temperature is above 25. The syntax is straightforward: `if [condition]:` followed by the code to execute, and ending with `end if`.

## If-Else Statements

Often, you want to do one thing if a condition is true and something else if it's false. That's where **if-else statements** come in:

```wfl
store temperature as 15

if temperature is above 25:
    display "It's hot today!"
else:
    display "It's not very hot today."
end if
```

Since the temperature (15) is not above 25, the program will display "It's not very hot today." The `else` clause provides an alternative action when the condition is false.

## Multiple Conditions with Else-If

What if you have more than two possible paths? You can use **else-if** to check multiple conditions in sequence:

```wfl
store temperature as 15

if temperature is above 30:
    display "It's very hot today!"
else if temperature is above 20:
    display "It's warm today."
else if temperature is above 10:
    display "It's mild today."
else:
    display "It's cold today."
end if
```

In this example, the program checks each condition in order until it finds one that's true. Since the temperature is 15, it will display "It's mild today." If none of the conditions are true, it executes the code in the `else` block.

## Nested Conditionals

You can place conditionals inside other conditionals, creating **nested conditionals**:

```wfl
store temperature as 28
store is raining as yes

if temperature is above 25:
    display "It's hot today!"
    
    if is raining is yes:
        display "Bring an umbrella and stay hydrated."
    else:
        display "Don't forget sunscreen and stay hydrated."
    end if
else:
    display "It's not very hot today."
    
    if is raining is yes:
        display "Bring an umbrella."
    end if
end if
```

This creates more complex decision trees. In this example, the program first checks if it's hot, then checks if it's raining, and provides specific advice based on both conditions.

## Combining Conditions with And/Or

Instead of nesting conditionals, you can often combine conditions using **and** and **or**:

```wfl
store temperature as 28
store is raining as yes

if temperature is above 25 and is raining is yes:
    display "It's hot and rainy. Bring an umbrella and stay hydrated."
else if temperature is above 25 and is raining is no:
    display "It's hot and sunny. Don't forget sunscreen and stay hydrated."
else if temperature is below or equal to 25 and is raining is yes:
    display "It's cool and rainy. Bring an umbrella and a jacket."
else:
    display "It's cool and not raining. Enjoy the pleasant weather!"
end if
```

- **and**: Both conditions must be true for the combined condition to be true.
- **or**: At least one condition must be true for the combined condition to be true.

You can use parentheses to clarify complex combinations:

```wfl
if (temperature is above 25 or humidity is above 80) and is outdoor event is yes:
    display "Consider rescheduling the outdoor event."
end if
```

## Switch-Like Behavior with When Statements

When you need to check a single variable against multiple possible values, WFL offers a cleaner alternative to multiple if-else statements:

```wfl
store day as "Monday"

check day:
    when "Monday":
        display "Start of the work week."
    when "Tuesday":
        display "Second day of work."
    when "Wednesday":
        display "Halfway through the week!"
    when "Thursday":
        display "Almost there."
    when "Friday":
        display "Last workday of the week!"
    when "Saturday" or "Sunday":
        display "It's the weekend!"
    otherwise:
        display "Not a valid day."
end check
```

This `check...when...otherwise` structure is similar to a switch or case statement in other languages. It's more readable than a long chain of if-else statements when you're checking the same variable against different values.

## Conditional Expressions

Sometimes you want to assign a value based on a condition. WFL allows for conditional expressions:

```wfl
store temperature as 28
store weather status as if temperature is above 25 then "hot" else "not hot"

display "The weather is " with weather status  // Displays: The weather is hot
```

This is a compact way to choose between two values based on a condition.

## Error Handling with Try-When-Otherwise

In real-world applications, things don't always go as planned. WFL provides a robust way to handle errors and exceptions using the **try-when-otherwise** pattern:

```wfl
try:
    open file "data.txt"
    read content from file
    display content
when file not found:
    display "The file data.txt does not exist."
when permission denied:
    display "You don't have permission to read data.txt."
otherwise:
    display "An unexpected error occurred while reading the file."
end try
```

This structure allows your program to gracefully handle different types of errors:
- The `try` block contains the code that might cause an error.
- Multiple `when` clauses catch specific types of errors.
- The `otherwise` clause catches any other errors not specifically handled.

## Conditional Loops

You can combine conditionals with loops to create powerful control structures:

```wfl
store count as 1

repeat while count is less than or equal to 10:
    if count is divisible by 2:
        display count with " is even"
    else:
        display count with " is odd"
    end if
    
    add 1 to count
end repeat
```

This loop counts from 1 to 10, displaying whether each number is even or odd.

## Best Practices for Conditionals

To write effective and maintainable conditionals, consider these best practices:

1. **Keep conditions simple and clear** – complex conditions can be hard to understand and debug.
2. **Use meaningful variable names** that make your conditions self-explanatory.
3. **Consider the order of conditions** – put the most common or important cases first.
4. **Avoid deeply nested conditionals** – consider refactoring with combined conditions or separate functions.
5. **Be careful with equality checks** – use the appropriate comparison for your data type.

## Common Conditional Patterns

Here are some common patterns you might use with conditionals:

### Input Validation

```wfl
store user age as input from user

if user age is less than 0:
    display "Age cannot be negative."
else if user age is greater than 120:
    display "Please enter a valid age."
else:
    display "Thank you for providing your age."
end if
```

### Finding the Maximum Value

```wfl
store a as 15
store b as 27
store c as 10

store max value as a

if b is greater than max value:
    store max value as b
end if

if c is greater than max value:
    store max value as c
end if

display "The maximum value is " with max value  // Displays: The maximum value is 27
```

### State Machines

```wfl
store traffic light as "red"

check traffic light:
    when "red":
        display "Stop!"
        store next light as "green"
    when "yellow":
        display "Prepare to stop."
        store next light as "red"
    when "green":
        display "Go!"
        store next light as "yellow"
    otherwise:
        display "Invalid traffic light state."
        store next light as "red"
end check

store traffic light as next light
```

## Conclusion

Conditionals are the decision-makers in your programs. They allow your code to respond differently based on varying circumstances, making your applications dynamic and interactive. WFL's natural language approach to conditionals makes them particularly accessible, with clear syntax that expresses your intent without cryptic symbols or complex structures.

In this section, we've covered:
- Basic if-else statements for simple decisions
- Multiple conditions with else-if for more complex branching
- Combining conditions with and/or for concise logic
- Switch-like behavior with when statements for cleaner code
- Error handling with try-when-otherwise for robust applications

As you practice with conditionals, you'll find they become an essential part of your programming toolkit. They're the key to making your programs more intelligent, responsive, and user-friendly.

In the next sections, we'll explore how to organize code into reusable blocks with functions and how to model real-world entities with containers. Happy coding!
# WFL Functions (Actions): Reusing Code with Natural Syntax

**Summary:** In this section, we'll explore **functions** (called **actions** in WFL) – powerful constructs that let you package code into reusable, named blocks. Actions are essential for organizing your code, avoiding repetition, and breaking complex problems into manageable pieces. WFL makes defining and using actions intuitive with natural language syntax that clearly expresses your intent. By the end of this guide, you'll understand how to create actions with parameters, return values from actions, work with asynchronous operations, handle errors gracefully, and organize your code effectively – all using WFL's friendly, English-like approach.

## What Are Functions (Actions)?

An **action** in WFL (called a function in many other programming languages) is a named block of code that performs a specific task. Think of it as a mini-program within your program that you can call whenever you need to perform that task. Actions help you organize your code, avoid repetition, and make your programs more modular and maintainable.

**Why use actions?** Imagine you need to calculate the average of a list of numbers in multiple places in your program. Instead of writing the same calculation code over and over, you can define an action called `calculate average` once and then call it whenever you need that functionality. This makes your code more concise, easier to understand, and simpler to maintain.

## Creating Basic Actions

In WFL, you define an action using the `define action` syntax. Here's a simple example:

```wfl
define action say hello:
    display "Hello, World!"
end action
```

This creates an action called `say hello` that displays the message "Hello, World!" when called. The syntax is straightforward: `define action [name]:` followed by the code to execute, and ending with `end action`.

To call (or execute) this action, you simply use its name:

```wfl
say hello  // Displays: Hello, World!
```

Notice how natural this reads – it's like giving a command in plain English.

## Actions with Parameters

Most actions need to work with different values each time they're called. **Parameters** allow you to pass information into an action. In WFL, parameters are defined using the `with` keyword:

```wfl
define action say hello with name:
    display "Hello, " with name with "!"
end action
```

Now our action takes a parameter called `name`. When we call the action, we provide a value for this parameter:

```wfl
say hello with "Alice"  // Displays: Hello, Alice!
say hello with "Bob"    // Displays: Hello, Bob!
```

You can define multiple parameters by separating them with `and`:

```wfl
define action calculate rectangle area with width and height:
    store area as width times height
    display "The area is " with area
end action

calculate rectangle area with 5 and 3  // Displays: The area is 15
```

The syntax for calling actions with multiple parameters is very natural: `action name with [value1] and [value2] and [value3]...`. It reads almost like a sentence.

### Optional Parameters with Default Values

Sometimes you want parameters to be optional, with sensible defaults if not provided. WFL supports this with the `default` keyword:

```wfl
define action greet user with name and greeting default "Hello":
    display greeting with ", " with name with "!"
end action

greet user with "Alice" and "Hi"      // Displays: Hi, Alice!
greet user with "Bob"                 // Displays: Hello, Bob!
```

In this example, `greeting` is an optional parameter with a default value of "Hello". If not provided when calling the action, the default value is used.

## Returning Values from Actions

Actions can also produce (or return) values that can be used elsewhere in your program. In WFL, you use the `provide` or `return` keyword to specify what value an action should return:

```wfl
define action calculate average of numbers:
    store sum as 0
    for each number in numbers:
        add number to sum
    end for
    
    store average as sum divided by length of numbers
    provide average
end action
```

This action calculates the average of a list of numbers and returns the result. To use the returned value, you can store it in a variable:

```wfl
create list test scores:
    add 85
    add 90
    add 78
    add 92
end list

store average score as calculate average of test scores
display "The average score is " with average score  // Displays: The average score is 86.25
```

The syntax for using a returned value is very natural: you can use the action call directly in expressions or assignments, just as you would use any other value.

## Asynchronous Actions

WFL supports asynchronous programming, which is essential for operations that might take time to complete, such as network requests, file operations, or database queries. Asynchronous actions are defined using the `async` keyword:

```wfl
define async action fetch weather for city:
    display "Fetching weather data for " with city with "..."
    wait for 2 seconds  // Simulating a network request
    provide "Sunny, 25°C"
end action
```

When calling an asynchronous action, you typically use the `wait for` syntax to wait for it to complete:

```wfl
display "Starting weather check..."
store weather as wait for fetch weather for "New York"
display "Weather in New York: " with weather
display "Weather check complete."
```

This would display:
```
Starting weather check...
Fetching weather data for New York...
Weather in New York: Sunny, 25°C
Weather check complete.
```

The `wait for` ensures that the program waits for the asynchronous action to complete before continuing. This is important for operations where you need the result before proceeding.

### Parallel Asynchronous Operations

One of the powerful features of WFL's asynchronous programming model is the ability to run multiple operations in parallel using the `wait for ... and ...` syntax:

```wfl
display "Fetching weather data..."
wait for:
    store ny weather as fetch weather for "New York"
    and store la weather as fetch weather for "Los Angeles"
end wait

display "New York: " with ny weather
display "Los Angeles: " with la weather
```

This runs both weather fetches concurrently, potentially saving time compared to running them sequentially. The program continues only after both operations have completed.

## Error Handling in Actions

Actions might encounter errors during execution. WFL provides a robust way to handle these errors using the `try-when-otherwise` pattern:

```wfl
define action read user file with filename:
    try:
        open file filename
        read content from file
        close file
        provide content
    when file not found:
        display "Error: File " with filename with " not found."
        provide nothing
    when permission denied:
        display "Error: No permission to read " with filename
        provide nothing
    otherwise:
        display "An unexpected error occurred while reading " with filename
        provide nothing
    end try
end action
```

This action attempts to read a file and handles different types of errors that might occur. The caller can check the returned value to determine if the operation was successful:

```wfl
store user data as read user file with "user.txt"
if user data is nothing:
    display "Failed to read user data."
else:
    display "User data: " with user data
end if
```

## Organizing Code with Actions

Actions are powerful tools for organizing your code. Here are some common patterns:

### Helper Actions

You can create small, focused actions that perform specific tasks and then use them in larger actions:

```wfl
define action validate email with email:
    check if:
        email contains "@"
        and email contains "." after "@"
        and email does not contain spaces
    end check
    
    provide result of check
end action

define action register user with name and email:
    if validate email with email:
        display "Registering user " with name with " with email " with email
        // Registration code here
    else:
        display "Invalid email address: " with email
    end if
end action
```

This approach makes your code more modular and easier to maintain. The `validate email` action can be reused wherever email validation is needed.

### Action Libraries

As your program grows, you might want to organize related actions into libraries or modules:

```wfl
create module math utilities:
    define action calculate average of numbers:
        // Implementation here
    end action
    
    define action find maximum in numbers:
        // Implementation here
    end action
    
    define action find minimum in numbers:
        // Implementation here
    end action
end module
```

This groups related actions together, making your code more organized and easier to navigate.

## Variable Scope in Actions

Variables defined inside an action are **local** to that action – they exist only while the action is running and can't be accessed from outside. This is important for preventing unintended interactions between different parts of your program.

```wfl
define action calculate square of number:
    store result as number times number  // 'result' is local to this action
    provide result
end action

calculate square of 5
display result  // Error: 'result' is not defined here
```

If you need to access variables from the surrounding context, you can do so directly – actions can "see" variables defined in their outer scope:

```wfl
store base value as 10

define action add to base with amount:
    store new value as base value plus amount  // Can access 'base value'
    provide new value
end action

store result as add to base with 5  // result = 15
```

This behavior allows actions to work with their surrounding context while still maintaining clean boundaries.

## Recursive Actions

Actions in WFL can call themselves, a technique known as **recursion**. This is useful for solving problems that can be broken down into smaller instances of the same problem:

```wfl
define action calculate factorial of n:
    if n is less than or equal to 1:
        provide 1
    else:
        store smaller factorial as calculate factorial of (n minus 1)
        provide n times smaller factorial
    end if
end action
```

This action calculates the factorial of a number by recursively calculating the factorial of smaller numbers. For example, `calculate factorial of 5` would compute 5 × 4 × 3 × 2 × 1 = 120.

## Best Practices for Actions

To write effective and maintainable actions, consider these best practices:

1. **Give actions clear, descriptive names** that indicate what they do. For example, `calculate average` is better than `calc avg`.

2. **Keep actions focused on a single task**. If an action is doing too many things, consider breaking it into smaller, more focused actions.

3. **Use parameters to make actions flexible**. Instead of hardcoding values, pass them as parameters so the action can work with different inputs.

4. **Handle errors gracefully**. Use try-when-otherwise to catch and handle potential errors, providing meaningful feedback to the caller.

5. **Document complex actions** with comments that explain what the action does, what parameters it expects, and what it returns.

6. **Avoid side effects** when possible. Actions that modify global state can make your program harder to understand and debug.

## Common Action Patterns

Here are some common patterns you might use with actions:

### Data Transformation

Actions are great for transforming data from one form to another:

```wfl
define action format name with first and last and middle default nothing:
    if middle is nothing:
        provide first with " " with last
    else:
        provide first with " " with middle with " " with last
    end if
end action

store formatted as format name with "John" and "Doe"  // "John Doe"
store formatted with middle as format name with "Jane" and "Smith" and "Marie"  // "Jane Marie Smith"
```

### Event Handlers

Actions can serve as event handlers, responding to user interactions or system events:

```wfl
define action handle button click with button id:
    if button id is "submit":
        validate and submit form
    else if button id is "cancel":
        clear form and return to main page
    else if button id is "help":
        show help documentation
    end if
end action
```

### Data Processing Pipelines

You can chain actions together to create data processing pipelines:

```wfl
define action process data:
    store raw data as fetch data from server
    store filtered data as filter invalid entries from raw data
    store sorted data as sort filtered data by date
    store formatted data as format data for display with sorted data
    provide formatted data
end action
```

Each step in the pipeline is a separate action, making the code modular and easier to maintain.

## Conclusion

Actions are the building blocks of reusable code in WFL. They allow you to organize your program into logical, focused units that can be combined in powerful ways. WFL's natural language approach to actions makes them particularly accessible, with clear syntax that expresses your intent without cryptic symbols or complex structures.

In this section, we've covered:
- Basic action definition and calling syntax
- Working with parameters and return values
- Asynchronous actions and parallel operations
- Error handling in actions
- Organizing code with actions
- Variable scope and recursion
- Best practices and common patterns

As you practice with actions, you'll find they become an essential part of your programming toolkit. They're the key to writing clean, maintainable, and reusable code that solves complex problems elegantly.

In the next section, we'll explore how to model real-world entities with containers, which build upon the concepts we've learned so far. Happy coding!
# WFL Containers (Classes): Structuring Data and Behavior Together

**Summary:** In this section, we'll explore **containers** in WebFirst Language (WFL) – a powerful way to organize related data and actions into cohesive units. Containers are similar to what other languages call "classes" but with WFL's natural language approach. They allow you to model real-world entities by bundling properties (data) and behaviors (actions) together. By the end of this guide, you'll understand how to create containers, define their properties and actions, create instances, and use inheritance to build relationships between different types of containers – all using WFL's friendly, English-like syntax.

## What Are Containers?

A **container** is a blueprint or template that defines a type of object in your program. Think of it as a custom data type that you design to represent something specific in your application – like a User, Product, Vehicle, or any other concept that has both data (properties) and behavior (actions).

**Why use containers?** As your programs grow more complex, organizing related data and functionality together becomes essential. Containers help you:

- **Group related data and actions** in one place
- **Create multiple instances** with the same structure but different values
- **Encapsulate complexity** by hiding implementation details
- **Model real-world entities** in a natural way

## Creating a Basic Container

In WFL, you create a container using the `create container` syntax. Here's a simple example:

```wfl
create container Person:
    // Properties (data)
    property name as text
    property age as number
    property email as text
    
    // Actions (behavior)
    define action greet:
        display "Hello, my name is " with name
    end action
    
    define action has birthday:
        add 1 to age
        display name with " is now " with age with " years old"
    end action
end container
```

This defines a `Person` container with:
- Three properties: `name`, `age`, and `email`
- Two actions: `greet` and `has birthday`

The container itself is just a blueprint – it doesn't represent any specific person yet. To create actual people based on this blueprint, you need to create instances.

## Creating and Using Container Instances

An **instance** is a specific object created from a container. If the container is a blueprint for a house, an instance is an actual house built from that blueprint.

Here's how to create instances of our `Person` container:

```wfl
create new Person as alice:
    set name to "Alice Smith"
    set age to 28
    set email to "alice@example.com"
end create

create new Person as bob:
    set name to "Bob Johnson"
    set age to 35
    set email to "bob@example.com"
end create
```

Now we have two `Person` instances: `alice` and `bob`. Each has its own set of property values.

To access properties or call actions on an instance, use the dot notation or possessive form:

```wfl
// Accessing properties
display alice's name        // Displays: Alice Smith
display bob's age           // Displays: 35

// Calling actions
alice greet                 // Displays: Hello, my name is Alice Smith
bob has birthday            // Displays: Bob Johnson is now 36 years old
```

Notice how natural this syntax is – `alice greet` reads like giving a command to Alice, and `bob has birthday` describes an event happening to Bob.

## Container Initialization and Constructors

When creating a new instance, you often need to set up its initial state. WFL provides a special action called `initialize` that runs automatically when an instance is created:

```wfl
create container Product:
    property name as text
    property price as number
    property in stock as yes
    
    define action initialize with product name and product price:
        set name to product name
        set price to product price
        display "Created new product: " with name
    end action
    
    define action mark as sold out:
        set in stock to no
    end action
end container
```

Now you can create a product with initial values in one step:

```wfl
create new Product with "Smartphone" and 499.99 as phone
```

This creates a new `Product` instance called `phone` with name "Smartphone" and price 499.99, and displays "Created new product: Smartphone".

## Properties with Validation and Default Values

You can make your containers more robust by adding validation rules and default values to properties:

```wfl
create container BankAccount:
    property account number as text:
        must not be empty
        must be exactly 10 characters
    end property
    
    property balance as number:
        must be at least 0
        defaults to 0
    end property
    
    property owner name as text
    
    define action deposit amount:
        check amount:
            must be greater than 0
        end check
        
        add amount to balance
        display "Deposited " with amount with ". New balance: " with balance
    end action
    
    define action withdraw amount:
        check amount:
            must be greater than 0
            must be less than or equal to balance
        end check
        
        subtract amount from balance
        display "Withdrew " with amount with ". New balance: " with balance
    end action
end container
```

This `BankAccount` container includes:
- Validation for `account number` (must be exactly 10 characters)
- Validation for `balance` (must be non-negative) with a default value of 0
- Actions that include their own validation logic

When you try to create an instance with invalid data or call an action with invalid parameters, WFL will provide clear error messages.

## Private and Public Members

In larger applications, you might want to control which properties and actions are accessible from outside the container. WFL allows you to mark members as `private` or `public`:

```wfl
create container User:
    // Public properties (accessible from anywhere)
    public property username as text
    public property display name as text
    
    // Private properties (only accessible within the container)
    private property password hash as text
    private property login attempts as number defaults to 0
    
    public define action greet:
        display "Hello, " with display name
    end action
    
    private define action hash password with raw password:
        // Implementation details hidden
        store password hash as secure hash of raw password
    end action
    
    public define action set password with new password:
        check new password:
            must be at least 8 characters
            must have uppercase letter
            must have number
        end check
        
        hash password with new password
        display "Password updated successfully"
    end action
end container
```

In this example:
- `username` and `display name` are public properties that can be accessed from anywhere
- `password hash` and `login attempts` are private properties that can only be accessed within the container
- `greet` and `set password` are public actions that can be called from anywhere
- `hash password` is a private action that can only be called by other actions within the container

This encapsulation helps protect sensitive data and implementation details.

## Container Inheritance

**Inheritance** allows you to create a new container based on an existing one. The new container (called a **child** or **subcontainer**) inherits all the properties and actions of the original container (called the **parent** or **supercontainer**), and can add its own or override existing ones.

Here's an example:

```wfl
create container Vehicle:
    property make as text
    property model as text
    property year as number
    
    define action describe:
        display year with " " with make with " " with model
    end action
end container

create container Car extends Vehicle:
    property number of doors as number defaults to 4
    property fuel type as text defaults to "gasoline"
    
    // Override the parent's describe action
    define action describe:
        // Call the parent's version first
        parent describe
        display "This car has " with number of doors with " doors and runs on " with fuel type
    end action
    
    define action honk:
        display "Beep beep!"
    end action
end container
```

Now you can create a `Car` instance that has all the properties and actions from both containers:

```wfl
create new Car as my car:
    set make to "Toyota"
    set model to "Corolla"
    set year to 2025
    set fuel type to "hybrid"
end create

my car describe
// Displays:
// 2025 Toyota Corolla
// This car has 4 doors and runs on hybrid

my car honk  // Displays: Beep beep!
```

Inheritance helps you build hierarchies of related containers, promoting code reuse and logical organization.

## Container Composition

While inheritance creates "is-a" relationships (a Car is a Vehicle), **composition** creates "has-a" relationships. This means one container can include instances of other containers as properties:

```wfl
create container Engine:
    property horsepower as number
    property cylinders as number
    
    define action start:
        display "Engine started with " with horsepower with " HP"
    end action
end container

create container Car:
    property make as text
    property model as text
    property engine as Engine  // Composition: a Car has an Engine
    
    define action start:
        display "Starting " with make with " " with model
        engine start  // Delegate to the engine's start action
    end action
end container
```

To use this:

```wfl
create new Engine as v6:
    set horsepower to 280
    set cylinders to 6
end create

create new Car as sports car:
    set make to "Nissan"
    set model to "370Z"
    set engine to v6
end create

sports car start
// Displays:
// Starting Nissan 370Z
// Engine started with 280 HP
```

Composition is powerful because it allows you to build complex objects from simpler ones, creating a modular design.

## Static Properties and Actions

Sometimes you want properties or actions that belong to the container itself, not to individual instances. These are called **static** members:

```wfl
create container MathUtils:
    static property PI as 3.14159
    
    static define action calculate circle area with radius:
        provide PI times radius times radius
    end action
end container
```

You can use static members directly through the container, without creating an instance:

```wfl
display MathUtils PI  // Displays: 3.14159

store area as MathUtils calculate circle area with 5
display "The area is " with area  // Displays: The area is 78.53975
```

Static members are useful for utility functions, constants, or tracking information across all instances of a container.

## Container Events and Callbacks

WFL containers can define and respond to events, allowing for reactive programming:

```wfl
create container Button:
    property label as text
    property is enabled as yes
    
    // Define events that this container can trigger
    event clicked
    event hover start
    event hover end
    
    define action click:
        if is enabled:
            trigger clicked  // Fire the clicked event
            display "Button '" with label with "' was clicked"
        end if
    end action
    
    define action on hover:
        trigger hover start
        display "Hovering over button: " with label
    end action
    
    define action end hover:
        trigger hover end
        display "No longer hovering over button: " with label
    end action
end container
```

Other parts of your code can listen for and respond to these events:

```wfl
create new Button as submit button:
    set label to "Submit"
end create

// Set up event handlers
on submit button clicked:
    submit form
end on

on submit button hover start:
    show tooltip with "Click to submit the form"
end on
```

This event-driven approach is particularly useful for user interfaces and asynchronous operations.

## Container Interfaces and Polymorphism

An **interface** defines a contract that containers can implement. It specifies a set of actions that a container must provide, without dictating how they should be implemented:

```wfl
create interface Drawable:
    requires action draw
    requires action resize with width and height
end interface

create container Circle implements Drawable:
    property radius as number
    property color as text
    
    define action draw:
        display "Drawing a " with color with " circle with radius " with radius
    end action
    
    define action resize with width and height:
        set radius to minimum of width and height divided by 2
        display "Circle resized to radius " with radius
    end action
end container

create container Rectangle implements Drawable:
    property width as number
    property height as number
    property color as text
    
    define action draw:
        display "Drawing a " with color with " rectangle " with width with "x" with height
    end action
    
    define action resize with new width and new height:
        set width to new width
        set height to new height
        display "Rectangle resized to " with width with "x" with height
    end action
end container
```

The power of interfaces comes from **polymorphism** – the ability to treat different types of objects uniformly as long as they implement the same interface:

```wfl
create list shapes:
    add new Circle with radius 5 and color "red"
    add new Rectangle with width 10 and height 20 and color "blue"
end list

for each shape in shapes:
    shape draw  // Works for both Circle and Rectangle
    shape resize with 30 and 30
    shape draw  // Shows the updated shapes
end for
```

This code works because both `Circle` and `Rectangle` implement the `Drawable` interface, guaranteeing they both have `draw` and `resize` actions.

## Best Practices for Containers

To write effective and maintainable containers, consider these best practices:

1. **Single Responsibility Principle**: Each container should have a single, well-defined purpose. If a container is doing too many things, consider splitting it into multiple containers.

2. **Meaningful Names**: Choose clear, descriptive names for containers, properties, and actions that reflect their purpose.

3. **Proper Encapsulation**: Use private members to hide implementation details and expose only what's necessary through public members.

4. **Validation**: Add validation rules to properties and action parameters to ensure data integrity.

5. **Documentation**: Include comments that explain the purpose of the container and any non-obvious behavior.

6. **Favor Composition Over Inheritance**: While inheritance is powerful, it can create tight coupling. When possible, use composition to build complex objects from simpler ones.

7. **Keep Inheritance Hierarchies Shallow**: Deep inheritance hierarchies can become difficult to understand and maintain. Try to limit inheritance to 2-3 levels.

## Common Container Patterns

Here are some common patterns you might use with containers:

### Model-View Pattern

Separate data (model) from its presentation (view):

```wfl
create container UserModel:
    property id as number
    property username as text
    property email as text
    property last login as date
    
    define action update last login:
        set last login to today
    end action
end container

create container UserView:
    property model as UserModel
    
    define action display user card:
        display "User: " with model username
        display "Email: " with model email
        display "Last login: " with model last login
    end action
    
    define action display compact:
        display model username with " (" with model email with ")"
    end action
end container
```

### Factory Pattern

Create a specialized container that knows how to create instances of other containers:

```wfl
create container VehicleFactory:
    static define action create sedan with make and model:
        create new Car as result:
            set make to make
            set model to model
            set number of doors to 4
            set vehicle type to "sedan"
        end create
        
        provide result
    end action
    
    static define action create truck with make and model and bed length:
        create new Truck as result:
            set make to make
            set model to model
            set bed length to bed length
        end create
        
        provide result
    end action
end container

// Usage
store my car as VehicleFactory create sedan with "Honda" and "Accord"
```

### Observer Pattern

Implement a publish-subscribe mechanism where objects can register interest in events:

```wfl
create container Subject:
    property observers as empty list
    
    define action add observer with observer:
        add observer to observers
    end action
    
    define action remove observer with observer:
        remove observer from observers
    end action
    
    define action notify with message:
        for each observer in observers:
            observer update with message
        end for
    end action
end container

create container ConcreteSubject extends Subject:
    property state as text
    
    define action change state to new state:
        set state to new state
        notify with "State changed to: " with state
    end action
end container

create container Observer:
    property name as text
    
    define action update with message:
        display name with " received: " with message
    end action
end container
```

## Conclusion

Containers in WFL provide a powerful way to organize your code by grouping related data and behavior together. They allow you to:

- Model real-world entities in a natural, intuitive way
- Create reusable blueprints for objects in your program
- Build relationships between different types of objects
- Encapsulate complexity and protect data integrity
- Structure your code in a modular, maintainable fashion

In this section, we've covered:
- Basic container creation and instantiation
- Properties with validation and default values
- Actions and initialization
- Access control with public and private members
- Inheritance and composition
- Static members and events
- Interfaces and polymorphism
- Best practices and common patterns

As you practice with containers, you'll find they become an essential tool for managing complexity in larger programs. They help you think about your code in terms of objects and their interactions, leading to more organized, maintainable, and intuitive code.

Happy coding!