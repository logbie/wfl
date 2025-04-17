Perfect, thank you! I’ll refine the WFL documentation into modular, beginner-friendly guides that align with the narrative tone of a 'for dummies' or 'complete idiot’s guide'—complete with clear explanations, natural-language code examples, and a gentle learning curve.

Each topic (variables, loops, control flow, containers, etc.) will get its own clearly structured section that builds logically on the previous one. I’ll also incorporate missing elements like `if/then` conditionals into the control flow chapter.

I’ll start with the restructuring and let you know when the refined documentation is ready for review.

# WFL Variables: Storing and Using Data in Plain English

**Summary:** In this section, we'll introduce **variables** in WebFirst Language (WFL) – how to create them, give them values, and use them in a very natural way. Think of variables as labeled containers for information. WFL is designed so that writing code feels like writing simple English sentences. Even if you've never coded before, you'll find WFL variables intuitive and friendly. By the end of this guide, you’ll know how to store different types of data (numbers, text, yes/no values, etc.), update those values, group values into collections, and even set default values or validate data – all with easy-to-read syntax.

## What Is a Variable?

A **variable** is like a named **storage box** for a piece of data. You give the box a name and put something inside it. Later, you can use the name to get the data back or change it. In traditional programming, you might use symbols or cryptic syntax to handle variables, but in WFL we do it using plain English.

**Why use variables?** Suppose you want to remember a user's age or a price of an item. Instead of writing the number everywhere, you store it in a variable with a clear name (like "age" or "price"). This way, you can refer to that piece of information later by name, which makes your code easier to read and change.

## Creating and Naming Variables

In WFL, creating a variable reads almost like a simple sentence. You typically use the keyword **`store`** or **`create`** to introduce a new variable. For example:

```wfl
store name as "Alice"
```

This line **creates a variable** called **`name`** and **stores** the text `"Alice"` in it. It’s like saying *“store name as Alice”* – very natural! Similarly, you can store numbers or other values. For instance:

```wfl
store count as 42
store temperature as -5
store message as "Hello, world!"
store is active as yes
```

Here we made a **`count`** variable with the number 42, a **`temperature`** with -5, a **`message`** with `"Hello, world!"`, and an **`is active`** flag with the value **yes** (WFL uses **yes/no** as natural equivalents to true/false). Notice that variable names can be multiple words (like **`is active`**) to clearly describe what they hold. WFL encourages descriptive names – it’s perfectly fine to have spaces in the name. This makes the code read like English (e.g., *“is active is yes”*).

A couple more examples of different value types:

- **Numbers:** You can write numbers in a friendly way. `store price as 10.99` (a decimal) or even `store very large as 1 million` (WFL understands common phrases for large numbers).
- **Text:** Always put text in quotes. For example, `store greeting as "Hello"`.
- **Yes/No (Booleans):** Use **yes** or **no** (or **true/false**, which WFL also accepts). For example, `store has account as no`.
- **Special values:** WFL has words for “nothingness” – for cases when a variable has no value. You can use **nothing**, **missing**, or **undefined** depending on context. For example, `store unknown value as nothing` might indicate an absence of a value.

### Creating Constants

Sometimes you have a value that never changes (like π = 3.14159). You can mark it as a **constant**. In WFL, that’s as easy as saying:

```wfl
create new constant PI as 3.14159
```

This works like a variable, except WFL knows you don't intend to change it later. Naming constants in all caps (like `PI`) is a common convention, but not required.

### A Note on “Let” Syntax

You might wonder if you can use more conversational phrasing. Indeed, one of WFL’s goals is to let you write code that sounds like English. For example, you could say something like *“Let the user’s name be Alice.”* While the official syntax uses `store` or `create`, the idea is the same. The key is that WFL tries to be as readable as possible. So even if we use `store X as Y` in these examples, you can read it in your head as “let X be Y.”

## Updating and Changing Variables

Once you have variables, you’ll often want to change their values. WFL provides natural-language ways to do this, avoiding cryptic operators. Suppose we have:

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

Let’s go through those:

- **`change count to 1`:** This sets the value of `count` to 1 (basically an assignment).
- **`add 5 to count`:** Increases `count` by 5. If count was 1, it becomes 6.
- **`subtract 2 from count`:** Decreases `count` by 2.
- **`multiply count by 3`:** Multiplies the value of `count` by 3.
- **`divide count by 2`:** Divides the value of `count` by 2.

These are equivalent to `count = count + 5`, `count = count - 2`, etc., but notice how **readable** they are. You can almost read them as simple instructions in English, which is exactly WFL’s goal!

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

Again, the syntax is self-explanatory: you read it as you would in English. *“Take the first 5 letters from message.”* It’s clear what's happening without needing any complex functions or slicing indices.

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

We tried to add "red" twice. WFL will automatically ignore duplicates in a set. So our `unique colors` set effectively contains just "red" and "blue". The order is not as important in a set, and you won’t have repeats.

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

A **record** in WFL is a way to group several related values together under one name. You can think of it like a simple object or a structured data type. If you have information that naturally goes together (like a person’s name, age, and email), you can store it in a record instead of separate variables.

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
- To **get** a field from a record, you can use a possessive style. For example, `get person's email` would retrieve `"john@example.com"`. (It reads like *“get person’s email”*. In WFL, the apostrophe in code might not actually be required – you might simply write `get person email` – but conceptually it's the same idea of ownership.)
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

The features above cover the basics of storing and manipulating data. WFL also provides higher-level constructs to make sure your data is in the right format and to convert between types safely. These are more intermediate features – you might not need them when you first start – but it’s nice to know they exist as you grow more confident.

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

In this example, we attempt to convert the text `"abc"` to a number. Clearly, `"abc"` isn’t a numeric string, so it would fail. The **`safely convert ...`** block catches that:
- `when invalid:` means if the conversion is invalid (e.g., not a number), then do the following indented steps. Here we say `use 0 instead`, meaning if `"abc"` can’t convert, just use 0.
- `when missing:` would handle the case where the value was missing/undefined. In our example it doesn’t apply, but we included it to show the pattern – maybe if the value was not there, we could prompt the user for a number.

This safe conversion block ensures your program can decide what to do if data isn’t in the expected format, without crashing. It reads like a plan: *“Try to convert, if invalid use 0, if missing ask user.”*

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

If any of these conditions are violated, WFL would throw a clear error message explaining which rule failed. This is great for catching mistakes early. The phrasing is so straightforward, it’s like writing documentation for your data that the computer also reads.

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
- It can’t be empty.
- It must have an "@" symbol.
- It must have a "." that comes after the "@"
- It must not have any spaces.

These rules cover basic email format checking, and again, they are written in plain terms. If `email` doesn’t meet these, WFL will report which rule was broken (e.g., "email must contain '@'").

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

This creates a reusable rule (kind of like a template for checking passwords). Later you might apply `valid password` to check a user’s password input. The idea is to keep your code DRY (Don’t Repeat Yourself) – define the checks once and reuse.

### Using Conditions in Expressions

We’ve seen how to enforce rules. WFL also lets you simply **check a condition** on the fly. For instance:

```wfl
check if temperature is above 0
check if name is "Alice"
```

These could be used inside other constructs (like in an if-statement, which we cover in the next section on Control Flow). The point here is that WFL uses very natural comparative phrases:
- *“is above 0”*, *“is at least 18”*, *“is at most 100”*, *“is between 1 and 10”* are all valid ways to compare values.
- You can combine conditions with **and** or **or** in a check. For example:

```wfl
check if:
    age is at least 18
    and country is "USA"
    and has consent is yes
end check
```

This checks that **all** the listed conditions are true (the person is at least 18 **and** from USA **and** consent is yes). We could similarly use **or** if only one of multiple conditions needs to be true.

(We will dive deeper into `if` statements and conditional logic in the Control Flow section, so don’t worry if you want to learn more about how `check if` works in practice. The takeaway here for the Variables chapter is just that WFL’s conditions read naturally and can be used to assert things about your data.)

### Default Values for Variables

When creating variables, you might want to specify a default value up front, especially if it’s something that might change later or if it’s optional information. WFL allows default values in declarations:

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
  This suggests we have a module (like a grouping of related code) named "user handling", and inside it we have `user count` and `active users` defined as shared. Code within the same module can use them, but outside code won’t see them (which helps prevent accidental misuse).

For a beginner, you don't need to deeply worry about scope if you’re writing small scripts – just create variables as needed. But it’s good to know these words (global, local, shared) exist and do exactly what they say. It keeps your code organized as it grows.

### Monitoring Changes to Variables (Advanced)

This is a more advanced feature, but an exciting one: WFL can **watch** for changes in variables and react to them. This is beyond what most beginner programmers need, but it shows the power of WFL’s natural syntax.

For example, say you want to take action whenever a variable changes value:

```wfl
track changes to user count:
    when increases:
        check if user count is above limit
    when decreases:
        update display
end track
```

This means: *“Track changes to `user count`. If it increases, then check if it went above some limit (perhaps to alert or prevent something). If it decreases, update the display.”* The keywords `when increases` and `when decreases` let you respond to how the value changed.

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

Here, we watch the `settings` (which could be a map or record of user settings). If the `theme` inside `settings` changes, we call some code to update the display theme. If `volume` changes, we adjust the audio output, and so on. It’s like saying “keep an eye on these and respond when needed.”

These features can make building interactive or reactive programs easier because the code reads like event descriptions. Again, this is an intermediate-to-advanced concept; new users of WFL can safely ignore tracking/watching until they have a need for it. But it’s nice to know WFL can do this with such clear syntax.

## Conclusion

In this chapter, we covered a lot of ground in how WFL handles data:

- We learned that variables in WFL are declared with everyday words like **store** and **create**, and you can name them in a descriptive, spaced-out way (making code self-documenting).
- We saw basic data types: numbers, text, yes/no booleans, and special placeholders like nothing/missing.
- We performed operations to update variables (in plain arithmetic language) and to manipulate text.
- We organized data using lists, sets, and maps, and grouped related data into records.
- We touched on converting between types safely, validating data with simple rules, setting default values, and even advanced features like scope control and change tracking.

The key theme is **clarity**. WFL’s variable and data handling is designed to be **clear at a glance**. You don't see cryptic symbols or hard-to-guess abbreviations – it’s all written out in a way that beginners can understand and experienced developers can appreciate for its readability.

Feel free to play around with these examples. Try creating your own variables and collections. In the next sections, we’ll use these variables in control structures (like loops and conditionals) and see how WFL continues the trend of English-like syntax for coding tasks. Happy coding!

---

# WFL Loops: Repeating Actions the Easy Way

**Summary:** This section introduces **loops** in WFL – ways to repeat an action or series of actions multiple times. Loops are incredibly useful when you want to avoid writing the same code over and over. In a friendly, beginner-oriented style, we'll explain three main types of loops in WFL: counting loops (for counting up or down), collection loops (for going through every item in a list or set), and conditional loops (repeat while/until some condition is met). By the end, you'll know how to read and write loops that sound almost like instructions you’d give to a person, thanks to WFL's natural syntax. We’ll also peek at loop controls like breaking out early or skipping an iteration, and mention some advanced loop capabilities to spark your curiosity.

## Why Loops? (A Gentle Introduction)

Imagine you need to print "Hello!" 5 times. You *could* write `display "Hello!"` five times in a row – but that’s tedious and not practical for larger tasks. Loops let you tell the computer, "Do this action X number of times" or "Do this action for each item in this list" without repeating yourself. It's like giving one instruction that expands into many.

For example, think of telling someone: *"For each student in the class, hand out a worksheet."* You don't list each student by name in your instruction; you use the loop concept ("for each student") to implicitly repeat the action.

WFL loops are designed to be as straightforward as such an instruction. They come in a few flavors to fit different scenarios.

## Counting Loops (Repeating a Certain Number of Times)

A **count loop** is useful when you know how many times you want to repeat something (or at least the start and end of a range of numbers to iterate over).

In WFL, a counting loop might look like this:

```wfl
count from 1 to 5:
    display "Hello!"
end count
```

Reading it in English: *“Count from 1 to 5: display 'Hello!' end count.”* This will display "Hello!" five times, with the loop automatically going through 1, 2, 3, 4, 5 as the counts.

You can use the current count number inside the loop if you want. WFL implicitly provides a loop variable (often just called `count` by default in a count loop) that holds the current number. For example:

```wfl
count from 1 to 5:
    display "This is iteration number " with count
end count
```

This would output:
```
This is iteration number 1
This is iteration number 2
...
This is iteration number 5
```
Each time, `count` had a different value from 1 through 5.

### Specifying Step Increments

By default, `count from 1 to 5` goes up by 1 each time. If you want a different step, you can say so:

```wfl
count from 0 to 10 by 2:
    display count
end count
```

This will count 0, 2, 4, 6, 8, 10 (printing each). We added **`by 2`** to indicate the step increment. You can use any number, even negative steps.

### Counting Downwards

Need to loop in reverse? Easy:

```wfl
count from 5 down to 1:
    display count
end count
```

This will count 5, 4, 3, 2, 1. We use **`down to`** for clarity. As you can see, WFL avoids needing something like a separate decrement operation; it directly understands the intention to count down.

Counting loops are great for numeric sequences or any case where you want a fixed number of repetitions. They’re also a good fit for scenarios like "do X ten times" or "for each number in this range, do Y."

## Looping Through Collections (For-Each Loops)

Often, you will have a collection of items (like a list or set, which we discussed in the Variables section) and you want to do something for each item in that collection. For example, print each element, or process each object.

WFL provides a **for-each loop** style that reads nicely:

```wfl
for each item in shopping:
    display item
end for
```

If `shopping` is a list (say `["milk", "bread", "eggs"]`), this loop will go through each element and display it. The variable `item` is a placeholder that takes the value of each element in turn:
- First iteration: `item` is `"milk"` – it displays "milk".
- Second: `item` is `"bread"` – displays "bread".
- Third: `item` is `"eggs"` – displays "eggs".

After the loop, it automatically stops when it reaches the end of the collection.

**Note:** You can use any name instead of `item` for the loop variable. If you're looping through a list of users, you might write `for each user in users: ...`. The name should describe each element.

### Using an Index in a Loop

Sometimes you want to know the index (position) of each item as you loop through. WFL can provide that too:

```wfl
for each index and item in shopping:
    display index with ": " with item
end for
```

This will output something like:
```
1: milk
2: bread
3: eggs
```
assuming indexing starts at 1 (which is a fair guess in a human-friendly language like WFL). Here, we wrote **`for each index and item in ...`**, so WFL gives us two loop variables: `index` (the position of the item, 1, 2, 3, ...) and `item` (the value itself). The display line then uses both.

Using an index is optional – only use it when needed.

### Looping in Reverse Order

If for some reason you want to loop through a collection backwards, WFL makes that easy too:

```wfl
for each item in shopping reversed:
    display item
end for
```

By adding **`reversed`**, the loop will go through the `shopping` list from the last item to the first. So it would display "eggs", then "bread", then "milk" in our example.

This saves you from having to manually figure out the last index and decrement, etc. Just state it and WFL handles it.

## Conditional Loops (Repeat While / Until)

The last major category of loops is the **conditional loop**, where you keep looping based on a condition rather than a fixed range or collection. This is typically the "while" loop in other languages. WFL offers a clear syntax for both "while" and "until" scenarios:

- Use **`repeat while ...`** to continue looping *as long as* a condition is true.
- Use **`repeat until ...`** to continue looping *until* a condition becomes true (i.e., loop while the condition is false, and stop once it’s true).

Example of a while-style loop:

```wfl
repeat while user is not logged in:
    prompt login
end repeat
```

This reads: *“Repeat while user is not logged in: prompt login.”* The loop will keep prompting the login until the condition `user is not logged in` becomes false (meaning the user has logged in, so `user is logged in` is true). Note the natural negation: "is not logged in".

Example of an until-style loop:

```wfl
repeat until temperature is above 100:
    increase heater
end repeat
```

This means *“keep increasing the heater until the temperature is above 100.”* The moment the temperature goes above 100, the loop stops.

You could phrase that as a while loop too (`repeat while temperature is at most 100` perhaps), but WFL gives you flexibility to choose what sounds clearer to you.

There’s also a shorthand if you want an infinite loop:

```wfl
repeat forever:
    check system status
end repeat
```

This will loop endlessly. Of course, in real scenarios you’d have some break condition inside, or you'd be running an ongoing service. But it shows how WFL even covers that with plain language – *“repeat forever”* is pretty unambiguous.

## Loop Control: Breaking and Skipping

Loops often need control statements to either **break out** of the loop early or **skip the rest of the current iteration** and continue with the next one. In many languages, these are `break` and `continue`. WFL uses more descriptive words:

- **`stop`** – to exit the loop entirely (break).
- **`skip`** – to skip to the next iteration (continue).

For example, let's say we are counting to 10 but want to stop if a condition is met:

```wfl
count from 1 to 10:
    check if some condition:
        stop count
    end check
    display "Continuing loop"
end count
```

If `some condition` becomes true, `stop count` will break out of the loop named `count`. We name the loop implicitly by its type (here we wrote `count from 1 to 10`, so using `stop count` refers to stopping that loop). If it were a `for each`, we’d say `stop for`, and for a `repeat` loop, likely `stop repeat`. (You can also give a loop a custom name to refer to it, which is useful in nested loops – more on that in a moment).

Skipping an iteration is similar. Suppose we want to skip even numbers in a count:

```wfl
count from 1 to 5:
    check if count is even:
        skip       // skip this iteration if count is even
    end check
    display count
end count
```

This would only display odd numbers 1, 3, 5 because when `count` was 2 or 4, it hit `skip` and jumped to the next number without executing the `display` line.

Again, the word **skip** perfectly describes what it does.

### Nested Loops and Naming Loops (Advanced Use)

If you put one loop inside another (nested loops), you might want to control the inner or outer loop specifically when breaking or skipping. WFL allows loops to be **named** for clarity:

```wfl
count from 1 to 3 as outer:
    count from 1 to 5 as inner:
        check if inner is 3:
            stop inner count
        end check
        display outer with " - " with inner
    end count
end count
```

Here we named the outer loop "outer" and the inner loop "inner". Inside, we say `stop inner count` when `inner is 3`. This will break out of the inner loop only, while the outer loop continues. If we said just `stop count` without specifying inner or outer, it might default to the current loop. But being explicit is good practice in nested loops.

The output of the above (if you run it) would show an outer counter and inner counter pairs, but it would stop the inner loop early each time when inner reaches 3.

This is a bit more advanced, but it shows how WFL lets you keep control flow understandable even in complex scenarios. You can also use `skip inner` or `stop outer`, etc., as needed.

## Advanced Loop Features (For the Curious)

Even though beginners can do a lot with loops as described above, WFL has some powerful loop features that you might explore as you become an intermediate user:

- **Error Handling in Loops:** WFL can handle errors during loop execution gracefully. For example:
  ```wfl
  try count from 0 to max:
      when error occurs:
          log error
          skip current iteration
      when timeout after 5 seconds:
          stop counting
      otherwise:
          process item
  end try
  ```
  This loop attempts to count from 0 to `max`. If any error occurs in the loop body, it logs the error and skips to the next iteration. If the loop runs for too long (5 seconds) without finishing (perhaps due to some external wait), it stops the loop. The `otherwise` clause is what to do normally each iteration. The syntax reads like a plan: "try this loop, if error then..., if timeout then..., otherwise do...". This feature ensures your loops can cope with unexpected issues (like a network failure inside the loop, etc.) without crashing the whole program. It’s a bit like a safety net around the loop.

- **Parallel Loops (Concurrency):** WFL can run loop iterations in parallel if you specify a concurrency. For example:
  ```wfl
  for each item in tasks with concurrency 4:
      process item in parallel
  end for
  ```
  This would try to process up to 4 items at the same time (in parallel). It’s like having multiple workers on the loop. This is useful for performance in heavy tasks, and WFL again expresses it clearly: *“with concurrency 4”* means four at a time.

- **Batch Processing:** You can even loop in chunks:
  ```wfl
  for each batch in collection with batch size 100:
      process batch
  end for
  ```
  That will iterate through your collection 100 items at a time as `batch`. If your collection has 1000 items, that loop will run 10 times, each time with a batch of 100 items. This is great for managing large data sets or limiting memory usage.

- **Resource Management in Loops:** An advanced scenario:
  ```wfl
  count from 0 to N with resources:
      allocate memory
      try:
          // do heavy processing
      finally:
          cleanup memory
      end try
  end count
  ```
  This hypothetical loop ensures some resource (like memory or file handle) is allocated and later cleaned up each iteration. The keywords and structure (`with resources`, `finally: cleanup`) show that WFL can integrate resource management into loop constructs, which experienced developers will appreciate.

These advanced topics go beyond what a beginner needs, but rest assured that as you learn more, WFL’s loop constructs will grow with you, offering capabilities for robust error handling and efficiency while still keeping the code readable.

## Conclusion

Loops in WFL let you do repetitive tasks without repetitive code, all in a style that reads naturally:
- **Counting loops** (`count from ... to ...`) handle numeric iterations.
- **For-each loops** (`for each ... in ...`) elegantly handle going through collections.
- **While/until loops** (`repeat while ...` / `repeat until ...`) cover conditional repetition.
- Clear keywords like `stop` and `skip` give you fine-grained control without confusion.
- And for those interested, WFL’s loop features extend to error handling, parallelism, and more, in a way that still feels high-level and intention-revealing.

Try using loops with the variables and collections you learned about in the previous section. For example, you could loop through a list of names and greet each one, or count down days till an event. With WFL, writing these loops will feel more like writing out instructions than wrestling with complex syntax. Enjoy your looping journey!

---

# WFL Conditionals and Control Flow: Making Decisions in Code

**Summary:** In this section, we focus on **conditionals** – the "if/then/else" logic that allows a program to make decisions. Control flow is all about guiding your program to do different things in different situations. WFL makes conditional logic simple by using words like **if**, **when**, and **otherwise** instead of symbols or hard-to-read syntax. We will start with basic `if` statements (if something is true, do this, otherwise do that), then explore checking multiple conditions at once, and handling multiple alternative cases. By the end, you’ll be comfortable reading and writing WFL code that says what it means, like: *“If the user is an admin, show the admin panel; otherwise, show the dashboard.”* This is crucial for creating dynamic, responsive programs.

## The Basics: If... Then... Otherwise

In everyday life, we make decisions based on conditions: *“If it’s raining, then take an umbrella, otherwise don’t.”* In programming, we want the same ability to choose actions based on conditions. This is done with **if statements**, and WFL implements them in a very straightforward way.

A simple if scenario in WFL:

```wfl
check if temperature is below 0:
    display "Brrr, it's freezing!"
end check
```

Let’s break that down:
- `check if temperature is below 0:` is the condition. It asks: is the value of `temperature` less than 0?
- If the answer is yes (true), then the indented code runs – here it displays a freezing message.
- We didn’t provide an "otherwise" in this example, so if the temperature is not below 0, the program just continues without doing the indented part. And that’s fine.

Often, you'll want an alternative action if the condition is false. That’s where **`otherwise`** comes in (which is essentially the else part):

```wfl
check if user is logged in:
    display "Welcome back!"
otherwise:
    display "Please log in."
end check
```

Reading this, it’s almost exactly how you’d describe the logic in English:
- *If user is logged in, display "Welcome back!", otherwise display "Please log in."*

Only one of the two display statements will run depending on the condition. If `user is logged in` is yes/true, the first message shows; if not, the second message shows.

**Important:** WFL uses the keyword **`check`** to start a conditional and pairs it with **`end check`** at the end (similar to how loops had `end count`, `end for`, etc.). Inside, you use `if ...` and `otherwise` to structure the branches. This is just WFL’s style; conceptually it’s *if/else*.

#### Variations of True/False in WFL

Remember, WFL’s booleans can be expressed as yes/no or true/false. So you might see conditions like `check if is active is yes` or `check if is active is true`. They mean the same thing. The phrasing “is yes” is very natural (reads almost like normal conversation), which is in line with WFL’s philosophy.

Also, you can phrase conditions positively or negatively:
- `if user is logged in` versus `if user is not logged in` (using **not** to negate).
- `if file exists` versus `if file does not exist` (you can use "not" or likely just phrase it accordingly).

WFL tries to let you express the condition in the clearest way. For instance, checking a numeric relation can be done with words:
- `if score is at least 50` (meaning score >= 50),
- `if price is above 100` (price > 100),
- `if items count is at most 10` (<= 10),
- `if temperature is between 20 and 30` (20 <= temp <= 30).

These are much easier to read than using symbols like `>=` or `<=`, especially for those new to coding.

## Combining Multiple Conditions (AND/OR)

Sometimes one condition isn’t enough; you need to check multiple things together. For example, *“if the user is an admin **and** the account is active, then allow access.”* In logic terms, that's an AND condition (both need to be true). Alternatively, *“if today is Saturday **or** Sunday, then it's the weekend.”* That's an OR condition (either one being true is enough).

WFL handles this with the words **and** and **or** within the `check if` block.

Example using AND:

```wfl
check if:
    age is at least 18
    and country is "USA"
    and has consent is yes
otherwise:
    deny access
end check
```

This ensures three conditions are all true:
1. `age is at least 18`
2. `country is "USA"`
3. `has consent is yes`

Only if **all three** are true will the check pass (meaning if we had some code in the first branch, it would run; here I directly put the action in `otherwise` just for illustration to deny if any fail).

We could flesh it out as:

```wfl
check if:
    age is at least 18
    and country is "USA"
    and has consent is yes
    and account is verified
:
    display "Access granted."
otherwise:
    display "Access denied."
end check
```

Now in this full form, the *Access granted* message appears only if every condition in that list is true. If any one of them is false, we fall to *Access denied*.

Example using OR:

```wfl
check if:
    answer is "yes"
    or answer is "y"
    or answer is "ok"
:
    proceed with operation
otherwise:
    cancel operation
end check
```

Here, if the `answer` is any of those acceptable strings, we proceed. Only if none of them match do we cancel. The conditions are stacked in a readable way:
- answer is "yes" OR "y" OR "ok" -> good to go.

Notice the syntax: we started with `check if:` and then listed conditions on new lines, indenting them. After listing, we put a lone `:` (colon) on a line (this is slightly stylistic; WFL might allow you to put the `otherwise` right after the last condition’s line, but the above format is clearer). Then the otherwise.

This multi-line format is essentially how you do complex logical AND/OR in WFL without writing a very long sentence. It’s neatly structured and indented, which aids readability (much like how you might break a long thought into bullet points).

You can mix and and or with proper grouping, but that can get tricky in plain language. WFL likely encourages keeping each check simple (or using nested checks) for clarity. If needed, you could always do something like:

```wfl
check if ( (A and B) or C ):
    ...
```

But since we don’t use a lot of symbols, you might express it as nested:
- Check if A and B:
  - do X
- Otherwise if C:
  - do X
- Otherwise:
  - do Y

Which brings us to the next topic: multiple branches.

## Handling Multiple Cases (Switch/Else-If Logic)

Often, it’s not just a binary choice. You might have multiple possible conditions and different actions for each. For example, *“If the light is green, go. If it's yellow, slow down. If it's red, stop. Otherwise (if it's any other color), call maintenance!”*

We can chain conditions in two ways:
1. Using `otherwise` with a new `check if` (effectively else-if).
2. Using a `check ... when ...` structure for one variable or situation with many cases (like a switch-case in other languages).

**Method 1: Else-If using `otherwise` + `if`**

```wfl
check if temperature is above 30:
    display "It's hot outside."
otherwise check if temperature is below 0:
    display "It's freezing outside."
otherwise:
    display "The temperature is moderate."
end check
```

This reads:
- If temperature > 30, say it’s hot.
- Otherwise, if temperature < 0, say it’s freezing.
- Otherwise (meaning it's neither >30 nor <0, so it's between 0 and 30 inclusive), say it's moderate.

We achieved a 3-case logic (hot, freezing, moderate) by stacking an `otherwise check if ...` in the middle.

You can extend this pattern for as many else-if cases as needed, each time writing `otherwise check if ...` with its block.

**Method 2: Using `when` for one subject**

If you are dealing with one variable or expression and many possible values or ranges, WFL allows a compact form:

```wfl
check traffic light:
    when "green":    proceed
    when "yellow":   slow down
    when "red":      stop
    otherwise:       call maintenance
end check
```

This is analogous to a switch on `traffic light`:
- If it's "green", do `proceed`.
- If it's "yellow", do `slow down`.
- If it's "red", do `stop`.
- Otherwise (if it's none of those, maybe a malfunction showing a weird color), do `call maintenance`.

This structure is very readable. It puts the variable (`traffic light`) up top after `check`, and then lists cases with **when "value":** action, and a final `otherwise` for anything not matched.

Another example:

```wfl
define action greet person:
    needs:
        language as text
    do:
        check language:
            when "English": display "Hello!"
            when "Spanish": display "¡Hola!"
            when "French":  display "Bonjour!"
            otherwise:      display "Hi!"
        end check
end action
```

In this hypothetical function, based on the `language` provided, it greets in that language, with a default to "Hi!" if the language didn't match any of the listed ones. This shows how `when ... otherwise` can be used inside an action, and how nicely it scales to multiple branches.

This style is especially helpful when you have to categorize a single value into many cases. It avoids writing `if language == X else-if language == Y ...` repeatedly.

## A Quick Note on Pattern Matching (Advanced)

WFL aims to cover pattern matching (mentioned in its design principles) in a natural way. While at the time of writing your basic toolkit is `if/when/otherwise` as shown, future or more advanced versions of WFL might allow things like:

- Checking the **type** of a variable or structure (e.g., *“Match the input: when it’s a number do ..., when it’s text do ...”*).
- Destructuring patterns (like matching a list that is empty vs non-empty, etc.), possibly with a natural phrasing.

For now, just be aware that WFL’s philosophy would handle pattern matching in an English-like manner as well. If you need complex matching, you usually can achieve it with the combination of multi-branch checks shown above.

## Putting It All Together: Examples of Control Flow

Let's combine loops and conditionals briefly to see a real-world-esque example in WFL:

```wfl
for each user in users:
    check if user is active:
        display user name with " is online."
    otherwise:
        display user name with " is offline."
    end check
end for
```

This loop goes through a list of `users` and, for each `user`, checks a condition (perhaps `user is active` is a truth value field in a user record). It then displays an appropriate message for each user. The code is easy to follow, almost like reading a report: *“For each user in users, if user is active, display '[name] is online.', otherwise '[name] is offline.'”*

Another example with conditional and action:

```wfl
check if score is at least passing score:
    perform congratulate with student
otherwise:
    perform offer remedial help with student
end check
```

This assumes we have some actions defined (like `congratulate` or `offer remedial help`) and a variable `passing score`. It demonstrates how you might call different actions based on a condition. We’ll cover how actions/functions work in the next section, but you can already see how nicely the conditional reads in WFL – there’s no weird punctuation cluttering the intent.

## Conclusion

Conditionals in WFL give your program the power of decision-making without bogging you down in syntax:
- The **`check if ... otherwise ... end check`** structure is the backbone, corresponding to if/else.
- **Multiple conditions** can be combined with **and/or** in a clear list format.
- **Multiple branches** (else-if chains or switch-case style) are handled with either subsequent `otherwise check if` or the tidy `when ...` syntax for one subject.
- You can express conditions with natural phrases like "at least", "contains", "is not", which make the code self-explanatory.
- Overall, WFL’s control flow reads like describing the logic in plain English, making it accessible for beginners to understand the flow of the program.

As you proceed, try writing some `if` statements in WFL for practice. For instance, *“If a number is even, say so, otherwise say it’s odd.”* or *“If a list is empty, display 'No items' otherwise display the count of items.”* You’ll find that writing these conditions feels almost like writing a normal sentence. This lowers the barrier to experimenting and learning. Happy coding, and let's move on to see how we can package reusable logic using functions (actions) next!

---

# WFL Functions (Actions): Reusing Code with Natural Syntax

**Summary:** This section is all about **functions** in WFL (often called **actions** to emphasize their behavior). Functions/actions let you define a set of instructions once and then use (or "call") them whenever needed, avoiding repetition and keeping code organized. We will learn how to define an action with or without parameters (inputs), how to return values from actions, and how to call these actions. The syntax is designed to feel like defining and invoking **tasks** or **capabilities** in plain language, e.g., *“Define action send email... end action”* and later *“perform send email with recipient as 'alice@example.com'”*. We’ll start simple and then touch on advanced features like asynchronous actions (for waiting on results), overloading actions by type, and generics – but don’t worry, we’ll keep explanations beginner-friendly and focus on the core idea: packaging logic under a clear name.

## What is a Function/Action?

If variables are the nouns of programming (data), then **functions** are the verbs – the actions or behaviors. A function is a reusable piece of code that can be "called" to perform a task. WFL refers to functions as **actions** to reinforce the idea that they do something active.

Think of it like a cooking recipe: you might define a recipe for "Bake a cake" (that's your function definition), and whenever you want a cake, you follow that recipe (that's calling the function). If you had to write down all the steps every time you wanted to bake, it’d be repetitive – so it's much better to have the recipe written once and just reference it.

In programming, if you find yourself writing similar code more than once, it's probably a good candidate to turn into an action.

## Defining a Simple Action

Defining an action in WFL is straightforward and very descriptive. Here’s an example of the simplest action (no inputs needed):

```wfl
define action say hello:
    display "Hello, world!"
end action
```

This creates a new action named **`say hello`**. Inside, it just does one thing: displays "Hello, world!". The structure:
- **`define action [name]:`** starts the definition. We chose the name "say hello".
- Then the body of the action is indented. We put whatever steps we want the action to do.
- Finally, **`end action`** closes the definition.

Now, how do we use this action? After defining it, elsewhere in our code we can **call** or **perform** it by name. For example:

```wfl
perform say hello
```

This line would trigger the action, causing "Hello, world!" to be displayed. In many languages, you'd call a function by writing something like `say_hello()` or similar. In WFL, we can often just use the phrase or use the keyword `perform` to make it clear. Both "perform say hello" or potentially just "say hello" in context could work. Using `perform` is explicit and always safe to show.

So the full usage would be:

```wfl
define action say hello:
    display "Hello, world!"
end action

// ... later ...
perform say hello
perform say hello   // call it as many times as you want
```

Each time, it prints the greeting. We defined it once, and we can reuse it easily.

Notice how the action name `say hello` is written in plain language (with a space). WFL allows that. It's like naming a recipe "Bake cake" or "Send email". Use a short phrase that clearly indicates what the action does.

## Actions with Parameters (Inputs)

Most of the time, actions need some information to do their job. For example, a `greet` action would likely need to know whom to greet (a name), and maybe how (the greeting message). These pieces of information are called **parameters** or **inputs** to the action.

Defining parameters in WFL is done in a section called **`needs:`** inside the action definition. Let's create an action that greets a person by name:

```wfl
define action greet person:
    needs:
        name as text
    do:
        display "Hello, " with name with "!"
end action
```

Here’s what’s happening:
- We start `define action greet person:`. We chose the name "greet person".
- We then specify `needs:` followed by an indented list of parameters.
- In this case, the action needs one thing: **`name as text`**. This means when we call `greet person`, we must provide a **name** and it should be a piece of text.
- After listing needs, we have a **`do:`** section where we write what the action actually does with those inputs.
- The body `display "Hello, " with name with "!"` simply prints a greeting including the given name.

Calling this action requires providing a `name`. How to call it:

```wfl
perform greet person with name as "Alice"
```

This would output: `Hello, Alice!`

Let's break the call:
- `perform greet person` – calls the action.
- `with name as "Alice"` – provides the parameter. We explicitly say `name as "Alice"` to match the `needs: name as text`. (WFL might be smart enough to allow just giving "Alice" positionally, but explicit is clearer here, especially with multiple parameters.)

If we had multiple parameters, we’d list them separated, for example:
```wfl
perform some action with param1 as X, param2 as Y
```
But WFL often uses a slightly different style (like a with block) for multiple parameters, which we'll see soon.

### Default Parameter Values

We can make parameters optional by providing default values. For instance, if our `greet person` action had a default greeting language or style, we could do:

```wfl
define action greet person:
    needs:
        name as text
        language as text with default "English"
    do:
        check language:
            when "English": display "Hello, " with name
            when "Spanish": display "¡Hola, " with name
            otherwise: display "Hi, " with name
        end check
end action
```

Now `greet person` can take two inputs: `name` and `language`. But if you don't specify `language` when calling, it will default to "English".

Calling examples:
- `perform greet person with name as "Carlos" and language as "Spanish"` – would output "¡Hola, Carlos".
- `perform greet person with name as "Bob"` – since language not provided, defaults to English, outputs "Hello, Bob".

*(The call syntax `with name as "Carlos" and language as "Spanish"` shows you can chain parameters with **and** in a very readable way. Alternatively, WFL might allow a block style:
```wfl
perform greet person with:
    name as "Carlos"
    language as "Spanish"
end with
```
Both mean the same thing. The block form can be useful if you have many parameters to clearly separate them.)*

Using default values is great for simplifying calls when common cases are used. We defined a default "English" so only when a different language is needed do we specify it.

### Multiple Parameters and Natural Order

WFL tries to make the action calls read nicely. You might define an action with multiple needs, and when calling, it could sound like a sentence.

For example, a slightly different spin:
```wfl
define action send message:
    needs:
        recipient as text
        content as text
    do:
        // ... imagine code to actually send a message ...
        display "Sent '" with content with "' to " with recipient
end action
```

Calling it:
```wfl
perform send message with recipient as "Bob" and content as "Meeting at 3 PM"
```

This reads: *“perform send message with recipient as Bob and content as Meeting at 3 PM”*. It’s clear who the recipient is and what the content is. The word **and** helps chain multiple parameters in the call.

Alternatively, the call could be:
```wfl
perform send message with:
    recipient as "Bob"
    content as "Meeting at 3 PM"
end with
```
Either way, it’s quite readable.

## Actions that Return a Value (Outputs)

Sometimes an action isn’t just doing something; it also computes and **gives back** a result. This is akin to a function returning a value in other languages.

In WFL, you specify what an action gives back using **`gives back:`** in the definition, and you return a value with **`give back`** in the body.

For example, let's define a simple action to add two numbers and return the sum:

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

- We declare `gives back: sum as number`. This means this action will produce an output called **sum**, which will be a number.
- In the body, instead of `display` or something, we use `give back x plus y`. WFL allows using the word **plus** as a natural addition operator (just like "and" in normal math speech). So x plus y is the sum. We could also do `x + y` if symbols were allowed, but WFL prefers words.

Using this action:
```wfl
store result as perform add numbers with x as 5 and y as 7
display result   // would display 12
```

We call `perform add numbers` with 5 and 7, and we **store result as** that call. The action returns a number (the sum 12), which gets stored in the variable `result`. Then we display `result`.

Important detail: We named the return value `sum` in the definition, but when capturing the result of the call, we don't have to use the same name. We used a variable `result`. The name `sum` in the definition is mostly to describe the output. (In some contexts, WFL might allow `perform add numbers with ... gives back sum` but typically you just capture it as shown.)

Another example: An action could return a more complex thing, like a record or text. For instance, a format action:
```wfl
define action make greeting:
    needs:
        name as text
    gives back:
        message as text
    do:
        give back join "Hello, " and name and "!"
end action
```

This returns a text. We could then:
```wfl
store greet_msg as perform make greeting with name as "Alice"
display greet_msg   // displays "Hello, Alice!"
```

## Calling Actions (Performing Actions)

We’ve already seen usage of `perform`. To summarize:
- **Defining** an action uses `define action ... end action`.
- **Calling** an action uses `perform ...`.

You might wonder: do we always need the word `perform`? Not necessarily – WFL might let you call an action just by using its name if the context is unambiguous. For example, perhaps writing `send message with recipient as "Bob"...` could implicitly perform it. However, using `perform` is never wrong and it makes it clear we are invoking an action.

Also, when an action is part of a container (like a method on an object), you'll use a slightly different syntax (we'll see that in the Containers section). But generally:
- If it's a standalone action, `perform actionName with ...` is a safe pattern.
- If it's an action that returns something, you can do `store X as perform actionName...` or use it in an expression where appropriate.

## Advanced Features of Actions (Optional Reading)

WFL, staying true to being beginner-friendly, doesn't force you to learn these upfront. But just so you know what’s possible as you advance:

### Asynchronous Actions

In web programming, some actions might take time, like fetching data from a server or waiting for user input. WFL allows actions to be asynchronous (runnning in the background or waiting without freezing everything).

You mark an action as **`async`**:

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

This is a conceptual example. The `await fetch from url` suggests WFL can integrate with asynchronous operations (perhaps wrapping a JavaScript fetch call). The key is:
- We declared it `async action`.
- Inside, we used `await` to wait for something (fetching from the URL).
- We then `give back response data`.

When calling an async action, you might also use `await` if you need the result:
```wfl
store info as await perform fetch data with url as "https://api.example.com/users"
```
This says "perform fetch data, but wait for it to finish and give me the result as info".

As a beginner, you might not deal with async immediately, but it’s nice that WFL supports it in a clean way.

### Action Visibility: Public/Private

Sometimes you write an action that's only used internally (perhaps within a container or module). WFL allows scoping actions:
- `define action ...` (by default likely public if at top level).
- `define private action ...` to restrict usage to the current context (like inside a container only).
- This is more relevant in containers (where you can have private helper actions not exposed outside).

If you see `define private action internal helper: ...` in some code, just know it's an action meant for internal use.

### Cleanup and Error Handling in Actions

WFL actions can include error handling logic similarly to loops:
- You might see a `try ... finally ... end try` inside an action, ensuring something happens regardless of errors (like closing a file).
- The earlier example in documentation:
  ```wfl
  define action process file with cleanup:
      needs:
          file path as text
      do:
          open file at file path
          try:
              // process file contents
          finally:
              close file
          end try
  end action
  ```
  The `with cleanup` in the name is just part of the name. The actual mechanism is the `try/finally` which ensures `close file` runs no matter what.

For beginners, the takeaway is that WFL can express these patterns in words (“finally: close file”) rather than symbols or obscure syntax.

### Overloading Actions

This means having the same action name do different things depending on context or parameter types. WFL supports it in a type-safe way:

Example:
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

Here we defined two `format value` actions: one for numbers, one for dates. WFL can tell which one to use based on what you pass in:
- If you `perform format value with number as 3.14`, it knows to use the first (giving back a text "3.14" maybe).
- If you `perform format value with date as today`, it uses the second.

This is advanced but powerful. It lets you use the same natural phrase for similar concepts and rely on WFL to pick the right one by the types.

### Generic Actions

For the truly curious: WFL can define actions that are generic, meaning they work with any type but maintain type safety. For example:
```wfl
define action identity Of Type T:
    needs:
        value as T
    gives back:
        result as T
    do:
        give back value
end action
```

This is a contrived example (an identity function that just returns what you give it). The `Of Type T` part means this action is parameterized by a type T. If you call `perform identity with value as 5` (an integer), WFL treats T as number and expects a number back. If you call it with text, T becomes text, etc.

Generic actions are more advanced and you'd use them rarely as a beginner, but WFL includes them to allow writing flexible, reusable code without sacrificing the clarity of types.

## Conclusion

Functions (actions) in WFL let you encapsulate logic and give it a name, making your code more organized and avoiding repetition. The highlights:
- **Defining actions** with `define action ...` and ending with `end action`, using `needs` for inputs and `gives back` for outputs when necessary.
- **Calling actions** with the `perform ... with ...` syntax, which reads like you're instructing the program what to do (very much like natural language).
- The ability to use default parameters and multiple parameters keeps action calls concise or detailed as needed.
- Returning values with `give back` integrates nicely with the rest of WFL (you can store the result or use it in further computation).
- Advanced features like async actions and overloading exist but you can gradually learn those.

With actions under your belt, you can now create more complex programs. For example, you could define actions to calculate something, actions to format output, actions to handle user input, etc., and then combine them. The code will read like a set of instructions or a story, thanks to WFL’s design. 

Next, we'll explore **containers (classes)**, which let you bundle related data and actions together – a concept that builds on what we know about variables and actions, but at a larger scale. It’s like creating your own custom structures in the language of the problem you’re solving. Onward!

---

# WFL Containers (Classes): Structuring Data and Behavior Together

**Summary:** In this final section, we look at **containers** in WFL, which are analogous to classes or objects in other languages. A container groups **data** (like variables) and **actions** (like functions) into a single unit, modeling a real-world entity or concept in your program. For example, you might have a `User` container that has the user's data (name, email, etc.) and actions (like `update email`, `get profile`). We'll start from scratch, introducing how to define a container, how to create instances (objects) from it, and how to use them. We’ll keep it beginner-friendly by analogy (think of a container as a template or blueprint). Then we’ll touch on more advanced OOP concepts like inheritance (one container building on another), interfaces (defining expected actions without implementation), and composition (containers using other containers). By the end, you'll see how WFL maintains its natural style even in the realm of object-oriented programming.

## What is a Container?

A **container** in WFL is like a blueprint or template for an object. If you’ve never worked with classes/objects:
- Imagine a **blueprint** for a house – it defines how a house is structured (number of rooms, layout), but it's not a real house yet. You can build multiple houses from the same blueprint.
- Similarly, a container defines the structure (data fields) and behavior (actions) for something, and then you can create actual **instances** of that container to use in your program.

For example, a `User` container might define that every user has a name, email, and role (data), and can perform actions like `get profile` or `update email`. Once defined, you can create a specific user (instance) with a name and email and use those actions on that instance.

If variables and records gave us structured data, containers take it a step further by bundling data with the functions that operate on that data. This helps organize code in larger programs, as everything related to "User" is in one place.

## Defining a Basic Container

Let's define a simple container step by step. We'll create a `User` container as an example.

```wfl
create container User:
    // Define what a User has and can do
end container
```

This is the shell. Now inside, we can specify:
- Data fields (as variables) that the User has.
- Actions that the User can perform.
- Special sections like a constructor (what to do when a new User is created).

WFL organizes container contents by **visibility**:
- **private:** only accessible inside the container itself.
- **protected:** accessible inside the container and any container that extends it (like subclasses).
- **public:** accessible from anywhere (the public interface of the container).

Let's fill in some details for `User`:

```wfl
create container User:
    // Private members (internal use only)
    private:
        store id as text
        store email as text

    // Public members (accessible from outside)
    public:
        when created:
            needs:
                email as text
            do:
                set id to generate unique id
                set email to email
        end when

        define action get email:
            give back email
        end action

        define action update email:
            needs:
                new email as text
            gives back:
                success as truth
            do:
                set email to new email
                give back yes
        end action
end container
```

Now, let's unpack that:
- We declared some **private** data: `id` and `email`. This means outside code can't directly access `User.id` or `User.email`. They are internal details. Perhaps `id` is used by the system for identification and `email` we want to guard behind an update process (for validation).
- Under **public**, we defined:
  - **`when created`**: This is the constructor, the code that runs when we create a new User. We specified that creating a User needs an `email` (as text). In the `do:` part, we set up the initial state:
    - `set id to generate unique id` suggests WFL has some way to generate IDs (or we assume it's a built-in or previously defined action).
    - `set email to email` stores the provided email in the field. (Yes, the word `email` is used for both the parameter name and the field; they are in different scopes, WFL likely figures it out. We could have named the constructor parameter differently for clarity, but let's assume this works or WFL allows `email` to refer to the field and maybe `email` from needs is clear by context).
  - **`define action get email`**: A public action that simply returns the email. We use `give back email` which returns the current email. This is a simple "getter".
  - **`define action update email`**: A public action to change the email. It needs a `new email` as text, and it returns a success as truth (yes/no). In the body, it sets the internal email to the new email and returns yes (indicating success). In a real scenario, you might add validation here to only update if `new email` is valid, otherwise perhaps return no or raise an error. But we keep it simple.

This container `User` defines what a User is (an id and email) and what it can do (get email, update email). We could also add `name`, `role`, etc., but let's keep it minimal for now.

### Creating an Instance of a Container

Now that we have a `User` container blueprint, how do we create an actual user? We use the `new` keyword much like we did for creating records or so, but referencing the container:

```wfl
create user1 as new User with email as "alice@example.com"
```

This line would:
- Create a new instance of `User` (call the constructor `when created` with the given email).
- Assign it to a variable `user1`.

After this, we have a `user1` that is a User object. We can call its actions:

```wfl
store currentEmail as perform user1 get email
display currentEmail               // displays "alice@example.com"

store result as perform user1 update email with new email as "alice@newdomain.com"
display result                     // displays yes (true) if success
display perform user1 get email    // should display "alice@newdomain.com"
```

Let’s clarify the syntax of calling an action on an object. We wrote `perform user1 get email`. This likely is how WFL lets you call a container’s action on a specific instance. Another way it might be seen is `perform get email on user1`. The spec example had `parent perform get profile` for calling a parent class action, and `perform cache get item with id` for calling an action of a contained object. It seems the pattern is:
- `perform [object] [action name] ...`

So `perform user1 update email with new email as "alice@newdomain.com"` means "on the object user1, perform the update email action providing new email ...".

This reads quite nicely as is, but you could also read it like `user1.updateEmail("alice@...")` in other languages.

Because WFL doesn't use dot notation, it uses this more English phrasing. Another hypothetical way could be:
```wfl
perform update email on user1 with new email as "alice@newdomain.com"
```
The spec didn't show the exact phrasing with "on", but they did use `perform [object] [action]`. We'll stick to that form.

So now we can interact with our user object through its public actions. The private fields `id` and `email` cannot be accessed directly (e.g., we can't just do `display user1 email` because email is private). This encapsulation is intentional: we provide controlled access via the `get email` and `update email` actions.

### Adding More to the Container (Optional Sections)

We kept our `User` simple. We could enhance it:
- Maybe add a name and a role field (perhaps role could be protected or public).
- Possibly add a `validate email` action internally to check if an email is valid, and call that in `when created` or `update email`.
- But those additions would just be more of the same patterns: storing data and using check conditions, which we covered before.

The main takeaway: a container’s syntax is block-structured like everything else in WFL, divided into sections by access level, and uses `when created` for the constructor.

## Inheritance: Extending a Container

**Inheritance** allows one container to build upon another, inheriting its fields and actions, and then adding or overriding as needed. For instance, perhaps we have Admin users that are like Users but with extra privileges.

In WFL, you can create a container **from** another container:

```wfl
create container Admin from User:
    // Admin is a specialized User
end container
```

By doing `from User`, Admin automatically has everything User had (id, email, get email, update email) unless we override something. Now let's add specifics:

```wfl
create container Admin from User:
    // Additional private data for Admin
    private:
        store admin level as number

    // Public constructor
    public when created:
        needs:
            email as text
            level as number
        do:
            parent when created with:
                email as email
            end with
            set admin level to level
        end when

    // Public action to check admin level
    public define action is super admin:
        gives back:
            result as truth
        do:
            give back admin level is above 5
        end action

    // Override an existing action (if needed, for example, update email differently)
    // (We won't override update email here, but we could if we wanted to add logging or restrictions.)
end container
```

Explanations:
- We added a private field `admin level` to Admin.
- The `when created` for Admin needs an email and a level. Inside, we see something new: **`parent when created with: email as email end with`**. This calls the parent container’s constructor (`User`'s when created) to handle setting up the inherited part (like generating id and setting email). We pass `email` along to it. After that, we do Admin-specific initialization: `set admin level to level` and maybe also set a role to "admin" if we had a role field in User.
- We define a new action `is super admin` that returns yes if `admin level` is above 5 (just an arbitrary criterion for demonstration). Notice we could directly access `admin level` since it's within the Admin container. We can also access parent's private fields? Actually, parent's private fields are not accessible in child if truly private, but `email` in User was private. However, since we called `parent when created`, the parent took care of setting email. If Admin needs to access email, it can't if email is private to User. Perhaps we should have made email protected or provided getters (we did get email). So Admin can call `perform get email` if needed to get its own email, which is a bit roundabout but possible. Alternatively, if we foresaw inheritance, we might have put email under protected instead of private in User, so Admin can see it. For this example, it's fine.
- We chose not to override `update email` or `get email`, but we could. Overriding in WFL is done by simply defining an action with the same name in the child container. We would mark it `public define action update email:` in Admin to override the User's version (maybe to add an audit log or additional checks), and if needed, call the parent one via `parent perform update email with ...`. Our Admin example didn't need to override it though.

So now Admin has everything User has, plus admin level and is super admin action.

Creating an Admin works similarly:
```wfl
create admin1 as new Admin with:
    email as "boss@example.com"
    level as 10
end with
```

This will:
- Call Admin’s `when created`, which calls User’s `when created` for email and then sets level to 10.
- Now `admin1` is an instance of Admin. We can do all User actions on it (get email, update email) and Admin actions:
```wfl
display perform admin1 get email          // "boss@example.com"
display perform admin1 is super admin    // yes (true) because level 10 is above 5
```

If we had overridden something, Admin’s version would run instead of User’s.

Inheritance allows code reuse and specialization. But not everything should be inheritance – sometimes composition is better. That leads us to interfaces and composition.

## Interfaces: Defining a Contract

An **interface** in programming is like a contract: it defines a set of actions that a container must have, without providing the implementation. This is useful when you want to specify that different containers, possibly unrelated by inheritance, should offer certain behaviors.

For instance, you might have different types of data storage (file system, database, etc.), and you want all of them to have actions like `save item`, `load item`, `delete item`. You can define an interface for a DataStore, then implement it in different containers.

In WFL:

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

    // You can even provide a default implementation for an action (optional in interface)
    action item exists:
        needs:
            id as text
        gives back:
            exists as truth
        do:
            try:
                perform load item with id
                give back yes
            catch not found:
                give back no
            end try
        end action
end interface
```

This interface `Data Store` declares three required actions and one optional (with default implementation). It doesn't say *how* to do them, just what they should look like (parameters and return types, and in one case, a generic way to implement `item exists` by trying a load).

Now, to use this, a container can **implement** the interface:

```wfl
create container File Store implements Data Store:
    private:
        store base path as text

    public when created:
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

    define action load item:
        needs:
            id as text
        gives back:
            item as any
        do:
            try:
                give back read file at join base path and id
            catch not found:
                raise error "Item not found"
            end try
        end action

    define action delete item:
        needs:
            id as text
        gives back:
            success as truth
        do:
            try:
                delete file at join base path and id
                give back yes
            catch any error:
                give back no
            end try
        end action

    // We didn't define item exists, so it will use the default from interface if called.
end container
```

This `File Store` container claims to implement `Data Store`. WFL will enforce that it provides all actions that interface requires:
- We provided `save item`, `load item`, `delete item`. We did not explicitly provide `item exists`, but since the interface had a default, `File Store` automatically gets that default implementation (which calls `load item` and sees if it errors).
- Each action uses file operations (conceptually). The details aren't important beyond showing try/catch usage and such.

We could make another container, say `Database Store`, also implement `Data Store` but with different code (perhaps interacting with a database). Any code that needs a `Data Store` could then work with either a `File Store` or `Database Store` interchangeably because they have the same interface.

Using an interface-based container:
```wfl
create store1 as new File Store with path as "/data/items/" end with
perform store1 save item with item as someData
```
If we had a function that took a `Data Store` as input, we could pass `store1` or any other that implements it.

Interfaces thus ensure a consistent API without forcing a particular inheritance structure. WFL's syntax for them is natural (defining actions without full implementation, except optional defaults).

## Composition: Containers Using Other Containers

**Composition** means a container can hold instances of other containers to build up functionality. Instead of using inheritance to extend, you use references to other objects to get their functionality.

For example, let's say we have our `User` container and we want a `UserManager` container that internally uses a `Data Store` to save user data and maybe a cache to speed things up. We can compose it:

```wfl
create container User Manager:
    private:
        store user store as Data Store
        store cache as Cache Of User

    public:
        when created:
            needs:
                store as Data Store
            do:
                set user store to store
                set cache to new Cache Of User with:
                    timeout seconds as 600
                end with
        end when

        define async action get user:
            needs:
                id as text
            gives back:
                user as User
            do:
                // Try cache first
                store cached as perform cache get item with id
                check if cached is not nothing:
                    give back cached
                end check

                // Not in cache, load from store
                store user as await perform user store load item with id

                // Save to cache for next time
                perform cache set item with:
                    key as id
                    value as user
                end with

                give back user
        end action
end container
```

Breaking it down:
- `User Manager` has two private members: `user store` which is a `Data Store` (meaning we can set it to any object that implements `Data Store` interface, like a File Store or Database Store), and `cache` which is a `Cache Of User` (implying there's a generic `Cache` container defined elsewhere that can hold User objects, with a timeout).
- In the constructor (`when created`), it needs a `store` as Data Store. We pass in an actual storage implementation when we create a User Manager. It sets its `user store` to that provided store. It also creates a new `Cache Of User` with a 600 second timeout.
- The action `get user` is asynchronous (maybe loading from a database or file could be async). It takes an `id` and returns a `User`.
- In `get user`:
  - It first checks the cache: `perform cache get item with id`. If something is returned (not nothing), it immediately returns that (`give back cached`).
  - If cache missed, it uses `await perform user store load item with id` to load the user from the underlying store (database or file). We assume `user store load item` returns a `User` object (since our Data Store interface said `gives back item as any`, in practice it would be returning a `User` record or container; we might refine the typing, but let's assume it's a User).
  - Then it updates the cache with `perform cache set item` with the user, so next time it will be fast.
  - Finally, returns the `user`.

This example demonstrates:
- **Composition**: User Manager has a Data Store and a Cache inside it.
- **Using an interface**: It doesn't care if `user store` is a File Store or any other, as long as it implements Data Store.
- **Using a generic container**: `Cache Of User` presumably is defined to store any type T, here T is User.
- **Async action and awaiting**: to fetch data possibly from an external source.

It shows how WFL can express higher-level logic in a very clear manner:
“Get user: if cached, return it; otherwise load from store, update cache, then return.”

From a beginner perspective, composition just means “one object has another object inside it as a part”. It’s like a Car object has an Engine object inside – rather than Car *is a* Engine.

## Conclusion

Containers in WFL pack a lot of power but remain aligned with the language’s core principles of clarity and simplicity:
- We create containers with a natural block syntax, specifying fields and actions inside.
- The use of `when created` (constructor) and `define action` inside containers is analogous to how we've been writing code outside, just grouped logically.
- Access levels (private, protected, public) are spelled out, which is great for understanding who can use what.
- Inheritance with `from` and interface implementation with `implements` are done in a readable way, and even advanced OOP features like overriding and calling parent actions use words like `parent` and the same `perform` syntax.
- Composition is straightforward – just include other container instances as fields and use them.

For someone new to coding, the idea of containers/classes can be a leap, but WFL’s syntax tries to make it as approachable as possible by avoiding cryptic notations. You describe the class almost like writing a document of what an entity has and does.

**Key takeaways:**
- Use containers to model complex data with behaviors.
- Use inheritance if you need an "is a" relationship to specialize containers.
- Use interfaces to define common behaviors across different containers without tying them in an inheritance hierarchy.
- Use composition to build bigger systems out of simpler parts, plugging in for example a storage mechanism into a manager.

Now you have walked through all major aspects of WFL from basic variables to advanced containers. Each piece builds on the previous ones:
- Variables gave you the basics of storing data.
- Loops and conditionals gave you control flow to make decisions and repeat tasks.
- Functions (actions) let you encapsulate reusable logic.
- Containers allowed grouping data with related actions, enabling higher-level program design.

With these, you can start writing WFL programs that are both easy to understand and powerful. The syntax will often feel like writing pseudo-code or even plain English descriptions, which is exactly what WFL strives for. Happy coding, and welcome to the world of WFL – where coding feels like natural language!