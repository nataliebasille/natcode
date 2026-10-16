# Natcore language specification — decision snapshot

**Status:** Working language design, capturing decisions approved in conversation through 2026-10-16. This is not a claim that any compiler behavior has been implemented.

## Contents

- [Language feature implementation scorecard (62 approved requirements)](SCORECARD.md) — track verified semantics and acceptance tests
- [Infrastructure implementation checklist](implementation/infrastructure.md) — track lexer, parser, resolver, types, evaluator, interpreter, code generators, debugging, and test harness
- [Backend coverage matrix](implementation/backend-coverage.md) — independently track Interpreter, JavaScript, WebAssembly, C, and prospective Python conformance
- [Variable declarations and identifiers](spec/variables.md)
- [Modules, imports, exports, and program entry](spec/modules.md)
- [Inference, evaluation, and debugger decisions](spec/evaluation.md)
- [Deferred questions and boundary notes](spec/deferred.md)

The combined document `natcore-spec.md` in the download is a single-file version of the approved specification sections. The scorecard is tracked separately because implementation progress is not itself a language rule.

Run `python3 tools/scorecard_status.py` to compute **three separate progress axes**: language conformance, infrastructure readiness, and per-target backend coverage. For automation, use `--json`; to additionally validate traceability for checked items, use `--check`. Agents should mark an item or matrix cell complete only after implementing it, running passing tests, and recording evidence. The package intentionally starts with nothing verified.

Suggested repository layout for tests (examples, not mandatory): `tests/lexer/`, `tests/parser/`, `tests/resolver/`, `tests/types/`, and `tests/conformance/<feature-id>/`, with a runner executing the same conformance cases across backends. No test suite or backend implementation is bundled or claimed to exist.

## How decisions are treated

Statements under **Approved** are decisions made by the language designer. An example may still use **illustrative syntax** for blocks, functions, or object types where those grammars have not yet been approved. **Deferred** means not yet approved. Later decisions supersede earlier contradictory ones (for example, same-scope redeclaration is allowed).

## Snapshot of the design

- Natcore is a functional language with immutable bindings and persistent-value updates.
- A source file defines exactly one module; modules are logical, scoped namespaces.
- Top-level source contains imports, declarations, and one final `export`, not standalone statements.
- Module exports are either an object-shaped library interface or an executable function. Export kind is statically decidable.
- Imports use `from "relative/path" import ...`; imports are static, first in the file, and may re-export through the module's final export.
- Lazy values are evaluated on demand and memoized, including independent object properties.

These files are a snapshot of the discussion and are not automatically synchronized with a Git repository.
