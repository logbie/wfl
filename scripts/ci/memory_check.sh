set -e

if ! command -v heaptrack &> /dev/null; then
    echo "Installing heaptrack..."
    sudo apt-get update
    sudo apt-get install -y heaptrack
fi

echo "Running heaptrack on WFL interpreter..."
heaptrack --output=heaptrack.wfl target/debug/wfl tests/data/nexus.wfl --timeout=30 --quiet

echo "Converting heaptrack output to JSON..."
heaptrack_print --json heaptrack.wfl.*.gz > current.json

if [ -f "baseline.json" ]; then
    echo "Comparing with baseline..."
    ./scripts/compare_heaptrack.py baseline.json current.json
else
    echo "Creating baseline..."
    cp current.json baseline.json
fi

echo "Running leak sanitizer tests..."
ASAN_OPTIONS=detect_leaks=1 RUSTFLAGS="-Zsanitizer=address" \
  cargo +nightly test --release
