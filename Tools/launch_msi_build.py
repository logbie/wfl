#!/usr/bin/env python3
"""
MSI Build Launcher for WFL

This script launches an MSI build session by coordinating existing build tools:
- Version management using scripts/bump_version.py
- MSI build using build_msi.ps1 
- Documentation updates in implementation_progress files
"""

import argparse
import datetime
import json
import os
import platform
import re
import subprocess
import sys
from pathlib import Path

# Constants
PROJECT_ROOT = Path(os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))
BUMP_VERSION_SCRIPT = PROJECT_ROOT / "scripts" / "bump_version.py"
BUILD_MSI_SCRIPT = PROJECT_ROOT / "build_msi.ps1"
BUILD_META_FILE = PROJECT_ROOT / ".build_meta.json"
DOCS_DIR = PROJECT_ROOT / "Docs"

def parse_arguments():
    """Parse command-line arguments."""
    parser = argparse.ArgumentParser(description="Launch an MSI build session for WFL")
    
    # Version control options
    version_group = parser.add_argument_group("Version Management")
    version_group.add_argument("--bump-version", action="store_true", 
                             help="Increment the build number")
    version_group.add_argument("--version-override", 
                             help="Override version (format: YYYY.MM)")
    
    # Build options
    build_group = parser.add_argument_group("Build Options")
    build_group.add_argument("--output-dir", 
                           help="Custom output directory for the MSI file")
    build_group.add_argument("--skip-tests", action="store_true",
                           help="Skip running tests before building")
    
    # Output options
    output_group = parser.add_argument_group("Output Options")
    output_group.add_argument("--verbose", action="store_true",
                            help="Show detailed output")
    
    return parser.parse_args()

def check_windows():
    """Check if running on Windows."""
    if platform.system() != "Windows":
        print("Error: This script only supports Windows.")
        print("The WFL MSI build process requires the WiX Toolset, which is Windows-only.")
        sys.exit(1)

def get_current_version():
    """Get the current version from .build_meta.json."""
    try:
        with open(BUILD_META_FILE, "r") as f:
            meta = json.load(f)
        return f"{meta.get('year', datetime.datetime.now().year)}.{meta.get('build', 0)}"
    except (FileNotFoundError, json.JSONDecodeError) as e:
        print(f"Error reading version from {BUILD_META_FILE}: {e}")
        sys.exit(1)

def run_version_update(bump=False, override=None):
    """Run the version update script with appropriate arguments."""
    cmd = [sys.executable, str(BUMP_VERSION_SCRIPT)]
    
    if not bump:
        cmd.append("--skip-bump")
    
    if override:
        print(f"Using version override: {override}")
        # We'll need to manually update the build metadata
        try:
            with open(BUILD_META_FILE, "r") as f:
                meta = json.load(f)
            
            parts = override.split(".")
            if len(parts) >= 2:
                meta["year"] = int(parts[0])
                meta["build"] = int(parts[1])
                
                with open(BUILD_META_FILE, "w") as f:
                    json.dump(meta, f, indent=2)
        except Exception as e:
            print(f"Error updating version metadata: {e}")
            return False
    
    cmd.extend(["--update-all", "--skip-git"])
    
    print(f"Running: {' '.join(cmd)}")
    result = subprocess.run(cmd, check=False)
    return result.returncode == 0

def run_msi_build(args):
    """Run the MSI build process using PowerShell."""
    # Ensure we run the command from the project root directory
    cmd = ["powershell", "-ExecutionPolicy", "Bypass", "-File", str(BUILD_MSI_SCRIPT)]
    
    # Add output directory parameter if specified
    if args.output_dir:
        output_dir = os.path.abspath(args.output_dir)
        cmd.extend(["-OutputDir", output_dir])
        print(f"Using custom output directory: {output_dir}")
    
    if args.verbose:
        cmd.append("-Verbose")
        print(f"Running: {' '.join(cmd)}")
    
    # Change to the project root directory before running the build
    current_dir = os.getcwd()
    os.chdir(str(PROJECT_ROOT))
    
    try:
        result = subprocess.run(cmd, check=False)
        success = result.returncode == 0
    finally:
        # Restore the original directory
        os.chdir(current_dir)
    
    return success

def update_progress_doc(version, success, output_path=None):
    """Update the implementation progress document for today's date."""
    today = datetime.datetime.now().strftime("%Y-%m-%d")
    progress_file = DOCS_DIR / f"implementation_progress_{today}.md"
    
    # Create file if it doesn't exist
    if not progress_file.exists():
        with open(progress_file, "w") as f:
            f.write(f"# Implementation Progress - {today}\n\n")
    
    # Determine default output path if none provided
    if output_path is None:
        output_path = f"target/x86_64-pc-windows-msvc/release/wfl-{version}.msi"
    
    # Append build information
    with open(progress_file, "a", encoding="utf-8") as f:
        timestamp = datetime.datetime.now().strftime("%H:%M:%S")
        # Use plain text status instead of emoji to avoid encoding issues
        status = "SUCCESS" if success else "FAILED"
        f.write(f"\n## MSI Build - {timestamp}\n\n")
        f.write(f"- Version: {version}\n")
        f.write(f"- Status: {status}\n")
        
        if success:
            f.write(f"- Output: `{output_path}`\n")
        
        f.write("\n")
    
    print(f"Updated progress in {progress_file}")
    return True

def main():
    """Main entry point."""
    args = parse_arguments()
    
    # Check if running on Windows
    check_windows()
    
    print("=== WFL MSI Build Launcher ===")
    
    # Handle version updates
    if args.bump_version or args.version_override:
        print("\n=== Updating Version Information ===")
        if not run_version_update(args.bump_version, args.version_override):
            print("Error: Version update failed")
            sys.exit(1)
    
    # Get the current version for documentation
    version = get_current_version()
    print(f"\nBuilding WFL version: {version}")
    
    # Run the MSI build
    print("\n=== Starting MSI Build Process ===")
    build_success = run_msi_build(args)
    
    # Determine output path for reporting
    output_path = f"target/x86_64-pc-windows-msvc/release/wfl-{version}.msi"
    if args.output_dir:
        output_dir = os.path.abspath(args.output_dir)
        output_path = os.path.join(output_dir, f"wfl-{version}.msi")
    
    # Update progress documentation
    update_progress_doc(version, build_success, output_path)
    
    # Report result
    if build_success:
        print("\n[SUCCESS] MSI Build Completed Successfully")
        print(f"Output: {output_path}")
        sys.exit(0)
    else:
        print("\n[FAILED] MSI Build Failed")
        print("Check the output above for errors")
        sys.exit(1)

if __name__ == "__main__":
    main()
