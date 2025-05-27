# W F L : AI Contributor Playbook

*(Because even silicon interns need rules before lighting the build pipeline on fire.)*

---

## 1. Prime Directives

1. **Fail Fast, Log Loud** – Treat every warning from `cargo`, Clippy, or the WFL linter as a TODO, not background music.
2. **Specs > Spaghetti** – Never start coding without a short spec in `/Dev diary/-YYYY-MM-DD-<topic>.md`. This is your “what/why/edge-cases” brain-dump; it lives **outside** `Docs/`.
3. **Tests First, Ego Later** – Each bug-fix or feature must ship with at least one regression-test (unit, integration, snapshot, or memory) so we never re-learn the same lesson twice. See the project’s snapshot and memory-test conventions for inspiration .
4. **Docs or It Didn’t Happen** – Any public-facing change (syntax, flag, std-lib call, etc.) requires an update to the relevant `.md` in `Docs/` plus the CHANGELOG.
5. **One Flag, One Commit** – Keep pull requests small and focused; reviewers prefer tapas over Thanksgiving dinner.

---

## 2. Daily Development Checklist

1. **Create /Dev diary entry** with date, intent, and acceptance criteria.
2. **Write or extend tests** that fail without the change.
3. **Code the feature / fix**, running:

   ```text
   $ wfl --lint --fix --diff <file>     # style police
   $ wfl --analyze <file>               # static sanity
   ```
4. **Run full test suite** (`cargo test && ./scripts/run_wfl_tests.sh`).
5. **Update docs** and bump examples.
6. **Commit** with conventional message (`fix:`, `feat:`, `docs:`, etc.).
7. **Push & open PR**; attach screenshots or `*_debug.txt` if fixing a bug.

---

## 3. Standard Debug Procedure

*(Follow in exact order—no “YOLO printf” until Step 4.)*

| Step | What to run                                                                                       | Purpose                                                                         |
| ---- | ------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------- |
| 1️⃣  | `wfl --lex script.wfl`                                                                            | Ensure tokenizer isn’t tripping over Unicode ghosts. Produces `script.lex.txt`  |
| 2️⃣  | `wfl --ast script.wfl`                                                                            | Confirm the AST is sane before deeper voodoo                                    |
| 3️⃣  | `wfl --analyze script.wfl`                                                                        | Static analysis: dead code, unused vars, type weirdness                         |
| 4️⃣  | `wfl --step script.wfl`                                                                           | Interactive run; walk statement-by-statement                                    |
| 5️⃣  | Enable `execution_logging = true` in `.wflcfg` for a time-stamped replay if the bug is slippery   |                                                                                 |
| 6️⃣  | If runtime panic, grab the auto-generated `*_debug.txt` attachment and add it to the issue queue. |                                                                                 |

*Golden rule:* **Never** poke at interpreter internals until Lex + AST are green.

---

## 4. Complete `wfl` CLI Flag Reference

| Flag                  | Function                                  | Quirks / Notes                                                               |
| --------------------- | ----------------------------------------- | ---------------------------------------------------------------------------- |
| `--help`              | Show built-in help text                   | Source of truth in `main.rs`                                                 |
| `--version`           | Print current version constant            |                                                                              |
| `--lint <file>`       | Style & structural checks                 | Can pair with `--fix`                                                        |
| `--fix`               | Auto-apply linter suggestions             | **Only valid after `--lint`;** add `--in-place` or `--diff` for output mode  |
| `--in-place`          | Overwrite source after `--lint --fix`     | Forbidden in CI                                                              |
| `--diff`              | Show unified diff instead of writing file |                                                                              |
| `--analyze <file>`    | Static (semantic) analyser                | Mutually exclusive with lint/fix modes                                       |
| `--step`              | Interactive, step-by-step execution       | Blocks for user input; don’t script in CI                                    |
| `--edit <file>`       | Open file in system editor                | Solo flag—no buddies allowed                                                 |
| `--lex <file>`        | Dump lexer tokens to `<name>.lex.txt`     | Great for weird encoding bugs                                                |
| `--ast <file>`        | Dump AST to `<name>.ast.txt`              | Use with a long-line-friendly viewer                                         |
| `--configCheck [dir]` | Validate `.wflcfg` files                  | Can’t mix with lint/analyze/fix                                              |
| `--configFix [dir]`   | Auto-repair common config issues          | Same mutual-exclusion rules                                                  |

---

## 5. Regression Safety Net

* **Unit tests** for every parser, linter, and analyzer rule.
* **Snapshot tests** for diagnostics—fail if wording/layout drifts unintentionally .
* **Memory tests** behind the `dhat-heap` feature where leaks are suspected.
* Gate all of the above in CI so no green tick ➜ no merge.

---

## 6. Developer Diary Etiquette

* Location: `/Dev diary/` (sibling to `src/` and `Docs/`).
* Filename: `YYYY-MM-DD-<short-topic>.md`.
* Content template:

  ```markdown
  ### Goal  
  Short bullet describing the feature/bug.

  ### Approach  
  Why this design beats the alternatives.

  ### Gotchas  
  Edge-cases, perf concerns, open questions.

  ### Outcome  
  Links to PR, tests added, docs updated.
  ```

The diary is informal but mandatory—future contributors should understand *why* you touched that gnarly code path.

---

### Remember

> **Lex first, AST second, tests always, diary forever.**
> Break any of these and future-you will show up from the timeline, wearing disappointment and wielding `git blame`.
