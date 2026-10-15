# Natcore Language Specification

> **Status:** In development
>
> **Purpose:** Define Natcore's language behavior for both language design and compiler implementation. Coding agents must treat the specification as authoritative only where a rule is explicitly defined and approved.

## Specification index

| Feature | Specification status | Implementation readiness | Implementation status |
| --- | --- | --- | --- |
| [Block and data delimiters](spec/syntax.md) | Partial | Blocked | Not tracked |
| [Type declarations, variants, and generics](spec/types.md) | Proposed | Blocked | Not tracked |
| [Function return types](spec/functions.md) | Proposed | Blocked | Not tracked |
| [Type inference and union equivalence](spec/type-inference.md) | Partial | Blocked | Not tracked |

Open design explorations: [Types as function-like constructs](design/type-function-relationship.md).

## Status conventions

**Specification status** describes the state of the language design:

- **Open:** Not yet designed, or a decision is needed.
- **Proposed:** A design exists, but has not been approved.
- **Partial:** Some rules are defined, but relevant details remain unresolved.
- **Defined:** The described scope has explicit, approved syntax and semantics.

**Implementation readiness** is separate from specification status:

- **Not reviewed:** No implementation-readiness assessment has occurred.
- **Blocked:** Unresolved design issues prevent implementing the specified scope safely.
- **Recommended:** An agent has assessed the scope as implementable; approval is still required.
- **Approved:** The language owner has explicitly approved the specified scope for implementation.

**Implementation status** tracks the implementation itself, separately from the design. It must not be inferred from specification status or readiness.

## Recording rules

- Document the syntax **and** meaning of a feature, including observable behavior when known.
- Distinguish approved decisions from proposals and unanswered questions.
- Mark missing, contradictory, or ambiguous behavior explicitly; do not silently fill gaps.
- Include examples of valid and invalid usage where useful and when their behavior is actually known.
- Use links to related specifications rather than duplicating authoritative rules.
- A feature may contain an independently implementable subset even when related features are incomplete, provided its boundaries and dependencies are explicit.
- An agent may recommend implementation readiness, but only the language owner may approve it.

## Document organization

- [`spec/`](spec/): Language syntax, semantics, and behavioral rules, with explicit statuses showing which portions are still proposals.
- [`design/`](design/): Open design explorations, proposals, and alternatives.
- [`compiler/`](compiler/): Compiler architecture, implementation choices, and algorithms. These do not redefine language semantics.
- [`AGENTS.md`](../AGENTS.md): Rules for coding agents working with this specification.

## Initial direction (not yet normative)

Natcore is intended to feel familiar to TypeScript developers while being more functional, generally pure, and supportive of functional features such as piping and pattern matching. This describes motivation, **not** concrete language semantics or approved feature designs.
