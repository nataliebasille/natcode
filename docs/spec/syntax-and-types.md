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
- `type` introduces named type declarations (PAR-TYPE-03). Constructor and generic declaration grammar remain open (PAR-TYPE-02).
- Ordinary variable bindings continue to use keyword-free `name = expression` declarations (PAR-DECL-01).

## Inference-first philosophy

Natcode seeks a TypeScript-like ergonomics with a substantially more functional model, emphasizing purity, piping, and pattern matching. These are **design goals** (see RUN-EFF-01, PAR-PIPE-01, and PAR-PAT-01).

Ordinary variable types and function return types are intended to be inferred. The optional declaration constraint `x: Type = expression` is permitted as an **assignability check** without replacing or widening the inferred type; this updates the earlier inference-only proposal.

## Named type identity

Named type declarations introduce distinct **nominal type identities** (TYP-NOM-01). Two independently declared named types are not interchangeable merely because they share the same structural members or underlying representation. A named type declaration does not create a transparent type alias.

For example, these declarations are *illustrative* (the exact declaration syntax is still PAR-TYPE-02):

```natcode
type UserId = number
type ProductId = number
```

Even with identical underlying representations, `UserId` and `ProductId` are different identities; the representation alone does not make their values assignable to each other. This identity rule does not by itself authorize constructing a `UserId` from an ordinary number.

## Union and intersection types

- `|` is type **union**.
- Simplification/consolidation of a union is permitted **only when it preserves the exact represented set of possibilities**. In particular, preserve correlations between fields and exclusions; do not merge distinct object variants in ways that admit combinations not present in the original union.
- `&` is reserved for type **intersection**, separate from `:` constraints.

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

See TYP-UNI-05 (difference and complement operators), TYP-CON-03 (subtyping), TYP-GEN-01 (generics), and TYP-ADT-01 (algebraic data types).

## Target directions

Natcode is intended to support a native interpreter and compilation to JavaScript, WebAssembly, C, and potentially Python (PY-TGT-01). These are project goals rather than language semantics. A shared compiler IR/runtime design and the implementation language are TOOL-ARCH-01.
