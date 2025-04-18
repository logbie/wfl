#!/usr/bin/env python3
"""
WFL File Combiner

This script combines multiple markdown (.md) files from the Docs directory or Rust (.rs) files from the src directory into a single markdown file
and a matching text file. Original files are preserved.

The script outputs to both .md and .txt formats by default, unless the --no-txt option is specified.

Author: WFL Team
"""

import os
import sys
import glob
import argparse
from datetime import datetime
import re

def parse_arguments():
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(
        description="WFL File Combiner - Combine multiple files into markdown and text files"
    )
    parser.add_argument(
        "-o", "--output",
        help="Path and filename for the combined output file"
    )
    parser.add_argument(
        "-i", "--input",
        help="Directory containing files (default: based on --type)"
    )
    parser.add_argument(
        "--type",
        choices=["docs", "src"],
        help="Type of files to process: 'docs' for markdown files or 'src' for Rust files"
    )
    parser.add_argument(
        "-r", "--recursive",
        action="store_true",
        help="Search subdirectories for files (always enabled for src)"
    )
    parser.add_argument(
        "-t", "--toc",
        action="store_true",
        help="Include table of contents"
    )
    parser.add_argument(
        "-s", "--sort",
        help="Sort files by: 'alpha', 'time', or comma-separated list for custom order"
    )
    parser.add_argument(
        "-l", "--header-level",
        type=int,
        default=1,
        help="Starting level for file headers (default: 1)"
    )
    parser.add_argument(
        "-p", "--separator",
        default=None,
        help="Custom separator between files (default: horizontal rule)"
    )
    parser.add_argument(
        "-a", "--all-files",
        action="store_true",
        help="Include all .md files in Docs, not just those with 'wfl-' in the name"
    )
    parser.add_argument(
        "--no-txt",
        action="store_true",
        help="Disable output to .txt format (by default outputs to both .md and .txt)"
    )
    return parser.parse_args()

def find_files(directory, extension, recursive=False, prefix_filter=False):
    """Find all files with the specified extension in the directory."""
    pattern = os.path.join(directory, "**/*" + extension if recursive else "*" + extension)
    files = glob.glob(pattern, recursive=recursive)
    if prefix_filter:
        files = [f for f in files if "wfl-" in os.path.basename(f)]
    return files

def sort_files(files, sort_option):
    """Sort files based on the provided option."""
    if not sort_option:
        return files
    if sort_option.lower() == 'alpha':
        return sorted(files)
    if sort_option.lower() == 'time':
        return sorted(files, key=os.path.getmtime)
    if ',' in sort_option:
        custom_order = [f.strip() for f in sort_option.split(',')]
        file_map = {os.path.basename(f): f for f in files}
        sorted_files = []
        for name in custom_order:
            if name in file_map:
                sorted_files.append(file_map[name])
                file_map.pop(name)
        sorted_files.extend(file_map.values())
        return sorted_files
    return files

def extract_title(content):
    """Extract title from markdown content."""
    match = re.search(r'^#\s+(.+)$', content, re.MULTILINE)
    if match:
        return match.group(1).strip()
    return None

def generate_toc(files, type, base_level=1):
    """Generate table of contents based on file titles or filenames."""
    toc = ["# Table of Contents\n"]
    for i, file_path in enumerate(files, 1):
        try:
            with open(file_path, 'r', encoding='utf-8') as file:
                content = file.read()
            if type == "docs":
                title = extract_title(content) or os.path.basename(file_path)
            else:  # type == "src"
                title = os.path.basename(file_path)
            indent = "  " * (base_level - 1)
            toc.append(f"{indent}{i}. [{title}](#{i}-{title.lower().replace(' ', '-')})")
        except Exception as e:
            print(f"Warning: Could not process {file_path} for TOC: {e}", file=sys.stderr)
            toc.append(f"{indent}{i}. {os.path.basename(file_path)}")
    return "\n".join(toc) + "\n\n"

def generate_combined_content(files, type, include_toc=False, header_level=1, separator=None):
    """Generate the combined content from files."""
    combined_content = []
    if include_toc:
        combined_content.append(generate_toc(files, type, header_level))
    if separator is None:
        separator = "\n\n---\n\n"
    else:
        separator = f"\n\n{separator}\n\n"
    for i, file_path in enumerate(files, 1):
        try:
            with open(file_path, 'r', encoding='utf-8') as file:
                content = file.read()
            filename = os.path.basename(file_path)
            if type == "docs":
                title = extract_title(content) or filename
                processed_content = re.sub(r'^#\s+.+\n', '', content, count=1, flags=re.MULTILINE).strip()
            else:  # type == "src"
                title = filename
                processed_content = f"```rust\n{content}\n```"
            file_header = '#' * header_level + f" {i}. {title}"
            if i > 1:
                combined_content.append(separator)
            combined_content.append(f"**Start of file: {filename}**")
            combined_content.append("\n\n")
            combined_content.append(file_header)
            combined_content.append("\n\n")
            combined_content.append(processed_content)
            combined_content.append("\n\n")
            combined_content.append(f"**End of file: {filename}**")
        except Exception as e:
            print(f"Error processing file {file_path}: {e}", file=sys.stderr)
            combined_content.append(f"# Error: Could not process {filename}\n\n")
    return "\n".join(combined_content), len(files)

def main():
    """Main function to orchestrate the file combining process."""
    args = parse_arguments()
    project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

    # Prompt user for type if not specified
    if args.type is None:
        while True:
            choice = input("Do you want to process 'docs' or 'src'? ").strip().lower()
            if choice in ["docs", "src"]:
                args.type = choice
                break
            else:
                print("Invalid choice. Please enter 'docs' or 'src'.")

    # Set input directory based on type if not provided
    if args.input is None:
        if args.type == "docs":
            input_dir = os.path.join(project_root, "Docs")
        elif args.type == "src":
            input_dir = os.path.join(project_root, "src")
    else:
        input_dir = args.input

    # Set file extension and filtering based on type
    if args.type == "docs":
        extension = ".md"
        prefix_filter = not args.all_files
    elif args.type == "src":
        extension = ".rs"
        prefix_filter = False

    # Enable recursion for src always, for docs only if -r is specified
    recursive = args.recursive or (args.type == "src")

    print(f"WFL File Combiner - Started at {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print(f"Searching for {extension} files in {input_dir}{' and subdirectories' if recursive else ''}...")
    files = find_files(input_dir, extension, recursive, prefix_filter)
    if not files:
        print(f"No {extension} files found in {input_dir}. Exiting.")
        return
    print(f"Found {len(files)} files.")
    if args.sort:
        print(f"Sorting files using '{args.sort}' method...")
        files = sort_files(files, args.sort)
    print("Combining files...")
    content, count = generate_combined_content(files, args.type, args.toc, args.header_level, args.separator)

    # Set output files, creating combined directory if necessary
    if args.output is None:
        combined_dir = os.path.join(project_root, "combined")
        os.makedirs(combined_dir, exist_ok=True)
        if args.type == "docs":
            md_output_file = os.path.join(combined_dir, "docs.md")
            txt_output_file = os.path.join(combined_dir, "docs.txt")
        elif args.type == "src":
            md_output_file = os.path.join(combined_dir, "src.md")
            txt_output_file = os.path.join(combined_dir, "src.txt")
    else:
        md_output_file = args.output
        # Generate txt output file path by replacing or adding .txt extension
        base, ext = os.path.splitext(args.output)
        txt_output_file = base + ".txt"
    
    # Create directory for output files if it doesn't exist
    output_dir = os.path.dirname(md_output_file)
    if output_dir and not os.path.exists(output_dir):
        os.makedirs(output_dir, exist_ok=True)

    # Write the combined content to the output files
    with open(md_output_file, 'w', encoding='utf-8') as f:
        f.write(content)
    
    # Write to txt file unless --no-txt is specified
    if not args.no_txt:
        with open(txt_output_file, 'w', encoding='utf-8') as f:
            f.write(content)
        print(f"Successfully combined {count} files into {md_output_file} and {txt_output_file}.")
    else:
        print(f"Successfully combined {count} files into {md_output_file}.")
    print(f"WFL File Combiner - Completed at {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")

if __name__ == "__main__":
    main()
