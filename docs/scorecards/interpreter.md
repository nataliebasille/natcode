# Interpreter scorecard

- **Prefix:** INT
- **Covers:** Running Natcode programs on the native interpreter, and passing the shared conformance cases for every runtime-observable behavior.
- **Note:** Execution targets are project goals rather than approved decisions, so their items stay Proposed until the language owner approves the target.

## Target

### INT-GEN-01 — Interpreter runs core programs

- **Status:** Proposed
- **Spec:** None
- **Done when:** A path-selected executable module runs in the interpreter, produces observable results, and reports runtime errors with usable source locations.

## Identifiers

### INT-NAME-01 — Emoji value names

- **Status:** Proposed
- **Spec:** [variables.md — Identifier categories](../spec/variables.md#identifier-categories)
- **Done when:** The shared conformance case passes on the native interpreter. A program that declares and reads emoji-named values, including a composed emoji such as `👩‍💻`, runs correctly.
- **Notes:** Emoji admission is LEX-ID-04.

## Bindings and scopes

### INT-SCOPE-01 — New binding per declaration

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on the native interpreter. A later same-name declaration doesn't change any value computed from or captured from the earlier binding.

### INT-SCOPE-02 — Same-scope redeclaration

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on the native interpreter. `x = 42` followed by `x = "hello"` runs, and later reads see `"hello"`.

### INT-SCOPE-03 — Initializer reads the previous binding

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on the native interpreter. `x = 10` then `x = x + 5` yields `15`.

### INT-SCOPE-04 — Nested shadowing

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on the native interpreter. After an indented scope shadows `x`, the outer `x` keeps its original value.

### INT-SCOPE-05 — Sequential visibility

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on the native interpreter. Reads of a redeclared name before and after the redeclaration see the respective bindings' values.

### INT-SCOPE-06 — Self-recursive functions

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on the native interpreter. A self-recursive function (for example factorial) computes correctly.

### INT-SCOPE-07 — Closures capture binding identities

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on the native interpreter. `x = 10; getX = () => x; x = 20; getX()` returns `10`.

### INT-SCOPE-08 — Local declarations shadow imports

- **Status:** Proposed
- **Spec:** [modules.md — Import names and shadowing](../spec/modules.md#import-names-and-shadowing)
- **Done when:** The shared conformance case passes on the native interpreter. Reads before a shadowing local declaration see the imported value, and reads after it see the local value.

## Modules and entry points

### INT-MOD-01 — One module per file

- **Status:** Proposed
- **Spec:** [modules.md — What is a module?](../spec/modules.md#what-is-a-module)
- **Done when:** The shared conformance case passes on the native interpreter. A multi-file program runs with each file as its own module namespace.

### INT-MOD-02 — Library and executable modules

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on the native interpreter. An executable module runs its exported entry function, and library modules provide their exported objects to importers.

### INT-MOD-03 — Export of a call expression

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on the native interpreter. A module exporting the result of a function call behaves as the library or executable its inferred kind dictates.

### INT-MOD-04 — Equivalent export object forms

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on the native interpreter. `export api` and `export { PI, square }` give importers identical working members.

### INT-MOD-05 — Types and values in one export

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on the native interpreter. Values exported alongside types are usable by importers.

### INT-MOD-06 — Export aliases

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on the native interpreter. `export { squareNumber = square }` gives importers a working `squareNumber`.

### INT-MOD-07 — Re-exports

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on the native interpreter. Re-exported values and namespaces are usable through the re-exporting module.

### INT-MOD-08 — Imported entry function

- **Status:** Proposed
- **Spec:** [modules.md — Entry modules](../spec/modules.md#entry-modules)
- **Done when:** The shared conformance case passes on the native interpreter. An executable module whose entry function is imported from a library runs that function.

### INT-MOD-09 — Entry module selected by path

- **Status:** Proposed
- **Spec:** [modules.md — Entry modules](../spec/modules.md#entry-modules)
- **Done when:** The shared conformance case passes on the native interpreter. With several executable modules, running one path runs only that module's entry function.

### INT-MOD-10 — Conditionally present export keys

- **Status:** Proposed
- **Spec:** [modules.md — Exported object shapes and optional properties](../spec/modules.md#exported-object-shapes-and-optional-properties)
- **Done when:** The shared conformance case passes on the native interpreter. Reading an export key that is absent in the current variant yields the `None`-like value.
- **Notes:** The representation of `None` is TYP-NONE-01.

## Imports

### INT-IMP-01 — Named and namespace imports

- **Status:** Proposed
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** The shared conformance case passes on the native interpreter. Values imported by name and through a namespace are correct.

### INT-IMP-02 — Wildcard imports

- **Status:** Proposed
- **Spec:** [modules.md — Exported object shapes and optional properties](../spec/modules.md#exported-object-shapes-and-optional-properties)
- **Done when:** The shared conformance case passes on the native interpreter. `import *` makes every statically possible export usable by its own name.

### INT-IMP-03 — Combined named and namespace import

- **Status:** Proposed
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** The shared conformance case passes on the native interpreter. `{ PI }, Math` provides a working `PI` and a `Math` namespace without `PI`.

### INT-IMP-04 — Import aliases

- **Status:** Proposed
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** The shared conformance case passes on the native interpreter. `{ formatText = format }` provides a working `formatText`.

### INT-IMP-05 — Repeated imports of one module

- **Status:** Proposed
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** The shared conformance case passes on the native interpreter. Separate named and namespace imports of one module both work, each with the full exports.

## Namespaces

### INT-NS-01 — First-class namespaces

- **Status:** Proposed
- **Spec:** [modules.md — First-class namespaces and persistent updates](../spec/modules.md#first-class-namespaces-and-persistent-updates)
- **Done when:** The shared conformance case passes on the native interpreter. A namespace can be assigned, passed to a function, returned, and nested in another object or export, and still works.

### INT-NS-02 — Shared namespace without reinitialization

- **Status:** Proposed
- **Spec:** [modules.md — First-class namespaces and persistent updates](../spec/modules.md#first-class-namespaces-and-persistent-updates)
- **Done when:** The shared conformance case passes on the native interpreter. Several imports of one module observe a single initialization of it.

### INT-NS-03 — Persistent dotted updates

- **Status:** Proposed
- **Spec:** [modules.md — First-class namespaces and persistent updates](../spec/modules.md#first-class-namespaces-and-persistent-updates)
- **Done when:** The shared conformance case passes on the native interpreter. After `Math.PI = value`, the current binding sees the new value while aliases and other imports keep the original.

## Laziness

### INT-LAZY-01 — Unused top-level bindings aren't evaluated

- **Status:** Proposed
- **Spec:** [modules.md — Lazy module evaluation](../spec/modules.md#lazy-module-evaluation)
- **Done when:** The shared conformance case passes on the native interpreter. Importing a module doesn't run its unused top-level initializers.

### INT-LAZY-02 — Memoized bindings

- **Status:** Proposed
- **Spec:** [evaluation.md — Laziness](../spec/evaluation.md#laziness)
- **Done when:** The shared conformance case passes on the native interpreter. A binding used repeatedly runs its initializer at most once.

### INT-LAZY-03 — Independent lazy object members

- **Status:** Proposed
- **Spec:** [modules.md — Lazy module evaluation](../spec/modules.md#lazy-module-evaluation)
- **Done when:** The shared conformance case passes on the native interpreter. Reading exported `cheap` doesn't force an unrelated `expensiveCalculation()` member.

### INT-LAZY-04 — Laziness through factory functions

- **Status:** Proposed
- **Spec:** [evaluation.md — Laziness](../spec/evaluation.md#laziness)
- **Done when:** The shared conformance case passes on the native interpreter. Unused member computations of an object returned by a factory never run.
