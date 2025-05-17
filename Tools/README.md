# WFL Tools

This directory contains utility tools for the WFL project.

## Available Tools

### WFL Configuration Checker (`wfl_config_checker.py`)

A utility for checking and fixing WFL configuration files.

#### Features
- Checks for existence and correctness of all `.wflcfg` files
- Validates configuration settings against expected types and values
- Provides detailed reports of any issues found
- Can automatically fix common configuration issues

#### Usage
```bash
python wfl_config_checker.py [options]
```

Options:
- `--project-dir DIR, -d DIR`: Specify project directory to check (default: current directory)
- `--fix`: Automatically fix issues found (creates missing files and corrects invalid settings)
- `--verbose, -v`: Show detailed information during execution

#### Examples

Check configuration in current directory:
```bash
python wfl_config_checker.py
```

Check configuration in a specific directory and fix issues:
```bash
python wfl_config_checker.py --project-dir /path/to/wfl --fix
```

### Rust Line Counter (`rust_loc_counter.py`)

A utility for counting lines of Rust code in the project.

#### Features
- Counts total lines, code lines, comments, and blank lines
- Provides a breakdown by directory and file
- Generates a formatted report as markdown

#### Usage
```bash
python rust_loc_counter.py
```

### WFL Markdown Combiner (`wfl_md_combiner.py`)

A utility for combining markdown files into a single document.

#### Usage
See the script's internal documentation for details.
