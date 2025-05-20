# WFL Tools

This directory contains utility tools for the WFL project.

## Available Tools

### MSI Build Launcher (`launch_msi_build.py`)

A utility for launching MSI build sessions for the WFL project.

#### Features
- Coordinates version management using `scripts/bump_version.py`
- Executes the MSI build process using `build_msi.ps1`
- Creates Windows MSI installer with .wfl file associations
- Automatically updates documentation in implementation progress files
- Provides clear feedback on build success/failure

#### Usage
```bash
python launch_msi_build.py [options]
```

Options:
- `--bump-version`: Increment the build number
- `--version-override VALUE`: Override the version number (format: YYYY.MM)
- `--output-dir DIR`: Specify custom output directory for the MSI file
- `--skip-tests`: Skip running tests before building
- `--verbose`: Show detailed output during execution

#### Examples

Launch a build with the current version:
```bash
python launch_msi_build.py
```

Launch a build with an incremented version number:
```bash
python launch_msi_build.py --bump-version
```

Launch a build with a specific version:
```bash
python launch_msi_build.py --version-override 2025.6
```

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
