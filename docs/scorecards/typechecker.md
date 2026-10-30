# Type Checker scorecard

- **Prefix:** CHK
- **Covers:** Compiler type-checking orchestration and integration with resolved AST, type algebra, inference, refinements, and diagnostics. Language semantics remain owned by existing `TYP`, `RES`, and `PAR` items.
- **Boundary:** Existing `TYP-*` IDs are retained. This scorecard tracks checker implementation responsibilities, not duplicate or competing language decisions. All items below are proposals, not owner-approved decisions.

## Pipeline and contracts

### CHK-PIPE-01 — Checker input and result contract

- **Status:** Proposed
- **Spec:** None
- **Done when:** A checker entry point consumes a parsed, name-resolved module and produces typed information plus diagnostics; the parser remains responsible for syntax, and the resolver remains responsible for binding identity.
- **Notes:** Coordinate with `PAR`, `RES`, and `TOOL` scorecards. Exact intermediate representation and pass ordering are not yet approved.

### CHK-PIPE-02 — Type information attached to resolved expressions

- **Status:** Proposed
- **Spec:** None
- **Done when:** All supported expressions and bindings can expose inferred or checked types to subsequent compiler passes, without requiring backends to re-implement inference.
- **Notes:** Representation choice (annotated AST, side table, or typed IR) remains to be decided.

### CHK-PIPE-03 — Type algebra boundary

- **Status:** Proposed
- **Spec:** None
- **Done when:** The checker delegates union, intersection, difference, complement, normalization, subtype and assignability operations to independently testable type logic instead of embedding those algorithms in syntax traversal.
- **Notes:** Existing `TYP-UNI-*`, `TYP-NOM-*`, and `TYP-REF-*` items own semantics.

## Inference and checking

### CHK-INF-01 — Literal and declaration inference

- **Status:** Proposed
- **Spec:** [evaluation.md](../spec/evaluation.md)
- **Done when:** Checker tests cover exact literal types, immutable bindings, declaration constraints that preserve inferred precision, and known compile-time constant expressions.
- **Notes:** Follow existing `TYP` inference and evaluation items; do not introduce new inference rules here.

### CHK-INF-02 — Function and call inference

- **Status:** Proposed
- **Spec:** [evaluation.md](../spec/evaluation.md)
- **Done when:** Checker tests cover parameter constraints, default unconstrained parameters, type preservation through calls, symbolic return inference, and supported return/call relationships.
- **Notes:** Depends on `TYP-FN-*`; outstanding advanced inference choices remain in the existing scorecard.

### CHK-INF-03 — Type compatibility and constructor checking

- **Status:** Proposed
- **Spec:** [syntax-and-types.md](../spec/syntax-and-types.md)
- **Done when:** Checker tests validate assignments, applications, nominal constructor arguments and identities, transparent aliases, and type-algebra-based assignability.
- **Notes:** Depends on `TYP-NOM-*`, `TYP-ALI-*`, and related type rules.

### CHK-INF-04 — Object, optional, array and export typing

- **Status:** Proposed
- **Spec:** [modules.md](../spec/modules.md)
- **Done when:** Checker integration tests cover implemented object shapes, optional key presence, array element types when approved, export kind classification, and guaranteed export-key availability.
- **Notes:** Depends on `TYP-OPT-*`, `TYP-ARR-01`, `TYP-OBJ-*`, `TYP-EXP-*` and `RES-IMP-*`; undecided language rules remain open in their original items.

## Refinements and flow

### CHK-REF-01 — Refinement proofs and validation

- **Status:** Proposed
- **Spec:** [evaluation.md](../spec/evaluation.md)
- **Done when:** Checker tests distinguish proven membership, proven nonmembership, and inconclusive proof, enforce type restrictions, and assign the approved stable result type to `as?`.
- **Notes:** Delegates proof semantics to `TYP-REF-*`, including the approved solver boundary.

### CHK-REF-02 — Flow-sensitive narrowing and pattern coverage

- **Status:** Proposed
- **Spec:** [evaluation.md](../spec/evaluation.md)
- **Done when:** Checker tests carry refinements through supported conditions and patterns and apply approved exhaustiveness and unreachable-branch rules.
- **Notes:** Depends on `TYP-REF-08/09`, `TYP-PAT-01`, and parser pattern grammar.

## Diagnostics and tests

### CHK-DIAG-01 — Structured type diagnostics

- **Status:** Proposed
- **Spec:** None
- **Done when:** Checker failures include stable error categories, source locations, and useful expected-versus-actual type details through the compiler's shared diagnostic mechanism.
- **Notes:** Respect `TYP-DIAG-*` decisions; sharing a diagnostic infrastructure with lexer/parser/resolver should not conflate error ownership.

### CHK-TEST-01 — Conformance and regression coverage

- **Status:** Proposed
- **Spec:** None
- **Done when:** Automated positive and negative checker tests cover each implemented checker item, including multi-module cases, and the scorecard records passing test evidence before any item moves to Implemented.
- **Notes:** Coordinate with `TOOL` conformance infrastructure; tests must distinguish parser, resolution, and checker diagnostics.
