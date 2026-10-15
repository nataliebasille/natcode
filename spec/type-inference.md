# Type inference and union equivalence

> **Specification status:** Partial -- union equivalence rules below are agreed; the broader inference system is not yet designed.
>
> **Implementation readiness:** Blocked -- inference rules, type representations, and interactions with generic constraints are not yet fully specified.
>
> **Implementation status:** Not tracked

## Agreed rules

- Natcore must preserve the semantic distinctions between possible inferred types. In particular, merging union members must not introduce additional combinations of values that the original type excluded.
- Treat the type-union operator `|` as a union of the sets of values described by its operands, for the purposes of the equivalences here.
- An equivalence of the form `F(A | B) = F(A) | F(B)` is valid **only when** `F` preserves unions in that position. Being a type constructor alone is not sufficient.
- If the equality does not hold, the two expressions must not be treated as interchangeable.

## Single-payload variant

For a variant constructor wrapping one payload, union distribution holds:

```natcore
Either.Left(number) | Either.Left(string)
```

is equivalent to:

```natcore
Either.Left(number | string)
```

These snippets illustrate *type expressions*, not yet-approved surface syntax for writing type annotations.

## Multiple related payload positions

Consider a two-field `Updated` variant with a common parameter `T` for both fields. In general:

```natcore
Updated(number, number) | Updated(string, string)
```

is **not** equivalent to:

```natcore
Updated(number | string, number | string)
```

The right-hand expression also includes `Updated(number, string)` and `Updated(string, number)` combinations, which the left-hand union excludes. This is an illustration of the type algebra, not a decision about Natcore's eventual generic declaration syntax.

In set terms, if `F(T) = T × T`, then generally:

- `F(A ∪ B)` includes `(A × B)` and `(B × A)`.
- `F(A) ∪ F(B)` includes only `(A × A)` and `(B × B)`.

## Open decisions

- Full inference rules for functions and values, including generalization, recursive inference, and branch types.
- How the `any` placeholder in proposed variant definitions behaves and what type-level information constructors retain.
- Whether developers can name or manipulate inferred types, and if so how. Explicit `val` and return-type annotations are not intended.
- How equivalent types are displayed in errors and tooling.

## Implementation boundary

The specification requires semantic equivalence and preservation of distinctions described above. The choice of internal type representation and normalization algorithm belongs in `compiler/` and is **not** specified here.
