# Tooling scorecard

- **Prefix:** TOOL
- **Covers:** Test harnesses, the shared conformance runner, the command-line interface, the build system, and shared compiler architecture.

## Tests

### TOOL-TEST-01 — Front-end unit tests run from one command

- **Status:** Proposed
- **Spec:** None
- **Done when:** Lexer, parser, resolver, and type tests run with `cargo test`, in CI or an equivalent repeatable runner.
- **Notes:** `cargo test` already runs the lexer tests; there is no CI yet.

### TOOL-CONF-01 — Shared conformance runner

- **Status:** Proposed
- **Spec:** None
- **Done when:** Conformance cases with stable IDs, inputs, and expected outcomes run against the interpreter and each backend independently.

### TOOL-CONF-02 — Normalized result comparison

- **Status:** Proposed
- **Spec:** None
- **Done when:** The runner compares resulting values, missing values, errors, and laziness across targets rather than generated code text, and records the reason for any skipped case.

## Command line

### TOOL-CLI-01 — Select the entry module by path

- **Status:** Approved
- **Spec:** [modules.md — Entry modules](../spec/modules.md#entry-modules)
- **Done when:** With several executable modules in a project, running one by its file path runs only that module's entry function.

### TOOL-CLI-02 — Command-line interface details

- **Status:** Open question
- **Spec:** [modules.md — Entry modules](../spec/modules.md#entry-modules)
- **Questions:**
  1. What are the command names and options? `natcode run src/main.nat` is illustrative.

## Built-in formatting

### TOOL-FMT-01 — Canonical formatting included in Natcode

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Canonical formatting](../spec/syntax-and-types.md#canonical-formatting)
- **Done when:** Natcode ships a built-in `natcode fmt` and a non-mutating `natcode fmt --check` mode, requiring no third-party linter or formatter. It produces canonical four-space indentation from valid, unambiguous parsed structure without changing semantics; input indentation widths may vary. It preserves explicitly written executable braces, expands one-line braced executable bodies into multiline blocks, and aligns `when`, `where`, and constructor lists. Continued expressions are normalized from their parsed structure: short expressions collapse to one line; long expressions wrap after binary operators with four-space continuation indentation; explicit grouping parentheses are preserved. It never joins distinct statements, silently fixes ambiguous ownership, or changes a program's meaning.
- **Notes:** Approved 2026-10-21; expression-wrapping policy approved 2026-10-23 (PAR-BLK-02). The precise wrap-width threshold is not chosen. Ordinary compilation does not rewrite source. CLI details beyond the specifically approved formatter commands remain TOOL-CLI-02; this item is not yet implemented.

## Build and architecture

### TOOL-BUILD-01 — Package layout and build system

- **Status:** Open question
- **Spec:** None
- **Questions:**
  1. How are projects and packages laid out, and how are they built and linked?

### TOOL-ARCH-01 — Shared compiler IR and runtime design

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Target directions](../spec/syntax-and-types.md#target-directions)
- **Questions:**
  1. Do all targets share one intermediate representation and runtime design, and what is it?
  2. Is Rust the confirmed implementation language? The current code is Rust.

## Scorecard lifecycle

### TOOL-SCORE-01 — Revision-aware scorecard status validation

- **Status:** In progress
- **Spec:** None
- **Done when:** The scorecard validator and status report recognize `Needs update` and `Superseded`, require historical evidence for revised implementations, validate replacement references and retirement reasons, and preserve existing `Implemented` evidence checks. The lifecycle unit tests and full scorecard check pass.
- **Notes:** Status behavior and validator tests have been added in PR #7; automated test execution has not yet been verified.
