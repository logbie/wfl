#!/usr/bin/env python3
"""
WFL Configuration Checker

This tool checks that all WFL configuration files (.wflcfg etc.) have been created
and are set correctly. It validates file existence, content, and provides 
recommendations for fixes.
"""

import os
import sys
import platform
from dataclasses import dataclass
from typing import Dict, List, Optional, Set, Tuple
from datetime import datetime

# Define config file locations
DEFAULT_GLOBAL_CONFIG_PATHS = {
    "Windows": "C:\\wfl\\config",
    "Linux": "/etc/wfl/wfl.cfg",
    "Darwin": "/etc/wfl/wfl.cfg",  # macOS
}

# Define expected settings with their types and whether they're required
EXPECTED_SETTINGS = {
    "timeout_seconds": {"type": int, "required": False, "default": 60},
    "logging_enabled": {"type": bool, "required": False, "default": False},
    "debug_report_enabled": {"type": bool, "required": False, "default": True},
    "log_level": {"type": str, "required": False, "default": "info", "valid_values": ["debug", "info", "warn", "error"]},
    "execution_logging": {"type": bool, "required": False, "default": False},
    "max_line_length": {"type": int, "required": False, "default": 100},
    "max_nesting_depth": {"type": int, "required": False, "default": 5},
    "indent_size": {"type": int, "required": False, "default": 4},
    "snake_case_variables": {"type": bool, "required": False, "default": True},
    "trailing_whitespace": {"type": bool, "required": False, "default": False},
    "consistent_keyword_case": {"type": bool, "required": False, "default": True},
}

@dataclass
class ConfigIssue:
    """Represents an issue with a configuration file"""
    file_path: str
    issue_type: str  # 'missing_file', 'missing_setting', 'invalid_value', etc.
    setting_name: Optional[str] = None
    current_value: Optional[str] = None
    expected_value: Optional[str] = None
    message: Optional[str] = None

def get_global_config_path() -> str:
    """Get the platform-specific path to the global config file"""
    # Check for environment variable first
    if "WFL_GLOBAL_CONFIG_PATH" in os.environ:
        return os.environ["WFL_GLOBAL_CONFIG_PATH"]
    
    # Otherwise use default for the current platform
    system = platform.system()
    return DEFAULT_GLOBAL_CONFIG_PATHS.get(system, "/etc/wfl/wfl.cfg")

def find_config_files(start_dir: str) -> List[str]:
    """Find all .wflcfg files in the given directory and its subdirectories"""
    config_files = []
    for root, _, files in os.walk(start_dir):
        for file in files:
            if file == ".wflcfg":
                config_files.append(os.path.join(root, file))
    return config_files

def parse_config_file(file_path: str) -> Dict[str, str]:
    """Parse a .wflcfg file and return a dictionary of settings"""
    settings = {}
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            for line in f:
                line = line.strip()
                if not line or line.startswith('#'):
                    continue
                if '=' in line:
                    key, value = line.split('=', 1)
                    settings[key.strip()] = value.strip()
    except Exception as e:
        print(f"Error parsing {file_path}: {e}")
    return settings

def validate_config_file(file_path: str, settings: Dict[str, str]) -> List[ConfigIssue]:
    """Validate the settings in a config file and return a list of issues"""
    issues = []
    
    # Check for required settings
    for setting_name, properties in EXPECTED_SETTINGS.items():
        if properties["required"] and setting_name not in settings:
            issues.append(ConfigIssue(
                file_path=file_path,
                issue_type="missing_setting",
                setting_name=setting_name,
                expected_value=str(properties["default"]),
                message=f"Required setting '{setting_name}' is missing"
            ))
    
    # Validate existing settings
    for setting_name, setting_value in settings.items():
        if setting_name in EXPECTED_SETTINGS:
            properties = EXPECTED_SETTINGS[setting_name]
            
            # Check type
            valid = True
            if properties["type"] == bool:
                valid = setting_value.lower() in ["true", "false"]
            elif properties["type"] == int:
                try:
                    int(setting_value)
                except ValueError:
                    valid = False
            
            # Check valid values if specified
            if valid and "valid_values" in properties:
                valid = setting_value.lower() in properties["valid_values"]
            
            if not valid:
                issues.append(ConfigIssue(
                    file_path=file_path,
                    issue_type="invalid_value",
                    setting_name=setting_name,
                    current_value=setting_value,
                    expected_value=str(properties.get("default", "")),
                    message=f"Invalid value for '{setting_name}'"
                ))
        else:
            # Unknown setting
            issues.append(ConfigIssue(
                file_path=file_path,
                issue_type="unknown_setting",
                setting_name=setting_name,
                current_value=setting_value,
                message=f"Unknown setting '{setting_name}'"
            ))
    
    return issues

def check_global_config() -> List[ConfigIssue]:
    """Check the global configuration file"""
    issues = []
    global_path = get_global_config_path()
    
    if not os.path.exists(global_path):
        issues.append(ConfigIssue(
            file_path=global_path,
            issue_type="missing_file",
            message=f"Global configuration file not found"
        ))
        return issues
    
    settings = parse_config_file(global_path)
    file_issues = validate_config_file(global_path, settings)
    issues.extend(file_issues)
    
    return issues

def check_project_configs(start_dir: str) -> Dict[str, List[ConfigIssue]]:
    """Check all .wflcfg files in the project"""
    results = {}
    
    # Check the root directory first
    root_config = os.path.join(start_dir, ".wflcfg")
    if not os.path.exists(root_config):
        results[root_config] = [ConfigIssue(
            file_path=root_config,
            issue_type="missing_file",
            message="Root configuration file not found"
        )]
    else:
        settings = parse_config_file(root_config)
        results[root_config] = validate_config_file(root_config, settings)
    
    # Check subdirectories
    for config_file in find_config_files(start_dir):
        if config_file != root_config:  # Skip root config as we've already checked it
            settings = parse_config_file(config_file)
            results[config_file] = validate_config_file(config_file, settings)
    
    return results

def generate_report(global_issues: List[ConfigIssue], project_issues: Dict[str, List[ConfigIssue]]) -> str:
    """Generate a formatted report of configuration issues"""
    report = []
    report.append("=" * 80)
    report.append("WFL CONFIGURATION CHECK REPORT")
    report.append(f"Generated on: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    report.append("=" * 80)
    
    # Global configuration
    report.append("\nGLOBAL CONFIGURATION:")
    if not global_issues:
        report.append("✅ No issues found with global configuration")
    else:
        for issue in global_issues:
            report.append(f"❌ {issue.message}")
    
    # Project configurations
    report.append("\nPROJECT CONFIGURATIONS:")
    all_ok = True
    
    for file_path, issues in project_issues.items():
        rel_path = os.path.relpath(file_path)
        if not issues:
            report.append(f"✅ {rel_path}: No issues found")
        else:
            all_ok = False
            report.append(f"❌ {rel_path}:")
            for issue in issues:
                if issue.issue_type == "missing_file":
                    report.append(f"  - File not found")
                elif issue.issue_type == "missing_setting":
                    report.append(f"  - Missing required setting: {issue.setting_name}")
                elif issue.issue_type == "invalid_value":
                    report.append(f"  - Invalid value for {issue.setting_name}: {issue.current_value}")
                elif issue.issue_type == "unknown_setting":
                    report.append(f"  - Unknown setting: {issue.setting_name}")
                else:
                    report.append(f"  - {issue.message}")
    
    if all_ok:
        report.append("✅ All project configuration files are valid")
    
    # Summary
    total_issues = len(global_issues) + sum(len(issues) for issues in project_issues.values())
    report.append("\nSUMMARY:")
    report.append(f"Total files checked: {len(project_issues) + 1}")
    report.append(f"Total issues found: {total_issues}")
    
    if total_issues > 0:
        report.append("\nRECOMMENDATIONS:")
        report.append("Run this tool with the --fix flag to automatically fix issues")
    
    return "\n".join(report)

def fix_issues(global_issues: List[ConfigIssue], project_issues: Dict[str, List[ConfigIssue]]) -> int:
    """Attempt to fix configuration issues, returns count of fixed issues"""
    fixed_count = 0
    
    # Fix global issues first
    global_path = get_global_config_path()
    global_settings = {}
    if os.path.exists(global_path):
        global_settings = parse_config_file(global_path)
    
    global_fixed = False
    for issue in global_issues:
        if issue.issue_type == "missing_file":
            # Create directory if needed
            os.makedirs(os.path.dirname(global_path), exist_ok=True)
            with open(global_path, 'w') as f:
                f.write("# WFL Global Configuration\n")
            global_fixed = True
            fixed_count += 1
        elif issue.issue_type in ["missing_setting", "invalid_value"]:
            if issue.setting_name and issue.expected_value:
                global_settings[issue.setting_name] = issue.expected_value
                global_fixed = True
                fixed_count += 1
    
    if global_fixed and global_settings:
        with open(global_path, 'w') as f:
            f.write("# WFL Global Configuration\n")
            for key, value in sorted(global_settings.items()):
                f.write(f"{key} = {value}\n")
    
    # Fix project issues
    for file_path, issues in project_issues.items():
        file_settings = {}
        if os.path.exists(file_path):
            file_settings = parse_config_file(file_path)
        
        file_fixed = False
        for issue in issues:
            if issue.issue_type == "missing_file":
                # Create directory if needed
                os.makedirs(os.path.dirname(file_path), exist_ok=True)
                with open(file_path, 'w') as f:
                    f.write("# WFL Configuration\n")
                file_fixed = True
                fixed_count += 1
            elif issue.issue_type in ["missing_setting", "invalid_value"]:
                if issue.setting_name and issue.expected_value:
                    file_settings[issue.setting_name] = issue.expected_value
                    file_fixed = True
                    fixed_count += 1
            elif issue.issue_type == "unknown_setting":
                if issue.setting_name:
                    # Don't remove unknown settings, just issue a warning
                    pass
        
        if file_fixed and file_settings:
            with open(file_path, 'w') as f:
                f.write("# WFL Configuration\n")
                for key, value in sorted(file_settings.items()):
                    f.write(f"{key} = {value}\n")
    
    return fixed_count

def main():
    """Main function to run the configuration checker"""
    import argparse
    
    parser = argparse.ArgumentParser(description="Check WFL configuration files")
    parser.add_argument("--project-dir", "-d", default=".", help="Project directory to check")
    parser.add_argument("--fix", action="store_true", help="Automatically fix issues")
    parser.add_argument("--verbose", "-v", action="store_true", help="Show detailed information")
    args = parser.parse_args()
    
    project_dir = os.path.abspath(args.project_dir)
    print(f"Checking WFL configuration in {project_dir}...")
    
    # Check global configuration
    global_issues = check_global_config()
    
    # Check project configurations
    project_issues = check_project_configs(project_dir)
    
    # Generate and print the report
    report = generate_report(global_issues, project_issues)
    print(report)
    
    # Fix issues if requested
    if args.fix:
        fixed_count = fix_issues(global_issues, project_issues)
        print(f"\nFixed {fixed_count} issues")
    
    # Return success only if no issues were found
    total_issues = len(global_issues) + sum(len(issues) for issues in project_issues.values())
    return 0 if total_issues == 0 else 1

if __name__ == "__main__":
    sys.exit(main())
