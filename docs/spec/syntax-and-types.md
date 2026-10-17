# Broader syntax and type-system decisions

Item status and open questions for this file are tracked in the [parser](../scorecards/parser.md), [types](../scorecards/types.md), and [tooling](../scorecards/tooling.md) scorecards.

## Code blocks and data structures

- **Code blocks are indentation-defined.** Braces `{}` are for **data structures**, not statement/function control-flow blocks.
- Conditional, lambda, function-definition, and data-object grammars are PAR-COND-01, PAR-FN-01, and PAR-OBJ-01; block syntax is PAR-BLK-02.

The following block is **conceptual**; its `if` form is illustrative:

```natcore
x = 10
if condition
    x = x + 1
result = x
```

It illustrates an indented nested scope, not a brace-delimited code block.

## Inference-first philosophy

Natcore seeks a TypeScript-like ergonomics with a substantially more functional model, emphasizing purity, piping, and pattern matching. These are **design goals** (see RUN-EFF-01, PAR-PIPE-01, and PAR-PAT-01).

Ordinary variable types and function return types are intended to be inferred. The optional declaration constraint `x: Type = expression` is permitted as an **assignability check** without replacing or widening the inferred type; this updates the earlier inference-only proposal.

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

See TYP-UNI-05 (difference and complement operators), TYP-CON-03 (subtyping), TYP-GEN-01 (generics), and TYP-ADT-01 (algebraic data types).

## Target directions

Natcore is intended to support a native interpreter and compilation to JavaScript, WebAssembly, C, and potentially Python (PY-TGT-01). These are project goals rather than language semantics. A shared compiler IR/runtime design and the implementation language are TOOL-ARCH-01.
