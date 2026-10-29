# Lexical structure (core subset)

Item status and open questions for this file are tracked in the [lexer scorecard](../scorecards/lexer.md).

Source text is UTF-8. Positions are reported as byte-offset spans plus 1-based line and column, where columns count Unicode scalar values.

## Tokens

| Category | Forms |
| --- | --- |
| Reserved keywords | `from`, `import`, `export`, `unique`, `type`, `fn`, `when`, `where`, `as`, `none`. These are case-sensitive, hard-reserved words (LEX-KW-01, LEX-KW-04 through LEX-KW-07, LEX-KW-12, LEX-KW-13, LEX-KW-15). |
| Boolean literals | `true`, `false` |
| Absence literal | `none` (built-in singleton value; LEX-KW-16) |
| Identifiers | Unicode identifiers per UAX #31: first character is `XID_Start` or `_`; following characters are `XID_Continue`. Letters of any script (`café`, `größe`, `π`, `中文`) are allowed. |
| Wildcard | A standalone `_` is its own token, never an identifier. `_x`, `__`, and `x_` are identifiers. |
| Numbers | Decimal integers (`42`) and decimal fractions (`3.14159`), using ASCII digits only. |
| Strings | Double-quoted, single-line: `"Alice"`, `"./math"`. Any character except a line break is allowed, including non-ASCII. |
| Operators and punctuation | `=` `=>` `:` `\|` `&` `\` `~` `/` `as?` `{` `}` `(` `)` `,` `.` `*` `+` `-` `%` `!` `<` `<=` `>` `>=` `==` `!=` `&&` `||` `?` `??` `@` |
| Comments | `// …` to end of line; `/* … */` block comments, which nest. Comment text produces no ordinary tokens. Physical line breaks inside block comments must remain visible to layout analysis (LEX-COM-04), whether through newline tokens or source-position/layout metadata. |
| Line breaks | LF or CRLF; each is one newline token. Spaces are insignificant between tokens. |

The reserved words are recognized as keywords regardless of position; they cannot name ordinary bindings. `unique` is the hard-reserved modifier in nominal declarations written `unique type`. `alias` is an ordinary identifier, not a declaration keyword. Keyword spellings are permitted as object property names, with details of the object grammar still to be determined (PAR-OBJ-01). `match`, `if`, `else`, `elif`, and `case` are not reserved words or language control-flow constructs. The unified matching and conditional keyword is `when` (LEX-KW-07). `return` is not a control-flow construct or keyword. `try`, `catch`, and `throw` are not reserved or built-in exception constructs (LEX-KW-11). `null` is not a native literal or reserved word. `none` is a hard-reserved literal whose type is the built-in singleton `None`, distinct from nominal constructors such as `Option.None`. `where` is the refinement-type keyword. `as` is a hard-reserved renaming keyword (LEX-KW-15) used in imports, exports, and function parameter aliases (PAR-IMP-07, PAR-EXP-02, PAR-FN-01). It cannot name a value, and it is not a type assertion or conversion. `as?` is one indivisible, distinct runtime-validation operator token (LEX-OP-05); whitespace-separated `as ?` is not the operator. Value-expression operators `-`, `!`, `<`, `<=`, `>`, `>=`, `==`, `!=`, `&&`, and `||` are tokenized per LEX-OP-03; `-` has both unary and binary parser roles. `/` is the arithmetic-division token in both value and type computations (LEX-OP-04). `\` and `~` are the type set difference and complement operator tokens (LEX-OP-07). `%` is the truncated-remainder token (LEX-OP-06). Euclidean modulo is approved semantically, but no spelling or token is yet approved (PAR-EXPR-03). Multi-character operators use maximal munch (LEX-OP-02). `?` is a postfix type-operator token (also used in optional record field declarations), `??` is a single value-level coalescing token by maximal munch, and `@` explicitly qualifies built-in type names. None of them changes the existing indivisible `as?` token (LEX-OP-05, LEX-OP-08). A future pipe operator remains open under PAR-PIPE-01. Further words, including `then` and generator/effect vocabulary, remain open questions (LEX-KW-03).

## Layout and indentation

Source indentation has **no fixed required width**. Explicit executable blocks are delimited by braces. Leading whitespace remains available for parsing `when` and `where` branch lists and nominal constructor lists, even when those constructs occur inside `{}` or `()`. These lists use their grammar and applicable branch constraints to establish membership: differing indentation alone is not a compile error when ownership is unambiguous. Blank lines and comment-only lines do not change grouping. An unresolved ambiguous grouping is a compilation error; the compiler must not guess or automatically reindent it.

Natcode has **no explicit line-continuation marker**. The `\` token is type set difference and never acts as a backslash continuation marker. Expressions may span physical lines after a trailing binary operator that still needs its right operand or within explicit parentheses without changing branch ownership (PAR-BLK-02). Canonical formatting uses four spaces and is part of the Natcode toolchain (TOOL-FMT-01). Tabs and other whitespace categories remain separately open in LEX-WS-02. Under LEX-COM-04, block comments preserve physical line boundaries for layout: removing comment text never joins two physical lines into one logical line. Comment-only lines remain layout-neutral, and following code retains the layout of its own physical line. The representation of these boundaries as newline tokens or source-position/layout metadata is an implementation choice.

## Identifier normalization

Identifiers are normalized to Unicode **NFC**, so visually identical spellings (precomposed `é` versus `e` followed by a combining acute accent) name the same identifier. String literal contents are **not** normalized.

## Number rules

- A digit separator `_` may appear only **between two digits**: `1_000` and `3.141_59` are valid; `1__0`, `1_`, `1_.5`, `1._5`, and `1.5_` are errors.
- A fraction requires a digit (or a misplaced `_`) immediately after the `.`. Otherwise the number ends before the `.`: `1.` is the number `1` followed by `.`, and `1.foo` is `1`, `.`, `foo`.
- A number cannot begin with `.`: `.5` is `.` followed by `5`.
- A leading `-` is not part of a number literal; it is the separate `-` operator token (LEX-OP-03), so `-1` is `-` followed by `1`.
- A number may not be directly followed by identifier characters. `42abc`, `1_a`, and `3.14x` are each a single error, as are `1e10` and `0x1F` (see LEX-NUM-05 and LEX-NUM-06).

## Strings

- Escape sequences are **not supported**. A backslash anywhere inside a string literal is an error, and it does not escape a following `"`.
- A string that reaches a line break or the end of input before its closing `"` is an error.

## Lexical errors

Any character that cannot begin a token is an error, including emoji, non-identifier symbols such as `∑` or `€`, and ASCII characters outside the operator set (for example `;`). An unterminated block comment is an error. Lexing recovers after every error and reports all diagnostics in source order.
