# Type declarations and variants

> **Specification status:** Proposed
>
> **Implementation readiness:** Blocked -- generic binding, constraints, and other semantics are not yet specified.
>
> **Implementation status:** Not tracked
>
> **Authority:** These are the language owner's current ideas, recorded without filling in missing rules. Nothing on this page is implementation-approved.

## Variants without payloads

Proposed syntax:

```natcore
type X =
  One
  Two
```

The intended values of `X` are `One` and `Two`. The owner specified qualified access:

```natcore
val o = X.One
val t = X.Two
```

Whether qualification is always required is not yet specified.

## Variants with generic placeholders

Proposed syntax:

```natcore
type Option =
  Some(T)
  None
```

```natcore
type Either =
  Left(L)
  Right(R)
```

These examples are intended to support generic payloads. The rules for introducing, binding, and inferring `T`, `L`, and `R` are **not yet defined**. The examples show variant *definitions*, not confirmed invocation syntax.

## Generic constraints

Proposed syntax:

```natcore
type X =
  One(T: string)
  Two(U: T)
```

The owner describes this as a generic type with constraints. The syntax places `: string` after `T` and `: T` after `U`. It is **not yet specified** exactly what those constraints mean or how `T` is in scope in `Two(U: T)`.

## Open decisions

- **Parameter scope:** Does `T` introduced in `One(T: string)` belong to the whole type `X`, or only to the `One` variant? If it is variant-local, how does `Two` refer to it?
- **Generic binding:** How are `T`, `L`, `R`, and `U` introduced and inferred when no type parameters appear in the type declaration's header?
- **Constraints:** What does `T: string` mean (e.g. assignability, exact type, or another constraint relationship)? How is `U: T` interpreted?
- **Construction:** What is the value-construction syntax for variants with payloads? Is `Option.Some(value)` required, or can a variant be used without qualification?
- **Payload-free variants:** How are generic arguments handled for variants such as `None`, which do not mention the placeholder?

## Related design idea

The owner wants Natcore types to resemble functions in some respects, inspired by viewing TypeScript types as function-like. This is a *design direction*, not a defined rule; see [type/function relationship](../design/type-function-relationship.md).
