# Modules, imports, exports, and program entry

Item status and open questions for this file are tracked in the [parser](../scorecards/parser.md), [resolver](../scorecards/resolver.md), [types](../scorecards/types.md), and [runtime](../scorecards/runtime.md) scorecards.

## What is a module?

A module is a **logical, lexically scoped collection of declarations** with its own namespace and an import/export boundary. Its conceptual role is distinct from a package and from a physical file. For Natcode's initial source layout, **each `.nat` source file defines exactly one module**; no explicit module declaration is required.

A source file permits **top-level declarations only** (including imports and the terminal export declaration), not standalone executable expressions. A top-level variable initializer is part of a declaration, not a standalone statement.

Functions associated with types, future extension functions, and constructor-like functions may live in separate file modules and be connected through the import/export and future type-association system. Their syntax and resolution are PAR-EXT-01 and RES-EXT-01.

## Module form

All imports must precede other declarations. Every module must have **exactly one** `export` declaration **at the end** of the module, with nothing declared after it.

```natcode
from "./math" import { PI }

area = PI * 5 * 5

export { area }
```

- `export {}` is required for a library exposing no names.
- A module cannot combine a function entry export with an ordinary object export or have multiple terminal exports.

## Exported kind: object library or function executable

The final export may be an **arbitrary expression**, but its inferred type must statically establish exactly one category:

- **Object** → importable **library module**.
- **Function** → **executable module**, with that function as the program entry point.
- **Primitive or directly exported type** → invalid. Types and primitive values must appear as **properties of an exported object**.
- **Expression potentially returning either object or function** → compilation error; module kind cannot depend on runtime outcomes.

```natcode
PI = 3.14159
fn square(x: number) { x * x }
export { PI, square }        // library: object-shaped public interface
```

```natcode
fn run() { print("Hello") }
export run                   // executable: function entry
```

```natcode
api = { PI: 3.14159, square: x => x * x }
export api                   // same public interface as export { PI, square }
```

Here `export { ... }` builds the module's exported object. `export api` where `api` is an object exposes its **properties as named exports**, not one implicitly named `api` export. Object literal property colons are approved (PAR-OBJ-02); the remaining object-literal grammar is PAR-OBJ-01. Function body grammar beyond the approved brace-delimited block boundary remains PAR-FN-01 and PAR-BLK-02.

- Both **types and values** can be placed in the same exported object; `export { Point, origin }` is valid.
- A module may **re-export previously imported declarations**, including complete imported namespace values.
- Object properties' **public names must be unique** within the exported interface. Unlike same-scope variable redeclarations, duplicate public export keys are compilation errors.
- **Export aliases** use `as`, original local name first: `export { localName as publicName }`.

```natcode
from "./math" import { square }
export { square as squareNumber }
```

## Entry modules

- An executable is selected by **source-file path**, allowing multiple executable files in the same project; command shape `natcode run src/main.nat` is illustrative.
- An executable module is **not importable** by other modules.
- Its entry function can be **imported from a library module**; it need not be declared locally.
- Each module has only one terminal export, so an executable module has one entry function.
- The **entry function's parameter shape, effect/purity model, and runtime context** are RUN-ENTRY-01. There is no mandated `main` name.

```natcode
// app.nat
fn run() { print("Hello") }
export { run }               // library

// main.nat (different file)
from "./app" import { run }
export run                   // executable
```

## Import syntax and ordering

Natcode uses **file-first syntax**:

```natcode
from "./math" import Math
from "./math" import { PI, square }
from "./math" import *
from "./math" import { PI }, Math
```

- `Math` is a **PascalCase namespace alias**.
- `{ PI, square }` imports named exports as direct local bindings.
- `*` imports **only exported names guaranteed in every possible object shape** as direct local bindings; `_` is **not** an import wildcard.
- The combined form `{ PI }, Math` imports `PI` directly and places only the **remaining** names in `Math`; `Math.PI` is absent from that combined import.
- The designer also wants remaining exports to be importable unqualified via wildcard in a combined selective import; the grammar for this is PAR-IMP-08.
- Import aliases use `as`, original exported name first: `from "./math" import { publicName as localName }`.
- Bare `from "./math"` without `import ...` is **invalid**.
- Multiple import declarations from the **same module are allowed** and resolved independently. For example, separate `{ PI }` and `Math` imports leave `PI` inside `Math`; one combined `{ PI }, Math` import removes `PI` from that `Math` view.

## Paths and dependencies

- Ordinary import paths must be **static string literals**, not computed expressions or runtime variables.
- Local imports use explicit relative prefixes `./` or `../`, resolved against the importing file.
- `.nat` can be omitted for Natcode source modules; non-Natcode formats (such as `.json`) use an explicit extension.
- Direct imports of **JSON** are supported in principle; their value representation is RES-MOD-08. Other non-Natcode file types are RES-MOD-09.
- Built-in and external package path schemes are RES-MOD-07. Built-in types alone are automatically in scope; other types require local declaration or explicit import. Ambient type declarations are not currently supported (RES-BLT-01).
- **Circular module dependencies are forbidden**, including cycles through several modules; the compiler reports an error.

## Import names and shadowing

- Two imports that introduce the **same local identifier** are a **compilation error**, even if imported from the same module, even if they identify the exact same source binding, and even if unused.
- Different aliases or namespaces may be used to disambiguate.
- This is not inconsistent with ordinary binding redeclaration: a later **local declaration can shadow an imported binding** according to Natcode's immutable sequential scope rules.

```natcode
from "./math" import { PI as MATH_PI }
from "./geometry" import { PI as GEO_PI }

PI = 3.14    // allowed local declaration
```

## First-class namespaces and persistent updates

Namespace imports are **first-class immutable values**, not compile-time-only symbols. They can be assigned, passed to functions, returned, and nested in other objects and namespaces.

Multiple imports of the same module initially reference the **same logical exported namespace value**. An update such as `Math.PI = 3.14` constructs a **new immutable namespace value** for the current binding rather than mutating the shared original. Other references and previously captured values retain the older version. Persistent data structures and structural sharing are intended implementation techniques; detailed dotted-update syntax and semantics are PAR-UPD-01 and RUN-UPD-02.

## Exported object shapes and optional properties

- The exported object may have multiple possible statically inferred shapes (e.g., a union of object types).
- The compiler must statically enumerate the **finite set of possible public property names**; their values and presence may vary across possible shapes.
- **Every named import requires its exported property to exist in every possible export shape**. Importing a conditionally absent or never-present property is a compile error; missing keys are never silently converted into `none`.
- Wildcard imports bind **only the keys present in every possible export shape**. Keys present in only some shapes are excluded, rather than synthesized as nullable bindings.
- A property whose value may explicitly be `none` is still present and therefore importable; its value type may include built-in `None` normally.
- Accessing a possibly absent property through an imported namespace has the same compile-time presence requirement as any other record member access.
- A dynamic object with arbitrary runtime keys may be exported **inside a statically named property** (e.g. `export { data }`); arbitrary unenumerable runtime keys are not allowed as the module's top-level public export interface.

```natcode
// api.nat: y exists in both shapes, even when its value is none.
api = when getCondition()
    true => ({ x: 10, y: none })
    false => ({ x: 30, y: 20 })
export api

// consumer.nat: both names may be imported.
// from "./api" import { x, y }
```

```natcode
// A separate export whose possibilities are:
// { x: 10, y: 20 } OR { x: 30, z: 40 }
// Named imports of y or z are errors; wildcard imports bind x only.
```

## Lazy module evaluation

- Importing a module does not eagerly evaluate all its top-level values.
- Bindings are evaluated on first demand and **memoized** (call-by-need).
- Exported object properties are independently lazy; accessing one property does not evaluate another unrelated property.
- Arbitrary factory expressions can produce exported objects, and deferred function bodies may expose lazily computed members. Lowering this through arbitrary function calls, and the purity/effect boundary, are RUN-LAZY-05.

```natcode
export { cheap = 42, expensive = expensiveCalculation() }
// Accessing cheap does not force expensiveCalculation().
```
