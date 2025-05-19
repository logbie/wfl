# PowerShell script to install VS Code extension
param (
    [string]$InstallDir
)

$ErrorActionPreference = "Stop"
$extensionPath = Join-Path $InstallDir "vscode-extension"

# Check if VS Code is installed
$vscodePaths = @(
    "${env:ProgramFiles}\Microsoft VS Code\bin\code.cmd",
    "${env:ProgramFiles(x86)}\Microsoft VS Code\bin\code.cmd",
    "${env:LOCALAPPDATA}\Programs\Microsoft VS Code\bin\code.cmd"
)

$vscodeCmd = $null
foreach ($path in $vscodePaths) {
    if (Test-Path $path) {
        $vscodeCmd = $path
        break
    }
}

if ($vscodeCmd) {
    Write-Host "Installing VS Code extension from $extensionPath"
    
    # Create a temporary VSIX package
    $tempDir = [System.IO.Path]::GetTempPath() + [System.Guid]::NewGuid().ToString()
    New-Item -ItemType Directory -Path $tempDir | Out-Null
    
    # Copy extension files to temp directory
    Copy-Item -Path "$extensionPath\*" -Destination $tempDir -Recurse
    
    # Use VS Code to install the extension
    & $vscodeCmd --install-extension "$tempDir" --force
    
    # Clean up
    Remove-Item -Path $tempDir -Recurse -Force
    
    Write-Host "VS Code extension installed successfully"
} else {
    Write-Host "VS Code not found. Extension not installed."
}
