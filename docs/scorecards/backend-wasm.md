# WebAssembly backend scorecard

- **Prefix:** WASM
- **Covers:** Running Natcore programs on WebAssembly, and passing the shared conformance cases for every runtime-observable behavior.
- **Note:** Execution targets are project goals rather than approved decisions, so their items stay Proposed until the language owner approves the target.

## Target

### WASM-GEN-01 — WebAssembly output runs a core program

- **Status:** Proposed
- **Spec:** None
- **Done when:** A representative executable module compiles to WebAssembly, runs in a harness, and passes the initial shared conformance cases.

### WASM-MAP-01 — Module mapping, linking, and source maps

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Target directions](../spec/syntax-and-types.md#target-directions)
- **Questions:**
  1. How are Natcore modules mapped onto WebAssembly files or modules?
  2. How is the output linked or packaged?
  3. Which source-map format links generated code back to Natcore source?

## Identifiers

### WASM-NAME-01 — Emoji value names

- **Status:** Proposed
- **Spec:** [variables.md — Identifier categories](../spec/variables.md#identifier-categories)
- **Done when:** The shared conformance case passes on WebAssembly. A program that declares and reads emoji-named values, including a composed emoji such as `👩‍💻`, runs correctly.
- **Notes:** Emoji admission is LEX-ID-04.

### WASM-NAME-02 — Collision-safe identifier mangling

- **Status:** Proposed
- **Spec:** [variables.md — Identifier mangling and compilation](../spec/variables.md#identifier-mangling-and-compilation)
- **Done when:** The shared conformance case passes on WebAssembly. Source identifiers the target can't spell (including emoji) are mangled without collisions, and source names stay available for debugging and source mapping.

## Bindings and scopes

### WASM-SCOPE-01 — New binding per declaration

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on WebAssembly. A later same-name declaration doesn't change any value computed from or captured from the earlier binding.

### WASM-SCOPE-02 — Same-scope redeclaration

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on WebAssembly. `x = 42` followed by `x = "hello"` runs, and later reads see `"hello"`.

### WASM-SCOPE-03 — Initializer reads the previous binding

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on WebAssembly. `x = 10` then `x = x + 5` yields `15`.

### WASM-SCOPE-04 — Nested shadowing

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on WebAssembly. After an indented scope shadows `x`, the outer `x` keeps its original value.

### WASM-SCOPE-05 — Sequential visibility

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on WebAssembly. Reads of a redeclared name before and after the redeclaration see the respective bindings' values.

### WASM-SCOPE-06 — Self-recursive functions

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on WebAssembly. A self-recursive function (for example factorial) computes correctly.

### WASM-SCOPE-07 — Closures capture binding identities

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on WebAssembly. `x = 10; getX = () => x; x = 20; getX()` returns `10`.

### WASM-SCOPE-08 — Local declarations shadow imports

- **Status:** Proposed
- **Spec:** [modules.md — Import names and shadowing](../spec/modules.md#import-names-and-shadowing)
- **Done when:** The shared conformance case passes on WebAssembly. Reads before a shadowing local declaration see the imported value, and reads after it see the local value.

## Modules and entry points

### WASM-MOD-01 — One module per file

- **Status:** Proposed
- **Spec:** [modules.md — What is a module?](../spec/modules.md#what-is-a-module)
- **Done when:** The shared conformance case passes on WebAssembly. A multi-file program runs with each file as its own module namespace.

### WASM-MOD-02 — Library and executable modules

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on WebAssembly. An executable module runs its exported entry function, and library modules provide their exported objects to importers.

### WASM-MOD-03 — Export of a call expression

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on WebAssembly. A module exporting the result of a function call behaves as the library or executable its inferred kind dictates.

### WASM-MOD-04 — Equivalent export object forms

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on WebAssembly. `export api` and `export { PI, square }` give importers identical working members.

### WASM-MOD-05 — Types and values in one export

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on WebAssembly. Values exported alongside types are usable by importers.

### WASM-MOD-06 — Export aliases

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on WebAssembly. `export { squareNumber = square }` gives importers a working `squareNumber`.

### WASM-MOD-07 — Re-exports

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on WebAssembly. Re-exported values and namespaces are usable through the re-exporting module.

### WASM-MOD-08 — Imported entry function

- **Status:** Proposed
- **Spec:** [modules.md — Entry modules](../spec/modules.md#entry-modules)
- **Done when:** The shared conformance case passes on WebAssembly. An executable module whose entry function is imported from a library runs that function.

### WASM-MOD-09 — Entry module selected by path

- **Status:** Proposed
- **Spec:** [modules.md — Entry modules](../spec/modules.md#entry-modules)
- **Done when:** The shared conformance case passes on WebAssembly. With several executable modules, running one path runs only that module's entry function.

### WASM-MOD-10 — Conditionally present export keys

- **Status:** Proposed
- **Spec:** [modules.md — Exported object shapes and optional properties](../spec/modules.md#exported-object-shapes-and-optional-properties)
- **Done when:** The shared conformance case passes on WebAssembly. Reading an export key that is absent in the current variant yields the `None`-like value.
- **Notes:** The representation of `None` is TYP-NONE-01.

## Imports

### WASM-IMP-01 — Named and namespace imports

- **Status:** Proposed
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** The shared conformance case passes on WebAssembly. Values imported by name and through a namespace are correct.

### WASM-IMP-02 — Wildcard imports

- **Status:** Proposed
- **Spec:** [modules.md — Exported object shapes and optional properties](../spec/modules.md#exported-object-shapes-and-optional-properties)
- **Done when:** The shared conformance case passes on WebAssembly. `import *` makes every statically possible export usable by its own name.

### WASM-IMP-03 — Combined named and namespace import

- **Status:** Proposed
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** The shared conformance case passes on WebAssembly. `{ PI }, Math` provides a working `PI` and a `Math` namespace without `PI`.

### WASM-IMP-04 — Import aliases

- **Status:** Proposed
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** The shared conformance case passes on WebAssembly. `{ formatText = format }` provides a working `formatText`.

### WASM-IMP-05 — Repeated imports of one module

- **Status:** Proposed
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** The shared conformance case passes on WebAssembly. Separate named and namespace imports of one module both work, each with the full exports.

## Namespaces

### WASM-NS-01 — First-class namespaces

- **Status:** Proposed
- **Spec:** [modules.md — First-class namespaces and persistent updates](../spec/modules.md#first-class-namespaces-and-persistent-updates)
- **Done when:** The shared conformance case passes on WebAssembly. A namespace can be assigned, passed to a function, returned, and nested in another object or export, and still works.

### WASM-NS-02 — Shared namespace without reinitialization

- **Status:** Proposed
- **Spec:** [modules.md — First-class namespaces and persistent updates](../spec/modules.md#first-class-namespaces-and-persistent-updates)
- **Done when:** The shared conformance case passes on WebAssembly. Several imports of one module observe a single initialization of it.

### WASM-NS-03 — Persistent dotted updates

- **Status:** Proposed
- **Spec:** [modules.md — First-class namespaces and persistent updates](../spec/modules.md#first-class-namespaces-and-persistent-updates)
- **Done when:** The shared conformance case passes on WebAssembly. After `Math.PI = value`, the current binding sees the new value while aliases and other imports keep the original.

## Laziness

### WASM-LAZY-01 — Unused top-level bindings aren't evaluated

- **Status:** Proposed
- **Spec:** [modules.md — Lazy module evaluation](../spec/modules.md#lazy-module-evaluation)
- **Done when:** The shared conformance case passes on WebAssembly. Importing a module doesn't run its unused top-level initializers.

### WASM-LAZY-02 — Memoized bindings

- **Status:** Proposed
- **Spec:** [evaluation.md — Laziness](../spec/evaluation.md#laziness)
- **Done when:** The shared conformance case passes on WebAssembly. A binding used repeatedly runs its initializer at most once.

### WASM-LAZY-03 — Independent lazy object members

- **Status:** Proposed
- **Spec:** [modules.md — Lazy module evaluation](../spec/modules.md#lazy-module-evaluation)
- **Done when:** The shared conformance case passes on WebAssembly. Reading exported `cheap` doesn't force an unrelated `expensiveCalculation()` member.

### WASM-LAZY-04 — Laziness through factory functions

- **Status:** Proposed
- **Spec:** [evaluation.md — Laziness](../spec/evaluation.md#laziness)
- **Done when:** The shared conformance case passes on WebAssembly. Unused member computations of an object returned by a factory never run.
