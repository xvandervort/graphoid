#!/usr/bin/env python3
"""
Validate that Graphoid code snippets in training data actually parse and run.

Extracts code blocks from instruction pairs and checks them against `gr --check`.
Reports broken examples that need fixing before training.
"""

import json
import subprocess
import sys
import tempfile
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parent.parent.parent
TRAINING_DIR = PROJECT_ROOT / "training"

# Path to gr binary — try installed first, then debug build
GR_PATHS = [
    Path.home() / ".local" / "bin" / "gr",
    PROJECT_ROOT / "target" / "release" / "gr",
    PROJECT_ROOT / "target" / "debug" / "gr",
]


def find_gr():
    """Find the gr binary."""
    for p in GR_PATHS:
        if p.exists():
            return str(p)
    print("ERROR: Cannot find 'gr' binary. Run 'make build' first.", file=sys.stderr)
    sys.exit(1)


def check_code(gr_bin: str, code: str, timeout: int = 5) -> tuple[bool, str]:
    """Run a code snippet through gr and check if it succeeds."""
    with tempfile.NamedTemporaryFile(suffix=".gr", mode="w", delete=True) as f:
        f.write(code)
        f.flush()
        try:
            result = subprocess.run(
                [gr_bin, f.name],
                capture_output=True,
                text=True,
                timeout=timeout,
            )
            if result.returncode == 0:
                return True, ""
            return False, result.stderr.strip()[:200]
        except subprocess.TimeoutExpired:
            return False, "TIMEOUT"
        except Exception as e:
            return False, str(e)[:200]


def validate_jsonl_files(gr_bin: str, directory: Path, field: str = "output"):
    """Validate code in JSONL instruction files."""
    if not directory.exists():
        return

    total = 0
    passed = 0
    failed = []

    for jsonl_file in sorted(directory.glob("*.jsonl")):
        print(f"\nValidating {jsonl_file.name}...")
        for line_num, line in enumerate(jsonl_file.open(), 1):
            line = line.strip()
            if not line:
                continue
            try:
                entry = json.loads(line)
            except json.JSONDecodeError:
                failed.append((jsonl_file.name, line_num, "Invalid JSON"))
                total += 1
                continue

            code = entry.get(field, "")
            if not code or not any(kw in code for kw in ["fn ", "print", "=", "graph", "import"]):
                continue  # Skip non-code entries (explanations, etc.)

            total += 1
            ok, err = check_code(gr_bin, code)
            if ok:
                passed += 1
                sys.stdout.write(".")
            else:
                failed.append((jsonl_file.name, line_num, err))
                sys.stdout.write("F")
            sys.stdout.flush()

    return total, passed, failed


def main():
    gr_bin = find_gr()
    print(f"Using gr: {gr_bin}")

    print("\n=== Validating Instruction Pairs ===")
    total, passed, failed = validate_jsonl_files(gr_bin, TRAINING_DIR / "instruct")

    if total == 0:
        print("\nNo instruction pairs found yet. Nothing to validate.")
        return

    print(f"\n\nResults: {passed}/{total} passed ({passed*100//total}%)")

    if failed:
        print(f"\n{len(failed)} FAILURES:")
        for fname, line, err in failed:
            print(f"  {fname}:{line} — {err}")
    else:
        print("All code snippets are valid!")


if __name__ == "__main__":
    main()
