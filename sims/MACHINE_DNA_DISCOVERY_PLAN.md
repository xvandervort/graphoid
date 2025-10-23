# Machine DNA — Discovery Plan and Early Experiments (Glang/Graphoid)

Updated: 2025-10-05

## Executive summary
- Goal: Define an immutable, compact, code-like sequence (alphabet-restricted) that fully specifies a machine, enabling reconstruction and repair from the sequence alone.
- Approach: Layered encoding from a base-N alphabet (e.g., A/C/G/T) to a structured genotype that compiles into a phenotype (assembly graph + parameters + processes).
- Deliverables (initial):
  - Minimal encoding spec (v0) with a 4-symbol alphabet 
  - Parser/encoder demo for a simple class (fastener)
  - Experiments for complexity ramp: single-piece → multi-part assembly → simple electronics
- Integrity, reproducibility, evolvability, and manufacturability are first-class concerns (checksums/hashes, canonicalization, units, error-correcting codes).
- Different strands or allelles may code different components such as casing, wheels, armatures, firmware, etc.

---

## Design principles
- **Immutability**: Sequences are content-addressed, canonical, and versioned.
- **Minimal alphabet**: Start with 4 symbols (A/C/G/T). Keep it simple and printable. (Later versions may expand the alphabet)
- **Local decodability**: Fixed-width segments and clear boundaries enable partial reads and targeted edits.
- **Genotype → Phenotype**: Sequence compiles to a machine phenotype: parts, parameters, materials, processes, assembly order.
- **Composability**: Genes form modules (subassemblies); modules compose into full devices via an assembly graph.
- **Manufacturability**: Encodings map to real tolerances, units, processes (additive/subtractive, electronics assembly, calibration).
- **Authenticity**: Checksums for basic integrity, hashes/signatures for provenance.
- **Evolvability**: Schema versions, reserved fields, and extendable codon tables.

---

## Encoding layers (v0 → v1 trajectory)
1. **Alphabet and base**
   - v0: base-4 alphabet `{A,C,G,T}` with A=0, C=1, G=2, T=3.
   - v1+: support base-8/16 (more compact), or base64 for density while keeping v0 backwards-compatible.

2. **Segments**
   - Fixed-width segments (codons) map to: version, class, material, numeric parameters, checksum.
   - Example (fastener v1):
     - 1 char: version
     - 2 chars: class id (0–15)
     - 2 chars: material id (0–15)
     - 4 chars: length_mm scaled (0–1000)
     - 3 chars: diameter_mm scaled (0–50)
     - 3 chars: hardness_hv scaled (20–60)
     - 4 chars: checksum (0–255)

3. **Codon table and grammar**
   - Map small integers to class labels (fastener/bracket/beam/...)
   - Genes/Regulatory sequences (v2): promoters select module libraries; terminators end genes; control loci gate expression/assembly.
   - Grammar: L-system or CFG expansions reference part families with parameters.

4. **Parameters and units**
   - Scaled numerics with explicit units; canonical forms (value + unit) to avoid ambiguity.
   - v1: simple min/max scaling; v2+: piecewise, log scaling for span across magnitudes.

5. **Integrity and authenticity**
   - v1: checksum over header+payload (e.g., mod 256 in 4 base-4 digits)
   - v2: SHA-256 and digital signatures (content-addressed identifiers)
   - v2+: Reed–Solomon/ECC segments for recovery from partial corruption

6. **Phenotype and assembly graph**
   - Output phenotype is a typed, directed multigraph:
     - Nodes: components with attributes (material, params, processes)
     - Edges: joints/constraints (fit, fasten, weld, electrical net joins)
   - Manufacturing views: CAD stubs, BOM, netlist, process steps, calibration recipes.

---

## Genotype→Phenotype pipeline (compiler architecture)
- Parser: sequence → tokens → AST of modules/fields
- Type checker: unit-safe, range-checked fields
- Resolver: link to part libraries (fastener families, bracket templates)
- Assembler: build assembly graph (structural/electrical)
- Backends (pluggable):
  - CAD stub export (parametric sketches)
  - Netlist/BOM for electronics
  - Manufacturing steps (3D print/CNC/SMT pick-place)
  - Verification (tolerances, mass props, interference)

---

## Immediate experiments (v0/v1)
1. **Single-piece fastener** (implemented as runnable sample)
   - Encode/decode using base-4 segments; verify checksum; round-trip stability.
   - Validate unit ranges and canonicalization.

2. **Multi-part bracket assembly** (v1)
   - Two modules (plate + gusset) with join edge (holes alignment). Parameters: thickness, width/height, hole pattern.
   - Test: decode → generate simple adjacency/slotting constraints; confirm graph representation.

3. **Electronics toy netlist** (v1)
   - Resistor/capacitor network with fixed topology id + parameterized values.
   - Output: simple netlist map `{ nodes: [...], nets: [...] }` (no external EDA yet).

4. **Variation and compatibility** (v1)
   - Mutate parameters within tolerance; test part interchangeability.
   - Define compatible ranges via regulatory fields.

5. **Integrity & resilience** (v1+)
   - Introduce bit flips; measure detection (checksum) and propose ECC lengths.

6. **On-device store** (v1+)
   - Define minimal header for devices: version, content-length, hash, signature placeholder.
   - Retrieval flow for repair: read → verify → decode → regenerate component.

---

## Vocabulary (initial)
- **Alphabet**: symbol set (e.g., A/C/G/T)
- **Codon**: fixed-width token (n symbols) → integer or enum
- **Gene**: module definition (subassembly)
- **Promoter/Terminator**: start/stop of module
- **Regulatory locus**: constraints/tolerances; process selectors
- **Phenotype**: machine graph (parts + joins + processes)

---

## Language and stdlib gaps (prioritized)
1. **Graph module** (High)
   - Typed nodes/edges, attributes, serialization; traversal; validators.
2. **Binary/encoding utilities** (High)
   - Base-N conversions, CRC/Checksum, SHA-256; ECC (Reed–Solomon) [see `dev_docs/BINARY_DATA_ENHANCEMENT_PLAN.md`]
3. **Units/types** (High)
   - Unit-safe numerics; conversions; compile-time or runtime checks.
4. **Schema/validation** (Med)
   - Declarative schemas for class-specific fields; constraints and helpful errors.
5. **JSON/IO** (Med)
   - Export decoded phenotype; import part libraries.
6. **Signature/crypto** (Med)
   - Ed25519, signature attach/verify; content addressing.
7. **CAD/geometry hooks** (Low→Med)
   - Minimal parametric shapes; bridges to external CAD for preview/validation.

---

## What exists today (usable now)
- Strong string processing + regex
- Deterministic/secure RNG with seeds
- Maps/lists; control flow; number methods
- Time module

These are enough to prototype v0/v1 encoding/decoding and simple assembly graphs represented as maps.

---

## Runnable sample
- Path: `rust/samples/machine_dna_demo.gr`
- Implements v1-style base-4 encoding for a fastener.
- Provides `encode_fastener(spec)` and `decode_machine_dna(seq)`; validates checksum; demonstrates round-trip.

---

## Next steps
- Extend sample to multi-part bracket with an assembly map output
- Add integrity fuzz tests and mutation experiments
- Draft `graph` and `binary` module APIs; align with Graphoid roadmap
- Plan JSON/IO hooks once Rust stdlib modules are migrated

---

## Firmware/Hardware alleles (wrinkle)

Complex devices require both hardware topology and firmware. Model these as distinct, linked alleles within the same Machine DNA record:

- Allele H (hardware): assembly graph genotype as above (mechanical + electrical netlists).
- Allele F (firmware):
  - v1: content-addressed blob manifest (hash, length, target MCU/SoC, entry point, version, signature), encoded in base-4 segments.
  - v2: bytecode blocks or delta-compressed segments embedded inline, with error-correcting codes and chunk checksums.
  - v3: support for multi-image (bootloader + app + config/calibration).

Linkage:
- Header binds H and F alleles: shared device-id, compatibility constraints (MCU pinout, memory map, peripheral set), and a cross-hash to prevent mismatched flashing.
- Repair flow: decode H to regenerate/verify board + harness; decode F manifest to fetch or reconstruct firmware by hash; verify signatures before deployment.

Encoding sketch (firmware v1 manifest):
- `[2]` MCU family id; `[2]` architecture id; `[3]` flash_kb; `[2]` vector_table_off; `[4×N]` hash (N=8 for 256-bit) ; `[2]` signature alg; `[2]` signature len; `[4×M]` signature blob (optional);
- Checksums per segment; overall record hash in outer header.

Language needs:
- Binary/base-N utilities, SHA-256, signatures (Ed25519), and ECC segments.
- File IO and JSON exports for manifests.
- Streaming chunk decode to avoid large in-memory buffers.
