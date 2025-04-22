
import json
import sys

def load_heaptrack_json(filename):
    with open(filename, 'r') as f:
        data = json.load(f)
    return data

def compare_heaptrack(baseline_file, current_file):
    baseline = load_heaptrack_json(baseline_file)
    current = load_heaptrack_json(current_file)
    
    baseline_peak_rss = baseline.get('peak_rss', 0) / (1024 * 1024)  # Convert to MB
    current_peak_rss = current.get('peak_rss', 0) / (1024 * 1024)    # Convert to MB
    
    baseline_total_allocs = baseline.get('total_allocations', 0)
    current_total_allocs = current.get('total_allocations', 0)
    
    print(f"Baseline peak RSS: {baseline_peak_rss:.2f} MB")
    print(f"Current peak RSS: {current_peak_rss:.2f} MB")
    print(f"RSS change: {(current_peak_rss - baseline_peak_rss):.2f} MB ({(current_peak_rss / baseline_peak_rss * 100):.1f}%)")
    
    print(f"Baseline total allocations: {baseline_total_allocs:,}")
    print(f"Current total allocations: {current_total_allocs:,}")
    print(f"Allocation change: {(current_total_allocs - baseline_total_allocs):,} ({(current_total_allocs / baseline_total_allocs * 100):.1f}%)")
    
    rss_threshold = baseline_peak_rss * 1.25
    alloc_threshold = baseline_total_allocs * 1.25
    
    if current_peak_rss > rss_threshold:
        print(f"ERROR: Peak RSS exceeds threshold of {rss_threshold:.2f} MB (125% of baseline)")
        return 1
    
    if current_total_allocs > alloc_threshold:
        print(f"ERROR: Total allocations exceeds threshold of {alloc_threshold:,} (125% of baseline)")
        return 1
    
    print("SUCCESS: Memory usage within acceptable thresholds")
    return 0

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print(f"Usage: {sys.argv[0]} baseline.json current.json")
        sys.exit(1)
    
    baseline_file = sys.argv[1]
    current_file = sys.argv[2]
    
    exit_code = compare_heaptrack(baseline_file, current_file)
    sys.exit(exit_code)
