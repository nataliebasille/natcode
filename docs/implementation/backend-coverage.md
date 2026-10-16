# Natcore backend conformance coverage

> **Independent target results; no backend support has been verified.** This matrix tracks execution-target conformance separately from language-feature implementation and infrastructure. All targets listed are project goals; Python is prospective. Do not infer a target is implemented simply because it appears in this matrix.

## How to mark cells

- `☐` = applicable, not verified. Replace with `☑` **only** after the feature's shared conformance test passes on that specific target, then record feature ID, target, test path/command and commit/PR below.
- `—` = no separate target-specific verification required: the row is a front-end/static semantic rule, or (for identifier mangling) not applicable to the native interpreter. Its language-level implementation is still tracked in [SCORECARD.md](../SCORECARD.md).
- A feature checked in `../SCORECARD.md` **does not check any target cell automatically**. Likewise, passing on JavaScript does not imply the native interpreter, C, WebAssembly, or Python passes.
- For blocked or partial work, leave the cell `☐` and describe the blocker in the evidence/notes table. If a target is not yet started, leave every applicable cell unchecked.
- Targets should execute the **same source-level conformance cases** when feasible, comparing resulting values, missing-value behavior, errors and laziness (rather than generated code text). A test can be skipped only with a recorded reason.

## Coverage by approved requirement

All **62** language-feature IDs are included so nothing is silently omitted. Rows with only `—` are verified in shared lexer/parser/resolver/type tests, not re-implemented by each code generator. This avoids artificially multiplying front-end-only tasks by five.

| Feature ID | Approved behavior / requirement | Interpreter | JavaScript | WebAssembly | C | Python |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| `SYN-01` | Top-level files accept declarations and reject standalone expressions | — | — | — | — | — |
| `SYN-02` | Indentation creates code scopes; braces do not create statement blocks | — | — | — | — | — |
| `ID-01` | Lowercase-leading Unicode ordinary value names | — | — | — | — | — |
| `ID-02` | Emoji value identifiers, including composed emoji | ☐ | ☐ | ☐ | ☐ | ☐ |
| `ID-03` | PascalCase names for types, constructors, and imported namespaces | — | — | — | — | — |
| `ID-04` | Allow top-level UPPER_SNAKE_CASE without requiring it | — | — | — | — | — |
| `ID-05` | Reserve standalone underscore as a wildcard | — | — | — | — | — |
| `ID-06` | Collision-safe identifier generation | — | ☐ | ☐ | ☐ | ☐ |
| `VAR-01` | Keyword-free declarations with required initializer | — | — | — | — | — |
| `VAR-02` | Optional colon constraints check assignability | — | — | — | — | — |
| `VAR-03` | New immutable binding on each declaration | ☐ | ☐ | ☐ | ☐ | ☐ |
| `VAR-04` | Allow same-scope redeclarations with unrelated types | ☐ | ☐ | ☐ | ☐ | ☐ |
| `VAR-05` | Initializer resolves an earlier same-name binding | ☐ | ☐ | ☐ | ☐ | ☐ |
| `VAR-06` | Nested shadowing restores outer visibility after exit | ☐ | ☐ | ☐ | ☐ | ☐ |
| `VAR-07` | Sequential lexical visibility | ☐ | ☐ | ☐ | ☐ | ☐ |
| `VAR-08` | Allow self-recursive function binding | ☐ | ☐ | ☐ | ☐ | ☐ |
| `VAR-09` | Capture lexical binding identities in closures | ☐ | ☐ | ☐ | ☐ | ☐ |
| `VAR-10` | Permit a local declaration to shadow an imported name | ☐ | ☐ | ☐ | ☐ | ☐ |
| `TYPE-01` | Exact primitive literal type inference | — | — | — | — | — |
| `TYPE-02` | Exact constant-expression type inference | — | — | — | — | — |
| `TYPE-03` | Union `\|` and intersection `&` types | — | — | — | — | — |
| `TYPE-04` | Sound, correlation-preserving union simplification | — | — | — | — | — |
| `TYPE-05` | Context-independent function return inference | — | — | — | — | — |
| `TYPE-06` | Propagate inferred types through function-local declarations | — | — | — | — | — |
| `TYPE-07` | Best-effort bounded compile-time evaluation | — | — | — | — | — |
| `MOD-01` | Exactly one module per Natcore source file | ☐ | ☐ | ☐ | ☐ | ☐ |
| `MOD-02` | Exactly one final export declaration | — | — | — | — | — |
| `MOD-03` | Export object => library, function => executable | ☐ | ☐ | ☐ | ☐ | ☐ |
| `MOD-04` | Reject invalid module export kinds | — | — | — | — | — |
| `MOD-05` | Accept arbitrary export expressions of proven valid kind | ☐ | ☐ | ☐ | ☐ | ☐ |
| `MOD-06` | Reject ambiguous object-or-function export kind | — | — | — | — | — |
| `MOD-07` | Equivalent exported-object forms | ☐ | ☐ | ☐ | ☐ | ☐ |
| `MOD-08` | Allow exported objects to include types and values | ☐ | ☐ | ☐ | ☐ | ☐ |
| `MOD-09` | Support public export aliases with `=` | ☐ | ☐ | ☐ | ☐ | ☐ |
| `MOD-10` | Reject duplicate public export property names | — | — | — | — | — |
| `MOD-11` | Re-export imported values and namespaces | ☐ | ☐ | ☐ | ☐ | ☐ |
| `MOD-12` | Executable modules are not importable | — | — | — | — | — |
| `MOD-13` | Allow an imported function as executable entry | ☐ | ☐ | ☐ | ☐ | ☐ |
| `MOD-14` | Resolve entry module by its file path | ☐ | ☐ | ☐ | ☐ | ☐ |
| `MOD-15` | Statically enumerate all possible exported keys | — | — | — | — | — |
| `MOD-16` | Conditionally present exported properties include absence | ☐ | ☐ | ☐ | ☐ | ☐ |
| `MOD-17` | Reject imports of never-present exported keys | — | — | — | — | — |
| `IMP-01` | File-first import syntax and mandatory import clause | — | — | — | — | — |
| `IMP-02` | Imports only at beginning of module | — | — | — | — | — |
| `IMP-03` | Require static string-literal import paths | — | — | — | — | — |
| `IMP-04` | Resolve explicit relative paths and optional `.nat` suffix | — | — | — | — | — |
| `IMP-05` | Named imports and PascalCase namespace imports | ☐ | ☐ | ☐ | ☐ | ☐ |
| `IMP-06` | Wildcard `*` imports all eligible public keys | ☐ | ☐ | ☐ | ☐ | ☐ |
| `IMP-07` | Combined selective import plus remaining namespace | ☐ | ☐ | ☐ | ☐ | ☐ |
| `IMP-08` | Import aliases use left-hand local name | ☐ | ☐ | ☐ | ☐ | ☐ |
| `IMP-09` | Allow independent repeated imports of same module | ☐ | ☐ | ☐ | ☐ | ☐ |
| `IMP-10` | Reject duplicate imported local names | — | — | — | — | — |
| `IMP-11` | Reject circular module dependencies | — | — | — | — | — |
| `IMP-12` | First-class imported namespace values | ☐ | ☐ | ☐ | ☐ | ☐ |
| `IMP-13` | Shared original namespace across multiple imports | ☐ | ☐ | ☐ | ☐ | ☐ |
| `IMP-14` | Persistent dotted namespace update semantics | ☐ | ☐ | ☐ | ☐ | ☐ |
| `EVAL-01` | Demand-driven top-level binding evaluation | ☐ | ☐ | ☐ | ☐ | ☐ |
| `EVAL-02` | Memoize lazy bindings after first demand | ☐ | ☐ | ☐ | ☐ | ☐ |
| `EVAL-03` | Independent lazy object-member evaluation | ☐ | ☐ | ☐ | ☐ | ☐ |
| `EVAL-04` | Preserve laziness through object-producing functions | ☐ | ☐ | ☐ | ☐ | ☐ |
| `DBG-01` | Debug builds retain lazy execution semantics | — | — | — | — | — |
| `DBG-02` | Debugger can explicitly force suspended computation | — | — | — | — | — |

## Backend completion evidence

Record one row per verified target cell. Never check a cell without an automated passing test and a traceable implementation revision.

| Feature ID | Target | Passing test(s) / command(s) | Commit or PR | Notes |
| --- | --- | --- | --- | --- |
| — | — | — | — | No target-specific implementation verified |

## Scope note

Some coverage cases will remain blocked until source grammar, `None` semantics, runtime effects, and entry-function contracts are specified. Do not invent those behaviors just to increase the score. This matrix reports **per-target feature conformance**, not aggregate engineering readiness, performance, or certification of every backend.
