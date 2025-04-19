Great — I’ll create a complete implementation plan for Milestone 10 focused on enhancing WFL’s error reporting and diagnostics system. This will include:

- Integrating the `codespan-reporting` crate into the compiler.
- Refactoring error types to carry detailed, structured diagnostic data.
- Mapping `Span` values to the source file and generating rich error messages.
- Crafting clear, actionable error phrasing inspired by Elm and beginner-friendly language.
- Example implementations of helper functions and sample output.

I'll also include practical suggestions for improving error feedback in the REPL and editor context.

I'll get started and update you when the full plan is ready.

# Milestone 10 Implementation Plan: WFL Diagnostics System

**Overview:**  
Milestone 10 focuses on overhauling the WebFirst Language (WFL) compiler/interpreter to provide a clear, user-friendly diagnostics system. We will integrate the `codespan-reporting` crate to produce rich error messages with annotated source code snippets, and refactor our error handling to carry detailed diagnostic data. The goal is to make WFL’s error messages as informative and approachable as those of modern languages like Rust and Elm, guiding the user to resolve issues with friendly, actionable feedback ([GitHub - brendanzab/codespan: Beautiful diagnostic reporting for text-based programming languages.](https://github.com/brendanzab/codespan#:~:text=Languages%20like%20Rust%20and%20Elm,and%20relatively%20painless%20for%20everyone)) ([Jamalambda's Blog - Elm - Amazing, Informative, Paternalistic Error Messages](https://jamalambda.com/posts/2021-06-13-elm-errors.html#:~:text=At%20the%20forefront%20of%20this,Of)). This document outlines the implementation plan in detail, including data structure changes, error message style guidelines, example improvements, testing strategy, and notes on REPL/editor integration.

## 1. Integrating `codespan-reporting` for Rich Diagnostics

We will use the Rust crate **`codespan-reporting`** (latest version, e.g. 0.12) to render compiler errors with source code highlights. This crate provides utilities to create formatted diagnostics (with error codes, labels pointing to spans in the source, and notes) and to emit them in the console with colors and ASCII pointers. By leveraging this, we can avoid writing a custom error-rendering engine from scratch – as the project README notes, *“the `codespan-reporting` crate aims to make beautiful error diagnostics easy and relatively painless for everyone!”* ([GitHub - brendanzab/codespan: Beautiful diagnostic reporting for text-based programming languages.](https://github.com/brendanzab/codespan#:~:text=Languages%20like%20Rust%20and%20Elm,and%20relatively%20painless%20for%20everyone)).

**Steps to integrate:**

- **Add Dependency:** Include `codespan-reporting` in `Cargo.toml`. For example:  
  ```toml
  codespan-reporting = "0.12"
  ```  
  This crate internally uses `termcolor` for colored output, which will allow our errors to be color-highlighted in terminals.

- **Import in Code:** In our compiler code, import the necessary types:  
  ```rust
  use codespan_reporting::diagnostic::{Diagnostic, Label};
  use codespan_reporting::files::{Files, SimpleFiles};
  use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
  use codespan_reporting::term;
  ```  
  We use `Diagnostic` and `Label` to construct error descriptions, `SimpleFiles` to manage source files and their contents, and the `term` module to print to the terminal with color support.

- **File Database:** Instantiate a `SimpleFiles<String, String>` to store source files. This structure assigns each added file a unique `FileId` (an index). For a given input source (like a file or REPL input), add it to `SimpleFiles` and get a `file_id`. For example:  
  ```rust
  let mut files = SimpleFiles::new();
  let file_id = files.add("main.wfl", source_code);
  ```  
  This `files` object will be used when rendering diagnostics so the crate can fetch the file content for displaying code frames.

- **Diagnostic Emission:** Use `codespan-reporting` to print errors. We will create a helper function (e.g. `report_error`) that takes an error and prints it nicely (details in Section 4). Under the hood, it will call `term::emit` to render to stderr. For example:  
  ```rust
  let writer = StandardStream::stderr(ColorChoice::Auto);
  let config = codespan_reporting::term::Config::default();
  term::emit(&mut writer.lock(), &config, &files, &diagnostic)?;
  ```  
  This will output the diagnostic with colors (we choose `Auto` to detect if the output is a tty). The default config is fine, or we can tweak it (for example, to adjust styles or tab widths).

By integrating `codespan-reporting`, WFL errors will be displayed with the offending code and caret markers, much like Rust’s `rustc` or Elm’s compiler outputs. This addresses the first goal: syntax, type, and runtime errors will be shown with **annotated code frames**, making it easy to locate and understand the problem in context.

## 2. Implementing a `Span` Type for Source Locations

A consistent **`Span`** structure will be introduced to represent positions in source code. All error types and AST nodes will use this to pinpoint where an error occurs. A span will ideally include a file identifier and byte offsets within that file:

- **Definition:** We will define `Span` as a struct containing a file ID and a start and end position. For example:  
  ```rust
  pub struct Span {
      pub file: usize,       // corresponds to codespan FileId
      pub start: usize,      // start byte offset in the source
      pub end: usize,        // end byte offset (exclusive)
  }
  ```  
  This could also be an alias for `codespan_reporting::files::FileId` and a range, but using our own struct decouples WFL from the specific crate if needed. The `file` field is an index (e.g. into the `SimpleFiles` we manage). The `start` and `end` are byte indices in the source string. We choose byte offsets because they are easy to compute during lexing and work directly with `codespan-reporting` APIs (which expect byte indices for labels).

- **Line/Column vs. Offsets:** We prefer storing byte offsets in `Span` rather than line/column, to avoid repeated conversion. The `codespan-reporting` library will handle converting these offsets to line/column for display. Internally, we may still track line and column for user messages (for example, in runtime errors or logs), but the primary representation will be offsets for consistency. If needed, we can provide helper methods on `Span` to get the line/column by querying the `files` database (e.g., `files.location(file, span.start)` can return a `Location { line, column }` if we implement or use `Files` trait methods).

- **Multiple Files:** The `file` field in `Span` allows supporting multiple source files. In a multi-file project, each file loaded into the compiler gets a unique ID via `SimpleFiles`. The span’s `file` distinguishes which source the error pertains to. For single-file usage (common for now), we’ll still use file ID 0 for the main source, but the system is extensible.

**Example:** If we have `files.add("main.wfl", source) -> file_id = 0`, and an error spans bytes 10 to 15 in that source, we represent it as `Span { file: 0, start: 10, end: 15 }`. Later, when rendering the diagnostic, we pass this to `Label::primary(span.file, span.start..span.end)`.

## 3. Propagating Span Information Through the Compiler

To make use of the new `Span`, all phases of the compiler/interpreter must produce and carry span data:

- **Lexer Updates:** Ensure the lexer associates each token with a span. If our lexer is hand-written, we will update it to track the current position (byte offset) as it reads characters. Each token produced (identifiers, keywords, literals, symbols) will get a `Span` covering its position in the source. For example, a token struct might become:  
  ```rust
  struct Token {
      kind: TokenKind,
      span: Span,
      // ... maybe lexeme string etc.
  }
  ```  
  If the lexer currently only tracks line/column, we will add a running byte index. Each time a token is emitted, calculate its start offset (e.g., at token start) and end offset (after consuming it). For multi-character tokens, the span covers the whole token text. This gives precise spans for syntax errors (like an unexpected token).

- **Parser Updates:** The parser (whether recursive descent, Pratt, or a parser generator) will use the tokens' spans to build spans for larger syntactic constructs:
  - For each grammar rule or AST node, compute the node’s span as covering from the start of the first token in the construct to the end of the last token. For instance, if parsing an `if` expression from token at offset 5 to token at offset 30, the resulting AST node for the `if` would have `Span { start:5, end:30 }`. 
  - If the parser encounters an error (e.g., unexpected token or missing token), it should produce a `ParseError` with an appropriate span:
    - *Unexpected token:* span = the token’s span that was not expected.
    - *Unexpected end-of-file:* span = a zero-length span at the end of the file (both start and end at the file length) to indicate the location of EOF; we can still point to the end of the file in the diagnostic.
    - *Other syntax error (like mismatched parentheses):* span = the location of the token that caused the issue, or the span of the construct up to where it noticed something’s wrong.
  - The parser can also carry context: e.g., if an `endif` is missing for an `if`, we might have the span of the `if` as context. (In practice, we might emit a secondary label in such cases — see Section 4.)

- **AST Nodes:** Refactor AST node definitions to include a `Span`. For example, if we have structures like `struct Expr { ... }`, we can add a `span: Span` field. If the AST is represented with enums for expressions and statements, we can include span in each variant or wrap nodes in a struct that has the span:
  ```rust
  enum ExprKind {
      Literal(LiteralValue),
      BinaryOp { left: Box<Expr>, op: Op, right: Box<Expr> },
      // ...
  }
  struct Expr {
      kind: ExprKind,
      span: Span,
  }
  ``` 
  This way, during parsing, when we construct an `Expr`, we set its span from the tokens as described. All semantic and type analysis will then have access to these spans via the AST.

- **Semantic Analyzer:** In semantic analysis (e.g., resolving identifiers, checking for undefined variables, scope rules), when an error is detected, retrieve the span from the relevant AST node or token:
  - If an undefined variable `x` is encountered, the AST node for that variable reference has a span; use that for the error.
  - If a function is redeclared, you might have spans for both the previous definition and the new one. You could attach both: primary span at the second occurrence, secondary span at the original definition (to say "first defined here").
  - Ensure any new errors introduced in this phase (`SemanticError`) carry the span of the code entity in question (identifier, expression, etc.). 

- **Type Checker:** For type errors, spans are crucial:
  - When a type mismatch happens, we often have two locations: one where an expression of a certain type is, and another where an incompatible type was expected. For example, if a return type is wrong, you have the function's return type declaration vs. the return expression. In these cases, gather both spans:
    - Primary span at the site of the problematic expression (e.g., the return expression).
    - Secondary span at the type expectation (e.g., the function signature, or the other operand in a binary operation).
  - Our `TypeError` structure can accommodate multiple spans (see Section 4 on labels). The type checker should include these spans when constructing the error object. If currently the type checker just returns an error message, change it to return a `TypeError` containing the span(s) and info about expected vs found types.
  - Example: If the code `x + true` is invalid (assuming `x` is a number and `true` is Boolean), the type checker might produce a `TypeError` with primary span at `true` and a message like "expected a Number, but found a Boolean", and maybe a secondary span under `x + ...` or just highlighting `x`'s type context. (We will refine message wording in Section 6.)

- **Interpreter / Runtime:** Even at runtime, we want errors (exceptions, panics) to be tied to source spans:
  - If the interpreter detects a runtime error (division by zero, index out of bounds, null reference, etc.), it should have a way to know which part of the source is executing. To achieve this, ensure that during AST interpretation, you carry the current node’s span:
    - If you have a function like `eval_expr(expr: &Expr)`, you know `expr.span`. If a runtime error occurs (say division by zero in a binary operation), you can report `expr.span` for that operation or specifically the span of the division operator/operands. Perhaps the AST for a binary operation can carry the operator’s span or you can use the whole expression’s span.
    - In a function call, if an error arises inside the function, a full stack trace might be ideal, but as a starting point we can at least show the span in the user code that caused the runtime error (e.g., the call site or the operation itself).
  - Wrap runtime errors in our `RuntimeError` type (see next section) with the span of the offending operation. For example, if an array indexing is out of range, the span would cover the index expression or the whole `arr[index]` expression in the source.
  - **Important:** This may require passing the span through the interpreter: for instance, when evaluating a binary op, check for a runtime issue and then produce an error with that op’s span. If using an AST, you already have that span; if using bytecode, you might need to map bytecode back to source spans (likely too complex for now, so we assume an AST interpreter).

By propagating spans everywhere, any error (syntax, semantic, type, or runtime) will have precise location info. This ensures we can present a highlighted snippet of code to the user pinpointing the error location and any related context.

## 4. Refactoring Error Types to Carry Diagnostic Data

We will refactor the existing error structures – `ParseError`, `SemanticError`, `TypeError`, `RuntimeError` – so that they carry rich diagnostic information: the `Span`, an error code, a human-friendly message, and optional additional labels or suggestions. This uniform approach turns simple error messages into structured data that we can feed into the diagnostic reporter.

**Changes for each error type:**

- **ParseError:** Currently likely an enum for different parsing issues. We will ensure every `ParseError` value has:
  - `span: Span` – location of the syntax issue.
  - `code: &'static str` – a unique error code (for example, `"E1001"` for an unexpected token, etc.).
  - `message: String` – a concise description of the error (to be used as the main diagnostic message or label text). e.g., `"Unexpected token 'ELSE'"` or `"Missing closing parenthesis"`.
  - Optionally, for certain variants, additional context:
    - e.g., `expected: TokenKind` or a description of what was expected, to include in the message or a secondary label.
    - We might include a `label: String` field for an additional label message if needed (though often parse errors only highlight one spot).
  - In Rust terms, the enum variant could hold these fields. For example:  
    ```rust
    enum ParseError {
        UnexpectedToken { span: Span, found: TokenKind, expected: String, code: &'static str, message: String },
        UnexpectedEOF { span: Span, expected: String, code: &'static str, message: String },
        // ...other variants...
    }
    ```  
    However, if this becomes cumbersome, an alternative is to make `ParseError` a struct with an inner enum for kind. But the above is fine if kept manageable.

- **SemanticError:** These are errors like undefined variables, duplicate definitions, etc. Refactor to include:
  - `span: Span` – typically the span of the identifier or construct that is problematic.
  - `code: &'static str` – e.g., `"E2001"` for undefined name, `"E2002"` for duplicate definition, etc.
  - `message: String` – e.g., `"Use of undefined variable 'x'"` or `"Function 'foo' is defined multiple times"`.
  - *Additional data:* If helpful, include:
    - For undefined variable, perhaps the name used (though that’s also evident in the source span).
    - For duplicate definition, the span of the original definition (for a secondary label). We can store that as a second `Span` inside the error or in a list of labels.
    - For other semantic checks, any relevant info (e.g., inaccessibility or misuse errors) with their spans.

- **TypeError:** Type errors often involve two or more pieces of information (expected vs actual types):
  - `span: Span` – location of the expression or item with the wrong type (primary span).
  - `code: &'static str` – e.g., `"E3001"` for type mismatch, `"E3002"` for calling a non-function, etc.
  - `message: String` – a high-level description, e.g., `"Type mismatch between branches of if-expression"` or `"Function call type error"`.
  - *Additional fields:* We likely need to capture:
    - `expected_type: Type` and `found_type: Type` (or their string representations) for crafting the message or suggestion.
    - `expected_span: Span` (if the expected type comes from some location, like a variable declaration or a function signature).
    - Possibly `label: String` or separate messages for primary vs secondary labels.
    - If multiple related spans (like two branches of an `if` that conflict), we might have a list of spans with notes. This can be handled by storing a vector of `(Span, String, LabelStyle)` or by constructing the labels on the fly when reporting (see next section).
  - If storing all that in the error struct feels heavy, we can opt to store just the primary span, and any secondary info in a vector of “related” spans with messages. For simplicity, we might add a field like `related: Vec<(Span, String)>` to `TypeError` to hold any secondary highlights (e.g., the other branch or the type definition site).

- **RuntimeError:** These errors occur during interpretation:
  - `span: Span` – point in source where the runtime error occurred.
  - `code: &'static str` – e.g., `"E4001"` for division by zero, `"E4002"` for index out of bounds, etc.
  - `message: String` – description, e.g., `"Division by zero"` or `"Index 5 is out of bounds for array of length 3"`.
  - Optionally, a field for a suggested fix if obvious (e.g., "check divisor for zero before dividing").
  - If a runtime error might have a cause (like an inner exception), we can hold that, but in our simple language it's likely just direct issues.
  - Note: Many runtime errors will only have a single span to highlight (the failing operation). However, if we had a call stack, we could consider multiple labels (one for the call site, one for inside the function). Implementing full stack trace diagnostics might be beyond this milestone; we can leave it as a future enhancement. For now, focusing on highlighting the immediate source of the error is sufficient.

**Error Code Scheme:** Assign a unique error code string to each distinct error condition. These codes will be included in the diagnostics (and can later be used for documentation or categorization). We can choose a scheme such as:
- **Syntax errors:** `E1xxx` (e.g., `E1001`, `E1002`, ...).
- **Semantic errors:** `E2xxx`.
- **Type errors:** `E3xxx`.
- **Runtime errors:** `E4xxx`.
The exact numbering can follow the order or importance of error kinds. For example:
  - `E1001`: Unexpected token / syntax error
  - `E1002`: Unclosed delimiter
  - `E2001`: Undefined variable
  - `E2002`: Duplicate definition
  - `E3001`: Type mismatch
  - `E3002`: Function not returning expected type
  - `E4001`: Division by zero
  - `E4002`: Index out of bounds  
These will be stored as static string codes in the error objects. We might define a const for each or just hardcode the string when constructing the error. The error types could also have an associated function that returns its code. As long as each code is unique and stable, it's fine.

By refactoring errors this way, **all necessary info (where, what, and context) travels with the error**. Instead of just an error message, we have a structured error object. This makes it straightforward to generate a `Diagnostic` in the next step.

## 5. Converting Errors to Diagnostics (`report_error` function)

We will implement a helper function (e.g. `report_error`) that takes one of our error types and renders it using `codespan-reporting`. The function will perform the mapping from our `ParseError/TypeError/...` to a `Diagnostic<usize>` (assuming we use `usize` or an alias for file IDs).

**Design of `report_error`:**

- **Function signature:** It could be:  
  ```rust
  fn report_error<E: ToDiagnostic>(error: E, files: &impl Files<Name=String, Source=String>);
  ```  
  Here `ToDiagnostic` is a trait we can define on our error types to convert into a `Diagnostic<usize>`. Alternatively, `report_error` could accept an enum that encompasses all error variants (like a unified `WflError` enum). Using a trait is more flexible and keeps concerns separated:
  ```rust
  trait ToDiagnostic {
      fn to_diagnostic(&self) -> Diagnostic<usize>;
  }
  ``` 
  We would implement this trait for `ParseError`, `TypeError`, etc. Then `report_error` can be a generic that calls `error.to_diagnostic()`. It will also handle printing (emitting) the diagnostic.

- **Mapping to Diagnostic:** Each error type’s `to_diagnostic` (or a match in `report_error` if not using trait) will create a `codespan_reporting::diagnostic::Diagnostic` with appropriate labels and notes. For example:
  - **Basic structure:**  
    ```rust
    Diagnostic::error()
        .with_message(error.message)
        .with_code(error.code)
        .with_labels(vec![ ... ])
        .with_notes(vec![ ... ]);
    ```  
    We use `Diagnostic::error()` since these are errors (the crate also supports warnings and notes). We attach the human-readable message and the error code string. Then we build one or more `Label`s for the source snippets.

  - **Primary vs Secondary Labels:** 
    - The **primary label** (`Label::primary`) will mark the main span where the error is. Typically, we use the `error.span` for this, highlighting the exact code fragment that is wrong. We also can attach a specific label message to it, which should describe the issue at that location. Often this can be similar to the main message but focused. For instance, in a type mismatch, the primary label might say *"found type `Boolean`"* under the expression, while the main message says *"Type mismatch between expected Int and found Boolean"*.
    - **Secondary labels** (`Label::secondary`) mark related spans. Use these for:
      - Context, such as "type expected here" or "previous declaration here".
      - Additional pieces of the error, like the span of an expected token in a parse error (pointing maybe at the location where something should have appeared, e.g., end of file for a missing brace).
      - We add as many secondary labels as needed, each with a message. For example, for an undefined variable, we might not have a secondary span (unless perhaps we point to the nearest in-scope similar name as a suggestion, though that's advanced). For a duplicate definition, secondary label would point to the first definition's span with message "first definition of `name` here".
      - The `Diagnostic` is created with a vector of labels, so we gather all relevant spans.

    - **No overlapping spans:** Ensure that the primary label is truly the main error location; other related info should be secondary. The codespan library will visually distinguish primary vs secondary (primary often in red caret, secondary in lighter underline). We should choose one primary even if multiple spans are involved.

  - **Notes / Suggestions:** We can add free-form text notes using `.with_notes(Vec<String>)`. This is a good place to put **helpful suggestions or hints**. Following Elm’s philosophy, we want to give the user guidance here. For example:
    - For a type mismatch, a note could be: *"Try converting the value to the expected type, or check if you used the correct variable."* 
    - For an undefined variable: *"If you meant to use a variable defined elsewhere, make sure it's in scope or imported. If it's a typo, fix the name."*
    - For a missing brace: *"Add a `}` at the end of the block to match the opening `{`."*
    - We can prefix these with "help:" in the text to emulate Rust’s style, or simply write them as sentences. `codespan-reporting` will render notes below the code frame (usually with a `= note:` or similar prefix by default).
    - **Example note usage:**  
      ```rust
      .with_notes(vec![
          "help: Ensure the types of both branches match.",
          "Hint: The branches of an `if` must return the same type."
      ])
      ```  
      Each string in the vector becomes a separate note in the output. We should keep them short and positive in tone.

- **Using `files` for rendering:** The `report_error` function will take a reference to the `files` (our `SimpleFiles` instance) that contains the source code. When calling `term::emit`, we pass `&files` so the library can fetch file contents for the given file IDs in the labels. We locked the writer and used the default config as shown earlier. We likely want `ColorChoice::Auto` for interactive use. In non-interactive contexts (like piping output or tests), it will automatically disable color.

- **Putting it together (example):** Suppose we have a `TypeError` for a mismatched return type in a function. We might implement `to_diagnostic` for it as:  
  ```rust
  impl ToDiagnostic for TypeError {
      fn to_diagnostic(&self) -> Diagnostic<usize> {
          match self {
              TypeError::Mismatch { span, expected, found, expected_span } => {
                  Diagnostic::error()
                      .with_message(format!("Type mismatch: expected `{}`, found `{}`", expected, found))
                      .with_code("E3001")
                      .with_labels(vec![
                          Label::primary(span.file, span.start..span.end)
                              .with_message(format!("found `{}` here", found)),
                          Label::secondary(expected_span.file, expected_span.start..expected_span.end)
                              .with_message(format!("expected `{}` here", expected)),
                      ])
                      .with_notes(vec![
                          "help: Make sure both expressions produce the same type.",
                          format!("hint: The value is expected to be a `{}` because of the function's return type.", expected),
                      ])
              }
              // other TypeError variants...
          }
      }
  }
  ```  
  In this snippet:
    - We create an error Diagnostic with a clear message including expected and found types.
    - Attach a primary label under the code that has the wrong type (`found` type) and a secondary label where the expected type comes from (perhaps a function signature or earlier annotation).
    - Provide a couple of notes: one general and one possibly specific hint. The wording is encouraging and avoids jargon.
  This is just an illustrative example; actual implementation will vary for different error kinds.

- **Printing the Diagnostic:** Finally, `report_error(error, files)` will obtain the `Diagnostic` via `error.to_diagnostic()`, then call `term::emit` to print it. Pseudocode:  
  ```rust
  pub fn report_error<E: ToDiagnostic>(error: E, files: &SimpleFiles<String, String>) {
      let diagnostic = error.to_diagnostic();
      let writer = StandardStream::stderr(ColorChoice::Auto);
      let config = codespan_reporting::term::Config::default();
      // Print to stderr with proper locking
      term::emit(&mut writer.lock(), &config, files, &diagnostic)
          .expect("Failed to write diagnostic");
  }
  ```  
  After this function, the user will see the nicely formatted error in the console. We should also make sure to **return a non-zero exit code** from the compiler if an error was reported (to indicate failure).

With this in place, any part of the compiler that detects an error will construct the appropriate error object (with span, code, message, etc.), then eventually pass it to `report_error` for display. This decouples error detection from error rendering, making the system cleaner and easier to maintain.

## 6. User-Friendly Error Message Guidelines (Elm-Inspired)

Simply displaying errors is not enough – the wording of messages must be **clear, beginner-friendly, and actionable**. We will revise all error message text to follow a style inspired by Elm’s compiler (friendly and helpful) while still being concise. The key guidelines:

- **Avoid Jargon:** Use simple language instead of compiler jargon. For example, instead of *"Unexpected token in input"* (jargon: "token"), say *"I got something here that I didn’t expect"* or *"I wasn’t expecting to see `<token>` at this point."* If a term like "token" or "expression" is too technical for a beginner, find another way (perhaps "thing" is too vague, but "value" or "symbol" might be friendlier depending on context).

- **Speak to the User:** It can help to phrase messages as if the compiler is talking to the programmer. Elm often uses a friendly tone, e.g., "I cannot find a variable named `x`" or "Looks like the types don’t match." This makes the interaction feel less like a cryptic error and more like advice. We should adopt a similar tone:
  - Use first-person ("I") for the compiler and second-person ("you") to refer to the code writer when appropriate. *Example:* "I expected a number but your code gave me a string." This frames the error as the compiler’s confusion rather than the programmer’s "fault".
  - Example revision: A current message *"Type error: int vs string"* could become *"I was expecting a number (Int) here, but it looks like you gave me a string."* This explicitly states what was expected and what was found in a conversational way.

- **State the Problem Clearly:** The first line of the error (after any prefix or code) should state what went wrong in plain terms. E.g., *"Cannot add a string and an integer"* or *"Unknown variable `x`"* or *"Missing `}` at end of block"*. This should be understandable without requiring deep CS terminology.

- **Provide Context in Labels:** The messages attached to the source snippet labels should add clarity:
  - Underline exactly the part of code involved and write a small note. For example, under a mismatched type expression: *"this is of type `String`"*, and maybe under the expected context: *"expected type `Int` here"*.
  - For a missing symbol (like `}`), point at the location where it should be and say *"expected `}` to close the `{` opened here"* (with a secondary label at the opening `{` if possible).
  - For an undefined name, point at the name and say *"`x` is not defined in this scope"*.

- **Give Suggestions or Hints:** This is crucial for a beginner-friendly experience. After explaining the error, guide the user toward a solution:
  - Preface suggestions with a keyword like "help:" or "hint:" to distinguish them as not part of the error proper but advice.
  - For parse errors: *"help: Did you forget a `)` somewhere before this?"* or *"help: Try adding a `}` to match the opening `{`."*
  - For type errors: *"help: If you want to concatenate strings, use the `++` operator instead of `+`."* or *"help: Convert the number to a string by using `toString(number)` if available."* (The exact suggestion depends on language features.)
  - For undefined variables: *"help: Did you mean `xyz` (a similar name in scope)? Or make sure you declared `x` before using it."*
  - These suggestions should be gentle and not assume too much. They can also educate (Elm’s errors often explain a bit of language concept as part of the message).

- **Positive/Encouraging Tone:** Avoid language that sounds like blame. Instead of *"error: mismatched types"* alone, say something like *"Oops, the types don’t match up."* Then explain what is expected. This softens the blow and keeps the user from feeling frustrated. For example: *"I ran into a problem with types. The left side of the addition is a Text, but the right side is a Number, and I don’t know how to add those together."* This reads like a mentor explaining the issue, rather than a terse compiler complaint.

- **Consistency and Formatting:** All messages should follow a consistent style and formatting:
  - Possibly start with an uppercase letter and no jargon code (Elm uses a format like `-- ERROR TYPE ----` but we might use Rust-like or our own; in any case, the message content should still be consistent).
  - If we choose to include the error code in output (as `codespan-reporting` will by default if we set `.with_code`), it will appear like `error[E3001]: ...`. We should still ensure the text after the colon is friendly. For example: `error[E3001]: I expected an Int but found a String` (this is slightly less formal than Rust might do, but fits our tone).
  - Use complete sentences or at least clear phrasing in label messages and notes.
  - Where applicable, prefer to speak in terms of the language’s own concepts. If WFL calls its string type "Text", use that word in messages instead of "string", etc.

- **Avoid Overwhelming the User:** If an error is complex, try not to dump too much information at once. Focus on the primary cause. (Elm famously only shows one error at a time.) Our system might show multiple labels, but ensure they all relate to the single error. We will generally report the first error encountered and stop, which is often easier for beginners to handle than a cascade of errors.

To solidify the style, we will review each existing error message in the codebase and rephrase it according to these principles. We’ll also borrow some wording ideas from Elm’s compiler messages and Rust’s, adjusting to our needs. Remember, as one article noted about Elm, it *"presents to you in plain, simple English the exact mistake it thinks you made, along with suggestions for fixing it."* ([Jamalambda's Blog - Elm - Amazing, Informative, Paternalistic Error Messages](https://jamalambda.com/posts/2021-06-13-elm-errors.html#:~:text=At%20the%20forefront%20of%20this,Of)) – that’s the experience we want to emulate.

## 7. Examples: Current vs. Improved Error Messages

Let's demonstrate how the diagnostics will improve by comparing hypothetical *current* error outputs to the *new* `codespan-reporting`-powered outputs. These examples illustrate both formatting and phrasing enhancements.

### Example 1: Syntax Error (Missing Brace)

**Code:** (`example.wfl`)  
```wfl
fn main() {
    print("Hello"
    print("world");
}
```  
*(In this code, there's a missing closing parenthesis after `"Hello"`.)*

- **Current Output (before Milestone 10):**  
  ```
  ParseError: Expected ')' before end of line at line 2, col 17
  ```  
  *This is a generic message with no code context, making it hard to spot exactly where the issue is.*

- **Improved Output (after integration):**  
  ```text
  error[E1002]: I was expecting a `)` here but found another token instead
    --> example.wfl:2:18
   1 | fn main() {
   2 |     print("Hello"
     |                 ^ missing `)` to close the string literal
   3 |     print("world");
   4 | }
   = help: Add a `)` after the string literal to close the `print(` call.
  ```  
  **What’s improved:** The new output shows the source code with an arrow pointing at the exact location of the missing parenthesis. The error message is phrased in a friendly way (*"I was expecting a `)`..."*). The label under the code clarifies what is missing, and a help note suggests how to fix it. This makes it immediately clear how to resolve the error, without having to manually count parentheses or find the line/col.

### Example 2: Type Error (Mismatched Types)

**Code:** (`example.wfl`)  
```wfl
let greeting: Int = "Hello" + 42;
```  
*(Here, a string is being added to an integer, and also the variable is declared as Int but assigned a string concatenated with int.)*

- **Current Output:**  
  ```
  TypeError: Type mismatch in addition
  ```  
  *This is vague and doesn't indicate where or what types are involved.*

- **Improved Output:**  
  ```text
  error[E3001]: Cannot add a Text and an Int
    --> example.wfl:1:20
   1 | let greeting: Int = "Hello" + 42;
     |                    ^^^^^^^^^^^^^ `+` can't combine `Text` (string) with `Int` (number)
     |                            ---- this is Text
     |                                    -- and this is Int
   = help: Convert the number to Text (e.g., using toString) or convert the Text to a number.
   = hint: All parts of an addition need to be numbers if you want to add them.
  ```  
  **What’s improved:** The error now clearly indicates the problem (*"Cannot add a Text and an Int"*). The snippet shows the whole expression `"Hello" + 42` underlined. We have a primary label with a message explaining that `+` can't combine those types. We also show smaller secondary markers on the `"Hello"` and `42` indicating their types (Text vs Int) for clarity. Two notes are provided: a help with a possible fix (conversion) and a more general hint about the rule. The tone is explanatory, not just stating "type mismatch". The user can easily see what went wrong and how to fix it.

### Example 3: Semantic Error (Undefined Variable)

**Code:** (`example.wfl`)  
```wfl
let result = x * 5;
```  
*(Variable `x` was not declared before use.)*

- **Current Output:**  
  ```
  SemanticError: Undefined variable x at line 1
  ```  
  *This identifies the issue but is terse and not very friendly.*

- **Improved Output:**  
  ```text
  error[E2001]: I can't find a definition for `x`
    --> example.wfl:1:14
   1 | let result = x * 5;
     |              ^ `x` is not defined in this scope
   = help: If `x` is a variable you intended to use, declare it with `let x = ...` before this line.
   = help: If `x` comes from another module, you might need to import it.
  ```  
  **Improvements:** The message now clearly says the compiler "can't find a definition for `x`", rather than just "undefined variable". It points at the exact `x` in the code. The note offers two possible suggestions (define it or import it). This anticipates the common reasons `x` might be undefined, helping the user figure out the next step. The overall tone is helpful rather than accusatory.

### Example 4: Runtime Error (Index Out of Bounds)

**Code:** (`example.wfl`)  
```wfl
let arr = [10, 20, 30];
print(arr[5]);
```  
*(Attempting to access index 5 which is out of bounds for a 3-element array.)*

- **Current Output:**  
  ```
  RuntimeError: Index out of bounds at line 2
  ```  
  *This tells what happened but without context of the array or index.*

- **Improved Output:**  
  ```text
  error[E4002]: Runtime error: index 5 is out of bounds for array
    --> example.wfl:2:11
   1 | let arr = [10, 20, 30];
   2 | print(arr[5]);
     |           ^^^ index 5 is invalid, the array has length 3
   = help: Valid indices for `arr` are 0, 1, and 2.
   = hint: Check that your index is less than the array length before accessing.
  ```  
  **Improvements:** We explicitly label it as a runtime error in the message (so the user knows it occurred during execution, not at compile-time). The snippet highlights `arr[5]` and notes that index 5 is not valid because the array length is 3. A help note even lists the valid indices, which is very actionable information. Another hint suggests a general practice (bounds checking). This turns a potentially confusing runtime exception into a clear guidance on what went wrong in the code.

These examples demonstrate the transformation: previously, WFL errors were one-liners with minimal info; after this milestone, errors are multi-line, richly annotated, and explanatory. Each example shows how a user can quickly pinpoint the error in their code and get hints to fix it, fulfilling the goal of a beginner-friendly diagnostics system.

## 8. Testing the Improved Error Output

Testing the new diagnostics system is crucial to ensure consistency and quality. We will employ both **automated snapshot tests** for error messages and manual verification of formatting and wording.

**Automated Testing Approach:**

- **Unit Tests for `to_diagnostic`:** Write unit tests for the conversion from error types to `Diagnostic`. We can construct a dummy `SimpleFiles` with a small source snippet and a known error, call `error.to_diagnostic()`, and then verify:
  - That the `Diagnostic`'s message, code, labels, and notes match expected values.
  - For example, create a `ParseError::UnexpectedToken` for a specific span and check that `diag.message == "...expected...found..."` and that `diag.labels` contains one primary label with the given span.
  - We can use `codespan-reporting`’s ability to emit to a buffer (by providing our own implementation of `termcolor::Write` or using `termcolor::Buffer`) to capture the output and then assert on the formatted string. This would test the integration end-to-end (ensuring colors and formatting are as expected, or use `ColorChoice::Never` to avoid ANSI codes in the output for comparison).

- **Snapshot Testing of Full Outputs:** Because error outputs are multi-line and rich, **snapshot tests** are very useful. We can use a crate like [`insta`](https://crates.io/crates/insta) to capture the output of `report_error` for various error scenarios and compare it to a reference:
  1. Prepare a set of representative WFL source snippets that produce each kind of error (parse, type, etc.).
  2. Write tests that run the compiler (or the specific stage) on those snippets. Instead of printing to stderr, redirect the output to a buffer (or have `report_error` accept a writer parameter in tests).
  3. Assert that the output matches the expected snapshot. The first run will record the snapshots, and subsequent runs will diff against them.
  4. This will catch any unintended changes in formatting or wording. If we intentionally improve wording later, we update the snapshots accordingly.
  - We will configure these tests to use `ColorChoice::Never` (monochrome) to avoid ANSI escape codes in the snapshots, making them clean text comparisons.

- **Line-by-line Verification:** In tests, we can also parse the output to ensure certain structure:
  - The first line should contain `error[E####]: ` and the main message.
  - Subsequent lines should contain the file path and line numbers (we can regex check the presence of `--> filename:line:col`).
  - Ensure that for each label we expect, there is a corresponding `^` or `~` underline in the output. This is harder to assert programmatically, but snapshot testing covers it by human inspection of the stored snapshot.
  - Check that help notes begin with `= help:` (the default format in codespan for notes is with an `=` indent and label).

- **Testing Span Accuracy:** We should also have tests for our span calculations:
  - e.g., feed a known string with a token at a certain position, lex it, and ensure the span’s start and end indices match the expected positions of that token. This ensures our lexer offsets are correct.
  - Test parser spans for composite expressions (e.g., parse an `if` expression and check that its span covers the whole `if`...`end` region).
  - These ensure that when errors are produced, the spans truly cover the intended code.

**Manual and Visual Testing:**

- After implementing, run the compiler on various faulty inputs and visually inspect the console output:
  - Verify colors are appearing correctly (e.g., file path and caret in bold/red, notes in a different color as per default config).
  - Check alignment of arrows under multi-byte characters or tabs (we might adjust `Config` for tab width or ensure our examples don't misalign).
  - Try errors with multiple secondary labels to see that they all show up without confusion.
  - If any output seems odd (like overlapping labels or truncated messages), adjust our spans or text accordingly.

- Test in different terminals to ensure color codes are supported (the `termcolor` crate should handle Windows vs Unix).

- Consider writing a few integration tests where we intentionally trigger two errors in one run (if our compiler continues after one error). `codespan-reporting` by default will separate diagnostics with a blank line. Ensure that looks okay. If Elm-style single error is desired, we might stop after one anyway.

**Snapshot Example:** Using `insta`, a test might look like:  
```rust
#[test]
fn test_type_error_message() {
    let source = r#"let x: Int = "hi";"#;
    let mut files = SimpleFiles::new();
    let file_id = files.add("test.wfl", source.to_string());
    // Induce a type error (this depends on calling our type checker function)
    let error = type_check(source).unwrap_err(); // Suppose it returns our TypeError
    // Capture output
    let mut buffer = termcolor::Buffer::no_color();
    let diagnostic = error.to_diagnostic();
    let config = codespan_reporting::term::Config::default();
    term::emit(&mut buffer, &config, &files, &diagnostic).unwrap();
    let output = String::from_utf8_lossy(buffer.as_slice());
    insta::assert_snapshot!(output, @r###"
    error[E3001]: I expected an Int but found a Text
      --> test.wfl:1:12
     1 | let x: Int = "hi";
       |            ^^^^ expected `Int` here
       |            ---- this is `Text`
     = help: Try annotating the variable with the correct type or change the value.
    "###);
}
```  
This snapshot, when approved, will ensure our error output remains as intended.

By building a comprehensive test suite, we gain confidence that our diagnostics system is reliable. It also allows future contributors to refactor error messages while easily updating expected outputs.

## 9. REPL and Editor Integration (Optional Enhancements)

Finally, while not strictly required for Milestone 10, it’s worth considering how the improved diagnostics can integrate with a REPL and developer tools:

- **REPL Integration:** If WFL has (or will have) a REPL, the `report_error` functionality can be used there as well. In a REPL:
  - The user inputs code (possibly multi-line). We can treat the input as a virtual "file" in our `SimpleFiles` database (e.g., with a name like `"REPL"` or even just an empty string for file name, since it’s not an actual file path).
  - When evaluating a line or block, if an error occurs, we call `report_error` just the same. The output will show `--> REPL:line:col` or similar. We should ensure the file name we use is something like `REPL` or `<repl>` for clarity.
  - For one-liner inputs, the snippet in the error output might seem redundant (since it's just the line the user wrote). But it still helps to point out exactly where on the line the issue is, especially if the input is complex. 
  - If the REPL reads multiple lines (e.g., user can define a function in multiple lines), our span covers those and the snippet will show them.
  - We might want to disable color in REPL if it doesn’t render well, but most terminals should be fine. Alternatively, use `ColorChoice::Auto` which will output colors if the REPL stdout is a terminal.

- **Language Server Protocol (LSP) and Editor Plugins:** The structured error information we now maintain can be readily used in an editor integration:
  - Most editors communicate errors via LSP, which has a `Diagnostic` type (not to be confused with codespan’s, but conceptually similar: it has a range (span), a message, a severity, and an optional error code and suggestions).
  - We can write an LSP server for WFL that, upon compilation or on-the-fly checking, uses our error objects to populate LSP `Diagnostic`s. For each error:
    - The `Span` gives us the `start` and `end` positions; we can translate those into LSP `Position { line, character }` (since we can get line/col from our `files` database easily).
    - The main error message (perhaps including our help suggestions) can be sent as the `Diagnostic.message`. We might combine the main message and notes for the LSP message, or send the notes as separate `Diagnostic.relatedInformation`.
    - The `error.code` can be sent in the LSP `Diagnostic.code` field, which editors might display or use to link to documentation.
    - Severity is obviously Error for these.
    - Secondary labels: LSP supports related locations via the `relatedInformation` field. We could send each secondary label as a related info entry, so that the editor can for example allow clicking to jump to the related span (e.g., jump to the first definition in a duplicate definition error).
  - **Code Actions:** With suggestions structured, we can potentially offer fix-its. For example, if we know an error that can be fixed by adding a missing semicolon, an LSP code action could be provided to do that. Our diagnostics structure can be extended to carry a machine-readable suggestion (like "insert token X at span Y"). `codespan-reporting` doesn't itself provide machine-readable fixes, but we can incorporate that in our error types if we plan for it (not in this milestone necessarily, but a thought). 
  - Even without automatic fixes, having spans and clear messages means an editor plugin can highlight the exact range of the error and possibly show the help text in a tooltip.

- **Consistent Data Usage:** The investment in attaching spans and codes to errors pays off here because we no longer rely on parsing error strings. The compiler can output JSON or another format for errors if needed (using the error objects), or directly interface with an LSP library. Essentially, we have a *single source of truth* for an error's details, used both for console output and potential IDE integration.

- **Future Integration Example:** If we were to implement an `--json` flag for the compiler to output errors in JSON (useful for editor integration without full LSP), it would simply iterate over error objects and print their span (file, start, end), code, message, and notes in JSON form. Because we designed `Span` and errors clearly, this would be straightforward.

In summary, while the immediate deliverable is the improved console diagnostics, this plan sets the stage for WFL to have excellent developer experience in any environment – REPL, command-line, or IDE. The errors will guide users like a “friendly mentor” (as Elm does), reducing frustration and making the language more accessible.

---

By following this implementation plan, Milestone 10 will deliver a robust diagnostics system: **well-structured error data, integrated pretty-printing, consistent spans, and polished messages**. Developers using WFL will benefit from clear compiler errors that not only pinpoint issues in their code but also suggest solutions. This will significantly enhance the usability of WFL and lay a foundation for future tooling integrations. ([GitHub - brendanzab/codespan: Beautiful diagnostic reporting for text-based programming languages.](https://github.com/brendanzab/codespan#:~:text=Languages%20like%20Rust%20and%20Elm,and%20relatively%20painless%20for%20everyone))