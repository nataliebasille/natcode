# Resolver scorecard

- **Prefix:** RES
- **Covers:** Connecting names to bindings: identifier categories, scopes, shadowing, closures, the module and import graph, and module export interfaces.

## Identifier categories

### RES-NAME-01 — Value names are lowercase-leading

- **Status:** Approved
- **Spec:** [variables.md — Identifier categories](../spec/variables.md#identifier-categories)
- **Done when:** Lowercase-leading and uncased Unicode value names (and admitted emoji, per LEX-ID-04) bind and resolve; an uppercase-leading ordinary local value name is rejected.

### RES-NAME-02 — Types and constructors are PascalCase

- **Status:** Approved
- **Spec:** [variables.md — Identifier categories](../spec/variables.md#identifier-categories)
- **Done when:** PascalCase type and constructor names resolve; emoji and UPPER_SNAKE_CASE type names are rejected.

### RES-NAME-03 — Namespace aliases are PascalCase

- **Status:** Approved
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** `import Math` binds a namespace; a lowercase namespace alias is rejected.

### RES-NAME-04 — Top-level UPPER_SNAKE_CASE values

- **Status:** Approved
- **Spec:** [variables.md — Identifier categories](../spec/variables.md#identifier-categories)
- **Done when:** `MAX_SIZE` and `maxSize` are both legal module-level values; UPPER_SNAKE_CASE local values are rejected.

### RES-NAME-05 — Precise case classification

- **Status:** Open question
- **Spec:** [variables.md — Identifier categories](../spec/variables.md#identifier-categories)
- **Questions:**
  1. Is an underscore-leading name such as `_private` a valid value name?
  2. How are all-caps words and single capitals classified, for example `PI` (used as a value in examples) or `A`: UPPER_SNAKE_CASE value, or PascalCase type/namespace?
  3. How are titlecase letters (such as `ǅ`) and uncased scripts classified for type and namespace names?

## Scopes and bindings

### RES-SCOPE-01 — New immutable binding per declaration

- **Status:** Approved
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** A later same-name declaration produces a separate symbol, not a write to the previous one.

### RES-SCOPE-02 — Same-scope redeclaration

- **Status:** Approved
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** `x = 42` followed by `x = "hello"` in the same scope is valid, and later lookups resolve to the new symbol.

### RES-SCOPE-03 — Initializer sees the previous binding

- **Status:** Approved
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** In `x = 10` then `x = x + 5`, the initializer's `x` resolves to the first binding.

### RES-SCOPE-04 — Nested shadowing restores the outer binding

- **Status:** Approved
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** After an indented nested scope shadows `x`, an outer-scope reference resolves to the original `x`.
- **Notes:** Needs a block construct (PAR-BLK-02).

### RES-SCOPE-05 — Sequential visibility

- **Status:** Approved
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** A reference to a name declared later is rejected; earlier bindings resolve.

### RES-SCOPE-06 — Self-recursive functions

- **Status:** Approved
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** A function binding can refer to itself, while an ordinary value initializer cannot.

### RES-SCOPE-07 — Recursion details

- **Status:** Open question
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Questions:**
  1. What exactly counts as a function binding for the self-recursion exception (a lambda initializer only, or any function-typed expression)?
  2. Is mutual recursion between bindings allowed, and how?

### RES-SCOPE-08 — Closures capture binding identities

- **Status:** Approved
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** In `x = 10; getX = () => x; x = 20`, `getX`'s `x` resolves to the first binding.

### RES-SCOPE-09 — Local declarations shadow imports

- **Status:** Approved
- **Spec:** [modules.md — Import names and shadowing](../spec/modules.md#import-names-and-shadowing)
- **Done when:** References before a shadowing local declaration resolve to the import, and references after it resolve to the local.

## Module graph

### RES-MOD-01 — One module per file

- **Status:** Approved
- **Spec:** [modules.md — What is a module?](../spec/modules.md#what-is-a-module)
- **Done when:** Each `.nat` file is its own module namespace with no module header.

### RES-MOD-02 — Relative path resolution

- **Status:** Approved
- **Spec:** [modules.md — Paths and dependencies](../spec/modules.md#paths-and-dependencies)
- **Done when:** `./` and `../` paths resolve against the importing file's directory.

### RES-MOD-03 — Optional `.nat` suffix

- **Status:** Approved
- **Spec:** [modules.md — Paths and dependencies](../spec/modules.md#paths-and-dependencies)
- **Done when:** `./math` resolves to `math.nat`; non-Natcore files require their explicit extension.

### RES-MOD-04 — Missing module diagnostics

- **Status:** Proposed
- **Spec:** None
- **Done when:** An import of a path with no matching file reports a diagnostic naming the path and import location.

### RES-MOD-05 — Circular dependencies rejected

- **Status:** Approved
- **Spec:** [modules.md — Paths and dependencies](../spec/modules.md#paths-and-dependencies)
- **Done when:** `a.nat → b.nat → a.nat` and longer cycles are rejected with an error showing the cycle path.

### RES-MOD-06 — Executable modules are not importable

- **Status:** Approved
- **Spec:** [modules.md — Entry modules](../spec/modules.md#entry-modules)
- **Done when:** Importing a module whose terminal export is a function is rejected.
- **Notes:** Relies on export classification (TYP-EXP-01).

### RES-MOD-07 — Built-in and package import specifiers

- **Status:** Open question
- **Spec:** [modules.md — Paths and dependencies](../spec/modules.md#paths-and-dependencies)
- **Questions:**
  1. How are built-in modules and external packages named in import paths?

### RES-MOD-08 — JSON imports

- **Status:** Open question
- **Spec:** [modules.md — Paths and dependencies](../spec/modules.md#paths-and-dependencies)
- **Questions:**
  1. JSON imports are supported in principle. How are JSON values represented in Natcore, and what import forms apply?

### RES-MOD-09 — Other non-Natcore file formats

- **Status:** Open question
- **Spec:** [modules.md — Paths and dependencies](../spec/modules.md#paths-and-dependencies)
- **Questions:**
  1. Which other file formats can be imported, and how do they map to Natcore values?

## Imports

### RES-IMP-01 — Named imports bind locals

- **Status:** Approved
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** `{ PI }` binds a local `PI` to the module's exported `PI`.

### RES-IMP-02 — Namespace imports

- **Status:** Approved
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** `import Math` binds `Math` to the module's exported namespace, and `Math.PI` resolves.

### RES-IMP-03 — Wildcard imports

- **Status:** Approved
- **Spec:** [modules.md — Exported object shapes and optional properties](../spec/modules.md#exported-object-shapes-and-optional-properties)
- **Done when:** `import *` binds every statically possible export name as a local, including conditionally present keys.

### RES-IMP-04 — Combined import removes named keys from the namespace

- **Status:** Approved
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** `{ PI }, Math` binds `PI` locally, and `Math.PI` is absent from that namespace.

### RES-IMP-05 — Import aliases

- **Status:** Approved
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** `{ formatText = format }` binds `formatText` locally to the exported `format`.

### RES-IMP-06 — Repeated imports resolve independently

- **Status:** Approved
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** Separate `{ PI }` and `Math` imports of one module both resolve, and `Math.PI` remains present.

### RES-IMP-07 — Duplicate imported names rejected

- **Status:** Approved
- **Spec:** [modules.md — Import names and shadowing](../spec/modules.md#import-names-and-shadowing)
- **Done when:** Two imports introducing the same local name are an error, even from the same source binding or when unused; distinct aliases are accepted.

### RES-IMP-08 — Imports of never-present keys rejected

- **Status:** Approved
- **Spec:** [modules.md — Exported object shapes and optional properties](../spec/modules.md#exported-object-shapes-and-optional-properties)
- **Done when:** A named import of a key absent from every possible export shape is an error.
- **Notes:** Relies on export shape inference (TYP-EXP-05).

### RES-IMP-09 — Conflicts within one compound import

- **Status:** Open question
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Questions:**
  1. How are aliases and name conflicts handled inside a single compound import?

## Export interfaces

### RES-EXP-01 — Equivalent export object forms

- **Status:** Approved
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** `export api` (an object) and `export { PI, square }` expose identical named members, not an `api` member.

### RES-EXP-02 — Types and values in one export

- **Status:** Approved
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** `export { Point, origin }` exposes both a type and a value through the same import system.

### RES-EXP-03 — Export aliases

- **Status:** Approved
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** `export { squareNumber = square }` exposes `squareNumber` and not `square`.

### RES-EXP-04 — Duplicate public keys rejected

- **Status:** Approved
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** Two export entries producing the same public key are an error.

### RES-EXP-05 — Re-exports

- **Status:** Approved
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** A module can export imported declarations and whole imported namespaces as nested PascalCase members.

### RES-EXP-06 — Imported function as entry

- **Status:** Approved
- **Spec:** [modules.md — Entry modules](../spec/modules.md#entry-modules)
- **Done when:** `from "./app" import { run }` followed by `export run` makes an executable module.

## Extensions

### RES-EXT-01 — Extension function resolution

- **Status:** Open question
- **Spec:** [modules.md — What is a module?](../spec/modules.md#what-is-a-module)
- **Questions:**
  1. How are functions associated with types selected, and how are conflicts resolved?
  2. What are their visibility and import rules?
