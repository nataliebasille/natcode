# Variable declarations and identifiers

Item status and open questions for this file are tracked in the [parser](../scorecards/parser.md), [resolver](../scorecards/resolver.md), and [types](../scorecards/types.md) scorecards.

## Declaration syntax

```natcode
identifier = expression
identifier: Type = expression
```

- No `val`, `let`, or `var` declaration keyword is required.
- Initializers are **mandatory**; `x: number` alone is invalid.
- `:` is an optional **assignability constraint**. The initializer's inferred type must be equal to or assignable to the declared type.
- Any type expression, including a `where` refinement, may appear after `:`. For example, `x: number where n => n > 0 = 5` requires the initializer to satisfy that restricted type (TYP-REF-11).
- A constraint **does not widen** or overwrite the inferred type. Thus `x: number = 42` still has inferred type `42`.
- The `&` operator is reserved for type **intersections**, not type constraints.

## Immutability, redeclaration, and scope

Each declaration creates a **new immutable binding**. Reusing the same identifier, including in the same scope and with an unrelated type, is valid. The newest visible binding shadows earlier bindings; it never mutates them.

```natcode
x = 10
x = x + 5           // initializer sees previous x; new x = 15
x: string = "hi"    // new binding; previous x remains immutable
```

A declaration's initializer resolves a same-name reference to the **previously visible** binding. This holds both for a same-scope redeclaration and when a declaration shadows a binding in an outer lexical scope.

```natcode
x = 10
getInner = () =>
    x = x + 1        // inner x = 11
    x
result = x           // outer x remains 10
```

The lambda syntax is illustrative; code blocks are **indentation-defined** rather than brace-delimited (see [syntax-and-types.md](syntax-and-types.md#code-blocks-and-data-structures)).

- Visibility is **sequential**: ordinary declarations become visible to later declarations, not earlier ones.
- Functions may be **self-recursive**, as an explicit exception to the ordinary visibility rule.
- A function's references to outer names resolve to the **specific bindings visible at the function's definition**, rather than a later same-name redeclaration or the caller's scope.

```natcode
x = 10
getX = () => x
x = 20
result = getX()       // 10
```

- Local declarations may shadow imported identifiers as well; this is distinct from duplicate *imported* bindings, which are an error.

## Identifier categories

- Ordinary value bindings use Haskell-style **lowercase-leading** names, including Unicode letters supported by Unicode identifier categories, or emoji identifiers.
- Types and constructors use **PascalCase** and cannot use emoji identifiers.
- Namespaces introduced by imports must use **PascalCase**.
- **Top-level values** may *also* use `UPPER_SNAKE_CASE`; this is **allowed but not required**. Types/constructors may not use `UPPER_SNAKE_CASE`. Ordinary local values do not gain this exception.
- A standalone `_` is reserved as a wildcard, not an ordinary binding identifier.
- Unicode emoji sequences are valid value names, including composed emoji such as `👩‍💻`. Which emoji sequences are admitted is LEX-ID-04.

```natcode
maxSize = 100
MAX_SIZE = 100         // allowed at module top level
🚀 = 42
type Point(number, number)  // positional nominal constructor declaration
```

## Identifier mangling and compilation

Identifiers unsupported by a target language (e.g., emoji) are mapped to generated, target-safe identifiers. Name generation must be **collision-safe**. Source names and internal symbol identities remain available for correct resolution and debugging. The exact output spelling is not part of source-language syntax.

## Exact type inference

- Direct primitive literals infer their exact literal types: `42`, `"Alice"`, `true`, rather than automatically widening to `number`, `string`, `boolean`.
- Supported core constant operations infer exact literal results deterministically as part of typing: `40 + 2` infers `42`. Optional evaluation of a user-defined call must not change its inferred type (TYP-LIT-02, TYP-EVAL-01).
- A binding's optional `:` constraint does not erase this precision.
- Precise rules for function return inference and evaluation are recorded in [evaluation.md](evaluation.md); they don't affect basic variable syntax.

## Declaration grammar (schematic)

```ebnf
variableDeclaration = valueIdentifier, [":", typeExpression], "=", expression ;
```

The `valueIdentifier` category depends on scope: top-level `UPPER_SNAKE_CASE` is additionally permitted. The lexical grammar is in [lexical.md](lexical.md); expression and type grammars are PAR-EXPR-01 and PAR-TYPE-01.
