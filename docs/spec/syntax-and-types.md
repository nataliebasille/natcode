# Broader syntax and type-system decisions

Item status and open questions for this file are tracked in the [parser](../scorecards/parser.md), [types](../scorecards/types.md), and [tooling](../scorecards/tooling.md) scorecards.

## Code blocks and data structures

- **Code blocks are indentation-defined.** Braces `{}` are for **data structures**, not statement/function control-flow blocks.
- Match-branch, lambda, function-definition, and data-object grammars are PAR-COND-01, PAR-FN-01, and PAR-OBJ-01; block syntax is PAR-BLK-02.

The following block is **conceptual**; its lambda form is illustrative:

```natcode
x = 10
getInner = () =>
    x = x + 1
    x
result = x
```

It illustrates an indented nested scope, not a brace-delimited code block.

## Match expressions

Natcode uses one `match` keyword for both structural pattern matching and conditional branching (PAR-COND-02). It does not have separate `if`/`else` or `case` control-flow constructs.

- **Subjectful** `match expression` selects a branch by matching patterns against that subject.
- **Subjectless** `match` evaluates Boolean branch conditions in source order and selects the first true condition.
- Both forms are expressions that produce the selected branch's value.

The following forms are **illustrative**, not a finalized branch grammar (PAR-COND-01, PAR-PAT-01):

```natcode
result = match option
    Some(value) => value
    None => 0

size = match
    x > 10 => "large"
    x > 5 => "medium"
    _ => "small"
```

The exact separator syntax, allowable patterns, branch bodies, fallback rules, and exhaustiveness handling remain open.

## Declaration keywords and expression results

- `fn` introduces named function declarations (PAR-FN-02). Parameter and body syntax remain open (PAR-FN-01).
- Functions return the value of their final expression; there is no `return` statement or explicit early return (PAR-FN-03).
- `type` introduces a new **nominal** type identity (PAR-TYPE-03, TYP-NOM-01); it never declares a transparent alias.
- `alias` names an existing type expression or a constructor symbol without creating a new nominal identity (PAR-TYPE-05, TYP-ALI-01, TYP-ALI-02). The right-hand side is introduced by `=`. The full expression grammar remains PAR-TYPE-01.
- Positional nominal constructor declarations and algebraic variants have approved forms (PAR-TYPE-04, PAR-TYPE-06). Further declaration and generic grammar remains open (PAR-TYPE-02).
- Ordinary variable bindings continue to use keyword-free `name = expression` declarations (PAR-DECL-01).

## Inference-first philosophy

Natcode seeks a TypeScript-like ergonomics with a substantially more functional model, emphasizing purity, piping, and pattern matching. These are **design goals** (see RUN-EFF-01, PAR-PIPE-01, and PAR-PAT-01).

Ordinary variable types and function return types are intended to be inferred. The optional declaration constraint `x: Type = expression` is permitted as an **assignability check** without replacing or widening the inferred type; this updates the earlier inference-only proposal.

## Nominal types, constructors, and aliases

A `type` declaration creates a fresh nominal identity. Constructing values of two independently declared nominal types does not make those values interchangeable, even when their payload types coincide (TYP-NOM-01).

```natcode
type UserId(number)
type ProductId(number)

userId = UserId(123)
productId = ProductId(123)
```

Nominal wrappers do **not** implicitly convert to their payload types or automatically inherit their operators (TYP-NOM-02). User-defined operator syntax and resolution remain open until function semantics are designed.

A constructor can have multiple positional parameters. The constructor form is also a pattern that binds the contents of its fields; there is no automatically generated `.value` accessor (TYP-NOM-03, TYP-NOM-04, PAR-PAT-01).

```natcode
type Point(number, number)

point = Point(10, 20)
coordinates = match point
    Point(x, y) => { x, y }
```

The `match` branch and record literal syntax in these examples is illustrative; the pattern-matching ability to destructure constructors is approved, while the complete grammar is PAR-PAT-01 and PAR-COND-01.

An algebraic nominal type may declare multiple constructors in an indented constructor list. A constructor is qualified by its declaring type by default; the constructor's unqualified name is **not** automatically in scope (TYP-ADT-01, PAR-TYPE-06, RES-NAME-06).

```natcode
type Option
    Some(any)
    None

present = Option.Some(42)
absent = Option.None
```

A transparent alias can name a type expression or an existing constructor symbol. Constructor aliases preserve the original constructor identity and are usable in value construction and constructor patterns (TYP-ALI-01, TYP-ALI-02).

```natcode
alias Some = Option.Some

value = Some(42)
unwrapped = match value
    Some(x) => x
```

In **type-expression context**, applying an existing nominal constructor to type arguments describes a subset of that nominal type constrained by its payload values; in **value-expression context**, applying it constructs a value (TYP-NOM-05). Omitting the arguments names the entire nominal type with no further restriction.

```natcode
type Value(string)
type Value2(number)

alias Z = Value("one" | "two") | Value2(1 | 2)
alias Unrestricted = Value | Value2
```

For a single-payload constructor, `Value(A | B)` represents the same set of possible values as `Value(A) | Value(B)`. For multiple arguments, correlations must be preserved: `Point(1 | 2, 3 | 4)` represents four combinations, while `Point(1, 3) | Point(2, 4)` represents only two. The complete grammar for literal type expressions is PAR-TYPE-01.

Nominal types may contain other nominal types through ordinary constructor parameters (TYP-NOM-06).

```natcode
type Position(number, number)
type Color(string)

type Sprint(Position, Color)
// Equivalent unrestricted parameter types:
// type Sprint(Position(number, number), Color(string))

sprint = Sprint(Position(10, 20), Color("red"))
```

These nested values are constructor arguments; this declaration **does not imply** implicit projection, inheritance, an interface, independent-aspect composition, or automatic reconstruction after a nested value is transformed. No separate nominal-combination operator or construct is approved. The previously explored special combination syntax is rejected in favor of ordinary nesting. How a function might preserve a containing value while transforming a nested value, versus return an unrelated value, remains an open question (TYP-FN-05).

## Union and intersection types

- `|` is mathematical **set union**: a value belongs to `A | B` precisely when it belongs to `A` or `B`. Neither union nor intersection creates a new nominal identity (TYP-UNI-06).
- Simplification/consolidation of a union is permitted **only when it preserves the exact represented set of possibilities**. In particular, preserve correlations between fields and exclusions; do not merge distinct object variants in ways that admit combinations not present in the original union.
- `&` is mathematical **set intersection**: a value belongs to `A & B` precisely when it belongs to **both** types. It never constructs or combines constituent values and is distinct from `:` constraints (TYP-UNI-06).
- `never` denotes the empty type: no values inhabit it. Thus `A | never = A` and `A & never = never` (TYP-UNI-07).
- An intersection proven empty resolves to `never` and is **valid**, whether direct, nested, or involving explicit `never`. The declaration itself causes no warning or error merely for being empty. Attempting to provide a value where the type is uninhabited is an error (TYP-UNI-08).
- Independently declared, unrelated nominal constructor types are disjoint: for example, `UserId & ProductId` is `never` (TYP-UNI-09). Nominal subtyping or other explicit overlapping-identity relationships are not introduced by ordinary constructors.

For example, this union:

```natcode
{ kind: "left", value: number }
|
{ kind: "right", value: string }
```

must not be simplified into:

```natcode
{ kind: "left" | "right", value: number | string }
```

because the latter would allow a `"left"` tag paired with a `string` value, which the original type excludes. The record-type syntax here is illustrative.

Set identities include `A | B = B | A`, `A | A = A`, `A & B = B & A`, and `A & A = A`. More detailed structural membership and normalization choices remain TYP-UNI-04. Difference/complement operators remain TYP-UNI-05; generic types remain TYP-GEN-01; broader subtyping remains TYP-CON-03.

## Target directions

Natcode is intended to support a native interpreter and compilation to JavaScript, WebAssembly, C, and potentially Python (PY-TGT-01). These are project goals rather than language semantics. A shared compiler IR/runtime design and the implementation language are TOOL-ARCH-01.
