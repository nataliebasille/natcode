# Lexer scorecard

- **Prefix:** LEX
- **Covers:** Turning source text into tokens: keywords, identifiers, literals, operators, comments, whitespace, indentation, positions, and lexical errors.
- **Implementation:** `src/lexer.rs` · **Tests:** `cargo test --test lexer` (`tests/lexer/main.rs`)

## Positions and token stream

### LEX-POS-01 — Source positions

- **Status:** Implemented
- **Spec:** [lexical.md — Lexical structure](../spec/lexical.md#lexical-structure-core-subset)
- **Done when:** Every token and diagnostic carries a byte-offset span and a 1-based line and column, with columns counted in Unicode scalar values.
- **Evidence:** `tests/lexer/main.rs::positions::spans_and_positions_on_one_line`, `tests/lexer/main.rs::positions::positions_across_lines`, `tests/lexer/main.rs::positions::crlf_positions`, `tests/lexer/main.rs::positions::columns_count_unicode_scalars_not_bytes`, `tests/lexer/main.rs::positions::positions_after_multiline_block_comment`, `tests/lexer/main.rs::positions::token_text_matches_its_span_for_nfc_source`, `tests/lexer/main.rs::identifiers::non_english_identifier_positions`

### LEX-POS-02 — End-of-input token

- **Status:** Implemented
- **Spec:** [lexical.md — Lexical structure](../spec/lexical.md#lexical-structure-core-subset)
- **Done when:** Every token stream ends with exactly one `Eof` token positioned at the end of input, including for empty and erroneous sources.
- **Evidence:** `tests/lexer/main.rs::positions::eof_on_empty_source`, `tests/lexer/main.rs::positions::eof_position_at_end_of_input`, `tests/lexer/main.rs::positions::always_exactly_one_trailing_eof`

## Keywords

### LEX-KW-01 — Reserved module keywords

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `from`, `import`, and `export` lex as keywords; matching is case-sensitive; longer words containing them are identifiers.
- **Evidence:** `tests/lexer/main.rs::keywords::module_keywords_are_reserved`, `tests/lexer/main.rs::keywords::keywords_are_case_sensitive`, `tests/lexer/main.rs::keywords::words_containing_keywords_are_identifiers`

### LEX-KW-02 — Boolean literals

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `true` and `false` lex as boolean literal tokens.
- **Evidence:** `tests/lexer/main.rs::keywords::true_and_false_are_boolean_literals`

### LEX-KW-03 — Remaining keyword inventory

- **Status:** Open question
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Questions:**
  1. Which additional words, if any, should be reserved beyond the approved set?
  2. Should generator/effect words such as `yield`, `async`, or `await` become reserved? Their semantics remain open.
  3. Is `then` or any other branch-related word necessary once `when` handles conditionals?
- **Notes:** The lexer currently recognizes `from`, `import`, `export`, `type`, `fn`, `when`, `true`, and `false`; the approved `alias` keyword (LEX-KW-12) and `where` keyword (LEX-KW-13) are not yet implemented. `match` is no longer reserved (LEX-KW-07). `if`, `else`, `elif`, `case`, `return`, and `null` are explicitly excluded by approved decisions. Other words, such as `then`, `let`, `val`, `var`, `yield`, `async`, and `await`, currently lex as identifiers (`tests/lexer/main.rs::keywords::undecided_words_are_not_reserved_for_now`).

### LEX-KW-04 — Hard-reserved keyword strategy

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** Every designated keyword lexes as its keyword token regardless of syntactic context; no contextual keyword classification is used.
- **Evidence:** `tests/lexer/main.rs::keywords::keywords_are_reserved_regardless_of_context`
- **Notes:** Whether a keyword token is allowed in a property-key position is determined by the parser; it is never re-lexed as an identifier.

### LEX-KW-05 — Type declaration keyword

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `type` lexes as a hard-reserved keyword; longer words containing it remain identifiers.
- **Evidence:** `tests/lexer/main.rs::keywords::declaration_and_when_keywords_are_reserved`, `tests/lexer/main.rs::keywords::keywords_are_case_sensitive`, `tests/lexer/main.rs::keywords::words_containing_keywords_are_identifiers`

### LEX-KW-06 — Function declaration keyword

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `fn` lexes as a hard-reserved keyword; longer words containing it remain identifiers.
- **Evidence:** `tests/lexer/main.rs::keywords::declaration_and_when_keywords_are_reserved`, `tests/lexer/main.rs::keywords::keywords_are_case_sensitive`, `tests/lexer/main.rs::keywords::words_containing_keywords_are_identifiers`

### LEX-KW-07 — Unified `when` keyword

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `when` is hard-reserved and `match` is no longer reserved; longer identifiers containing `when` remain identifiers.
- **Evidence:** `tests/lexer/main.rs::keywords::declaration_and_when_keywords_are_reserved`, `tests/lexer/main.rs::keywords::match_is_an_identifier`, `tests/lexer/main.rs::keywords::keywords_are_case_sensitive`, `tests/lexer/main.rs::keywords::words_containing_keywords_are_identifiers`, `tests/lexer/main.rs::keywords::keywords_are_reserved_regardless_of_context`
- **Previous evidence:** `tests/lexer/main.rs::keywords::declaration_and_match_keywords_are_reserved`, `tests/lexer/main.rs::keywords::keywords_are_case_sensitive`, `tests/lexer/main.rs::keywords::words_containing_keywords_are_identifiers`
- **Notes:** Originally implemented for `match`; approved language syntax switched to `when` on 2026-10-18, and the lexer now emits `TokenKind::When` instead of `TokenKind::Match`.

### LEX-KW-08 — No separate conditional or case keywords

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `if`, `else`, `elif`, and `case` are not treated as reserved words.
- **Evidence:** `tests/lexer/main.rs::keywords::conditional_and_case_words_are_not_reserved`

### LEX-KW-09 — No null literal

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `null` is not recognized as a keyword or literal token; absence is represented through ordinary sum-type constructors such as `None`.
- **Evidence:** `tests/lexer/main.rs::keywords::null_is_an_identifier`
- **Notes:** Foreign-language null conversions are a separate interoperability issue.

### LEX-KW-10 — No return keyword

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `return` is not treated as a reserved word or control-flow token; functions return their final expression (PAR-FN-03).
- **Evidence:** `tests/lexer/main.rs::keywords::return_is_an_identifier`

### LEX-KW-12 — Alias declaration keyword

- **Status:** Approved
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `alias` lexes as a case-sensitive, hard-reserved keyword, and longer identifier spellings containing it remain identifiers.
- **Notes:** Documentation decision only; lexer implementation is future work.

### LEX-KW-13 — Refinement `where` keyword

- **Status:** Approved
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `where` lexes as a case-sensitive, hard-reserved keyword, and longer identifier spellings containing it remain identifiers.
- **Notes:** Required by the approved refinement-type syntax; lexer implementation is future work.

### LEX-KW-11 — No exception-handling keywords

- **Status:** Approved
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `try`, `catch`, and `throw` are ordinary identifiers, not exception-related keywords or tokens.
- **Notes:** The language-level exception mechanism is rejected (RUN-EFF-04).

## Identifiers

### LEX-ID-01 — Unicode identifiers (UAX #31)

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** An identifier starts with `XID_Start` or `_` and continues with `XID_Continue`, so letters of any script are accepted; combining marks and other-script digits may continue but not start one.
- **Evidence:** `tests/lexer/main.rs::identifiers::ascii_identifiers_of_every_case_style`, `tests/lexer/main.rs::identifiers::non_english_letters_are_identifiers`, `tests/lexer/main.rs::identifiers::non_english_identifier_in_declaration`, `tests/lexer/main.rs::identifiers::combining_marks_and_other_script_digits_continue_an_identifier`, `tests/lexer/main.rs::identifiers::combining_mark_cannot_start_an_identifier`, `tests/lexer/main.rs::identifiers::other_script_digit_cannot_start_a_token`

### LEX-ID-02 — NFC identifier normalization

- **Status:** Implemented
- **Spec:** [lexical.md — Identifier normalization](../spec/lexical.md#identifier-normalization)
- **Done when:** Identifier token text is NFC-normalized, so precomposed and decomposed spellings are equal; the span still covers the raw source.
- **Evidence:** `tests/lexer/main.rs::identifiers::decomposed_identifier_is_nfc_normalized`, `tests/lexer/main.rs::identifiers::precomposed_and_decomposed_spellings_have_equal_text`, `tests/lexer/main.rs::identifiers::already_normalized_identifier_text_is_unchanged`

### LEX-ID-03 — Standalone underscore token

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** A standalone `_` is its own token; `_x`, `__`, and `x_` are identifiers.
- **Evidence:** `tests/lexer/main.rs::identifiers::standalone_underscore_is_its_own_token`, `tests/lexer/main.rs::identifiers::underscore_prefixed_names_are_identifiers`

### LEX-ID-04 — Emoji identifiers

- **Status:** Open question
- **Spec:** [variables.md — Identifier categories](../spec/variables.md#identifier-categories)
- **Questions:**
  1. Which emoji sequences are admitted (single emoji, ZWJ sequences such as `👩‍💻`, skin-tone modifiers, variation selectors, flags, keycaps)?
  2. May an identifier mix emoji with letters or digits (`rocket🚀`, `🚀2`)?
  3. Are emoji identifiers normalized or compared by exact code points?
- **Notes:** Emoji are value names in principle, but are currently rejected as unexpected characters (`tests/lexer/main.rs::identifiers::emoji_identifier_is_rejected_for_now`, `tests/lexer/main.rs::identifiers::composed_emoji_is_a_single_error_token`).

### LEX-ID-05 — Joiner characters inside identifiers

- **Status:** Open question
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Questions:**
  1. May ZWJ (U+200D) and ZWNJ (U+200C) appear inside identifiers?
- **Notes:** Current Unicode includes both in `XID_Continue`, so the lexer accepts them after the first character (`a‍b` is one identifier). Untested.

## Numbers

### LEX-NUM-01 — Decimal integers and fractions

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `42` and `3.14159` each lex as one number token holding the source text; only ASCII digits form numbers.
- **Evidence:** `tests/lexer/main.rs::numbers::decimal_integers`, `tests/lexer/main.rs::numbers::decimal_fractions`

### LEX-NUM-02 — Digit separators between digits

- **Status:** Implemented
- **Spec:** [lexical.md — Number rules](../spec/lexical.md#number-rules)
- **Done when:** `1_000` and `3.141_59` are numbers; `1__0`, `1_`, `1_.5`, `1._5`, `1.5_`, and `1_000_` are single error tokens, and lexing recovers.
- **Evidence:** `tests/lexer/main.rs::numbers::digit_separators_between_digits`, `tests/lexer/main.rs::numbers::misplaced_digit_separators_are_errors`, `tests/lexer/main.rs::numbers::malformed_number_recovers`

### LEX-NUM-03 — Dots around numbers

- **Status:** Implemented
- **Spec:** [lexical.md — Number rules](../spec/lexical.md#number-rules)
- **Done when:** `1.` lexes as `1` then `.`; `.5` as `.` then `5`; `1.foo` as `1`, `.`, `foo`; a leading `-` is not part of a number.
- **Evidence:** `tests/lexer/main.rs::numbers::trailing_dot_is_not_part_of_number`, `tests/lexer/main.rs::numbers::leading_dot_is_not_part_of_number`, `tests/lexer/main.rs::numbers::dot_followed_by_identifier_after_number`, `tests/lexer/main.rs::numbers::minus_is_not_part_of_a_number`

### LEX-NUM-04 — Number directly followed by letters

- **Status:** Implemented
- **Spec:** [lexical.md — Number rules](../spec/lexical.md#number-rules)
- **Done when:** A number directly followed by identifier characters (`42abc`, `1_a`, `1e10`, `0x1F`) is one error token with an invalid-suffix diagnostic; other characters after a number are not absorbed.
- **Evidence:** `tests/lexer/main.rs::numbers::number_directly_followed_by_letters_is_one_error`, `tests/lexer/main.rs::numbers::number_suffix_error_recovers`, `tests/lexer/main.rs::numbers::non_identifier_character_after_number_is_not_a_suffix`

### LEX-NUM-05 — Exponent notation

- **Status:** Open question
- **Spec:** [lexical.md — Number rules](../spec/lexical.md#number-rules)
- **Questions:**
  1. Are exponents (`1e10`, `1E10`, `1e+10`, `1e-10`, `1.5e3`) supported, and which spellings?
- **Notes:** Currently rejected under LEX-NUM-04.

### LEX-NUM-06 — Non-decimal number literals

- **Status:** Open question
- **Spec:** [lexical.md — Number rules](../spec/lexical.md#number-rules)
- **Questions:**
  1. Are hexadecimal, octal, or binary literals (`0xFF`, `0o17`, `0b1010`) supported?
- **Notes:** Currently rejected under LEX-NUM-04.

## Strings

### LEX-STR-01 — Double-quoted single-line strings

- **Status:** Implemented
- **Spec:** [lexical.md — Strings](../spec/lexical.md#strings)
- **Done when:** `"…"` lexes as one string token whose text includes the quotes; any non-line-break character, including non-ASCII and comment markers, is content.
- **Evidence:** `tests/lexer/main.rs::strings::simple_string_includes_quotes_in_text`, `tests/lexer/main.rs::strings::empty_string`, `tests/lexer/main.rs::strings::relative_import_paths`, `tests/lexer/main.rs::strings::non_ascii_content_is_allowed`, `tests/lexer/main.rs::strings::comment_markers_inside_string_are_content`, `tests/lexer/main.rs::strings::adjacent_strings_are_separate_tokens`

### LEX-STR-02 — Backslashes rejected in strings

- **Status:** Implemented
- **Spec:** [lexical.md — Strings](../spec/lexical.md#strings)
- **Done when:** A string containing `\` is an error token with a diagnostic at the backslash; the backslash does not escape a following `"`.
- **Evidence:** `tests/lexer/main.rs::strings::backslash_in_string_is_rejected`, `tests/lexer/main.rs::strings::backslash_does_not_escape_closing_quote`

### LEX-STR-03 — Unterminated strings

- **Status:** Implemented
- **Spec:** [lexical.md — Strings](../spec/lexical.md#strings)
- **Done when:** A string reaching a line break or end of input is an error token; the line break is still emitted and lexing recovers.
- **Evidence:** `tests/lexer/main.rs::strings::unterminated_string_at_eof`, `tests/lexer/main.rs::strings::unterminated_string_stops_at_newline_and_recovers`

### LEX-STR-04 — String contents are not normalized

- **Status:** Implemented
- **Spec:** [lexical.md — Identifier normalization](../spec/lexical.md#identifier-normalization)
- **Done when:** String token text keeps the source code points exactly, without NFC normalization.
- **Evidence:** `tests/lexer/main.rs::identifiers::string_contents_are_not_normalized`

### LEX-STR-05 — String escape sequences

- **Status:** Open question
- **Spec:** [lexical.md — Strings](../spec/lexical.md#strings)
- **Questions:**
  1. Are escape sequences supported, and which ones (`\"`, `\\`, `\n`, `\t`, `\r`, `\u{…}`)?
- **Notes:** Currently any backslash is an error (LEX-STR-02).

### LEX-STR-06 — Multi-line strings

- **Status:** Open question
- **Spec:** [lexical.md — Strings](../spec/lexical.md#strings)
- **Questions:**
  1. Is there a multi-line string form, and how does it interact with indentation?

## Comments

### LEX-COM-01 — Line comments

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `//` starts a comment that ends before the line break and produces no token.
- **Evidence:** `tests/lexer/main.rs::comments::line_comment_ends_before_newline`, `tests/lexer/main.rs::comments::line_comment_at_eof`, `tests/lexer/main.rs::comments::line_comment_only_source`, `tests/lexer/main.rs::comments::comments_may_contain_non_ascii`

### LEX-COM-02 — Nested block comments

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `/* … */` comments produce no token and nest to any depth; `//` inside them is ignored; a stray `*/` is not a comment.
- **Evidence:** `tests/lexer/main.rs::comments::block_comment_inline`, `tests/lexer/main.rs::comments::block_comment_without_spaces`, `tests/lexer/main.rs::comments::nested_block_comments`, `tests/lexer/main.rs::comments::deeply_nested_block_comments`, `tests/lexer/main.rs::comments::line_comment_marker_inside_block_comment_is_ignored`, `tests/lexer/main.rs::comments::stray_block_comment_close_is_not_a_comment`

### LEX-COM-03 — Unterminated block comments

- **Status:** Implemented
- **Spec:** [lexical.md — Lexical errors](../spec/lexical.md#lexical-errors)
- **Done when:** A block comment without a matching `*/` is one error token to end of input with a diagnostic at its start.
- **Evidence:** `tests/lexer/main.rs::comments::unterminated_block_comment`, `tests/lexer/main.rs::comments::unterminated_nested_block_comment`

### LEX-COM-04 — Line breaks inside block comments

- **Status:** Open question
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Questions:**
  1. Do line breaks inside a multi-line block comment produce newline tokens, or does the comment join the two physical lines into one logical line?
  2. How does this interact with indentation (LEX-IND-01) and the broader comment design?
- **Notes:** Provisionally no newline tokens are produced (`tests/lexer/main.rs::comments::multiline_block_comment_emits_no_newlines`).

## Operators and punctuation

### LEX-OP-01 — Core operator and punctuation set

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `=` `=>` `:` `|` `&` `{` `}` `(` `)` `,` `.` `*` `+` each lex as their own token, with or without surrounding spaces.
- **Evidence:** `tests/lexer/main.rs::operators::every_approved_operator_and_punctuation`, `tests/lexer/main.rs::operators::operators_without_whitespace`

### LEX-OP-02 — Fat arrow by maximal munch

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `=>` is one token, `==` is two `=` tokens, and `= >` is not an arrow.
- **Evidence:** `tests/lexer/main.rs::operators::fat_arrow_uses_maximal_munch`, `tests/lexer/main.rs::operators::separated_equals_and_greater_than_is_not_an_arrow`

### LEX-OP-03 — Additional operators

- **Status:** Open question
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Questions:**
  1. Which further operators exist (`-`, comparisons, logical operators, a pipe operator)?
  2. Is negation a unary operator or part of number literals?
- **Notes:** Currently any other ASCII character is an error (LEX-ERR-02).

### LEX-OP-04 — Slash token for type set difference

- **Status:** Approved
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `/` lexes as its own token so type expressions can use the approved `A / B` set-difference operator.
- **Notes:** Any future value-expression meaning of `/` is separate from the approved type-expression meaning.

### LEX-OP-05 — Runtime-validation `as?` token

- **Status:** Approved
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `as?` lexes as one indivisible operator token; `as` alone remains an ordinary identifier, and whitespace-separated `as ?` does not form the operator.
- **Notes:** Parser and type semantics are PAR-VAL-01 and TYP-REF-12. The lexer implementation is future work.

## Whitespace and indentation

### LEX-WS-01 — Spaces and line breaks

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** Spaces are skipped; each LF or CRLF produces one newline token; leading indentation produces no token yet.
- **Evidence:** `tests/lexer/main.rs::whitespace::spaces_are_skipped`, `tests/lexer/main.rs::whitespace::lf_newline`, `tests/lexer/main.rs::whitespace::crlf_is_a_single_newline`, `tests/lexer/main.rs::whitespace::each_line_break_is_a_newline_token`, `tests/lexer/main.rs::whitespace::trailing_newline`, `tests/lexer/main.rs::whitespace::leading_indentation_is_not_tokenized`

### LEX-WS-02 — Tabs, lone carriage returns, and non-ASCII whitespace

- **Status:** Open question
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Questions:**
  1. Are tab characters allowed, and if so, between tokens only or also in indentation?
  2. Is a lone `\r` a line break, whitespace, or an error?
  3. Are non-ASCII spaces (for example U+00A0) whitespace or errors?
- **Notes:** All three are currently lexed as unexpected characters. Untested.

### LEX-IND-01 — Indentation tokens

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Code blocks and data structures](../spec/syntax-and-types.md#code-blocks-and-data-structures)
- **Questions:**
  1. How is the indentation unit defined (any consistent number of spaces, or a fixed width)?
  2. Are blank and comment-only lines ignored for indentation?
  3. Is indentation ignored inside `{}` and `()` (implicit line continuation)?
  4. Is there an explicit line-continuation form?
- **Notes:** Indentation-defined code blocks are part of the language; how they are tokenized is undecided. Depends on statement and block syntax (PAR-BLK-02).

### LEX-IND-02 — Malformed indentation errors

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Code blocks and data structures](../spec/syntax-and-types.md#code-blocks-and-data-structures)
- **Questions:**
  1. Which indentation errors exist (dedent to a level never opened, inconsistent units, mixed tabs and spaces) and how does lexing recover?

## Errors

### LEX-ERR-01 — Recoverable lexical errors

- **Status:** Implemented
- **Spec:** [lexical.md — Lexical errors](../spec/lexical.md#lexical-errors)
- **Done when:** Each error yields an error token plus a diagnostic with matching span, lexing continues, and diagnostics are reported in source order.
- **Evidence:** `tests/lexer/main.rs::positions::error_token_and_diagnostic_positions_agree`, `tests/lexer/main.rs::positions::diagnostics_are_reported_in_source_order`, `tests/lexer/main.rs::positions::unterminated_string_with_backslash_reports_both_in_source_order`, `tests/lexer/main.rs::positions::diagnostic_starts_never_decrease`, `tests/lexer/main.rs::operators::lexing_recovers_after_unexpected_characters`

### LEX-ERR-02 — Unexpected characters

- **Status:** Needs update
- **Spec:** [lexical.md — Lexical errors](../spec/lexical.md#lexical-errors)
- **Done when:** A run of characters that cannot start a token (emoji, symbols such as `∑` or `€`, ASCII characters outside the approved operator set) is one error token with one diagnostic; the newly approved `/` token is not rejected.
- **Previous evidence:** `tests/lexer/main.rs::operators::unsupported_ascii_characters_are_rejected`, `tests/lexer/main.rs::operators::adjacent_unexpected_characters_form_one_error`, `tests/lexer/main.rs::identifiers::non_letter_symbols_are_rejected`, `tests/lexer/main.rs::identifiers::rejected_symbol_after_identifier_recovers`
- **Notes:** The historical unsupported-character test currently includes `/`; it must be updated when LEX-OP-04 is implemented.
