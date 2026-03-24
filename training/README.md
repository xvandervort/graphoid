# Graphoid LLM Training Data

Training corpus for fine-tuning a local LLM to work with the Graphoid programming language.

See `dev_docs/LLM_FINE_TUNING_PLAN.md` for the full plan.

## Directory Structure

```
training/
├── scripts/          # Data collection and formatting scripts
├── raw/              # Collected plain text corpus (auto-generated)
├── annotated/        # Hand-written commented example programs
├── instruct/         # Instruction pairs (JSONL)
├── dpo/              # Preference/anti-pattern pairs (JSONL)
├── rosetta/          # Cross-language equivalent programs
├── formatted/        # Ready-to-train outputs (auto-generated)
└── eval/             # GraphoidEval benchmark problems
```

## Quick Start

```bash
# Collect raw corpus from all .gr files
python training/scripts/collect_corpus.py

# Check corpus statistics
python training/scripts/stats.py

# Validate that code snippets in instruction pairs actually parse
python training/scripts/validate_corpus.py
```

## Data Formats

- **raw/**: Plain `.gr` files concatenated with `<|endoftext|>` separators
- **instruct/**: Alpaca JSONL — `{"instruction", "input", "output"}`
- **dpo/**: Preference JSONL — `{"prompt", "chosen", "rejected"}`
- **rosetta/**: Paired files — `foo.py` + `foo.gr` implementing the same program
- **formatted/**: Final outputs ready for training frameworks (Unsloth/Axolotl)
