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
- **Notes:** The lexer currently recognizes `from`, `import`, `export`, `unique`, `type`, `fn`, `when`, `where`, `as`, `true`, and `false`. `match` is no longer reserved (LEX-KW-07). `if`, `else`, `elif`, `case`, `return`, and `null` are explicitly excluded by approved decisions. Other words, such as `then`, `let`, `val`, `var`, `yield`, `async`, and `await`, currently lex as identifiers (`tests/lexer/main.rs::keywords::undecided_words_are_not_reserved_for_now`).

### LEX-KW-04 — Hard-reserved keyword strategy

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** Every designated keyword, including `as` (LEX-KW-15), lexes as its keyword token regardless of syntactic context; no contextual keyword classification is used.
- **Evidence:** `tests/lexer/main.rs::keywords::keywords_are_reserved_regardless_of_context`, `tests/lexer/main.rs::keywords::as_is_reserved`
- **Previous evidence:** `tests/lexer/main.rs::keywords::keywords_are_reserved_regardless_of_context`
- **Notes:** The context test now also covers `as` in property-key and member positions. Whether a keyword token is allowed in a property-key position is determined by the parser; it is never re-lexed as an identifier.

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

### LEX-KW-16 — Built-in `none` literal keyword

- **Status:** Approved
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** Lowercase `none` lexes as a hard-reserved literal token with no ordinary identifier interpretation, while PascalCase `None` remains a type/constructor identifier.

### LEX-KW-10 — No return keyword

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `return` is not treated as a reserved word or control-flow token; functions return their final expression (PAR-FN-03).
- **Evidence:** `tests/lexer/main.rs::keywords::return_is_an_identifier`

### LEX-KW-12 — Unique nominal modifier keyword

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `unique` lexes as a case-sensitive, hard-reserved keyword, and longer identifiers containing it remain identifiers. The approved nominal introducer is `unique type` (PAR-TYPE-03).
- **Evidence:** `tests/lexer/main.rs::keywords::unique_and_where_keywords_are_reserved`, `tests/lexer/main.rs::keywords::keywords_are_case_sensitive`, `tests/lexer/main.rs::keywords::words_containing_keywords_are_identifiers`, `tests/lexer/main.rs::keywords::keywords_are_reserved_regardless_of_context`
- **Previous evidence:** `tests/lexer/main.rs::keywords::alias_and_where_keywords_are_reserved`

### LEX-KW-14 — `alias` is an ordinary identifier

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `alias` is not a reserved keyword and lexes as an identifier; it cannot introduce a type declaration.
- **Evidence:** `tests/lexer/main.rs::keywords::alias_is_an_identifier`

### LEX-KW-15 — Hard-reserved renaming keyword `as`

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `as` lexes as a hard-reserved, case-sensitive keyword regardless of context, so `as = 42` is invalid. Longer names such as `aside` remain identifiers. `as?` remains one distinct runtime-validation operator token (LEX-OP-05).
- **Evidence:** `tests/lexer/main.rs::keywords::as_is_reserved`, `tests/lexer/main.rs::keywords::words_containing_as_are_identifiers`, `tests/lexer/main.rs::keywords::keywords_are_case_sensitive`, `tests/lexer/main.rs::keywords::keywords_are_reserved_regardless_of_context`, `tests/lexer/main.rs::operators::as_alone_is_a_keyword`
- **Notes:** Approved for renaming syntax in imports, exports, and named function parameters (PAR-IMP-07, PAR-EXP-02, PAR-FN-01); never treat plain `as` as a type assertion or conversion.

### LEX-KW-13 — Refinement `where` keyword

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `where` lexes as a case-sensitive, hard-reserved keyword, and longer identifier spellings containing it remain identifiers.
- **Evidence:** `tests/lexer/main.rs::keywords::unique_and_where_keywords_are_reserved`, `tests/lexer/main.rs::keywords::keywords_are_case_sensitive`, `tests/lexer/main.rs::keywords::words_containing_keywords_are_identifiers`, `tests/lexer/main.rs::keywords::keywords_are_reserved_regardless_of_context`
- **Notes:** Required by the approved refinement-type syntax.

### LEX-KW-11 — No exception-handling keywords

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `try`, `catch`, and `throw` are ordinary identifiers, not exception-related keywords or tokens.
- **Evidence:** `tests/lexer/main.rs::keywords::exception_words_are_identifiers`
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

- **Status:** Approved
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** Block comments preserve the physical line boundaries they contain for layout analysis. Removing comment text must never join physically separate lines into one logical line. Blank or comment-only physical lines are layout-neutral and do not establish or change indentation ownership; code following a comment is interpreted using its own physical line and indentation. The front end may represent retained boundaries as newline tokens or source-position/layout metadata.
- **Notes:** Approved 2026-10-22. The current lexer emits no newline tokens for multi-line block comments (`tests/lexer/main.rs::comments::multiline_block_comment_emits_no_newlines`); this is acceptable only if the parser/layout analysis retains the physical boundaries by other means. Implementation conformance and layout integration remain to be established under LEX-IND-01/PAR-LAYOUT-01.

## Operators and punctuation

### LEX-OP-01 — Core operator and punctuation set

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `=` `=>` `:` `|` `&` `{` `}` `(` `)` `,` `.` `*` `+` each lex as their own token, with or without surrounding spaces.
- **Evidence:** `tests/lexer/main.rs::operators::every_approved_operator_and_punctuation`, `tests/lexer/main.rs::operators::operators_without_whitespace`

### LEX-OP-02 — Multi-character operators by maximal munch

- **Status:** Needs update
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `=>`, `<=`, `>=`, `==`, `!=`, `&&`, `||`, and `??` each lex as one token by maximal munch; whitespace-separated characters do not combine into an operator.
- **Previous evidence:** `tests/lexer/main.rs::operators::multi_character_operators_are_single_tokens`, `tests/lexer/main.rs::operators::maximal_munch_takes_the_longest_operator_first`, `tests/lexer/main.rs::operators::separated_characters_do_not_form_an_operator`, `tests/lexer/main.rs::operators::fat_arrow_uses_maximal_munch`, `tests/lexer/main.rs::operators::separated_equals_and_greater_than_is_not_an_arrow`
- **Notes:** Maximal munch applies left to right, so `===` is `==` `=`, `<=>` is `<=` `>`, `&&&` is `&&` `&`, and `|||` is `||` `|`. Characters separated by whitespace, a line break, or a block comment never combine. The previous tests pinned the superseded rule that `==` is two `=` tokens.

### LEX-OP-03 — Value-expression operators

- **Status:** Implemented
- **Spec:** [syntax-and-types.md — Value expression grammar and precedence](../spec/syntax-and-types.md#value-expression-grammar-and-precedence)
- **Done when:** `-`, `!`, `<`, `<=`, `>`, `>=`, `==`, `!=`, `&&`, and `||` are recognized as operator tokens in addition to the existing core set. `!` is a prefix value operator; `-` has both prefix and binary roles, determined by the parser; `&&` and `||` are Boolean value operators. Their parser precedence is PAR-EXPR-01.
- **Evidence:** `tests/lexer/main.rs::operators::value_expression_operators`, `tests/lexer/main.rs::operators::value_expression_operators_without_whitespace`, `tests/lexer/main.rs::operators::value_operator_ends_an_unexpected_character_run`, `tests/lexer/main.rs::numbers::minus_is_not_part_of_a_number`
- **Notes:** A future pipe operator remains open under PAR-PIPE-01 and is not approved here. Leading `-` remains an operator rather than part of a number literal (LEX-NUM-03). The lexer only produces the tokens; unary/binary roles and precedence belong to the parser (PAR-EXPR-01).

### LEX-OP-04 — Slash token for arithmetic division

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `/` lexes as its own arithmetic division token in both value expressions and type-level numeric computations; its behavior is not type set difference.
- **Evidence:** `tests/lexer/main.rs::operators::slash_is_a_token`, `tests/lexer/main.rs::operators::double_slash_and_slash_star_still_start_comments`, `tests/lexer/main.rs::operators::every_approved_operator_and_punctuation`, `tests/lexer/main.rs::comments::stray_block_comment_close_is_not_a_comment`
- **Notes:** The implemented `/` token remains unchanged and its passing lexer tests still apply; only the formerly approved type-context interpretation has been replaced by `\` (PAR-TYPE-08). `//` and `/*` still begin comments, so `A//B` and `A/*B` start comments rather than producing `/` tokens; a stray `*/` lexes as `*` followed by `/`.

### LEX-OP-05 — Runtime-validation `as?` token

- **Status:** Needs update
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `as?` lexes as one indivisible operator token; `as` alone lexes as a hard-reserved keyword (LEX-KW-15), and whitespace-separated `as ?` does not form the operator.
- **Previous evidence:** `tests/lexer/main.rs::operators::as_question_is_one_token`, `tests/lexer/main.rs::operators::as_alone_is_a_keyword`, `tests/lexer/main.rs::operators::separated_as_and_question_mark_is_not_the_operator`, `tests/lexer/main.rs::operators::as_question_requires_the_whole_word_as`, `tests/lexer/main.rs::operators::question_mark_after_as_question_is_unexpected`, `tests/lexer/main.rs::operators::every_approved_operator_and_punctuation`, `tests/lexer/main.rs::operators::as_alone_is_an_identifier`
- **Notes:** Parser and type semantics are PAR-VAL-01 and TYP-REF-12. The operator forms only when the complete identifier is exactly `as` and is immediately followed by `?`; in `has?`, `As?`, or `alias?`, the `?` is a separate legal punctuation token, not an unexpected character. The former `as_alone_is_an_identifier` test was replaced by `as_alone_is_a_keyword`. Plain `as` is hard-reserved; it is not a type assertion or conversion.

### LEX-OP-06 — Percent token for truncated remainder

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `%` lexes as a single operator token for value-expression truncated remainder (PAR-EXPR-01, TYP-ARITH-02), with ordinary surrounding tokens lexed separately.
- **Evidence:** `tests/lexer/main.rs::operators::percent_is_a_remainder_token`, `tests/lexer/main.rs::operators::every_approved_operator_and_punctuation`, `tests/lexer/main.rs::operators::new_operators_end_an_unexpected_character_run`, `tests/lexer/main.rs::operators::new_operator_spans`
- **Notes:** Do not infer a token for Euclidean modulo; its spelling remains open (PAR-EXPR-03). The lexer emits `TokenKind::Remainder`; `%` never combines with a following character, so `%%=` is `%`, `%`, `=`.

### LEX-OP-07 — Set difference and complement tokens

- **Status:** Implemented
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `\` and `~` lex as independent ASCII operator tokens for type-expression set difference and prefix set complement respectively; `/` continues to lex as division, `!` continues to lex as Boolean NOT, and strings still reject backslash characters under LEX-STR-02. Backslash is not an explicit line-continuation token.
- **Evidence:** `tests/lexer/main.rs::operators::backslash_and_tilde_are_set_operator_tokens`, `tests/lexer/main.rs::operators::set_operators_stay_distinct_from_slash_and_not`, `tests/lexer/main.rs::operators::backslash_is_not_a_line_continuation`, `tests/lexer/main.rs::operators::backslash_inside_a_string_is_still_rejected`, `tests/lexer/main.rs::operators::every_approved_operator_and_punctuation`, `tests/lexer/main.rs::operators::new_operators_end_an_unexpected_character_run`, `tests/lexer/main.rs::operators::new_operator_spans`, `tests/lexer/main.rs::strings::backslash_in_string_is_rejected`
- **Notes:** Approved 2026-10-25 (PAR-TYPE-08, TYP-UNI-05). The lexer emits `TokenKind::SetDifference` and `TokenKind::SetComplement`. Neither combines with a following character, so `\\` and `~~` are each two tokens and `~=` is `~` then `=`. A `\` at the end of a line is followed by an ordinary newline token.

## Whitespace and indentation

### LEX-OP-08 — Optional, coalescing and built-in qualifier tokens

- **Status:** Approved
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** `?`, maximal-munch `??`, and `@` lex as punctuation/operator tokens, while `as?` remains one indivisible token; whitespace prevents forming `??` or `as?` across separate tokens.

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

### LEX-IND-01 — Context-sensitive layout and indentation

- **Status:** Approved
- **Spec:** [lexical.md — Layout and indentation](../spec/lexical.md#layout-and-indentation)
- **Done when:** The front end retains enough source positions, line breaks, and leading whitespace to recognize layout-sensitive `when`, `where`, and nominal constructor lists, including within `{}` and `()`, without requiring a fixed indentation width. Blank and comment-only lines do not alter branch ownership. Syntactically unambiguous structures are accepted regardless of indentation width or alignment, while unresolved layout ambiguity is a compile error rather than silently guessed. No explicit backslash line continuation is supported.
- **Notes:** Approved 2026-10-21. Braces delimit executable scopes, while branch and constructor lists have their own layout-sensitive structure. Exact token representation and malformed-layout recovery are implementation concerns (LEX-IND-02, PAR-LAYOUT-01). Canonical formatting uses four spaces (TOOL-FMT-01). Whether newlines internal to block comments emit tokens remains LEX-COM-04; tab acceptance remains LEX-WS-02. This is a design approval, not an implementation claim.

### LEX-IND-02 — Malformed indentation errors

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Code blocks and data structures](../spec/syntax-and-types.md#code-blocks-and-data-structures)
- **Questions:**
  1. Which invalid or ambiguous layout cases require diagnostics after grammar-based recovery, and how does the lexer/parser recover to report subsequent errors?
  2. How should diagnostics describe each plausible invalid branch grouping without silently changing meaning? Tabs and non-ASCII whitespace remain LEX-WS-02.

## Errors

### LEX-ERR-01 — Recoverable lexical errors

- **Status:** Implemented
- **Spec:** [lexical.md — Lexical errors](../spec/lexical.md#lexical-errors)
- **Done when:** Each error yields an error token plus a diagnostic with matching span, lexing continues, and diagnostics are reported in source order.
- **Evidence:** `tests/lexer/main.rs::positions::error_token_and_diagnostic_positions_agree`, `tests/lexer/main.rs::positions::diagnostics_are_reported_in_source_order`, `tests/lexer/main.rs::positions::unterminated_string_with_backslash_reports_both_in_source_order`, `tests/lexer/main.rs::positions::diagnostic_starts_never_decrease`, `tests/lexer/main.rs::operators::lexing_recovers_after_unexpected_characters`

### LEX-ERR-02 — Unexpected characters

- **Status:** Needs update
- **Spec:** [lexical.md — Lexical errors](../spec/lexical.md#lexical-errors)
- **Done when:** A run of characters that cannot start a token (emoji, symbols such as `∑` or `€`, ASCII characters outside the approved operator set) is one error token with one diagnostic; the approved `/`, `@`, and `?` tokens are not rejected.
- **Previous evidence:** `tests/lexer/main.rs::operators::unsupported_ascii_characters_are_rejected`, `tests/lexer/main.rs::operators::adjacent_unexpected_characters_form_one_error`, `tests/lexer/main.rs::operators::slash_ends_an_unexpected_character_run`, `tests/lexer/main.rs::identifiers::non_letter_symbols_are_rejected`, `tests/lexer/main.rs::identifiers::rejected_symbol_after_identifier_recovers`
- **Notes:** The unsupported-character test no longer includes `/`, which is a token under LEX-OP-04, or `-`, `!`, `<`, and `>`, which are tokens under LEX-OP-03. `tests/lexer/main.rs::operators::lexing_recovers_after_unexpected_characters` now uses `;` instead of `-`, and `tests/lexer/main.rs::numbers::minus_is_not_part_of_a_number` now expects a `-` token rather than an error. It also no longer includes `%`, `\`, or `~`, which are tokens under LEX-OP-06 and LEX-OP-07. This new revision additionally accepts `@` and `?` under LEX-OP-08.
