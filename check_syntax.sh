

if [ $# -eq 0 ]; then
    echo "Usage: $0 <file_or_directory> [file_or_directory ...]"
    echo "Examples:"
    echo "  $0 Test\\ Programs/test.wfl"
    echo "  $0 Test\\ Programs/"
    exit 1
fi

for arg in "$@"; do
    if [ -d "$arg" ]; then
        echo "Checking all .wfl files in directory: $arg"
        find "$arg" -name "*.wfl" -type f -exec cargo run --bin syntax_checker -- {} \;
    elif [ -f "$arg" ]; then
        echo "Checking file: $arg"
        cargo run --bin syntax_checker -- "$arg"
    else
        echo "Error: $arg is not a valid file or directory"
    fi
done
