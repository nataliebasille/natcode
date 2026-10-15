# Types as function-like constructs

> **Status:** Open design exploration -- not normative and not approved for implementation.

## Motivation

The language owner observes that TypeScript types can, in some ways, be viewed as functions, and wants to explore making Natcore types similar to functions in some respects.

This is a conceptual goal only. It does **not** establish whether Natcore types are first-class values, whether generic application uses function-call syntax, whether type constructors are callable, or whether type-level evaluation is supported.

## To explore later

- What aspects of functions should types share?
- Which operations happen at compile time versus runtime?
- How does the idea relate to generic placeholders, variant constructors, and constraints in [type declarations](../spec/types.md)?
