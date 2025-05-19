# PowerShell script to initialize WFL configuration
param (
    [string]$InstallDir
)

$ErrorActionPreference = "Stop"
$configDir = Join-Path $InstallDir "config"
$configFile = Join-Path $configDir "wfl.cfg"

# Ensure config directory exists
if (-not (Test-Path $configDir)) {
    New-Item -ItemType Directory -Path $configDir | Out-Null
}

# Check if config file already exists
if (-not (Test-Path $configFile)) {
    # Create default config file
    @"
# WFL Configuration File
# Created by WFL installer

# General Runtime Settings
timeout_seconds = 60
logging_enabled = false
debug_report_enabled = true
log_level = info

# Code Quality/Linter Settings
max_line_length = 100
max_nesting_depth = 5
indent_size = 4
snake_case_variables = true
trailing_whitespace = false
consistent_keyword_case = true
"@ | Out-File -FilePath $configFile -Encoding utf8
    
    Write-Host "Created default configuration file at $configFile"
} else {
    Write-Host "Configuration file already exists at $configFile"
}

# Run configfix to initialize the config file
$wflExe = Join-Path (Join-Path $InstallDir "bin") "wfl.exe"
if (Test-Path $wflExe) {
    & $wflExe configfix
    Write-Host "Ran configfix to initialize configuration"
} else {
    Write-Host "WFL executable not found at $wflExe"
}
