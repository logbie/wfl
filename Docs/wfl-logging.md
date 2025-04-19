Thanks! I’ll design a logging and debug report generation system for WFL that:

- Captures timestamps, error type, file/line number, and execution time.
- Supports multiple log levels (debug, info, warn, error).
- Writes logs to both file and console.
- Automatically generates a plain text debug report (saved to disk) unless disabled via `.wflcfg`.
- Includes stack trace, local variable values, and either the full action/function or just surrounding lines depending on what’s feasible.
- Adds new `.wflcfg` keys to control logging and debugging behavior.

I’ll review the existing code in `src.md` and draft an implementation plan, identifying where these features should be integrated and what infrastructure needs to be added.
I’ll get back to you with a full plan shortly.

# Logging and Debug Report System Design for WFL

**Overview:** This design introduces a structured runtime logging system and an automatic debug report generator for the WebFirst Language (WFL) interpreter/compiler. The goal is to aid debugging by recording key execution details and producing a plain-text report when an uncaught error occurs. By default, debug reports will be generated on errors (opt-out via config), and logging can be toggled via configuration. We detail the logging format and levels, the content/format of debug reports, necessary `.wflcfg` configuration changes, and how these integrate into the existing codebase.

## Logging System

The WFL logging system will capture significant runtime events and errors with timestamps and context, outputting to both the console and a log file. We will use a **log level** approach (debug, info, warn, error) to filter messages. Logging is **disabled by default** (to keep normal usage output clean), but can be enabled via configuration.

**What to Log:** At a minimum, the following will be logged when logging is enabled:

- **Startup/Shutdown:** An info-level entry when execution starts (with script name) and when it ends. On normal completion, log the total execution time. On error, log that execution aborted due to an error.
- **Errors:** Each runtime error will produce an error-level log entry including timestamp, error type/message, file name, and line number. We also log the **execution time** at the moment of error (time since start) to know how long the program ran before failing. For example, a log entry might look like:  
  ```text
  [2025-04-19T01:39:00Z] ERROR – RuntimeError at script.wfl:37 – Division by zero (after 1.352s)
  ```  
  This indicates an error (“Division by zero”) at line 37 of the script, logged at the given timestamp, and that ~1.352 seconds elapsed since start. The “ERROR” level and message make the nature clear. The error type (e.g. runtime error) is part of the message or could be a separate field.
- **Warnings:** Any non-fatal issues should be logged at warn level. For example, the interpreter currently prints a warning for using `count` outside a loop; this will be converted to a `log::warn!` call so that if logging is on, it appears in the log (in addition to printing to console) ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=if%20name%20%3D%3D%20,line%2C%20column)). All such internal warnings will follow a similar format with timestamp and context.
- **Info/Debug Traces:** Key execution steps can be logged at info or debug level. For instance, opening or closing a file, starting or ending an HTTP request, entering or exiting a user-defined action, etc. These debug-level messages can help trace program flow or performance. For example, when logging at debug level, we might log entering a function (“Calling action `foo` with arguments x=…”) and exiting it (“Action `foo` returned in 5ms”). By default, the log level threshold might be set to **Info** (so debug messages are omitted unless the level is raised in config).

**Log Output Destinations:** The system will output logs to:
- **Console:** Important log messages (info, warnings, errors) will appear on stderr or stdout. For example, errors might already be shown via the diagnostic system, but if logging is on we ensure an entry is logged as well. We may filter debug-level logs from the console by default to avoid clutter, focusing console output on info/warn/error.
- **Log File:** All log messages (down to the selected minimum level) will be appended to a log file on disk. By default, we can create a file named, for example, `<script_name>.log` or `wfl.log` in the script’s directory. This file will receive the same messages with timestamps. Using a logging facade like Rust’s `log` crate (already in use for debug messages in config loading ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=if%20let%20Some%28rest%29%20%3D%20line.strip_prefix%28,value%2C%20file.display%28%29))) with a backend logger (e.g. `env_logger` or a custom logger) will allow us to easily direct output to both console and file. We can initialize a combined logger at startup that writes to stdout and to `wfl.log`. If needed, a third-party crate (such as **simplelog** or **fern**) can simplify multi-output logging. For example, we could initialize logging as: “console at info level, file at debug level,” so that detailed traces go to the file while the console shows only higher-level events.

**Log Format:** Each log entry will include:
- A timestamp (date and time).
- The log level (DEBUG/INFO/WARN/ERROR).
- Context like the file name and line number (for errors or where applicable).
- A descriptive message. For errors, the message will include the error description. For other events, it will be a brief English statement of what happened (following WFL’s ethos of readability). 

For instance:  
```
[2025-04-19T01:39:00Z] INFO  – Started execution of script.wfl
[2025-04-19T01:39:00Z] DEBUG – Calling action "calculateStats" (defined at line 10)…
[2025-04-19T01:39:02Z] ERROR – RuntimeError at script.wfl:37 – File not found: "data.csv" (after 2.014s)
[2025-04-19T01:39:02Z] INFO  – Execution aborted after 2.014s due to error
```  
This illustrates an info start message, a debug trace (function call), an error with context, and a closing info. All entries also go to the log file for later review.

**Implementation:** In code, we will leverage the existing `log` crate usage. For example, in `config.rs` the system already calls `log::debug!` when a config override is loaded ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=log%3A%3Adebug%21%28%20,value%2C%20file.display%28%29)). We will extend this by initializing the logger in `main.rs` if logging is enabled (using `log::set_max_level` and initializing an output). We’ll parse the desired log level from config (e.g. default to Info if not specified) and apply it. Each significant event in the interpreter will invoke the appropriate `log::info!`, `log::warn!`, etc. The **error handling** code will be instrumented to log errors: for instance, when a runtime error is caught in the interpreter, do `log::error!("Runtime error: {} at line {}", err.message, err.line)` along with timing. We can obtain the current timing via `interpreter.started.elapsed()` (since the Interpreter struct already stores a start time ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=current_count%3A%20RefCell%3A%3Anew,)) ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=pub%20fn%20with_timeout%28seconds%3A%20u64%29%20,max_duration%20%3D%20Duration%3A%3Afrom_secs%28seconds))). This allows including the “(after X seconds)” detail in the log. 

Additionally, operations like file I/O or HTTP requests in `IoClient` can have debug logs. For example, log at debug level when an HTTP GET starts and when it completes, possibly with status code, to trace asynchronous operations. These would be optional deeper traces visible when log level is set to debug.

**Log Level Control:** The `.wflcfg` file will allow specifying the minimum log level and whether logging is on. For simplicity, we introduce a boolean key to toggle logging (see **Configuration** below), and optionally a `log_level` setting (e.g. “debug”, “info”) to refine the verbosity. If no level is specified, we assume a default (Info). The logging system will respect these: debug-level messages only appear if the level is debug, etc. This prevents flooding the log with low-level traces unless the user explicitly wants them.

By implementing this logging system, developers can monitor WFL program execution in real time and review the history after a run, which is especially useful for long-running scripts or diagnosing issues that don’t necessarily crash the program.

## Debug Report Generation

In addition to logging, the system will **automatically generate a detailed debug report** whenever an uncaught runtime error occurs (unless this feature is disabled in configuration). The debug report is a plain text file that captures a snapshot of the program state at the moment of failure, helping developers diagnose the issue. This report is meant to be human-readable and in line with WFL’s ethos of clarity (using natural language and actual code from the script rather than raw data or memory dumps).

**Trigger:** The debug report is generated when a runtime error is about to terminate the program. (In the future, when `try/when` error-handling is implemented, the report will only be produced for uncaught exceptions that propagate out of all `try` blocks.) If logging is enabled, the error would have been logged; regardless, the debug report provides a deeper post-mortem. This is on by default – any crash will produce `"<scriptname>_debug.txt"` (for example) – unless the user opts out via `.wflcfg`.

**Contents of the Report:** Each report will include the following components:

- **Error Summary:** A brief description of the error that occurred, including the error message and its location. For example: *“Runtime Error: Division by zero at line 37, column 12.”* This is essentially the same info as a one-line error message but serves as a header for the report.
- **Stack Trace:** A stack trace of WFL calls leading to the error. This lists each active action (function) call at the time of error, from the top-level script down to the deepest function where the error occurred. Each entry will show the function or action name and the location of the call. For instance, if action `C` crashed, which was called by `B` at line 20, which in turn was called by `A` at line 5 of the main script, the trace might look like:  
  ```
  Stack Trace:
    at action C (error occurred in C’s body, defined at line 50)
    called from action B (line 20 of B’s body)
    called from action A (line 5 of A’s body)
    called from main script (line 12 of script.wfl)
  ```  
  The lowest entry is the site of the error (C), and above it the chain of calls. If the error happens in the top-level (not inside any action), the stack trace will simply indicate it was in the “main script” context. The interpreter will collect this call stack information during execution (see Implementation below).
- **Source Code Snippet:** The report will show the **actual source code** around the point of failure. This includes the line that likely caused the error (and a few lines of context). We will highlight or mark the exact line/column if possible. For example:  
  ```text
  Code at error location (script.wfl line 37):
     35 |    compute average of values
     36 |    divide total by count   <-- prior line
  >> 37 |    divide total by 0       <-- ERROR here: Division by zero
     38 |    display result
  ```  
  This snippet is extracted from the script, with an indicator (`>>`) or an arrow pointing to the line 37 where the error occurred. This gives immediate context on what the code was doing. We will retrieve these lines from the original source file (we have the source in memory from `main.rs` as `input`). Since the diagnostic system already can map line/column to source (using `codespan_reporting`), we may reuse it or do a simple manual extraction of a few relevant lines.
- **Full Action Body (if applicable):** If the error occurred inside a user-defined action (function), the report will include the **entire body of that action** for reference. This is important because WFL actions are defined in plain language and seeing the whole function helps understand the logic context. We will list the function definition (all its lines) as written in the source. For example:  
  ```text
  Action `C` definition (lines 45–60):
  45 | action C with parameters x, y:
  46 |     if y is 0:
  47 |         fail with "Division by zero"
  48 |     otherwise:
  49 |         compute z as x / y
  50 |         return z
  51 | end action C
  ```  
  (This is a hypothetical syntax illustration.) By providing the whole function, the developer can see everything that `C` does, rather than just the single failing line. If the error was in the main script (not in a function), we skip this, or we might include a larger snippet of the surrounding script if that seems helpful. The ethos is to give as much context as possible in an accessible format.
- **Local Variables:** The report will dump the **values of local variables in the scope of the error** at the time it occurred. This is essentially a snapshot of the interpreter’s environment for that stack frame. For an error in action `C`, we list all variables local to `C` (including parameters and any locals defined in `C` up to that point). For an error in the main script, we list the global variables (since those are the “locals” of the top-level). The variables will be printed in a `name = value` format. We will use the `Value` debug representation for each value, which already formats WFL values nicely (e.g., lists as `[1, 2, 3]`, text in quotes, etc.) ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=impl%20fmt%3A%3ADebug%20for%20Value%20,borrow%28%29%3B%20write%21%28f%2C)) ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=Value%3A%3AObject%28o%29%20%3D,k%2C%20v%29%3F%3B)). For example:  
  ```text
  Local Variables at error:
    total = 42
    count = 0
    result = [Null]  (not yet assigned a valid value)
  ```  
  This section shows the state that likely led to the error (here we can clearly see `count = 0`, which explains the division by zero). Only the innermost scope’s variables are listed by default – typically those are the most relevant. (If needed in the future, we could also include some global or outer scope variables, especially if an outer variable is being used in the failing function via closure. But to keep the report focused, we’ll primarily show the immediate local environment.)

All of the above sections will be formatted in plain text with clear headings (e.g., “Stack Trace:”, “Local Variables:”) so that it reads like a coherent report rather than a raw dump. The language used will align with WFL’s friendly tone (for instance, using "error occurred here" or similar phrasing, akin to the hints we provide in diagnostics). No specialized formatting (like JSON or XML) is used – just human-readable text.

**File and Format:** The debug report will be written to a file on disk in the project or script directory. We might name it `<script_name>_debug.txt` or always `wfl_debug_report.txt`. Using the script’s name helps identify which script the report is for. Each run would overwrite the file unless we choose to timestamp it; for simplicity, we can overwrite the last report (assuming one failure per run). The report is plain text so it can be opened in any editor. We will ensure the file writing is done safely (catching any I/O errors writing the report and perhaps logging them).

**Implementation Approach:** To generate this report, we need to gather the required information at the moment of error:

- **Call Stack Tracking:** We will enhance the interpreter to maintain a call stack of WFL function calls. We introduce a data structure, e.g. `Vec<CallFrame>`, where `CallFrame` could be a struct containing the function/action name (if applicable) and perhaps the location where it was called. The `Interpreter` struct can have a field `call_stack: RefCell<Vec<CallFrame>>`. Whenever a user-defined action is invoked, we push a new frame. When the action returns, we pop the frame. Specifically, in `Interpreter.call_function`, before executing the function body we push a frame (with the function’s name or "<anonymous>" if unnamed, and the call site line number) onto `call_stack`. After the function’s body executes, we pop it off. If the call is nested, multiple frames will accumulate. For example, if main calls A, which calls B, which calls C, then at C’s execution we have frames for A, B, C in the stack (A at bottom, C at top). 

  If an error occurs, we will *not* immediately pop the frame for the failing function – we want it to remain on the stack to be reported. In fact, we will likely abort further execution at that point (see below), so the stack remains intact. The stack trace in the report can then be constructed by reading `Interpreter.call_stack`: it will contain all active calls at the time of error. We can iterate over these frames to print the trace as shown. For the main script (which is not a function call per se), we can either push a special frame at start (like “<main>”) or simply handle it as a special case when printing (if the stack is empty, assume the error was in global context). We’ll likely just note “main script” as the outer context in the trace.

- **Capturing Local Variables:** To retrieve local variables of the failing scope, we use the `Environment`. In the interpreter, each function call has its own `Environment` (created via `Environment::new` with a parent pointer) ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=let%20call_env%20%3D%20Environment%3A%3Anew%28%26func)). The `Environment` struct holds a `values` hashmap of all variables in that scope ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=%5Bderive%28Debug%29%5D%20pub%20struct%20Environment%20,Environment%3E%3E%3E%2C)). We will capture this when the error happens. There are a couple ways:
  - Eager approach: At the moment a runtime error is raised, gather the local variables from the current environment.
  - Lazy approach: After an error, use the saved environment reference to gather variables.

  We plan to modify the error propagation so that when an error is detected, we have access to the environment. For example, in `execute_statement` or `evaluate_expression`, errors are returned as `RuntimeError` objects. We will augment the error handling to also record the environment. One strategy is to extend `RuntimeError` with a reference or pointer to the environment or variables, but that can be complicated (and we want to keep `RuntimeError` simple for diagnostics). Instead, we will do this: when a function call fails, we know the top of the call stack corresponds to that function and we have its `Environment`. We can capture the variables *before* we unwind. We might store them in the `CallFrame` for that function. For instance, if `call_function` catches an `Err`, we can do: 
  ```rust
  if let Err(error) = self.execute_block(&func.body, call_env).await {
      // on error, snapshot local variables
      let locals = call_env.borrow().values.clone();
      current_frame.locals = Some(locals);
      return Err(error);
  }
  ```
  Here `current_frame` is the frame we pushed for this function. We clone the `values` map so we have a snapshot of all variables at error time. We then leave the frame on the stack (not popped). The error is propagated upward. Using this method, by the time the error reaches the top-level, the `Interpreter.call_stack` contains the frames for each call, and the bottom-most frame (stack.last()) will have a `locals` field populated with the variables of the failing function. For an error in the main script, there is no function frame, but we can directly use the global environment (`Interpreter.global_env`) for variables.

- **Obtaining Source Code:** We have the source code as a string (in `main.rs`, `input` contains the program text). We can reuse the `DiagnosticReporter`’s file store or just keep the `input` string. To get the code snippet and function body:
  - For the immediate error line snippet: we know the line number (`error.line`) and column (`error.column`) from the `RuntimeError` ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=,usize%2C%20pub%20column%3A%20usize%2C)). We can split the input by lines and take, say, a few lines before and after `error.line`. We will then format them with line numbers and an arrow pointing to `error.line:column`. (The codespan library could do this formatting; in fact, it already produces a pretty error message with an arrow – we may borrow that approach for consistency). The key is to ensure the snippet in the debug report is clear even without color: we’ll use indicators like `>>` or `^^` under the text to mark the error location.
  - For the full function body: If `error` occurred inside an action, we need to identify that function’s start and end in the source. The AST has an `ActionDefinition` node with a starting line ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=ActionDefinition%20,line%3A%20usize%2C%20column%3A%20usize%2C)), but not an explicit end line. However, WFL uses an `end` keyword for blocks (as seen by `Token::KeywordEnd`), or at least we can assume the function body is contiguous. We can find the function by name or by matching the line. Another approach is to use the stored `FunctionValue` of the function: when the interpreter created the function (on parsing an ActionDefinition), it stored the body statements and the definition line/column in `FunctionValue` ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=%5Bderive%28Clone%29%5D%20pub%20struct%20FunctionValue%20,usize%2C%20pub%20column%3A%20usize%2C)). We have the function’s body AST and its starting line. We can retrieve all source lines from the function’s start line up to the last line of its body. Since we know the statements in the body (each with their own line info), the end of the function is either just after the last statement or at an `end` keyword. A simple solution is to print from the function’s start line to the line of the last statement in its body (or one line further if we suspect an `end` is present). This will effectively print the entire function. We will include the function signature line (with the action name and parameters) and all indented lines until the dedentation or `end`. Implementation-wise, we can store the function’s start and end lines when the function is defined, or just dynamically scan the `input` lines (since the function likely appears as one continuous block in the source).
  - All of this extraction will happen when creating the report (not during normal execution). We will likely implement a helper in a new module (say `debug_report.rs`) that takes the source text, an error, and the interpreter state (call stack, etc.) and produces the formatted report text.

- **Generating and Saving the File:** Once we assemble the content (stack trace string, code snippets, variable listings), we concatenate it into a single report string. We then use Rust’s `std::fs::write` or similar to write it to the output file. This will be invoked in the error handling path of `main.rs`. Specifically, in `main.rs` where we handle the result of `Interpreter.interpret(...)`, if an `Err(errors)` occurs, we will add logic: if debug-report generation is enabled (per config), generate the report. We can call a function like `debug_report::generate(&interpreter, &errors[0], &input, &script_path)`. We pass the interpreter (to access call stack and environment) and the error (or the first error if there are multiple). We primarily focus on the first error because once one runtime error occurs, subsequent ones may be side-effects; it’s most useful to debug the initial failure. (In fact, we may modify the interpreter to stop execution on the first error to avoid cascading failures – see **Integration Points** below.)

  The `generate` function will use the methods described to gather stack trace info from `interpreter.call_stack`, get the code from `input`, and so on. It will then write out the file. We will ensure to log this action (e.g., `log::info!("Debug report saved to {}.", report_path)`) and perhaps print to console a one-liner like *“Debug report written to file <...>”* so the user knows to check it. If writing the file fails (disk error, etc.), we catch that and print an error to console (since if we can’t save the report, that’s important to know).

**Consistency with WFL Ethos:** The debug report, while detailed, will be presented in a user-friendly manner. We avoid overly technical jargon; for example, instead of “NullReferenceException in function foo” we use WFL’s style: “Runtime Error” and plain descriptions. We show actual WFL code in the report rather than internal representations. This ensures that even less experienced users can follow the report to see what went wrong. The format is plain text so it’s universally accessible. By default, this is turned on to help beginners get maximum information on crashes. Advanced users or production deployments can disable it via config if it’s not needed.

## Configuration (.wflcfg) Support

We will extend the WFL configuration file (`.wflcfg`) to allow toggling the logging and debug report features. The config file is a simple text file with `key = value` pairs (one per line, `#` for comments) placed in the script’s directory. Currently, it supports a `timeout_seconds` setting ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=if%20let%20Some%28rest%29%20%3D%20line.strip_prefix%28,value%2C%20file.display%28%29)). We will add two new keys:

- **`logging_enabled`** – Boolean (`true`/`false` or `yes`/`no` etc.) to enable or disable runtime logging. If `logging_enabled = true`, the interpreter will initialize the logging system (writing logs to console/file as described). If false or not present, no logs are produced (except the normal console outputs). Default if not specified is **false** (off).
- **`debug_report_enabled`** – Boolean to enable/disable automatic debug report generation. Default is **true** (so a report will be generated by default). If a user sets `debug_report_enabled = false`, the interpreter will skip creating the debug report on errors (it will rely on the console error message and logs only). This gives users control in case they do not want extra files created.

Optionally, we could support a `log_level` setting (e.g. `log_level = "debug"` or `"info"`). This would allow users to choose how verbose the logging should be. For this design, if not provided, we default to Info or Debug depending on preferences (likely Info to avoid huge logs unless needed). We can parse it similarly and set the logger level accordingly.

**Parsing Config:** In the code, the config loader will be updated to parse these new keys. We may refactor `config.rs` to handle multiple settings in one pass. For example, we can introduce a `Config` struct:
```rust
pub struct WflConfig {
    pub timeout_seconds: u64,
    pub logging_enabled: bool,
    pub debug_report_enabled: bool,
    // potentially log_level: LevelFilter, etc.
}
``` 
and a function `load_config(dir: &Path) -> WflConfig`. This function would read the `.wflcfg` file if it exists and initialize a `WflConfig` with default values (e.g. timeout 60, logging_enabled = false, debug_report_enabled = true). It then iterates over each non-comment, non-blank line, and parses `key = value`. Pseudocode:
```rust
let text = std::fs::read_to_string(file)?;
for line in text.lines().map(|l| l.trim()) {
    if line.is_empty() || line.starts_with('#') { continue; }
    if let Some(val) = line.strip_prefix("timeout_seconds") {
        if let Some(num_str) = val.split('=').nth(1) {
            if let Ok(num) = num_str.trim().parse::<u64>() {
                config.timeout_seconds = num.max(1);
                log::debug!("Loaded timeout override: {}s from config", config.timeout_seconds);
            }
        }
    } else if let Some(val) = line.strip_prefix("logging_enabled") {
        if let Some(bool_str) = val.split('=').nth(1) {
            let b = bool_str.trim().eq_ignore_ascii_case("true") || bool_str.trim() == "1";
            config.logging_enabled = b;
        }
    } else if let Some(val) = line.strip_prefix("debug_report_enabled") {
        if let Some(bool_str) = val.split('=').nth(1) {
            let b = bool_str.trim().eq_ignore_ascii_case("true") || bool_str.trim() == "1";
            config.debug_report_enabled = b;
        }
    } /* else if log_level, etc. */
}
```
This is similar to how `timeout_seconds` is handled now ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=if%20let%20Some%28rest%29%20%3D%20line.strip_prefix%28,value%2C%20file.display%28%29)), just extended for new keys. We’ll ensure to trim whitespace and handle common truthy/falsy values. After reading, we return the filled `WflConfig`. (The existing `load_timeout` function may be updated to call this or be deprecated in favor of the unified loader. We can keep it for compatibility in tests, but internally we might use the new loader and just extract the timeout field for simplicity.)

We will also add tests for these new config options, similar to the ones for timeout ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=,wflcfg)). For example, create a temp `.wflcfg` with `logging_enabled = true` and `debug_report_enabled = false` and ensure `load_config` returns the right booleans. Also test that absent keys use defaults.

**Using Config in Main:** In `main.rs`, instead of just calling `load_timeout`, we will load the full config:
```rust
let config = config::load_config(script_dir);
```
Then:
- Use `config.timeout_seconds` to initialize the interpreter with a timeout (as done now).
- Check `config.logging_enabled`: if true, set up logging. For instance:
  ```rust
  if config.logging_enabled {
      // initialize logger (console + file) at config.log_level (or default Info)
      init_logging(config.log_level.unwrap_or(Level::Info));
      log::info!("Logging enabled (level = {:?})", config.log_level);
  }
  ```
- Check `config.debug_report_enabled`: if false, we record that so we know to skip report generation. We might pass this flag to the interpreter or just keep it in main’s scope for when handling errors.

The interpreter itself might not need to know the `debug_report_enabled` flag, since we can decide in main whether to call the report generator. The interpreter will always maintain the stack info regardless (a minor overhead even if we don’t use it every time).

## Integration Points in the Codebase

To implement these features cleanly, we will inject functionality into several parts of the codebase, leveraging the existing structure:

- **`config.rs`:** Extend the config parsing as described. We likely create a new `WflConfig` struct and a `load_config` function. We will reuse the pattern shown in `load_timeout` (iterating lines, skipping comments) ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=if%20let%20Some%28rest%29%20%3D%20line.strip_prefix%28,value%2C%20file.display%28%29)). The new keys `logging_enabled` and `debug_report_enabled` will be recognized and their values stored. The default values (false and true respectively) are set when initializing the config struct. This keeps configuration concerns centralized in `config.rs`. The existing `load_timeout` can call `load_config` internally and return `config.timeout_seconds` for backward compatibility (so existing code/tests using `load_timeout` still work). We will update tests or add new ones for the new keys accordingly.

- **`main.rs`:** This is where everything comes together. After reading the input file, we determine the script directory and load the config:
  ```rust
  let script_dir = Path::new(&args[1]).parent().unwrap_or(Path::new("."));
  let config = config::load_config(script_dir);
  ```
  We then initialize logging if enabled:
  ```rust
  if config.logging_enabled {
      // e.g., use env_logger or custom init
      setup_logger(config.log_level.unwrap_or("info"))?;
      println!("Logging to wfl.log (level = {})", config.log_level.unwrap_or("info"));
  }
  ```
  (The `println!` is optional feedback; the logger itself will also note it in the log file.)
  
  Next, create the interpreter with the specified timeout:
  ```rust
  let mut interpreter = Interpreter::with_timeout(config.timeout_seconds);
  ```
  Then execute `interpreter.interpret(&program).await`. The result handling will be expanded. Currently, it distinguishes runtime errors, type errors, etc., and uses `DiagnosticReporter` to print nicely to stderr. We will insert additional logic for runtime errors:
  ```rust
  match interpret_result {
      Ok(val) => { println!("Execution completed successfully."); /* possibly log success */ },
      Err(runtime_errors) => {
          eprintln!("Runtime errors occurred:");
          // print errors via DiagnosticReporter (unchanged behavior)
          … 
          // **New**: generate debug report if enabled
          if config.debug_report_enabled && !runtime_errors.is_empty() {
              let report_path = debug_report::create_report(&interpreter, &runtime_errors[0], &input, &args[1]);
              eprintln!("Debug report saved to {}", report_path);
              log::info!("Debug report generated at {}", report_path);
          }
      }
  }
  ```
  If multiple runtime errors are present in the vector, we take the first one for the report (assuming the first is the root cause). We pass the interpreter by reference so the report generator can inspect the call stack and environments. We also pass the source code (`input`) and maybe the script file name (for labeling). The `debug_report::create_report` will produce the file and return the path for logging/printing. We ensure to log that the report was created (so it appears in the log file as well).

  Additionally, even if debug reports are disabled, our logging system will have still logged the error events. And if logging is disabled but debug reports enabled, we still get the report file.

- **`Interpreter` (and submodules):** We integrate the call stack and error context tracking here:
  - Add a field `call_stack: RefCell<Vec<CallFrame>>` to the `Interpreter` struct (likely in `interpreter/mod.rs`). Define a struct `CallFrame` with fields like `func_name: String` (or `Option<String>` for anonymous), `call_line: usize`, `call_col: usize`, and maybe `locals: Option<HashMap<String, Value>>` for captured locals on error. Mark `CallFrame` as `Debug` for ease of logging if needed.
  - Initialize `call_stack` in `Interpreter::new()` and `with_timeout` as empty. Each time `interpret` is called on a new program, we should clear the stack to start fresh (e.g. at the top of `interpret`, do `self.call_stack.borrow_mut().clear()`). We might also push a frame for the “main script” context here (with name e.g. "<main>" or the script filename, and no caller line since it’s top-level). That frame would remain throughout execution and be popped at end. However, it may be simpler to not push a frame for main and just handle it in output by saying “(in main script)” when the stack is empty. Either approach is fine; pushing a main frame (with perhaps line 0) can make the stack trace uniformly structured.
  - In `Interpreter.call_function(&self, func: &FunctionValue, args: Vec<Value>, line: usize, column: usize)`: we insert the stack push/pop. Pseudocode:
    ```rust
    // Before executing:
    let frame = CallFrame {
        func_name: func.name.clone().unwrap_or_else(|| "<anonymous>".to_string()),
        call_line: line,
        call_col: column,
        locals: None,
    };
    self.call_stack.borrow_mut().push(frame);
    // Execute the function body:
    let result = self.execute_block(&func.body, call_env).await;
    // After execution:
    match result {
        Ok(val) => {
            self.call_stack.borrow_mut().pop();  // function returned normally
            Ok(val)
        },
        Err(err) => {
            // Capture locals of this environment:
            let mut frame_ref = self.call_stack.borrow_mut();
            if let Some(last_frame) = frame_ref.last_mut() {
                let vars = call_env.borrow().values.clone();
                last_frame.locals = Some(vars); 
            }
            // Do NOT pop the frame here, leave it on stack to report error
            Err(err)
        }
    }
    ```
    This ensures that if an error propagates, the current function’s frame stays in the stack with its locals recorded. If the function returns successfully, we pop it as usual. We do similar wrapping for calls to the `main` function (if one is defined) after interpreting global statements ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=if%20let%20Some,errors.push%28err%29%2C%20%7D)) – that is also using `call_function`, so it’s covered.

  - In other parts of the interpreter, like `execute_statement` and `evaluate_expression`, we typically propagate errors with the `?` operator. After our modifications, as soon as an error occurs deep inside, it will bubble up through `?` operators, skipping normal execution. To prevent multiple errors, we will alter `interpret` to stop on the first error. In the current code, `interpret` collects errors in a vector and continues the loop ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=match%20self%20,errors.push%28err%29%2C)). We will change this behavior: when an `Err(err)` occurs, instead of pushing and continuing, we break out of the loop immediately. For example:
    ```rust
    for statement in program.statements {
        if let Err(err) = self.execute_statement(stmt, Rc::clone(&self.global_env)).await {
            errors.push(err);
            break;  // stop execution on first runtime error
        }
    }
    ```
    This way, we don’t attempt to run further statements after a failure (which could cause confusing secondary errors or use incomplete state). We still wrap the single error in a vector to return (to conform to the `Result<Value, Vec<RuntimeError>>` type). This change makes runtime error handling more like an exception – once something fails, we unwind. It also means the call stack and environment remain at the error point, which is perfect for our debug report. (If we did not break, subsequent statements would potentially pop frames or change the environment, losing the original context. So breaking is important for accurate debug info.) This is a slight change in semantics (no longer gathering multiple runtime errors), but it aligns with the idea of aborting on failure and providing a report.

  - **Note:** The interpreter’s `RuntimeError` struct remains unchanged (still just message, line, column) ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=,usize%2C%20pub%20column%3A%20usize%2C)). We are not embedding stack or env info inside it, to keep it lightweight for the diagnostic printing. Our call stack and env snapshots are handled separately in the interpreter’s state. Thus, the existing diagnostic reporter (`convert_runtime_error`) continues to work without modification for console output ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=let%20end_offset%20%3D%20start_offset%20%2B,1)). Our additions operate in parallel: they don’t interfere with the codespan error printing, they just use the same data plus additional context.

- **`debug_report.rs` (New Module):** We will create a new module (maybe under `diagnostics` or standalone) to encapsulate report generation. This module could have a function like `pub fn create_report(interpreter: &Interpreter, error: &RuntimeError, source: &str, script_path: &str) -> PathBuf`. It will perform:
  1. Build the stack trace string from `interpreter.call_stack`. For each frame, get its name and call location. The bottom of the stack might correspond to the main script if not an action. We format lines as described (“at action X (called from …)”). This function can also utilize any `parent` links or such if needed, but since we explicitly push frames, the stack itself is enough.
  2. Determine the function (if any) in which the error occurred. This is the last frame of the stack. If the stack is not empty, the last frame’s `func_name` is the function we’re in. We retrieve the `locals` from that frame (populated if it was the one that errored) and format the Local Variables section by iterating over the hashmap. If a value’s debug output is long (like a large list), we will print it in full anyway (assuming that’s acceptable; if not, we could truncate or summarize, but given WFL’s ethos, showing it is fine).
  3. Using the error’s `line` and `column`, extract a snippet of the source around that line. We can do:
     ```rust
     let lines: Vec<&str> = source.lines().collect();
     let err_line_index = error.line - 1; // 0-based index
     for i in err_line_index.saturating_sub(2) ..= err_line_index.saturating_add(2) {
         if i < lines.len() {
             // mark line i+1, maybe with '>>' if i == err_line_index
         }
     }
     ```
     This would gather two lines before and after the error line (adjusting for file bounds). We prefix each with its line number and an arrow for the error line. Also, we can add a marker under the specific column if needed (like printing spaces and a caret `^` under the column position). Since the diagnostic output already highlights the column, we can mimic that textually.
  4. If the last frame (error frame) corresponds to a user function, find the range of source lines for that function. We know the function’s start line from the frame (or from the `FunctionValue` if we had access to it – we might store the function’s defining line in the frame as well when pushing it). We can scan forward from the start line until we either encounter the next `end` or next function definition or reach the error line’s function end. E.g., use the fact that the AST `ActionDefinition` had a known body length, or simpler: if the function’s start line is known, include lines from start to just before the next `action ` or `end action` keyword in the file. This is a bit heuristic; an easier method is to identify the function by name in source (if unique) or use the AST. Since the AST is accessible at parse time, we could potentially store a mapping of function name to source lines when the program is parsed. However, to avoid over-engineering, a reasonable approach is to rely on indent or end markers. Assuming WFL functions are ended with "end" or similar, we can include lines from the start line up to the matching "end". If indent-based, include until dedent. We’ll implement a simple parser in the report generator: start at the function’s start line, and include lines until the indentation level decreases to that of the start (or until an `end` keyword line is found).
  5. Compose the sections: error summary, stack trace, code snippet, function body (if any), and locals. Use clear separators or headings in the text.

  By isolating this in a module, we keep `main.rs` concise and `Interpreter` focused on execution. The report generator can also be tested independently by simulating an interpreter state and an error.

- **REPL (`repl.rs`):** The REPL likely won’t write debug reports to disk, as it’s an interactive session. We might not enable debug reports for REPL usage (or if we do, perhaps we could print the same info to the REPL output instead of saving a file). For now, we can make the debug report feature only active for script execution mode. `repl.rs` can ignore it. The logging system, however, could still be enabled in REPL (if logging is on and a user triggers an error in REPL, it would log to file). This is a minor consideration; we can leave REPL unchanged, or read config in REPL as well (maybe user has a global `.wflcfg`). If we find a `.wflcfg` in the current directory, we could apply the same toggles to the REPL (so if logging_enabled, REPL would also log commands and errors). This can be a future enhancement. The core integration for this task focuses on script execution path.

## Recommendations and Further Considerations

**Separation of Concerns:** The logging and debug-report functionality should be kept as modular as possible. We recommend creating a `logging` module or at least clearly separating logger initialization in `main.rs`, and a dedicated `debug_report` module for report generation. This avoids cluttering the interpreter code with file I/O or formatting logic. The interpreter simply gathers context (stack frames, etc.), and the reporter module handles presenting that context. 

**Testing and Validation:** After implementing, test the system with various scenarios:
- A simple script that triggers a runtime error (e.g., divide by zero, undefined variable) – verify a debug report is created with correct stack, code, and locals.
- Nested function calls where an inner function errors – ensure the stack trace lists all calls and the correct function body is printed.
- Scripts with no errors – ensure no debug file is created, and if logging was on, only normal logs appear.
- Config toggles – turn off debug_report and confirm no file is written on error; turn off logging and confirm no log file or console logs appear (other than normal prints).
- Timeout errors (when execution exceeds `max_duration`) – those produce a `RuntimeError` via `check_time` ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=fn%20check_time%28%26self%29%20,as_secs%28%29%20%29%2C%200%2C%200%2C)) with line 0, col 0. Our system will treat it like any other runtime error. The debug report for a timeout might not have a specific code line (line 0), so it should handle that gracefully (we can state “(no specific code line – execution timed out)” in the report). It’s a edge case to consider in formatting.

**Performance Impact:** The overhead of logging and debug-report generation is minimal in normal operation:
- If logging is off (default), the `log::...` macros are no-ops, so there’s no runtime cost beyond a negligible check. If logging is on, writing to file/console has I/O cost, but that’s acceptable for debugging; in production, one might leave it off for maximum performance.
- Maintaining the call stack (pushing frames on function calls) is very lightweight (just a vector push/pop), and only user-defined actions (which are likely not extremely deep) use it. This has trivial memory cost.
- Snapshooting local variables on error involves cloning a HashMap of variables. Typically there won’t be an enormous number of variables, so this is fine. It only happens on error, not in normal flow.
- Generating the debug report involves reading the source string and writing a file, which only happens on errors. This is perfectly acceptable since the program is anyway crashing at that point – the user is waiting for error info, so spending a few milliseconds to produce a thorough report is desirable.

**Future Improvements:**
- We might allow customizing the log file path or name via config (e.g., `log_file = "myrun.log"`). For now, using a default name is fine.
- We should ensure that sensitive information is not inadvertently exposed in logs/reports. For example, if the program had secrets in variables, the debug report will dump them. Given WFL is a general language, we assume this is acceptable for debugging, but in a multi-user environment one might redact certain values. The documentation suggests consideration of not exposing passwords in error messages ([docs.md](file://file-X3pW9xWh9qAZ3dJVa189Mb#:~:text=logging%20exceptions%2C%20we%20do%20not,to%20not%20show%20the%20password)), so if WFL had known sensitive data types, we might sanitize them in the report.
- Once the `try/when` error handling is implemented (as indicated by the AST node ([src.md](file://file-JyLaaKcYuXrvPoh9mDHJUw#:~:text=TryStatement%20%7B%20body%3A%20Vec,line%3A%20usize%2C%20column%3A%20usize%2C))), we should revisit the debug report trigger. If an error is caught by user code, we likely should not generate a report (as the program handled it). Our implementation will naturally skip because the error won’t propagate out to `main.rs`. But if we wanted to support optional manual debug dumps, we could provide a library function to dump a report on demand (not required now).
- We could integrate the debug report output into an IDE or web interface (since WFL is “WebFirst”, maybe in a web context the report could be shown in-browser). Having it as a text file is a first step; integration with a UI could follow.

**Maintaining WFL Style:** We will ensure all messages, whether in logs or the debug report, are phrased clearly. For example, in the stack trace we use “action” to refer to functions (matching WFL terminology for ActionDefinition), and in the code snippet we annotate lines in an intuitive way. The overall experience should feel like WFL is guiding the user through the error – the logs tell the story of execution, and the debug report gives a post-mortem explanation with all the evidence (code and data) needed to solve the problem.

By implementing these changes, we add robust debugging support to WFL. Developers can enable logging to trace execution and get insight into performance and flow. And when things go wrong, the automatic debug report will greatly simplify pinpointing the cause by providing a comprehensive snapshot. This will make the WFL development experience more transparent and error-friendly, living up to the language’s goals of clarity and approachability.