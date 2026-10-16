# Natcore implementation scorecard

> **Progress tracker, not an additional language specification.** The checklist comes from approved decisions in `spec/`. No tasks are pre-checked: this package does not inspect a Natcore code repository.

## Three independent completion tracks

1. **Language feature conformance — this file (62 approved-rule acceptance checks).** These checks certify the specified behavior with automated front-end tests, and where execution is needed, a verified reference implementation. They do **not** mean every code-generation backend supports the feature.
2. **[Infrastructure readiness](implementation/infrastructure.md).** Tracks the lexer, parser, resolver, type system, evaluation engine, native interpreter, code generators, and test tooling independently. These are engineering milestones rather than new approved language rules.
3. **[Backend coverage matrix](implementation/backend-coverage.md).** Tracks individual executable feature behavior for the native interpreter, JavaScript, WebAssembly, C, and prospective Python targets. Target results are independent from this checklist.

To view all three metrics, run `python3 tools/scorecard_status.py`.

## Agent instructions

1. Treat `spec/*.md` as the semantic authority; this scorecard describes the implementable acceptance tests, not new syntax or design decisions.
2. Change `- [ ]` to `- [x]` **only after** its required compiler/semantic behavior is verified by automated tests. For runtime semantics, verify the behavior through a reference execution path; do **not** require all backends to pass before checking this language-level item. Backend support is scored independently. Do not check items merely because the *specification* is approved.
3. Include a brief record in the **Completion evidence** table: item ID, test path or test command, and commit/PR. If tests cannot run, leave the item unchecked.
4. If an item depends on an undecided grammar or runtime feature, keep it unchecked and note the blocker; do not invent semantics to finish it.
5. Keep existing IDs stable. Add new approved decisions with new IDs; never silently rewrite approved acceptance criteria.
6. To get live totals, run `python3 tools/scorecard_status.py` from the package root; run with `--json` for machine-readable output or `--check` for validation.
7. For runtime behavior, maintain tests under a shared `tests/conformance/` suite (recommended layout, not a spec mandate). Link each passing test to its feature ID and record evidence.

**Scoring:** one point per checked language acceptance item; partially implemented items remain unchecked. Infrastructure and backend coverage use their **own denominators**. Do not combine the three percentages into an overall score. Some items remain blocked pending separate specifications.

## Progress summary

Run `python3 tools/scorecard_status.py` to calculate current totals from the checkboxes (do not maintain percentages manually).

| Area | Items | Verified |
| --- | ---: | ---: |
| Syntax and identifiers | 8 | 0 |
| Bindings, inference, and scoping | 17 | 0 |
| Module structure and exports | 17 | 0 |
| Import resolution and namespace values | 14 | 0 |
| Lazy evaluation and debugging | 6 | 0 |
| **Total** | **62** | **0** |

*The Verified column is an initial snapshot only; the script is the current score after agents start checking boxes.*

## Verification checklist

### Syntax and identifiers

- [ ] **SYN-01 — Top-level files accept declarations and reject standalone expressions**
  - **Done when:** Parse a top-level binding, import, or export; reject a standalone top-level call.  ([approved spec](spec/modules.md))
- [ ] **SYN-02 — Indentation creates code scopes; braces do not create statement blocks**
  - **Done when:** Nested scopes parse using indentation; braces are used for data constructs, not blocks.  ([approved spec](spec/syntax-and-types.md))
- [ ] **ID-01 — Lowercase-leading Unicode ordinary value names**
  - **Done when:** Valid lowercase/uncased Unicode value names bind and resolve; uppercase ordinary local value names are rejected.  ([approved spec](spec/variables.md))
- [ ] **ID-02 — Emoji value identifiers, including composed emoji**
  - **Done when:** An emoji and a multi-codepoint sequence such as 👩‍💻 can each declare and reference a value; neither can name a type.  ([approved spec](spec/variables.md))
- [ ] **ID-03 — PascalCase names for types, constructors, and imported namespaces**
  - **Done when:** Valid PascalCase names resolve in their respective roles; lowercase namespace aliases are rejected.  ([approved spec](spec/variables.md))
- [ ] **ID-04 — Allow top-level UPPER_SNAKE_CASE without requiring it**
  - **Done when:** Both MAX_SIZE and maxSize are legal module-level values; UPPER_SNAKE_CASE remains invalid for types and local values.  ([approved spec](spec/variables.md))
- [ ] **ID-05 — Reserve standalone underscore as a wildcard**
  - **Done when:** `_` cannot be bound as a normal variable identifier.  ([approved spec](spec/variables.md))
- [ ] **ID-06 — Collision-safe identifier generation**
  - **Done when:** When emitting a target language, unsupported source identifiers (including emoji) are mangled without collisions and retain debug/source mapping.  ([approved spec](spec/variables.md))

### Bindings, inference, and scoping

- [ ] **VAR-01 — Keyword-free declarations with required initializer**
  - **Done when:** `x = 42` and `x: number = 42` parse; `x: number` alone is rejected.  ([approved spec](spec/variables.md))
- [ ] **VAR-02 — Optional colon constraints check assignability**
  - **Done when:** `x: number = 42` passes; `x: string = 42` fails; an annotation never widens the resulting inferred type.  ([approved spec](spec/variables.md))
- [ ] **VAR-03 — New immutable binding on each declaration**
  - **Done when:** A later same-name declaration produces a separate symbol, not a write to previous storage.  ([approved spec](spec/variables.md))
- [ ] **VAR-04 — Allow same-scope redeclarations with unrelated types**
  - **Done when:** `x = 42; x = "hello"` is valid, and later lookups use the new symbol and inferred type.  ([approved spec](spec/variables.md))
- [ ] **VAR-05 — Initializer resolves an earlier same-name binding**
  - **Done when:** `x = 10; x = x + 5` yields 15 and reads the original x.  ([approved spec](spec/variables.md))
- [ ] **VAR-06 — Nested shadowing restores outer visibility after exit**
  - **Done when:** After an indented nested scope shadows x, an outer-scope reference still uses the original x.  ([approved spec](spec/variables.md))
- [ ] **VAR-07 — Sequential lexical visibility**
  - **Done when:** Ordinary references to names declared later are rejected; prior bindings remain resolvable.  ([approved spec](spec/variables.md))
- [ ] **VAR-08 — Allow self-recursive function binding**
  - **Done when:** A function can refer to its own binding without making arbitrary value initializers self-recursive.  ([approved spec](spec/variables.md))
- [ ] **VAR-09 — Capture lexical binding identities in closures**
  - **Done when:** `x = 10; getX = () => x; x = 20; getX()` resolves to the first x.  ([approved spec](spec/variables.md))
- [ ] **VAR-10 — Permit a local declaration to shadow an imported name**
  - **Done when:** An imported binding resolves before the shadowing local declaration, and the new binding resolves afterward.  ([approved spec](spec/variables.md))
- [ ] **TYPE-01 — Exact primitive literal type inference**
  - **Done when:** Bindings initialized with 42, "Alice", true retain their respective literal types.  ([approved spec](spec/variables.md))
- [ ] **TYPE-02 — Exact constant-expression type inference**
  - **Done when:** Known pure constant arithmetic such as `40 + 2` infers literal `42` when evaluation succeeds.  ([approved spec](spec/evaluation.md))
- [ ] **TYPE-03 — Union `|` and intersection `&` types**
  - **Done when:** Check unions/intersections as type operations distinct from colon constraints.  ([approved spec](spec/syntax-and-types.md))
- [ ] **TYPE-04 — Sound, correlation-preserving union simplification**
  - **Done when:** A discriminated record union is never broadened into invalid cross-combinations of fields.  ([approved spec](spec/syntax-and-types.md))
- [ ] **TYPE-05 — Context-independent function return inference**
  - **Done when:** With parameter number, x * 2 infers number; with parameter literal 24, result infers 48 regardless of call-site values.  ([approved spec](spec/evaluation.md))
- [ ] **TYPE-06 — Propagate inferred types through function-local declarations**
  - **Done when:** A function with x: 24 and a local multiplier = 2 can infer literal return 48.  ([approved spec](spec/evaluation.md))
- [ ] **TYPE-07 — Best-effort bounded compile-time evaluation**
  - **Done when:** Budget exhaustion falls back to sound static inference rather than failing compilation solely for budget exhaustion.  ([approved spec](spec/evaluation.md))

### Module structure and exports

- [ ] **MOD-01 — Exactly one module per Natcore source file**
  - **Done when:** Each .nat file is its own module namespace; no explicit module header required.  ([approved spec](spec/modules.md))
- [ ] **MOD-02 — Exactly one final export declaration**
  - **Done when:** `export {}` is valid, missing exports or any declaration after export are errors, and multiple exports are errors.  ([approved spec](spec/modules.md))
- [ ] **MOD-03 — Export object => library, function => executable**
  - **Done when:** Statically classify exported object and function expressions; allow path-selected entry module.  ([approved spec](spec/modules.md))
- [ ] **MOD-04 — Reject invalid module export kinds**
  - **Done when:** Direct primitive and direct type exports are rejected; types/primitives within an exported object are accepted.  ([approved spec](spec/modules.md))
- [ ] **MOD-05 — Accept arbitrary export expressions of proven valid kind**
  - **Done when:** An export of a function call expression is accepted when its inferred result is provably object-only or function-only.  ([approved spec](spec/modules.md))
- [ ] **MOD-06 — Reject ambiguous object-or-function export kind**
  - **Done when:** An export expression whose type can produce either an object or a function is a compile error.  ([approved spec](spec/modules.md))
- [ ] **MOD-07 — Equivalent exported-object forms**
  - **Done when:** `export API` for an object and `export { PI, square }` expose identical named members.  ([approved spec](spec/modules.md))
- [ ] **MOD-08 — Allow exported objects to include types and values**
  - **Done when:** `export { Point, origin }` exposes both via the same import system.  ([approved spec](spec/modules.md))
- [ ] **MOD-09 — Support public export aliases with `=`**
  - **Done when:** `export { squareNumber = square }` exposes squareNumber, not a separate square name.  ([approved spec](spec/modules.md))
- [ ] **MOD-10 — Reject duplicate public export property names**
  - **Done when:** Two aliases producing the same public key result in a compile error.  ([approved spec](spec/modules.md))
- [ ] **MOD-11 — Re-export imported values and namespaces**
  - **Done when:** A module may export imported declarations and nested PascalCase namespace objects.  ([approved spec](spec/modules.md))
- [ ] **MOD-12 — Executable modules are not importable**
  - **Done when:** Importing a module with a function-valued terminal export is rejected.  ([approved spec](spec/modules.md))
- [ ] **MOD-13 — Allow an imported function as executable entry**
  - **Done when:** A module may `from "./app" import { run }` then `export run`.  ([approved spec](spec/modules.md))
- [ ] **MOD-14 — Resolve entry module by its file path**
  - **Done when:** More than one executable module may exist; selecting one path runs only its entry. Entry signature remains deferred.  ([approved spec](spec/modules.md))
- [ ] **MOD-15 — Statically enumerate all possible exported keys**
  - **Done when:** Union-shaped objects contribute every possible key; unknown/arbitrary top-level export key sets are rejected.  ([approved spec](spec/modules.md))
- [ ] **MOD-16 — Conditionally present exported properties include absence**
  - **Done when:** An export property present in some variants is named-importable with an inferred None-like case; exact None representation is deferred.  ([approved spec](spec/modules.md))
- [ ] **MOD-17 — Reject imports of never-present exported keys**
  - **Done when:** Named import of a key absent from every possible export shape is a compile error.  ([approved spec](spec/modules.md))

### Import resolution and namespace values

- [ ] **IMP-01 — File-first import syntax and mandatory import clause**
  - **Done when:** Support `from "./math" import ...`; reject bare `from "./math"`.  ([approved spec](spec/modules.md))
- [ ] **IMP-02 — Imports only at beginning of module**
  - **Done when:** Reject import statements following ordinary declarations.  ([approved spec](spec/modules.md))
- [ ] **IMP-03 — Require static string-literal import paths**
  - **Done when:** Computed identifiers or runtime expressions cannot stand in for file paths.  ([approved spec](spec/modules.md))
- [ ] **IMP-04 — Resolve explicit relative paths and optional `.nat` suffix**
  - **Done when:** Relative `./`, `../` paths resolve against importing file, and `./math` resolves math.nat.  ([approved spec](spec/modules.md))
- [ ] **IMP-05 — Named imports and PascalCase namespace imports**
  - **Done when:** Both `{ PI }` and `Math` resolve; lowercase namespace alias is invalid.  ([approved spec](spec/modules.md))
- [ ] **IMP-06 — Wildcard `*` imports all eligible public keys**
  - **Done when:** Wildcard adds all statically possible export names, including conditionally present keys; `_` is not a wildcard import.  ([approved spec](spec/modules.md))
- [ ] **IMP-07 — Combined selective import plus remaining namespace**
  - **Done when:** `from "./math" import { PI }, Math` imports PI locally and excludes PI from Math.  ([approved spec](spec/modules.md))
- [ ] **IMP-08 — Import aliases use left-hand local name**
  - **Done when:** `{ formatText = format }` binds formatText locally from exported format.  ([approved spec](spec/modules.md))
- [ ] **IMP-09 — Allow independent repeated imports of same module**
  - **Done when:** Separate named and namespace imports each resolve the full source exports independently.  ([approved spec](spec/modules.md))
- [ ] **IMP-10 — Reject duplicate imported local names**
  - **Done when:** Duplicate local import names are compile errors even from same source binding or if unused; distinct aliases work.  ([approved spec](spec/modules.md))
- [ ] **IMP-11 — Reject circular module dependencies**
  - **Done when:** Reject a.nat -> b.nat -> a.nat and longer cycles with a helpful error path.  ([approved spec](spec/modules.md))
- [ ] **IMP-12 — First-class imported namespace values**
  - **Done when:** A namespace may be assigned, passed to functions, returned, or exported as a nested namespace.  ([approved spec](spec/modules.md))
- [ ] **IMP-13 — Shared original namespace across multiple imports**
  - **Done when:** Separate imports reference the same original immutable namespace value without independently reinitializing the module.  ([approved spec](spec/modules.md))
- [ ] **IMP-14 — Persistent dotted namespace update semantics**
  - **Done when:** `Math.PI = value` creates a new Math version; aliases and other imports retain original values. Detailed object update grammar is deferred.  ([approved spec](spec/modules.md))

### Lazy evaluation and debugging

- [ ] **EVAL-01 — Demand-driven top-level binding evaluation**
  - **Done when:** Unused top-level initializers are not evaluated simply because their module is imported.  ([approved spec](spec/evaluation.md))
- [ ] **EVAL-02 — Memoize lazy bindings after first demand**
  - **Done when:** Repeated use of a binding runs its initializer at most once and reuses the immutable result.  ([approved spec](spec/evaluation.md))
- [ ] **EVAL-03 — Independent lazy object-member evaluation**
  - **Done when:** Accessing exported cheap does not force unrelated expensiveCalculation() property.  ([approved spec](spec/evaluation.md))
- [ ] **EVAL-04 — Preserve laziness through object-producing functions**
  - **Done when:** A factory may return an object with still-lazy property computations; unused calculations do not run.  ([approved spec](spec/evaluation.md))
- [ ] **DBG-01 — Debug builds retain lazy execution semantics**
  - **Done when:** Debug mode never auto-forces expressions solely to mimic eager stepping; suspended work is identified.  ([approved spec](spec/evaluation.md))
- [ ] **DBG-02 — Debugger can explicitly force suspended computation**
  - **Done when:** A debugging action can step into/force a suspended call while preserving ordinary default lazy behavior.  ([approved spec](spec/evaluation.md))

## Completion evidence

Add a row whenever an agent checks an item. If implementation is partly complete, leave the box unchecked and put the blocker in the notes instead.

| Item ID | Test(s) / command(s) | Commit or PR | Notes |
| --- | --- | --- | --- |
| — | — | — | No implementation has been verified by this package. |

## Deferred decisions — not implementation checkboxes

The following subjects still need language-design approval. Agents must not assume a particular answer or mark them implemented on the authority of this checklist alone.

- Entry function parameter and effect/purity contract.
- Exact `None` representation and subtype/type checking rules for optional exports.
- Final grammar for objects, functions, combinations of wildcard imports, and Unicode emoji admission.
- JSON import value representation and rules for non-Natcore file formats.
- Built-in and external package import specifiers.
- General persistent object updates, effect-safe evaluation of lazy computations, and debugging APIs.
- See [full deferred decisions](spec/deferred.md) for the current unresolved list.

**Future spec work is not scored as implemented functionality.** When a new rule is approved, add a new acceptance item and its tests before implementing it.
