#!/bin/bash
# Run all stdlib module tests
# Usage: cd rust && bash ../tests/stdlib/run_all_tests.sh

# Check that we're in the rust directory
if [ ! -f "Cargo.toml" ]; then
    echo "ERROR: This script must be run from the rust/ directory"
    echo "Usage: cd rust && bash ../tests/stdlib/run_all_tests.sh"
    exit 1
fi

CARGO="$HOME/.cargo/bin/cargo"
TESTS_DIR="/home/irv/work/grang/tests/stdlib"

echo "=========================================="
echo "Running All Stdlib Module Tests"
echo "=========================================="
echo ""

TESTS=(
    "test_statistics"
    "test_csv"
    "test_json"
    "test_regex"
    "test_time"
    "test_collections"
    "test_http"
    "test_pp"
    "test_optparse"
    "test_sql"
    "test_html"
)

PASSED=0
FAILED=0

for test in "${TESTS[@]}"; do
    echo ">>> Running ${test}.gr..."
    if $CARGO run --quiet "$TESTS_DIR/${test}.gr" 2>&1 | grep -q "ALL TESTS PASSED"; then
        echo "✓ PASSED"
        ((PASSED++))
    else
        echo "✗ FAILED"
        ((FAILED++))
    fi
    echo ""
done

echo "=========================================="
echo "Summary: $PASSED passed, $FAILED failed"
echo "=========================================="

if [ $FAILED -eq 0 ]; then
    exit 0
else
    exit 1
fi
