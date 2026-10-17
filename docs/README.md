# Natcore documentation

## Specification

The language semantics live in [`spec/`](spec/). [`natcore-spec.md`](natcore-spec.md) is a single-file copy generated from them with `python docs/tools/build_spec.py`.

- [Lexical structure](spec/lexical.md)
- [Broader syntax and type-system decisions](spec/syntax-and-types.md)
- [Variable declarations and identifiers](spec/variables.md)
- [Modules, imports, exports, and program entry](spec/modules.md)
- [Type inference, lazy evaluation, and debugging](spec/evaluation.md)

Spec files describe semantics only. Examples may use **illustrative syntax** where a grammar is still an open question; the spec says so wherever that applies. Later decisions supersede earlier contradictory ones (for example, same-scope redeclaration is allowed).

## Scorecards

Every system has a [scorecard](scorecards/README.md) listing its atomic items. Each item records its status (Proposed, Open question, Approved, In progress, or Implemented), its open questions, and its test evidence. The scorecards are the only record of what is decided and what is built.

```
python docs/tools/scorecard_status.py           # status counts per system
python docs/tools/scorecard_status.py --check   # validate scorecards, links, evidence, and the merged spec
```

## Design snapshot

- Natcore is a functional language with immutable bindings and persistent-value updates.
- A source file defines exactly one module; modules are logical, scoped namespaces.
- Top-level source contains imports, declarations, and one final `export`, not standalone statements.
- Module exports are either an object-shaped library interface or an executable function. Export kind is statically decidable.
- Imports use `from "relative/path" import ...`; imports are static, first in the file, and may re-export through the module's final export.
- Lazy values are evaluated on demand and memoized, including independent object properties.
