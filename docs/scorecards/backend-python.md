# Python backend scorecard

- **Prefix:** PY
- **Covers:** Running Natcode programs on Python, and passing the shared conformance cases for every runtime-observable behavior.
- **Note:** Python is a prospective target. Its items stay Proposed until PY-TGT-01 is answered.

## Target

### PY-TGT-01 — Adopt Python as a target

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Target directions](../spec/syntax-and-types.md#target-directions)
- **Questions:**
  1. Natcode may compile to Python. Is Python adopted as a supported target?

### PY-GEN-01 — Python output runs a core program

- **Status:** Proposed
- **Spec:** None
- **Done when:** Generated Python runs and passes the initial shared conformance cases.

### PY-MAP-01 — Module mapping, linking, and source maps

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Target directions](../spec/syntax-and-types.md#target-directions)
- **Questions:**
  1. How are Natcode modules mapped onto Python files or modules?
  2. How is the output linked or packaged?
  3. Which source-map format links generated code back to Natcode source?

## Identifiers

### PY-NAME-01 — Emoji value names

- **Status:** Proposed
- **Spec:** [variables.md — Identifier categories](../spec/variables.md#identifier-categories)
- **Done when:** The shared conformance case passes on Python. A program that declares and reads emoji-named values, including a composed emoji such as `👩‍💻`, runs correctly.
- **Notes:** Emoji admission is LEX-ID-04.

### PY-NAME-02 — Collision-safe identifier mangling

- **Status:** Proposed
- **Spec:** [variables.md — Identifier mangling and compilation](../spec/variables.md#identifier-mangling-and-compilation)
- **Done when:** The shared conformance case passes on Python. Source identifiers the target can't spell (including emoji) are mangled without collisions, and source names stay available for debugging and source mapping.

## Bindings and scopes

### PY-SCOPE-01 — New binding per declaration

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on Python. A later same-name declaration doesn't change any value computed from or captured from the earlier binding.

### PY-SCOPE-02 — Same-scope redeclaration

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on Python. `x = 42` followed by `x = "hello"` runs, and later reads see `"hello"`.

### PY-SCOPE-03 — Initializer reads the previous binding

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on Python. `x = 10` then `x = x + 5` yields `15`.

### PY-SCOPE-04 — Nested shadowing

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on Python. After an indented scope shadows `x`, the outer `x` keeps its original value.

### PY-SCOPE-05 — Sequential visibility

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on Python. Reads of a redeclared name before and after the redeclaration see the respective bindings' values.

### PY-SCOPE-06 — Self-recursive functions

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on Python. A self-recursive function (for example factorial) computes correctly.

### PY-SCOPE-07 — Closures capture binding identities

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on Python. `x = 10; getX = () => x; x = 20; getX()` returns `10`.

### PY-SCOPE-08 — Local declarations shadow imports

- **Status:** Proposed
- **Spec:** [modules.md — Import names and shadowing](../spec/modules.md#import-names-and-shadowing)
- **Done when:** The shared conformance case passes on Python. Reads before a shadowing local declaration see the imported value, and reads after it see the local value.

## Modules and entry points

### PY-MOD-01 — One module per file

- **Status:** Proposed
- **Spec:** [modules.md — What is a module?](../spec/modules.md#what-is-a-module)
- **Done when:** The shared conformance case passes on Python. A multi-file program runs with each file as its own module namespace.

### PY-MOD-02 — Library and executable modules

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on Python. An executable module runs its exported entry function, and library modules provide their exported objects to importers.

### PY-MOD-03 — Export of a call expression

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on Python. A module exporting the result of a function call behaves as the library or executable its inferred kind dictates.

### PY-MOD-04 — Equivalent export object forms

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on Python. `export api` and `export { PI, square }` give importers identical working members.

### PY-MOD-05 — Types and values in one export

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on Python. Values exported alongside types are usable by importers.

### PY-MOD-06 — Export aliases

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on Python. `export { squareNumber = square }` gives importers a working `squareNumber`.

### PY-MOD-07 — Re-exports

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on Python. Re-exported values and namespaces are usable through the re-exporting module.

### PY-MOD-08 — Imported entry function

- **Status:** Proposed
- **Spec:** [modules.md — Entry modules](../spec/modules.md#entry-modules)
- **Done when:** The shared conformance case passes on Python. An executable module whose entry function is imported from a library runs that function.

### PY-MOD-09 — Entry module selected by path

- **Status:** Proposed
- **Spec:** [modules.md — Entry modules](../spec/modules.md#entry-modules)
- **Done when:** The shared conformance case passes on Python. With several executable modules, running one path runs only that module's entry function.

### PY-MOD-10 — Conditionally present export keys

- **Status:** Proposed
- **Spec:** [modules.md — Exported object shapes and optional properties](../spec/modules.md#exported-object-shapes-and-optional-properties)
- **Done when:** The shared conformance case passes on Python. Reading an export key that is absent in the current variant yields the `None`-like value.
- **Notes:** The representation of `None` is TYP-NONE-01.

## Imports

### PY-IMP-01 — Named and namespace imports

- **Status:** Proposed
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** The shared conformance case passes on Python. Values imported by name and through a namespace are correct.

### PY-IMP-02 — Wildcard imports

- **Status:** Proposed
- **Spec:** [modules.md — Exported object shapes and optional properties](../spec/modules.md#exported-object-shapes-and-optional-properties)
- **Done when:** The shared conformance case passes on Python. `import *` makes every statically possible export usable by its own name.

### PY-IMP-03 — Combined named and namespace import

- **Status:** Proposed
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** The shared conformance case passes on Python. `{ PI }, Math` provides a working `PI` and a `Math` namespace without `PI`.

### PY-IMP-04 — Import aliases

- **Status:** Proposed
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** The shared conformance case passes on Python. `{ formatText = format }` provides a working `formatText`.

### PY-IMP-05 — Repeated imports of one module

- **Status:** Proposed
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** The shared conformance case passes on Python. Separate named and namespace imports of one module both work, each with the full exports.

## Namespaces

### PY-NS-01 — First-class namespaces

- **Status:** Proposed
- **Spec:** [modules.md — First-class namespaces and persistent updates](../spec/modules.md#first-class-namespaces-and-persistent-updates)
- **Done when:** The shared conformance case passes on Python. A namespace can be assigned, passed to a function, returned, and nested in another object or export, and still works.

### PY-NS-02 — Shared namespace without reinitialization

- **Status:** Proposed
- **Spec:** [modules.md — First-class namespaces and persistent updates](../spec/modules.md#first-class-namespaces-and-persistent-updates)
- **Done when:** The shared conformance case passes on Python. Several imports of one module observe a single initialization of it.

### PY-NS-03 — Persistent dotted updates

- **Status:** Proposed
- **Spec:** [modules.md — First-class namespaces and persistent updates](../spec/modules.md#first-class-namespaces-and-persistent-updates)
- **Done when:** The shared conformance case passes on Python. After `Math.PI = value`, the current binding sees the new value while aliases and other imports keep the original.

## Laziness

### PY-LAZY-01 — Unused top-level bindings aren't evaluated

- **Status:** Proposed
- **Spec:** [modules.md — Lazy module evaluation](../spec/modules.md#lazy-module-evaluation)
- **Done when:** The shared conformance case passes on Python. Importing a module doesn't run its unused top-level initializers.

### PY-LAZY-02 — Memoized bindings

- **Status:** Proposed
- **Spec:** [evaluation.md — Laziness](../spec/evaluation.md#laziness)
- **Done when:** The shared conformance case passes on Python. A binding used repeatedly runs its initializer at most once.

### PY-LAZY-03 — Independent lazy object members

- **Status:** Proposed
- **Spec:** [modules.md — Lazy module evaluation](../spec/modules.md#lazy-module-evaluation)
- **Done when:** The shared conformance case passes on Python. Reading exported `cheap` doesn't force an unrelated `expensiveCalculation()` member.

### PY-LAZY-04 — Laziness through factory functions

- **Status:** Proposed
- **Spec:** [evaluation.md — Laziness](../spec/evaluation.md#laziness)
- **Done when:** The shared conformance case passes on Python. Unused member computations of an object returned by a factory never run.
