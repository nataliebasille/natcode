# Lexical structure (core subset)

Item status and open questions for this file are tracked in the [lexer scorecard](../scorecards/lexer.md).

Source text is UTF-8. Positions are reported as byte-offset spans plus 1-based line and column, where columns count Unicode scalar values.

## Tokens

| Category | Forms |
| --- | --- |
| Reserved keywords | `from`, `import`, `export`, `type`, `alias`, `fn`, `match`. These are case-sensitive, hard-reserved words (LEX-KW-01, LEX-KW-04 through LEX-KW-07, LEX-KW-11). |
| Boolean literals | `true`, `false` |
| Identifiers | Unicode identifiers per UAX #31: first character is `XID_Start` or `_`; following characters are `XID_Continue`. Letters of any script (`café`, `größe`, `π`, `中文`) are allowed. |
| Wildcard | A standalone `_` is its own token, never an identifier. `_x`, `__`, and `x_` are identifiers. |
| Numbers | Decimal integers (`42`) and decimal fractions (`3.14159`), using ASCII digits only. |
| Strings | Double-quoted, single-line: `"Alice"`, `"./math"`. Any character except a line break is allowed, including non-ASCII. |
| Operators and punctuation | `=` `=>` `:` `\|` `&` `{` `}` `(` `)` `,` `.` `*` `+` |
| Comments | `// …` to end of line; `/* … */` block comments, which nest. Comments produce no tokens. Whether line breaks inside a block comment produce newline tokens is LEX-COM-04. |
| Line breaks | LF or CRLF; each is one newline token. Spaces are insignificant between tokens. |

The reserved words are recognized as keywords regardless of position; they cannot name ordinary bindings. Keyword spellings are permitted as object property names, with details of the object grammar still to be determined (PAR-OBJ-01). `if`, `else`, `elif`, and `case` are not reserved words or language control-flow constructs. `return` is not a control-flow construct or keyword. `try`, `catch`, and `throw` are not reserved or built-in exception constructs (LEX-KW-11). `null` is not a native literal or reserved word; absence is represented using constructors such as `None`. Further words, including `then` and generator/effect vocabulary, remain open questions (LEX-KW-03).

## Identifier normalization

Identifiers are normalized to Unicode **NFC**, so visually identical spellings (precomposed `é` versus `e` followed by a combining acute accent) name the same identifier. String literal contents are **not** normalized.

## Number rules

- A digit separator `_` may appear only **between two digits**: `1_000` and `3.141_59` are valid; `1__0`, `1_`, `1_.5`, `1._5`, and `1.5_` are errors.
- A fraction requires a digit (or a misplaced `_`) immediately after the `.`. Otherwise the number ends before the `.`: `1.` is the number `1` followed by `.`, and `1.foo` is `1`, `.`, `foo`.
- A number cannot begin with `.`: `.5` is `.` followed by `5`.
- A leading `-` is not part of a number literal, and `-` is not an operator (see LEX-OP-03).
- A number may not be directly followed by identifier characters. `42abc`, `1_a`, and `3.14x` are each a single error, as are `1e10` and `0x1F` (see LEX-NUM-05 and LEX-NUM-06).

## Strings

- Escape sequences are **not supported**. A backslash anywhere inside a string literal is an error, and it does not escape a following `"`.
- A string that reaches a line break or the end of input before its closing `"` is an error.

## Lexical errors

Any character that cannot begin a token is an error, including emoji, non-identifier symbols such as `∑` or `€`, and ASCII characters outside the operator set (for example `-`, `/`, `;`, `@`). An unterminated block comment is an error. Lexing recovers after every error and reports all diagnostics in source order.
