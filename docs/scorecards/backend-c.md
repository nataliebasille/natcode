# C backend scorecard

- **Prefix:** C
- **Covers:** Running Natcode programs on C, and passing the shared conformance cases for every runtime-observable behavior.
- **Note:** Execution targets are project goals rather than approved decisions, so their items stay Proposed until the language owner approves the target.

## Target

### C-GEN-01 — C output runs a core program

- **Status:** Proposed
- **Spec:** None
- **Done when:** Generated C compiles with a supported C toolchain, runs, and passes the initial shared conformance cases.

### C-MAP-01 — Module mapping, linking, and source maps

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Target directions](../spec/syntax-and-types.md#target-directions)
- **Questions:**
  1. How are Natcode modules mapped onto C files or modules?
  2. How is the output linked or packaged?
  3. Which source-map format links generated code back to Natcode source?

## Identifiers

### C-NAME-01 — Emoji value names

- **Status:** Proposed
- **Spec:** [variables.md — Identifier categories](../spec/variables.md#identifier-categories)
- **Done when:** The shared conformance case passes on C. A program that declares and reads emoji-named values, including a composed emoji such as `👩‍💻`, runs correctly.
- **Notes:** Emoji admission is LEX-ID-04.

### C-NAME-02 — Collision-safe identifier mangling

- **Status:** Proposed
- **Spec:** [variables.md — Identifier mangling and compilation](../spec/variables.md#identifier-mangling-and-compilation)
- **Done when:** The shared conformance case passes on C. Source identifiers the target can't spell (including emoji) are mangled without collisions, and source names stay available for debugging and source mapping.

## Bindings and scopes

### C-SCOPE-01 — New binding per declaration

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on C. A later same-name declaration doesn't change any value computed from or captured from the earlier binding.

### C-SCOPE-02 — Same-scope redeclaration

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on C. `x = 42` followed by `x = "hello"` runs, and later reads see `"hello"`.

### C-SCOPE-03 — Initializer reads the previous binding

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on C. `x = 10` then `x = x + 5` yields `15`.

### C-SCOPE-04 — Nested shadowing

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on C. After an indented scope shadows `x`, the outer `x` keeps its original value.

### C-SCOPE-05 — Sequential visibility

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on C. Reads of a redeclared name before and after the redeclaration see the respective bindings' values.

### C-SCOPE-06 — Self-recursive functions

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on C. A self-recursive function (for example factorial) computes correctly.

### C-SCOPE-07 — Closures capture binding identities

- **Status:** Proposed
- **Spec:** [variables.md — Immutability, redeclaration, and scope](../spec/variables.md#immutability-redeclaration-and-scope)
- **Done when:** The shared conformance case passes on C. `x = 10; getX = () => x; x = 20; getX()` returns `10`.

### C-SCOPE-08 — Local declarations shadow imports

- **Status:** Proposed
- **Spec:** [modules.md — Import names and shadowing](../spec/modules.md#import-names-and-shadowing)
- **Done when:** The shared conformance case passes on C. Reads before a shadowing local declaration see the imported value, and reads after it see the local value.

## Modules and entry points

### C-MOD-01 — One module per file

- **Status:** Proposed
- **Spec:** [modules.md — What is a module?](../spec/modules.md#what-is-a-module)
- **Done when:** The shared conformance case passes on C. A multi-file program runs with each file as its own module namespace.

### C-MOD-02 — Library and executable modules

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on C. An executable module runs its exported entry function, and library modules provide their exported objects to importers.

### C-MOD-03 — Export of a call expression

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on C. A module exporting the result of a function call behaves as the library or executable its inferred kind dictates.

### C-MOD-04 — Equivalent export object forms

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on C. `export api` and `export { PI, square }` give importers identical working members.

### C-MOD-05 — Types and values in one export

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on C. Values exported alongside types are usable by importers.

### C-MOD-06 — Export aliases

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on C. `export { squareNumber = square }` gives importers a working `squareNumber`.

### C-MOD-07 — Re-exports

- **Status:** Proposed
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** The shared conformance case passes on C. Re-exported values and namespaces are usable through the re-exporting module.

### C-MOD-08 — Imported entry function

- **Status:** Proposed
- **Spec:** [modules.md — Entry modules](../spec/modules.md#entry-modules)
- **Done when:** The shared conformance case passes on C. An executable module whose entry function is imported from a library runs that function.

### C-MOD-09 — Entry module selected by path

- **Status:** Proposed
- **Spec:** [modules.md — Entry modules](../spec/modules.md#entry-modules)
- **Done when:** The shared conformance case passes on C. With several executable modules, running one path runs only that module's entry function.

### C-MOD-10 — Conditionally present export keys

- **Status:** Proposed
- **Spec:** [modules.md — Exported object shapes and optional properties](../spec/modules.md#exported-object-shapes-and-optional-properties)
- **Done when:** The shared conformance case passes on C. Reading an export key that is absent in the current variant yields the `None`-like value.
- **Notes:** The representation of `None` is TYP-NONE-01.

## Imports

### C-IMP-01 — Named and namespace imports

- **Status:** Proposed
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** The shared conformance case passes on C. Values imported by name and through a namespace are correct.

### C-IMP-02 — Wildcard imports

- **Status:** Proposed
- **Spec:** [modules.md — Exported object shapes and optional properties](../spec/modules.md#exported-object-shapes-and-optional-properties)
- **Done when:** The shared conformance case passes on C. `import *` makes every statically possible export usable by its own name.

### C-IMP-03 — Combined named and namespace import

- **Status:** Proposed
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** The shared conformance case passes on C. `{ PI }, Math` provides a working `PI` and a `Math` namespace without `PI`.

### C-IMP-04 — Import aliases

- **Status:** Proposed
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** The shared conformance case passes on C. `{ formatText = format }` provides a working `formatText`.

### C-IMP-05 — Repeated imports of one module

- **Status:** Proposed
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** The shared conformance case passes on C. Separate named and namespace imports of one module both work, each with the full exports.

## Namespaces

### C-NS-01 — First-class namespaces

- **Status:** Proposed
- **Spec:** [modules.md — First-class namespaces and persistent updates](../spec/modules.md#first-class-namespaces-and-persistent-updates)
- **Done when:** The shared conformance case passes on C. A namespace can be assigned, passed to a function, returned, and nested in another object or export, and still works.

### C-NS-02 — Shared namespace without reinitialization

- **Status:** Proposed
- **Spec:** [modules.md — First-class namespaces and persistent updates](../spec/modules.md#first-class-namespaces-and-persistent-updates)
- **Done when:** The shared conformance case passes on C. Several imports of one module observe a single initialization of it.

### C-NS-03 — Persistent dotted updates

- **Status:** Proposed
- **Spec:** [modules.md — First-class namespaces and persistent updates](../spec/modules.md#first-class-namespaces-and-persistent-updates)
- **Done when:** The shared conformance case passes on C. After `Math.PI = value`, the current binding sees the new value while aliases and other imports keep the original.

## Laziness

### C-LAZY-01 — Unused top-level bindings aren't evaluated

- **Status:** Proposed
- **Spec:** [modules.md — Lazy module evaluation](../spec/modules.md#lazy-module-evaluation)
- **Done when:** The shared conformance case passes on C. Importing a module doesn't run its unused top-level initializers.

### C-LAZY-02 — Memoized bindings

- **Status:** Proposed
- **Spec:** [evaluation.md — Laziness](../spec/evaluation.md#laziness)
- **Done when:** The shared conformance case passes on C. A binding used repeatedly runs its initializer at most once.

### C-LAZY-03 — Independent lazy object members

- **Status:** Proposed
- **Spec:** [modules.md — Lazy module evaluation](../spec/modules.md#lazy-module-evaluation)
- **Done when:** The shared conformance case passes on C. Reading exported `cheap` doesn't force an unrelated `expensiveCalculation()` member.

### C-LAZY-04 — Laziness through factory functions

- **Status:** Proposed
- **Spec:** [evaluation.md — Laziness](../spec/evaluation.md#laziness)
- **Done when:** The shared conformance case passes on C. Unused member computations of an object returned by a factory never run.
