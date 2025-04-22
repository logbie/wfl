
set -e

if [ $# -lt 1 ]; then
    echo "Usage: $0 <wfl_script_path>"
    exit 1
fi

SCRIPT_PATH="$1"
SCRIPT_NAME=$(basename "$SCRIPT_PATH" .wfl)
OUTPUT_DIR="./heaptrack_results"

mkdir -p "$OUTPUT_DIR"

if ! command -v heaptrack &> /dev/null; then
    echo "Error: heaptrack is not installed. Please install it first."
    echo "On Ubuntu: sudo apt-get install heaptrack"
    exit 1
fi

echo "Running heaptrack on $SCRIPT_PATH..."
HEAPTRACK_OUTPUT="$OUTPUT_DIR/heaptrack.$SCRIPT_NAME.$(date +%Y%m%d_%H%M%S)"

heaptrack --output "$HEAPTRACK_OUTPUT" cargo run --release -- "$SCRIPT_PATH"

LATEST_HEAPTRACK=$(ls -t "$OUTPUT_DIR"/heaptrack.*.gz 2>/dev/null | head -n 1)

if [ -z "$LATEST_HEAPTRACK" ]; then
    echo "Error: No heaptrack output file found."
    exit 1
fi

echo "Heaptrack data saved to: $LATEST_HEAPTRACK"

if command -v heaptrack_print &> /dev/null; then
    echo "Analyzing heaptrack data..."
    SUMMARY_FILE="$OUTPUT_DIR/$SCRIPT_NAME.memory_summary.txt"
    
    heaptrack_print --summary "$LATEST_HEAPTRACK" > "$SUMMARY_FILE"
    
    echo "Memory usage summary:"
    cat "$SUMMARY_FILE"
    
    echo "Peak memory usage should be less than 800 MB"
    PEAK_MB=$(grep "peak heap memory consumption" "$SUMMARY_FILE" | grep -o '[0-9]\+\.[0-9]\+' | head -n 1)
    
    if [ -n "$PEAK_MB" ]; then
        if (( $(echo "$PEAK_MB < 800" | bc -l) )); then
            echo "✅ Peak memory usage is under the 800 MB limit: $PEAK_MB MB"
        else
            echo "❌ Peak memory usage exceeds the 800 MB limit: $PEAK_MB MB"
        fi
    fi
    
    echo "Full analysis available in: $SUMMARY_FILE"
else
    echo "heaptrack_print not found. Install it to analyze the heaptrack data."
    echo "You can analyze the data manually with heaptrack_gui $LATEST_HEAPTRACK"
fi

echo "Done."
