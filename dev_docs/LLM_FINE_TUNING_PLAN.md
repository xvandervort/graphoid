# LLM Fine-Tuning Plan for Graphoid

## Goal

Fine-tune a local LLM to write, understand, debug, and reason about Graphoid code. Since Graphoid exists in zero pre-training corpora, this requires a multi-stage approach: continued pretraining to teach syntax, then instruction tuning to teach tasks.

---

## 1. Training Data Categories

### 1.1 Raw Graphoid Code (Continued Pretraining)

**Purpose**: Teach the model what Graphoid looks like — syntax, patterns, idioms.

**Sources**:
- `samples/**/*.gr` — example programs
- `stdlib/**/*.gr` — standard library implementations
- `tests/gspec/**/*.gr` — spec tests
- `tests/integration/**/*.gr` — integration tests

**Target**: 500K+ tokens minimum (currently ~198 .gr files; need 3-5x more)

**Format**: Plain text, files separated by `<|endoftext|>` tokens

**Action items**:
- Write additional sample programs covering every language feature with 3-5 variations
- Create "cookbook" programs: real-world tasks implemented in Graphoid
- Generate Rosetta Stone pairs (same program in Python and Graphoid)

### 1.2 Commented/Annotated Code (Code Understanding)

**Purpose**: Teach the model to reason about what Graphoid code does.

**Format**: `.gr` files with rich inline comments explaining each block

**Example**:
```graphoid
# Create a directed acyclic graph for task dependencies
tasks = graph { type: :dag }

# Add nodes representing tasks with priority values
tasks.add_node("build", 1)
tasks.add_node("test", 2)
tasks.add_node("deploy", 3)

# Edges represent "must complete before" relationships
tasks.add_edge("build", "test", "before")
tasks.add_edge("test", "deploy", "before")

# Topological sort gives execution order
order = tasks.topo_sort()
# => ["build", "test", "deploy"]
```

**Action items**:
- Create `training/annotated/` directory for these
- Write 100+ annotated programs covering all major features

### 1.3 Instruction Pairs (Supervised Fine-Tuning)

**Purpose**: Teach the model to follow instructions about Graphoid.

**Categories**:

| Category | Example Instruction | Count Target |
|----------|-------------------|--------------|
| Code generation | "Write a Graphoid function that reverses a list" | 2000+ |
| Code explanation | "Explain what this Graphoid code does: ..." | 1000+ |
| Bug fixing | "This Graphoid code raises an error. Fix it: ..." | 500+ |
| Code conversion | "Convert this Python to idiomatic Graphoid: ..." | 1000+ |
| Language questions | "How do I create a binary tree in Graphoid?" | 500+ |
| Error diagnosis | "What does this error mean: ..." | 500+ |
| Refactoring | "Refactor this to use Graphoid idioms: ..." | 500+ |

**Target**: 5,000-10,000 high-quality pairs

**Format**: Alpaca JSON-lines
```json
{"instruction": "Write a Graphoid function that checks if a graph has cycles",
 "input": "",
 "output": "fn has_cycles(g) {\n    g.add_rule(\"no_cycles\")\n    # If the rule fails, the graph has cycles\n    try {\n        g.validate()\n        return false\n    } catch as e {\n        return true\n    }\n}"}
```

### 1.4 Anti-Pattern / Preference Pairs (DPO Alignment)

**Purpose**: Teach Graphoid design philosophy — what NOT to do.

**Key anti-patterns to encode**:
- No generics (EVER) — use duck typing
- No boilerplate constructors — use `ClassName { prop: value }`
- No getter methods — use `configure { readable: :name }`
- No method proliferation — one method with parameters
- No `#[cfg(test)]` in src/ — tests in tests/
- Graph-first design — prefer graph operations over imperative loops
- No unnecessary type annotations — trust inference

**Format**: DPO JSON-lines
```json
{"prompt": "Create a generic container class in Graphoid",
 "chosen": "Graphoid doesn't use generics. Use duck typing instead:\n\ncontainer = []\ncontainer.append(42)\ncontainer.append(\"hello\")\n# Lists accept any type naturally",
 "rejected": "class Container<T> {\n    items: list<T>\n    fn add(item: T) {\n        items.append(item)\n    }\n}"}
```

**Target**: 500-1000 preference pairs

### 1.5 Documentation Q&A (Knowledge)

**Purpose**: Ground the model in language specification knowledge.

**Sources**:
- `dev_docs/LANGUAGE_SPECIFICATION.md` → factual Q&A
- `dev_docs/NO_GENERICS_POLICY.md` → policy Q&A
- `dev_docs/ARCHITECTURE_DESIGN.md` → design rationale Q&A
- `docs/user-guide/` → tutorial-style Q&A

**Target**: 1,000+ Q&A pairs

---

## 2. Data Formatting

All training data lives in `training/` with this structure:

```
training/
├── README.md                    # Overview and instructions
├── scripts/
│   ├── collect_corpus.py        # Scrape all .gr files into plain text
│   ├── generate_instruct.py     # Generate instruction pairs
│   ├── generate_dpo.py          # Generate DPO preference pairs
│   ├── generate_qa.py           # Parse docs into Q&A
│   ├── format_alpaca.py         # Convert to Alpaca format
│   ├── format_sharegpt.py       # Convert to ShareGPT format
│   ├── validate_corpus.py       # Validate .gr snippets parse/run
│   └── stats.py                 # Corpus statistics
├── raw/                         # Collected plain text corpus
│   └── graphoid_corpus.txt      # All .gr files concatenated
├── annotated/                   # Commented example programs
│   ├── basics/
│   ├── graphs/
│   ├── concurrency/
│   └── ffi/
├── instruct/                    # Instruction pairs (source of truth)
│   ├── code_generation.jsonl
│   ├── code_explanation.jsonl
│   ├── bug_fixing.jsonl
│   ├── conversion.jsonl
│   └── language_qa.jsonl
├── dpo/                         # Preference pairs
│   └── anti_patterns.jsonl
├── rosetta/                     # Cross-language pairs
│   ├── python_graphoid/         # Python ↔ Graphoid equivalents
│   └── javascript_graphoid/     # JS ↔ Graphoid equivalents
├── formatted/                   # Ready-to-train outputs
│   ├── cpt_corpus.txt           # For continued pretraining
│   ├── sft_alpaca.jsonl         # Alpaca format for SFT
│   ├── sft_sharegpt.jsonl       # ShareGPT format for SFT
│   └── dpo_pairs.jsonl          # For DPO training
└── eval/                        # Evaluation benchmark
    ├── graphoid_eval.jsonl       # Test problems
    └── run_eval.py              # Auto-scoring script
```

### Format Specifications

**Continued Pretraining (CPT)**:
```
<|endoftext|>
# filename: samples/01-basics/hello_world.gr
print("Hello, World!")
<|endoftext|>
# filename: stdlib/math.gr
...
```

**Alpaca (SFT)**:
```json
{"instruction": "...", "input": "...", "output": "..."}
```

**ShareGPT (SFT)**:
```json
{"conversations": [
  {"from": "human", "value": "..."},
  {"from": "gpt", "value": "..."}
]}
```

**DPO**:
```json
{"prompt": "...", "chosen": "...", "rejected": "..."}
```

---

## 3. Fine-Tuning Pipeline

### 3.1 Base Model Selection

**Recommended** (as of early 2026):

| Model | Size | Why | VRAM Needed |
|-------|------|-----|-------------|
| **Qwen2.5-Coder** | 7B | Strong code understanding, good tokenizer | 24GB (QLoRA) |
| **Qwen2.5-Coder** | 14B | Better reasoning, still local-runnable | 48GB or 2x24GB |
| **DeepSeek-Coder-V2** | 16B (MoE) | Excellent code, efficient MoE | 24GB (QLoRA) |

**Why code-specialized**: General models (Llama, Mistral) learn code syntax slower. Code models already understand programming concepts — they just need to learn Graphoid's specific syntax.

**Tokenizer consideration**: Graphoid keywords (`graph`, `spawn`, `configure`, `behavior`) may tokenize into multiple tokens. If token efficiency is poor, consider:
1. Adding Graphoid keywords to the tokenizer vocabulary
2. Training a BPE merge on Graphoid corpus and merging into base tokenizer
3. Accepting the overhead (simplest, usually fine for <100 new tokens)

### 3.2 Training Steps

#### Step 1: Continued Pretraining (CPT)

**What**: Feed raw Graphoid code to teach syntax patterns.

**Config**:
```yaml
# Axolotl / Unsloth config
base_model: Qwen/Qwen2.5-Coder-7B
training_type: continued_pretraining
dataset: training/formatted/cpt_corpus.txt

# Hyperparameters
learning_rate: 1e-5          # Low — don't destroy pretrained knowledge
num_epochs: 3-5
batch_size: 4
gradient_accumulation_steps: 8
warmup_ratio: 0.05
max_seq_length: 4096

# LoRA (if not full fine-tune)
lora_r: 64                   # High rank for CPT
lora_alpha: 128
lora_target_modules: all     # All linear layers
```

**Duration**: ~2-4 hours on single 24GB GPU for 500K tokens

**Validation**: Generate 100 Graphoid snippets, check parse rate with `gr --check`

#### Step 2: Supervised Fine-Tuning (SFT)

**What**: Teach the model to follow instructions about Graphoid.

**Config**:
```yaml
base_model: ./output/cpt_checkpoint   # From Step 1
training_type: instruction_tuning
dataset: training/formatted/sft_alpaca.jsonl

learning_rate: 2e-5
num_epochs: 3
batch_size: 4
gradient_accumulation_steps: 4

lora_r: 16-32                # Lower rank OK for SFT
lora_alpha: 32-64
```

**Duration**: ~1-2 hours

**Validation**: Run GraphoidEval benchmark, measure correctness

#### Step 3: DPO Alignment (Optional but Recommended)

**What**: Teach Graphoid idioms and anti-patterns via preference pairs.

**Config**:
```yaml
base_model: ./output/sft_checkpoint   # From Step 2
training_type: dpo
dataset: training/formatted/dpo_pairs.jsonl

learning_rate: 5e-6          # Very low for DPO
beta: 0.1                    # KL penalty
num_epochs: 1-2
```

**Duration**: ~30-60 minutes

**Validation**: Present anti-pattern prompts, verify model chooses idiomatic approach

### 3.3 Tooling

| Tool | Purpose |
|------|---------|
| **Unsloth** | Fast LoRA/QLoRA training, 2x speedup, memory efficient |
| **Axolotl** | Full-featured training framework, supports all formats |
| **llama.cpp** | GGUF quantization and local inference |
| **Ollama** | Easy local model serving with Modelfile |
| **lm-evaluation-harness** | Benchmark evaluation framework |

### 3.4 Export and Serving

```bash
# After training, merge LoRA weights
python -m unsloth.merge --base Qwen2.5-Coder-7B --lora output/final

# Quantize to GGUF for local inference
python llama.cpp/convert_hf_to_gguf.py merged_model/ --outtype q4_k_m

# Create Ollama model
cat > Modelfile <<EOF
FROM ./graphoid-coder-7b-q4_k_m.gguf
SYSTEM "You are a Graphoid programming language expert..."
PARAMETER temperature 0.2
PARAMETER top_p 0.9
EOF
ollama create graphoid-coder -f Modelfile

# Use
ollama run graphoid-coder "Write a Graphoid function that finds shortest path in a weighted graph"
```

---

## 4. Evaluation: GraphoidEval Benchmark

### 4.1 Problem Categories

| Category | Count | Description |
|----------|-------|-------------|
| **Syntax** | 50 | Complete partial Graphoid code |
| **Functions** | 40 | Write functions given spec |
| **Graph ops** | 30 | Graph creation, traversal, rules |
| **Collections** | 20 | List/map/tree operations |
| **Concurrency** | 20 | Spawn, channels, actors |
| **Behaviors** | 15 | Intrinsic behavior system |
| **Modules** | 15 | Import, stdlib usage |
| **Debugging** | 10 | Fix broken Graphoid code |
| **Total** | **200** | |

### 4.2 Auto-Scoring

```python
# eval/run_eval.py pseudocode
for problem in eval_problems:
    response = model.generate(problem.prompt)
    code = extract_code(response)

    # Level 1: Does it parse?
    parse_ok = run("gr --check", code)

    # Level 2: Does it produce correct output?
    output = run("gr", code)
    correct = output == problem.expected_output

    # Level 3: Does it follow idioms?
    idiom_score = check_anti_patterns(code)

    scores.append({parse_ok, correct, idiom_score})
```

### 4.3 Target Metrics

| Metric | Minimum | Good | Excellent |
|--------|---------|------|-----------|
| Parse rate | 70% | 85% | 95% |
| Correctness | 40% | 60% | 80% |
| Idiom compliance | 60% | 80% | 95% |

---

## 5. Corpus Growth Strategy

Training data should grow as a natural byproduct of development:

| Graphoid Phase | Training Data Action |
|---|---|
| **Phase 20-21** (current) | Set up `training/` directory, write collection scripts, start Rosetta pairs |
| **Phase 22-25** | Database/distributed examples add domain-rich training data |
| **Phase 26-28** | Reflection/debugger introspection examples |
| **Post Phase 28** | Enough corpus for first CPT attempt (~500K tokens) |
| **Phase 29+** | Self-hosting code is ultimate training corpus |

### Ongoing Practices

1. **Every new feature**: Write 3-5 sample programs (not just 1)
2. **Every bug fix**: Document the error and fix as an instruction pair
3. **Every design decision**: Create a Q&A pair explaining the rationale
4. **Monthly**: Run `scripts/stats.py` to track corpus size and coverage
5. **Cross-language**: When solving a problem in Graphoid, also write the Python version

---

## 6. Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| **Corpus too small** | Overfitting, poor generalization | Aggressive sample generation, cross-language pairs, data augmentation |
| **Tokenizer mismatch** | Inefficient encoding of Graphoid keywords | Measure token efficiency early; add custom tokens if >2x overhead |
| **Syntax drift** | Training data becomes stale as language evolves | Version corpus, regenerate from source .gr files |
| **Overfitting** | Memorizes training data, can't generalize | Held-out eval set, early stopping, low epoch counts |
| **Anti-pattern leakage** | Model learns rejected examples as valid | Careful DPO training, verify with idiom compliance eval |
| **Base model obsolescence** | Better base models released | Modular pipeline — swap base model, re-run training |

---

## 7. Hardware Requirements

### Training

| Config | GPU | VRAM | Cost (cloud) |
|--------|-----|------|-------------|
| 7B QLoRA | 1x RTX 4090 | 24GB | ~$1/hr |
| 7B Full | 2x RTX 4090 | 48GB | ~$2/hr |
| 14B QLoRA | 1x A6000 | 48GB | ~$2/hr |
| 14B Full | 2x A100 | 160GB | ~$6/hr |

### Inference

| Config | GPU | VRAM | Tokens/sec |
|--------|-----|------|------------|
| 7B Q4_K_M | 1x RTX 4090 | ~6GB | ~80-100 |
| 14B Q4_K_M | 1x RTX 4090 | ~10GB | ~40-60 |
| 7B Q4_K_M (CPU) | None | 8GB RAM | ~10-15 |

Total estimated training time (all stages): **4-8 hours** on a single 24GB GPU.
Total estimated cloud cost: **$10-20** per full training run.

---

## Summary

1. **Start now**: Set up `training/` directory and collection scripts
2. **Grow naturally**: Generate training data as byproduct of development
3. **Two-stage training**: CPT (learn syntax) → SFT (learn tasks) → DPO (learn idioms)
4. **Evaluate rigorously**: GraphoidEval benchmark with auto-scoring
5. **Target**: Post-Phase 28, first fine-tuned model; iterate as corpus grows
