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
