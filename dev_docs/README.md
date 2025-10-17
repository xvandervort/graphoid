# Developer Documentation Organization

This directory contains all development documentation for the Graphoid project. Files are organized to distinguish between **permanent working documents** and **temporary/archived materials**.

---

## Permanent Working Documents

These files are actively maintained and referenced throughout development:

| File | Purpose | Update Frequency |
|------|---------|------------------|
| **LANGUAGE_SPECIFICATION.md** | Canonical language specification | Frequent - as language evolves |
| **RUST_IMPLEMENTATION_ROADMAP.md** | 14-phase implementation plan | Periodic - as phases complete |
| **ARCHITECTURE_DESIGN.md** | Internal architecture decisions | Occasional - major design changes |
| **PRODUCTION_TOOLING_SPECIFICATION.md** | Testing/debugging/packaging specs | Periodic - tooling evolution |

**These files should NEVER be moved to archive.** They are the core reference for all development work.

---

## Directory Structure

```
dev_docs/
├── README.md                              # This file
├── LANGUAGE_SPECIFICATION.md              # PERMANENT
├── RUST_IMPLEMENTATION_ROADMAP.md         # PERMANENT
├── ARCHITECTURE_DESIGN.md                 # PERMANENT
├── PRODUCTION_TOOLING_SPECIFICATION.md    # PERMANENT
├── archive/                               # Historical/completed work
│   └── sessions/                          # Session-specific documents
│       ├── 2025-01-spec-review/
│       ├── 2025-01-design-decisions/
│       └── 2025-01-roadmap-updates/
└── scratch/                               # Temporary working files (gitignored)
```

---

## Archive Organization

The `archive/` directory contains:

### Session-Specific Work (`archive/sessions/`)

Documents created during specific work sessions that capture decisions, analysis, or explorations at a point in time. These are valuable for historical context but not actively referenced.

**Examples**:
- Design decision logs
- Feature analysis documents
- Completeness reviews
- Comparison studies

**Naming Convention**: `YYYY-MM-description/`

**When to Archive**:
- At the end of a major session (spec review, feature design, etc.)
- When a document's analysis has been incorporated into permanent docs
- When a temporary exploration document is no longer actively referenced

---

## Scratch Directory

The `scratch/` directory is for truly temporary working files:

- Quick notes during development
- Experimental code snippets
- Draft documents before they're finalized
- Brainstorming files

**Important**: `scratch/` should be added to `.gitignore` so these files aren't committed.

---

## Organization Guidelines

### At the Start of Each Session

1. **Review `dev_docs/`**: Check for any files that don't belong in the root
2. **Archive completed work**: Move session-specific docs to appropriate `archive/sessions/` folders
3. **Clean scratch/`**: Delete any scratch files no longer needed

### When Creating New Documents

**Ask yourself**:
- **Will this be referenced throughout development?** → Root directory (permanent)
- **Is this session-specific analysis/decisions?** → Plan to archive when session ends
- **Is this temporary brainstorming?** → Put in `scratch/`

**Examples**:

✅ **Permanent (Root)**:
- Language specification
- Implementation roadmap
- Architecture design
- Production tooling specs

📦 **Archive (after session)**:
- "Design Decisions Session 1"
- "Spec Completeness Review"
- "Philosophy-Driven Recommendations"
- "Roadmap Updates for New Features"

🗑️ **Scratch (temporary)**:
- "notes_on_graph_querying.txt"
- "experiment_with_syntax.md"
- "draft_error_handling.md"

---

## Current Archive Contents

### `archive/sessions/2025-01-spec-review/`

Documents from the January 2025 language specification review session:
- `PHILOSOPHY_DRIVEN_RECOMMENDATIONS.md` - Recommendations for language philosophy alignment
- `SPEC_COMPLETENESS_REVIEW.md` - Completeness analysis vs other languages
- `PRODUCTION_TOOLING_SUMMARY.md` - Executive summary of production tooling
- `TESTING_FRAMEWORK_COMPARISON.md` - Before/after comparison of testing approaches

### `archive/sessions/2025-01-design-decisions/`

Documents from design decision sessions:
- `DESIGN_DECISIONS_SESSION_1.md` - Decisions on inline conditionals, mutation operators, etc.

### `archive/sessions/2025-01-roadmap-updates/`

Documents from roadmap update sessions:
- `ROADMAP_UPDATES_FOR_NEW_FEATURES.md` - Impact analysis of 10 new language features

---

## Maintenance Schedule

**Every Session Start** (5 minutes):
1. Scan `dev_docs/` root for any non-permanent files
2. Move completed session work to appropriate archive folders
3. Delete unnecessary files from `scratch/`
4. Update this README if organization changes

**Every Phase Completion** (15 minutes):
1. Review archive for any documents that can be deleted
2. Consolidate related session docs if needed
3. Ensure permanent docs are up to date

---

## Questions?

If you're unsure whether a document should be:
- **Permanent**: Will it be referenced for months? Does it define the language/architecture?
- **Archived**: Is it session-specific? Has its analysis been incorporated elsewhere?
- **Scratch**: Is it truly temporary? Will you delete it in days or weeks?

**When in doubt, start in `scratch/` and promote to permanent only when clearly needed.**

---

Last Updated: 2025-01-17
