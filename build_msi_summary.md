# WFL MSI Installer Build Process

## What We've Achieved

We've created a comprehensive PowerShell script (`build_msi.ps1`) that automates the process of building an MSI installer for the WFL project. The script:

1. Checks for required dependencies (WiX Toolset, cargo-wix)
2. Creates and configures the necessary config files
3. Generates WiX source files
4. Builds the MSI with the correct version number (2025.4)

## Current Status

The script successfully detects if the required dependencies are present and provides helpful instructions if they are not. Currently, the **WiX Toolset** is not installed on this system, which is a prerequisite for building MSI installers.

## Next Steps

To successfully build the MSI installer, you would need to:

1. Install the WiX Toolset (requires administrator privileges):
   ```powershell
   # Method 1: Using Chocolatey (Run PowerShell as Administrator)
   # Install Chocolatey (if not already installed)
   Set-ExecutionPolicy Bypass -Scope Process -Force
   [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
   Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
   
   # Install WiX Toolset
   choco install wixtoolset -y
   ```
   
   Alternative method:
   ```
   # Method 2: Direct Download
   # Download the installer from https://wixtoolset.org/releases/ and run it
   # After installation, ensure the bin directory is in your PATH environment variable
   ```

2. Run the script again:
   ```
   .\build_msi.ps1
   ```

3. The resulting MSI will be located at:
   ```
   target/x86_64-pc-windows-msvc/release/wfl-2025.4.msi
   ```

## Key Issues Fixed in the Script

1. **Incorrect Parameter Syntax**: Changed `--define Version=2025.4` to `-C "-dVersion=2025.4"`
2. **Missing Package Specification**: Added `-p wfl` to specify which package to build in the workspace
3. **Missing WiX Source Files**: Added step to generate WiX source files using `cargo wix init`
4. **Dependency Checking**: Added comprehensive checks for WiX Toolset and cargo-wix

## Notes on the GitHub Actions Workflow

The GitHub Actions workflow in `.github/workflows/nightly.yml` uses a slightly different approach by:

1. Installing WiX Toolset via Chocolatey
2. Installing cargo-wix with a specific version
3. Setting up the target directory structure
4. Building the MSI with cargo-wix

Our local script is based on this workflow but has been adapted to work better in a manual development environment, with improved error handling and user feedback.
