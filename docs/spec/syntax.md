# Block delimiters and data structure delimiters

> **Specification status:** Partial -- high-level delimiter choices are decided; parsing details are not.
>
> **Implementation readiness:** Blocked -- indentation and data structure grammar remain unspecified.
>
> **Implementation status:** Not tracked

## Agreed design decisions

- **Code blocks use indentation**, not curly braces (`{}`), to indicate nesting and boundaries.
- **Curly braces are reserved for data structures** and will not serve as alternate code-block delimiters.
- There is no plan to support a second brace-based code-block syntax.

These decisions apply to the language's intended syntax; they do **not** yet define a complete grammar or approve an implementation.

## Illustrative example (not a confirmed grammar)

```natcore
val person = {
    name: "Ada"
}
```

This demonstrates the intended distinction between braces for data and indentation for code. The syntax of record literals, `val` declarations, and record fields still needs separate definition.

## Open decisions

- Exactly how indentation is measured and where an indented block is required or permitted.
- How multiline expressions, especially pipes, interact with indentation.
- How blank lines, comments, and mixed tabs/spaces affect block boundaries.
- Which data structures use `{}` and what other delimiters (e.g. `[]`) mean.
- Whether data structure type declarations and value literals use different syntaxes.
