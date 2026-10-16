# Natcore — consolidated language specification

**Decision snapshot:** 2026-10-16. Approved rules are distinguished from deferred/illustrative details.

This document consolidates the decisions approved in Natcore design discussions through this date, including relevant earlier project conversations. It is a working specification, not an implementation claim.

**Implementation tracking:** This document is a language decision snapshot. For agent-checkable milestones, see `SCORECARD.md` (approved-rule conformance), `implementation/infrastructure.md` (compiler/interpreter infrastructure), and `implementation/backend-coverage.md` (per-target conformance). Their statuses are independent from language-design approval.


---

# Broader syntax and type-system decisions

**Status:** The rules below were approved in earlier Natcore planning discussions. They are included so this snapshot covers more than the current module design discussion.

## Code blocks and data structures

- **Code blocks are indentation-defined.** Braces `{}` are for **data structures**, not statement/function control-flow blocks.
- Full conditional, lambda, function-definition, and data-object grammars have not yet been finalized.

The following block is **conceptual**, not an approved `if` grammar:

```natcore
x = 10
if condition
    x = x + 1
result = x
```

It illustrates an indented nested scope, not a brace-delimited code block.

## Inference-first philosophy

Natcore seeks a TypeScript-like ergonomics with a substantially more functional model, emphasizing purity, piping, and pattern matching. These are **design goals**, not yet complete purity or pipe/pattern syntax rules.

Ordinary variable types and function return types are intended to be inferred. The later approved optional declaration constraint `x: Type = expression` is permitted as an **assignability check** without replacing or widening the inferred type; this updates the earlier inference-only proposal.

## Union and intersection types

- `|` is type **union**.
- Simplification/consolidation of a union is permitted **only when it preserves the exact represented set of possibilities**. In particular, preserve correlations between fields and exclusions; do not merge distinct object variants in ways that admit combinations not present in the original union.
- `&` is reserved for type **intersection**, separate from `:` constraints.

For example, this union:

```natcore
{ kind: "left", value: number }
|
{ kind: "right", value: string }
```

must not be simplified into:

```natcore
{ kind: "left" | "right", value: number | string }
```

because the latter would allow a `"left"` tag paired with a `string` value, which the original type excludes. The record-type syntax here is illustrative.

Difference/complement operators, semantic subtyping details, generic parameter grammar, and algebraic data constructor design remain open.

## Target directions (goals, not approved language semantics)

Natcore is intended to support a native interpreter and compilation to JavaScript, WebAssembly, C, and potentially Python. A shared compiler IR/runtime design and implementation-language choice were discussed as recommendations, not finalized architectural rules.


---

# Variable declarations and identifiers

**Status: Approved, with stated boundaries.**

## Declaration syntax

```natcore
identifier = expression
identifier: Type = expression
```

- No `val`, `let`, or `var` declaration keyword is required.
- Initializers are **mandatory**; `x: number` alone is invalid.
- `:` is an optional **assignability constraint**. The initializer's inferred type must be equal to or assignable to the declared type.
- A constraint **does not widen** or overwrite the inferred type. Thus `x: number = 42` still has inferred type `42`.
- The `&` operator is reserved for type **intersections**, not type constraints.

## Immutability, redeclaration, and scope

Each declaration creates a **new immutable binding**. Reusing the same identifier, including in the same scope and with an unrelated type, is valid. The newest visible binding shadows earlier bindings; it never mutates them.

```natcore
x = 10
x = x + 5           // initializer sees previous x; new x = 15
x: string = "hi"    // new binding; previous x remains immutable
```

A declaration's initializer resolves a same-name reference to the **previously visible** binding. This holds both for a same-scope redeclaration and when a declaration shadows a binding in an outer lexical scope.

```natcore
x = 10
if condition
    x = x + 1        // inner x = 11
result = x           // outer x remains 10
```

The `if` form is illustrative, but **indentation-defined code blocks** (rather than brace-delimited blocks) were separately approved.

- Visibility is **sequential**: ordinary declarations become visible to later declarations, not earlier ones.
- Functions may be **self-recursive**, as an explicit exception to the ordinary visibility rule.
- A function's references to outer names resolve to the **specific bindings visible at the function's definition**, rather than a later same-name redeclaration or the caller's scope.

```natcore
x = 10
getX = () => x
x = 20
result = getX()       // 10
```

- Local declarations may shadow imported identifiers as well; this is distinct from duplicate *imported* bindings, which are an error.

## Identifier categories

- Ordinary value bindings use Haskell-style **lowercase-leading** names, including Unicode letters supported by Unicode identifier categories, or approved emoji identifiers.
- Types and constructors use **PascalCase** and cannot use emoji identifiers.
- Namespaces introduced by imports must use **PascalCase**.
- **Top-level values** may *also* use `UPPER_SNAKE_CASE`; this is **allowed but not required**. Types/constructors may not use `UPPER_SNAKE_CASE`. Ordinary local values do not gain this exception.
- A standalone `_` is reserved as a wildcard, not an ordinary binding identifier.
- Unicode emoji sequences are valid value names, including composed emoji such as `👩‍💻`. The precise emoji lexer grammar beyond these examples can be finalized separately.

```natcore
maxSize = 100
MAX_SIZE = 100         // allowed at module top level
🚀 = 42
type Point(x: number, y: number)  // illustrative type syntax
```

## Identifier mangling and compilation

Identifiers unsupported by a target language (e.g., emoji) are mapped to generated, target-safe identifiers. Name generation must be **collision-safe**. Source names and internal symbol identities remain available for correct resolution and debugging. The exact output spelling is not part of source-language syntax.

## Exact type inference

- Direct primitive literals infer their exact literal types: `42`, `"Alice"`, `true`, rather than automatically widening to `number`, `string`, `boolean`.
- Compile-time constant expressions infer exact literal results when statically evaluable: `40 + 2` may infer `42`.
- A binding's optional `:` constraint does not erase this precision.
- Precise rules for function return inference and evaluation are recorded in [evaluation.md](evaluation.md); they are not a requirement to finalize basic variable syntax.

## Declaration grammar (schematic)

```ebnf
variableDeclaration = valueIdentifier, [":", typeExpression], "=", expression ;
```

The `valueIdentifier` category depends on scope: top-level `UPPER_SNAKE_CASE` is additionally permitted. Full lexer, expressions, and type grammar remain separate subjects.


---

# Modules, imports, exports, and program entry

**Status: Approved module rules with some deferred details.**

## What is a module?

A module is a **logical, lexically scoped collection of declarations** with its own namespace and an import/export boundary. Its conceptual role is distinct from a package and from a physical file. For Natcore's initial source layout, **each `.nat` source file defines exactly one module**; no explicit module declaration is required.

A source file permits **top-level declarations only** (including imports and the terminal export declaration), not standalone executable expressions. A top-level variable initializer is part of a declaration, not a standalone statement.

Functions associated with types, future extension functions, and constructor-like functions may live in separate file modules and be connected through the import/export and future type-association system. The extension/constructor syntax remains deferred.

## Module form

All imports must precede other declarations. Every module must have **exactly one** `export` declaration **at the end** of the module, with nothing declared after it.

```natcore
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

```natcore
PI = 3.14159
square = x => x * x
export { PI, square }        // library: object-shaped public interface
```

```natcore
run = () => print("Hello")
export run                   // executable: function entry
```

```natcore
api = { PI = 3.14159, square = x => x * x }
export api                   // same public interface as export { PI, square }
```

Here `export { ... }` builds the module's exported object. `export api` where `api` is an object exposes its **properties as named exports**, not one implicitly named `api` export. Object literal and function body syntax above is illustrative.

- Both **types and values** can be placed in the same exported object; `export { Point, origin }` is valid.
- A module may **re-export previously imported declarations**, including complete imported namespace values.
- Object properties' **public names must be unique** within the exported interface. Unlike same-scope variable redeclarations, duplicate public export keys are compilation errors.
- **Export aliases** use `=` with the public name on the left: `export { publicName = localName }`.

```natcore
from "./math" import { square }
export { squareNumber = square }
```

## Entry modules

- An executable is selected by **source-file path**, allowing multiple executable files in the same project; command shape `natcore run src/main.nat` is illustrative.
- An executable module is **not importable** by other modules.
- Its entry function can be **imported from a library module**; it need not be declared locally.
- Each module has only one terminal export, so an executable module has one entry function.
- The **entry function's parameter shape, effect/purity model, and runtime context** are deliberately **deferred** until function/effect specifications are designed. There is no mandated `main` name.

```natcore
// app.nat
run = () => print("Hello")
export { run }               // library

// main.nat (different file)
from "./app" import { run }
export run                   // executable
```

## Import syntax and ordering

Natcore uses **file-first syntax**:

```natcore
from "./math" import Math
from "./math" import { PI, square }
from "./math" import *
from "./math" import { PI }, Math
```

- `Math` is a **PascalCase namespace alias**.
- `{ PI, square }` imports named exports as direct local bindings.
- `*` imports **all possible exported names** as direct local bindings; `_` is **not** an import wildcard.
- The combined form `{ PI }, Math` imports `PI` directly and places only the **remaining** names in `Math`; `Math.PI` is absent from that combined import.
- The designer also wants remaining exports to be importable unqualified via wildcard in a combined selective import; exact combined `*, { ... }` grammar can be finalized with import syntax.
- Import aliases use `=` with local alias on the left: `from "./math" import { localName = publicName }`.
- Bare `from "./math"` without `import ...` is **invalid**.
- Multiple import declarations from the **same module are allowed** and resolved independently. For example, separate `{ PI }` and `Math` imports leave `PI` inside `Math`; one combined `{ PI }, Math` import removes `PI` from that `Math` view.

## Paths and dependencies

- Ordinary import paths must be **static string literals**, not computed expressions or runtime variables.
- Local imports use explicit relative prefixes `./` or `../`, resolved against the importing file.
- `.nat` can be omitted for Natcore source modules; non-Natcore formats (such as `.json`) use an explicit extension.
- Direct imports of **JSON** are supported in principle. Other non-Natcore file types may be supported later; their mapping into Natcore values is not yet defined.
- Future built-in and external package path schemes are deferred.
- **Circular module dependencies are forbidden**, including cycles through several modules; the compiler reports an error.

## Import names and shadowing

- Two imports that introduce the **same local identifier** are a **compilation error**, even if imported from the same module, even if they identify the exact same source binding, and even if unused.
- Different aliases or namespaces may be used to disambiguate.
- This is not inconsistent with ordinary binding redeclaration: a later **local declaration can shadow an imported binding** according to Natcore's immutable sequential scope rules.

```natcore
from "./math" import { MATH_PI = PI }
from "./geometry" import { GEO_PI = PI }

PI = 3.14    // allowed local declaration
```

## First-class namespaces and persistent updates

Namespace imports are **first-class immutable values**, not compile-time-only symbols. They can be assigned, passed to functions, returned, and nested in other objects and namespaces.

Multiple imports of the same module initially reference the **same logical exported namespace value**. An update such as `Math.PI = 3.14` constructs a **new immutable namespace value** for the current binding rather than mutating the shared original. Other references and previously captured values retain the older version. Persistent data structures and structural sharing are intended implementation techniques; detailed dotted-update semantics belong to the later object specification.

## Exported object shapes and optional properties

- The exported object may have multiple possible statically inferred shapes (e.g., a union of object types).
- The compiler must statically enumerate the **finite set of possible public property names**, but the presence and values of each property can vary at runtime.
- A named import is legal when the property exists in **at least one** possible export shape. If absent in some branches, its inferred type includes a provisional `None` case.
- Importing a property that exists in **no** possible export shape is a compile-time error.
- Wildcard imports include the **union of all possible export names**, even conditionally present ones.
- No `null` or `undefined` values are intended for Natcore. The exact semantics/spelling of `None` will be established in the type specification.
- A dynamic object with arbitrary runtime keys may be exported **inside a statically named property** (e.g. `export { data }`); arbitrary unenumerable runtime keys are not allowed as the module's top-level public export interface.

```natcore
// api.nat (illustrative conditional syntax)
export if condition then
    { x = 10, y = 20 }
else
    { x = 30, z = 40 }
```

```natcore
// consumer.nat
from "./api" import *
// x: 10 | 30
// y: 20 | None
// z: 40 | None
export {}
```

## Lazy module evaluation

- Importing a module does not eagerly evaluate all its top-level values.
- Bindings are evaluated on first demand and **memoized** (call-by-need).
- Exported object properties are independently lazy; accessing one property does not evaluate another unrelated property.
- Arbitrary factory expressions can produce exported objects, and deferred function bodies may expose lazily computed members. The details of lowering this through arbitrary function calls and the purity/effect boundary are deferred to the evaluation specification.

```natcore
export { cheap = 42, expensive = expensiveCalculation() }
// Accessing cheap does not force expensiveCalculation().
```


---

# Evaluation and debugging

**Status:** Relevant cross-cutting decisions already made; exact semantics not fully specified.

## Literal precision

- Literal expressions have exact literal types by default.
- Pure constant operations may propagate exact results: `40 + 2` can infer `42`.
- Type constraints (`:`) check assignability without widening.

## Function return inference: context-independent

The designer revised an earlier broader constant-evaluation proposal: **function return types are inferred from parameter types and the function's own body**, not specialized using the specific values passed at each call site, nor values outside the function. Exact types can propagate through function-local variables.

```natcore
// Illustration of intended syntax and inference:
test(x: number) => x * 2   // return: number
test(x: 24) => x * 2       // return: 48
```

These examples do **not** approve the syntax for declaring functions or overloads. They illustrate type inference rules only.

Compile-time evaluation rules should be standardized and deterministic. When an attempt to evaluate a pure expression exceeds its budget, **fall back to ordinary sound static inference rather than failing solely due to the budget**. Exact budgets and the relationship to type checking are not yet specified.

## Laziness

- Top-level module bindings and exported object members are call-by-need and cached after evaluation.
- Unused work should not execute, regardless of whether the enclosing object originates in an expression or factory function, subject to actual data dependencies.
- Persistent immutable data structures allow versions to share storage; memoization avoids repeated computation. These are complementary, not synonymous.

## Debugging

Debug builds **preserve the language's lazy semantics**. Debugging tools may show suspended computations and let the developer explicitly force/step into them. Automatically evaluating every function in debug mode is rejected, because it changes program behavior. Production builds may remove unused pure computations and debug builds may retain richer source metadata.

Detailed debugger UI, effects, resource limits, and evaluation mechanisms remain separate specifications.


---

# Deferred decisions

These are **not approved rules** and should not be treated as completed specification text.

- **Entry functions:** argument(s), command-line parameters, runtime context, return contract, effects/purity, and external capability handling.
- **Function syntax and return inference:** complete grammar, overloading, detailed context-independent analysis, recursion, type precision, and standardized constant-evaluation budgets.
- **Types:** exact status/representation of `None`, assignability and subtype relations, algebraic type definitions, constructors, intersections/unions, runtime representation of type values within first-class namespaces.
- **General objects:** literal grammar, computed/dynamic keys, nested property updates such as `Math.PI = value`, persistence/laziness details beyond approved outcomes.
- **Imports:** final grammar of combinations such as named imports plus a wildcard remainder; package/built-in import specifiers; JSON module representation and other external data formats; exact handling of aliases or conflicts within a single compound import.
- **Extensions/constructors:** syntax for associating independently defined functions with types; selection, conflicts, visibility and import behavior.
- **Lexer / grammar:** keyword list; precise Unicode normalization and emoji-sequence admission policy; statement and block syntax; type and object grammars.
- **Build system:** package layout, CLI details, backend mapping to JavaScript/WebAssembly/C/Python, linking and source maps.
- **Lazy execution and effects:** impure computations, memoization across runtime scopes, dependency-driven forcing through arbitrary user-defined factories, errors/nontermination, debug force semantics.

The immediate design process remains: address **one unresolved question at a time**, keep decisions distinct from illustrative code, then request a final approval for each specification area.
