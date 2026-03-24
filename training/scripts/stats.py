#!/usr/bin/env python3
"""
Report statistics on the Graphoid training corpus.

Tracks corpus growth over time and identifies coverage gaps.
"""

import json
import sys
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parent.parent.parent
TRAINING_DIR = PROJECT_ROOT / "training"


def count_gr_files():
    """Count .gr files by directory."""
    dirs = {
        "stdlib": PROJECT_ROOT / "stdlib",
        "samples": PROJECT_ROOT / "samples",
        "tests/gspec": PROJECT_ROOT / "tests" / "gspec",
        "tests/integration": PROJECT_ROOT / "tests" / "integration",
        "training/annotated": TRAINING_DIR / "annotated",
        "training/rosetta": TRAINING_DIR / "rosetta",
    }
    total = 0
    for name, path in dirs.items():
        if path.exists():
            count = len(list(path.rglob("*.gr")))
            total += count
            print(f"  {name + '/':30s} {count:4d} files")
        else:
            print(f"  {name + '/':30s}    - (not found)")
    return total


def count_jsonl(directory: Path, label: str):
    """Count entries in JSONL files."""
    total = 0
    if not directory.exists():
        print(f"  {label + '/':30s}    - (not found)")
        return 0
    for f in sorted(directory.glob("*.jsonl")):
        count = sum(1 for line in f.open() if line.strip())
        total += count
        print(f"  {f.name:30s} {count:4d} entries")
    if total == 0:
        print(f"  {label + '/':30s}    0 entries (empty)")
    return total


def corpus_size():
    """Check raw corpus size."""
    corpus = TRAINING_DIR / "raw" / "graphoid_corpus.txt"
    if corpus.exists():
        text = corpus.read_text()
        chars = len(text)
        tokens_est = chars // 4
        return chars, tokens_est
    return 0, 0


def rosetta_pairs():
    """Count Rosetta Stone cross-language pairs."""
    rosetta_dir = TRAINING_DIR / "rosetta"
    if not rosetta_dir.exists():
        return 0
    pairs = 0
    for subdir in rosetta_dir.iterdir():
        if subdir.is_dir():
            gr_files = set(f.stem for f in subdir.glob("*.gr"))
            py_files = set(f.stem for f in subdir.glob("*.py"))
            js_files = set(f.stem for f in subdir.glob("*.js"))
            pairs += len(gr_files & py_files) + len(gr_files & js_files)
    return pairs


def main():
    print("=" * 60)
    print("GRAPHOID TRAINING CORPUS STATISTICS")
    print("=" * 60)

    print("\n--- .gr Source Files ---")
    total_files = count_gr_files()
    print(f"  {'TOTAL':30s} {total_files:4d} files")

    print("\n--- Raw Corpus ---")
    chars, tokens = corpus_size()
    if chars:
        print(f"  Size: {chars:,} chars (~{tokens:,} tokens)")
        target = 500_000
        pct = min(100, tokens * 100 // target)
        print(f"  Progress to 500K token target: {pct}%")
    else:
        print("  Not yet generated. Run: python training/scripts/collect_corpus.py")

    print("\n--- Instruction Pairs (SFT) ---")
    instruct_total = count_jsonl(TRAINING_DIR / "instruct", "instruct")
    if instruct_total:
        target = 5000
        pct = min(100, instruct_total * 100 // target)
        print(f"  Progress to 5K pair target: {pct}%")

    print("\n--- Preference Pairs (DPO) ---")
    dpo_total = count_jsonl(TRAINING_DIR / "dpo", "dpo")
    if dpo_total:
        target = 500
        pct = min(100, dpo_total * 100 // target)
        print(f"  Progress to 500 pair target: {pct}%")

    print("\n--- Rosetta Stone Pairs ---")
    pairs = rosetta_pairs()
    print(f"  Cross-language pairs: {pairs}")

    print("\n--- Eval Benchmark ---")
    eval_file = TRAINING_DIR / "eval" / "graphoid_eval.jsonl"
    if eval_file.exists():
        count = sum(1 for line in eval_file.open() if line.strip())
        print(f"  Eval problems: {count}")
        target = 200
        pct = min(100, count * 100 // target)
        print(f"  Progress to 200 problem target: {pct}%")
    else:
        print("  Not yet created.")

    print("\n" + "=" * 60)
    print("TARGETS: 500K tokens (CPT) | 5K pairs (SFT) | 500 pairs (DPO) | 200 eval problems")
    print("=" * 60)


if __name__ == "__main__":
    main()
