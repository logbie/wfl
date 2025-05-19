#!/usr/bin/env python3
import json
import datetime
import os
import subprocess
import sys

BUILD_META_FILE = ".build_meta.json"
VERSION_FILE = "src/version.rs"

def main():
    if not os.path.exists(BUILD_META_FILE):
        print(f"Error: {BUILD_META_FILE} not found")
        sys.exit(1)
    
    with open(BUILD_META_FILE, "r") as f:
        try:
            meta = json.load(f)
        except json.JSONDecodeError:
            print(f"Error: {BUILD_META_FILE} is not valid JSON")
            sys.exit(1)
    
    current_year = datetime.datetime.now().year
    build_num = meta.get("build", 0)
    last_year = meta.get("year", current_year)
    
    if current_year != last_year:
        build_num = 1
        meta["year"] = current_year
    else:
        build_num += 1
    
    meta["build"] = build_num
    
    new_version = f"{current_year}.{build_num}"
    print(f"New version: {new_version}")
    
    with open(BUILD_META_FILE, "w") as f:
        json.dump(meta, f, indent=2)
    
    os.makedirs(os.path.dirname(VERSION_FILE), exist_ok=True)
    
    with open(VERSION_FILE, "w") as vf:
        vf.write(f'pub const VERSION: &str = "{new_version}";\n')
    
    try:
        subprocess.run(["git", "config", "user.name", "github-actions"], check=True)
        subprocess.run(["git", "config", "user.email", "github-actions@github.com"], check=True)
        subprocess.run(["git", "add", BUILD_META_FILE, VERSION_FILE], check=True)
        commit_msg = f"Bump version to {new_version} [skip ci]"
        subprocess.run(["git", "commit", "-m", commit_msg], check=True)
    except subprocess.CalledProcessError as e:
        print(f"Error during git operations: {e}")
        sys.exit(1)
    
    return 0

if __name__ == "__main__":
    sys.exit(main())
