#!/bin/bash
# Integration test runner for Graphoid
# Runs all .gr test files in tests/integration/

set -e

# Color codes for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
TEST_DIR="$SCRIPT_DIR/integration"
GRAPHOID="$HOME/.cargo/bin/cargo"

# Set stdlib path
export GRAPHOID_STDLIB_PATH="$SCRIPT_DIR/../../stdlib"

echo "================================================"
echo "  Graphoid Integration Test Runner"
echo "================================================"
echo ""
echo "Test directory: $TEST_DIR"
echo "Stdlib path: $GRAPHOID_STDLIB_PATH"
echo ""

# Counters
total=0
passed=0
failed=0
skipped=0

# Failed test tracking
declare -a failed_tests

# Find all .gr files in the integration directory
for test_file in "$TEST_DIR"/*.gr; do
    if [ -f "$test_file" ]; then
        total=$((total + 1))
        test_name=$(basename "$test_file")

        # Check if test should be skipped (files with _skip suffix)
        if [[ "$test_name" == *"_skip.gr" ]]; then
            echo -e "${YELLOW}⊘ SKIP${NC} $test_name (marked as skip)"
            skipped=$((skipped + 1))
            continue
        fi

        # Run the test
        echo -n "Running $test_name ... "

        # Run test and capture output
        if output=$($GRAPHOID run --quiet "$test_file" 2>&1); then
            echo -e "${GREEN}✓ PASS${NC}"
            passed=$((passed + 1))
        else
            echo -e "${RED}✗ FAIL${NC}"
            failed=$((failed + 1))
            failed_tests+=("$test_name")

            # Show error output
            echo "  Error output:"
            echo "$output" | sed 's/^/    /'
            echo ""
        fi
    fi
done

echo ""
echo "================================================"
echo "  Test Summary"
echo "================================================"
echo "Total:   $total"
echo -e "Passed:  ${GREEN}$passed${NC}"
echo -e "Failed:  ${RED}$failed${NC}"
echo -e "Skipped: ${YELLOW}$skipped${NC}"
echo ""

# List failed tests if any
if [ $failed -gt 0 ]; then
    echo "Failed tests:"
    for test_name in "${failed_tests[@]}"; do
        echo -e "  ${RED}✗${NC} $test_name"
    done
    echo ""
    exit 1
else
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
fi
