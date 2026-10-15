# Functions: return types

> **Specification status:** Proposed
>
> **Implementation readiness:** Blocked -- type inference and associated edge cases are not defined.
>
> **Implementation status:** Not tracked
>
> **Authority:** This records the language owner's stated intent. It is not yet implementation-approved.

## Proposed rule

**Return types are always implicit.** Natcore should infer a function's return type rather than require an explicit return-type declaration.

The owner has not provided function declaration syntax yet, so no example grammar is proposed here.

## Open decisions

- How are return types inferred, particularly for recursion and mutually recursive functions?
- What happens when inference cannot determine a return type or branches return incompatible types?
- What restrictions, if any, apply to declaring functions with no inferable return type?
