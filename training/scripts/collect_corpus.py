#!/usr/bin/env python3
"""
Collect all .gr files from the Graphoid project into a single corpus file
suitable for continued pretraining (CPT).

Output: training/raw/graphoid_corpus.txt
"""

import os
import sys
from pathlib import Path

# Project root is two levels up from this script
PROJECT_ROOT = Path(__file__).resolve().parent.parent.parent

# Directories to collect .gr files from, in order of priority
SOURCE_DIRS = [
    "stdlib",              # Standard library — canonical Graphoid
    "samples",             # Example programs — idiomatic usage
    "tests/gspec",         # Spec tests — behavioral examples
    "tests/integration",   # Integration tests — feature validation
    "training/annotated",  # Hand-annotated training examples
]

# Separator token (common across most tokenizers)
SEPARATOR = "<|endoftext|>"

OUTPUT_FILE = PROJECT_ROOT / "training" / "raw" / "graphoid_corpus.txt"


def collect_gr_files(base_dir: Path, source_dirs: list[str]) -> list[Path]:
    """Find all .gr files in the specified directories."""
    files = []
    for rel_dir in source_dirs:
        full_dir = base_dir / rel_dir
        if not full_dir.exists():
            print(f"  Skipping {rel_dir}/ (not found)", file=sys.stderr)
            continue
        found = sorted(full_dir.rglob("*.gr"))
        print(f"  {rel_dir}/: {len(found)} files", file=sys.stderr)
        files.extend(found)
    return files


def build_corpus(files: list[Path], base_dir: Path) -> str:
    """Concatenate files with separators and source annotations."""
    chunks = []
    total_tokens_est = 0

    for filepath in files:
        rel_path = filepath.relative_to(base_dir)
        try:
            content = filepath.read_text(encoding="utf-8").strip()
        except (OSError, UnicodeDecodeError) as e:
            print(f"  WARNING: Skipping {rel_path}: {e}", file=sys.stderr)
            continue

        if not content:
            continue

        # Annotate with source path (helps model learn file organization)
        chunk = f"{SEPARATOR}\n# source: {rel_path}\n{content}\n"
        chunks.append(chunk)

        # Rough token estimate: ~4 chars per token for code
        total_tokens_est += len(content) // 4

    return "".join(chunks), total_tokens_est, len(chunks)


def main():
    print("Collecting Graphoid corpus...", file=sys.stderr)
    print(f"Project root: {PROJECT_ROOT}", file=sys.stderr)
    print(file=sys.stderr)

    files = collect_gr_files(PROJECT_ROOT, SOURCE_DIRS)

    if not files:
        print("ERROR: No .gr files found!", file=sys.stderr)
        sys.exit(1)

    corpus, token_est, file_count = build_corpus(files, PROJECT_ROOT)

    # Ensure output directory exists
    OUTPUT_FILE.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_FILE.write_text(corpus, encoding="utf-8")

    print(file=sys.stderr)
    print(f"Corpus written to: {OUTPUT_FILE.relative_to(PROJECT_ROOT)}", file=sys.stderr)
    print(f"Files included: {file_count}", file=sys.stderr)
    print(f"Corpus size: {len(corpus):,} chars", file=sys.stderr)
    print(f"Estimated tokens: ~{token_est:,}", file=sys.stderr)


if __name__ == "__main__":
    main()
