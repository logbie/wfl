# WFL Deployment Guide

This document outlines the deployment process for WFL (WebFirst Language), specifically focusing on building Windows MSI installers.

## Overview

The WFL project uses the WiX Toolset and cargo-wix to build Windows MSI installers. This document explains the prerequisites, build process, common issues, and best practices based on our experiences.

## Prerequisites

### Required Software

1. **Rust and Cargo**: The latest stable version is recommended
2. **WiX Toolset**: Version 3.14 or higher
   - Installation options:
     - Via Chocolatey: `choco install wixtoolset -y` (requires admin privileges)
     - Direct download from [wixtoolset.org/releases](https://wixtoolset.org/releases/)
3. **cargo-wix**: Rust crate for integrating with WiX Toolset
   - Install using: `cargo install cargo-wix@0.3.3 --locked`

### Environment Setup

- Ensure WiX Toolset's bin directory is in your PATH
- If using GitHub Actions, the workflow in `.github/workflows/nightly.yml` provides a reference implementation

## Build Process

The MSI build process consists of these steps:

1. **Dependency Checking**: Verify all required tools are installed
2. **Configuration**: Create necessary config files
3. **WiX Source Generation**: Generate WiX source files if not present
4. **Version Configuration**: Set correct version in wix.toml
5. **Binary Building**: Build the WFL executable(s)
6. **MSI Creation**: Package everything into the MSI installer

The `build_msi.ps1` script in the root directory automates this process.

## Key Files

- **build_msi.ps1**: Main script for building the MSI installer
- **wix.toml**: WiX configuration file for the project
- **.wflcfg**: Default configuration file included in the installer
- **wix/main.wxs**: WiX source file defining the installer structure
- **wix/License.rtf**: License file included in the installer

## Common Issues and Solutions

### 1. Parameter Conflicts

**Issue**: Using `--define` with cargo-wix can conflict with existing variables in WiX source files.
**Solution**: Directly update the `wix.toml` file with the desired version instead of passing it via command line.

Example:
```powershell
# Update version in wix.toml
$wixTomlContent = Get-Content -Path "wix.toml" -Raw
$updatedWixTomlContent = $wixTomlContent -replace 'version = "0.0.0.0"', 'version = "2025.4.0.0"'
Set-Content -Path "wix.toml" -Value $updatedWixTomlContent
```

### 2. Missing Binaries

**Issue**: cargo-wix reports "The system cannot find the file..." when the binary doesn't exist.
**Solution**: Always build the binary before running cargo-wix, or remove the `--no-build` option.

```powershell
# Check and build the binary if necessary
if (-not (Test-Path "target/release/wfl.exe")) {
    cargo build --release -p wfl
}
```

### 3. Workspace Projects

**Issue**: cargo-wix needs to know which package to build in a workspace project.
**Solution**: Always specify the package with `-p <package_name>`.

```powershell
cargo wix --nocapture -p wfl
```

### 4. Missing WiX Source Files

**Issue**: No WXS files available to create the installer.
**Solution**: Initialize WiX source files first using `cargo wix init`.

```powershell
if (-not (Test-Path "wix")) {
    cargo wix init -p wfl
}
```

## Versioning

The WFL project uses the following versioning scheme:

- Release versions: `YYYY.MM.patch.build` (e.g., `2025.4.0.0`)
- The version should be set in both `Cargo.toml` and `wix.toml`
- For automated builds, extract the version from `.build_meta.json` or set it via environment variables

## Best Practices

1. **Separate Configuration**: Store default configuration in a separate file (.wflcfg)
2. **Dependency Checking**: Always verify dependencies before building
3. **Error Handling**: Provide clear error messages and recovery options
4. **Doc Generation**: Update documentation as part of the build process
5. **Build Artifacts**: Store build artifacts in a structured directory hierarchy
6. **CI Integration**: Use GitHub Actions to automate builds for each release
7. **Testing**: Test the installer in a clean environment to verify all components are correctly included

## MSI Features

The WFL MSI installer includes the following features:

- WFL compiler and runtime
- Default configuration file
- VS Code extension (optional)
- PATH environment variable integration
- Customizable installation directory
- Support for 64-bit Windows systems

## Future Improvements

1. **Digital Signing**: Implement code signing for the MSI installer
2. **Multi-platform Packages**: Extend build process to create Debian and macOS packages
3. **Auto-Updates**: Add mechanism for checking and applying updates
4. **Telemetry Option**: Allow users to opt in/out of anonymous usage statistics
5. **Custom Actions**: Add post-install configuration steps

## References

- [WiX Toolset Documentation](https://wixtoolset.org/docs/)
- [cargo-wix Repository](https://github.com/volks73/cargo-wix)
- [Windows Installer Documentation](https://docs.microsoft.com/en-us/windows/desktop/msi/windows-installer-portal)
- [Rust Packaging Guide](https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html)
