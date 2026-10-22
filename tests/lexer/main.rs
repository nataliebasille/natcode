//! Lexer tests. Each test is cited as evidence by an item in
//! `docs/scorecards/lexer.md`.
//!
//! Behavior for Open question items (emoji identifiers, indentation, tabs,
//! lone `\r`, non-ASCII whitespace, ZWJ/ZWNJ inside identifiers) is
//! deliberately untested, except where a test pins current provisional
//! behavior and says so.

use TokenKind::*;
use natcode::lexer::{DiagnosticKind, LexResult, Position, Span, Token, TokenKind, tokenize};

// `TokenKind::String` shadows the prelude `String` in this file.
type Text = std::string::String;

/// Tokens without the trailing `Eof`, as `(kind, text)` pairs.
fn toks(src: &str) -> Vec<(TokenKind, Text)> {
    let result = tokenize(src);
    assert_single_trailing_eof(src, &result);
    result.tokens[..result.tokens.len() - 1]
        .iter()
        .map(|t| (t.kind, t.text.clone()))
        .collect()
}

fn expected(pairs: &[(TokenKind, &str)]) -> Vec<(TokenKind, Text)> {
    pairs.iter().map(|(k, s)| (*k, s.to_string())).collect()
}

/// Asserts the token stream and that no diagnostics were reported.
fn assert_clean(src: &str, pairs: &[(TokenKind, &str)]) {
    let result = tokenize(src);
    assert!(
        result.diagnostics.is_empty(),
        "unexpected diagnostics for {src:?}: {:?}",
        result.diagnostics
    );
    assert_eq!(toks(src), expected(pairs), "tokens for {src:?}");
}

/// Asserts the token stream regardless of diagnostics.
fn assert_tokens(src: &str, pairs: &[(TokenKind, &str)]) {
    assert_eq!(toks(src), expected(pairs), "tokens for {src:?}");
}

fn kinds(src: &str) -> Vec<TokenKind> {
    toks(src).into_iter().map(|(k, _)| k).collect()
}

fn diag_kinds(src: &str) -> Vec<DiagnosticKind> {
    tokenize(src).diagnostics.iter().map(|d| d.kind).collect()
}

fn assert_single_trailing_eof(src: &str, result: &LexResult) {
    let eofs = result.tokens.iter().filter(|t| t.kind == Eof).count();
    assert_eq!(eofs, 1, "expected exactly one Eof for {src:?}");
    assert_eq!(
        result.tokens.last().map(|t| t.kind),
        Some(Eof),
        "Eof must be last for {src:?}"
    );
}

fn token(src: &str, index: usize) -> Token {
    tokenize(src).tokens[index].clone()
}

fn span(start: usize, end: usize) -> Span {
    Span { start, end }
}

fn pos(line: u32, column: u32) -> Position {
    Position { line, column }
}

mod keywords {
    use super::*;

    #[test]
    fn module_keywords_are_reserved() {
        assert_clean(
            "from import export",
            &[(From, "from"), (Import, "import"), (Export, "export")],
        );
    }

    #[test]
    fn declaration_and_when_keywords_are_reserved() {
        assert_clean(
            "type unique fn when",
            &[
                (Type, "type"),
                (Unique, "unique"),
                (Fn, "fn"),
                (When, "when"),
            ],
        );
    }

    #[test]
    fn unique_and_where_keywords_are_reserved() {
        assert_clean("unique where", &[(Unique, "unique"), (Where, "where")]);
    }

    #[test]
    fn alias_is_an_identifier() {
        assert_clean(
            "alias = 1",
            &[(Identifier, "alias"), (Equals, "="), (Number, "1")],
        );
    }

    #[test]
    fn match_is_an_identifier() {
        assert_clean(
            "match = when",
            &[(Identifier, "match"), (Equals, "="), (When, "when")],
        );
    }

    #[test]
    fn true_and_false_are_boolean_literals() {
        assert_clean("true false", &[(True, "true"), (False, "false")]);
    }

    #[test]
    fn keywords_are_case_sensitive() {
        assert_clean(
            "From IMPORT Export True FALSE Type FN When Unique WHERE",
            &[
                (Identifier, "From"),
                (Identifier, "IMPORT"),
                (Identifier, "Export"),
                (Identifier, "True"),
                (Identifier, "FALSE"),
                (Identifier, "Type"),
                (Identifier, "FN"),
                (Identifier, "When"),
                (Identifier, "Unique"),
                (Identifier, "WHERE"),
            ],
        );
    }

    #[test]
    fn words_containing_keywords_are_identifiers() {
        assert_clean(
            "fromage imports exported trueish false_ _from types typeof fnord _fn fn2 whenever somewhen when_ uniqueness _unique aliases _alias nowhere wherever where2",
            &[
                (Identifier, "fromage"),
                (Identifier, "imports"),
                (Identifier, "exported"),
                (Identifier, "trueish"),
                (Identifier, "false_"),
                (Identifier, "_from"),
                (Identifier, "types"),
                (Identifier, "typeof"),
                (Identifier, "fnord"),
                (Identifier, "_fn"),
                (Identifier, "fn2"),
                (Identifier, "whenever"),
                (Identifier, "somewhen"),
                (Identifier, "when_"),
                (Identifier, "uniqueness"),
                (Identifier, "_unique"),
                (Identifier, "aliases"),
                (Identifier, "_alias"),
                (Identifier, "nowhere"),
                (Identifier, "wherever"),
                (Identifier, "where2"),
            ],
        );
    }

    #[test]
    fn keywords_are_reserved_regardless_of_context() {
        // Property-key and member positions still yield keyword tokens; whether
        // the parser accepts them there is PAR-OBJ-01.
        assert_clean(
            "{ type: 1, fn: 2 }.when",
            &[
                (LBrace, "{"),
                (Type, "type"),
                (Colon, ":"),
                (Number, "1"),
                (Comma, ","),
                (Fn, "fn"),
                (Colon, ":"),
                (Number, "2"),
                (RBrace, "}"),
                (Dot, "."),
                (When, "when"),
            ],
        );
        assert_clean("when = fn", &[(When, "when"), (Equals, "="), (Fn, "fn")]);
        assert_clean(
            "x.unique.where",
            &[
                (Identifier, "x"),
                (Dot, "."),
                (Unique, "unique"),
                (Dot, "."),
                (Where, "where"),
            ],
        );
    }

    #[test]
    fn conditional_and_case_words_are_not_reserved() {
        assert_clean(
            "if else elif case",
            &[
                (Identifier, "if"),
                (Identifier, "else"),
                (Identifier, "elif"),
                (Identifier, "case"),
            ],
        );
    }

    #[test]
    fn null_is_an_identifier() {
        assert_clean(
            "null = None",
            &[(Identifier, "null"), (Equals, "="), (Identifier, "None")],
        );
    }

    #[test]
    fn return_is_an_identifier() {
        assert_clean(
            "return = 1",
            &[(Identifier, "return"), (Equals, "="), (Number, "1")],
        );
    }

    #[test]
    fn exception_words_are_identifiers() {
        assert_clean(
            "try catch throw",
            &[
                (Identifier, "try"),
                (Identifier, "catch"),
                (Identifier, "throw"),
            ],
        );
    }

    #[test]
    fn undecided_words_are_not_reserved_for_now() {
        // Pins provisional behavior: further reserved words are LEX-KW-03.
        assert_clean(
            "then let val var yield async await",
            &[
                (Identifier, "then"),
                (Identifier, "let"),
                (Identifier, "val"),
                (Identifier, "var"),
                (Identifier, "yield"),
                (Identifier, "async"),
                (Identifier, "await"),
            ],
        );
    }
}

mod identifiers {
    use super::*;

    #[test]
    fn ascii_identifiers_of_every_case_style() {
        // Case-role validation is a parser/resolver concern, not the lexer's.
        assert_clean(
            "x maxSize MAX_SIZE Point area2 _private x_y a1_b2",
            &[
                (Identifier, "x"),
                (Identifier, "maxSize"),
                (Identifier, "MAX_SIZE"),
                (Identifier, "Point"),
                (Identifier, "area2"),
                (Identifier, "_private"),
                (Identifier, "x_y"),
                (Identifier, "a1_b2"),
            ],
        );
    }

    #[test]
    fn standalone_underscore_is_its_own_token() {
        assert_clean("_", &[(Underscore, "_")]);
    }

    #[test]
    fn underscore_prefixed_names_are_identifiers() {
        assert_clean(
            "_ __ _x x_",
            &[
                (Underscore, "_"),
                (Identifier, "__"),
                (Identifier, "_x"),
                (Identifier, "x_"),
            ],
        );
    }

    #[test]
    fn non_english_letters_are_identifiers() {
        assert_clean(
            "café niño größe straße π Ωmega привет 中文 ελληνικά",
            &[
                (Identifier, "café"),
                (Identifier, "niño"),
                (Identifier, "größe"),
                (Identifier, "straße"),
                (Identifier, "π"),
                (Identifier, "Ωmega"),
                (Identifier, "привет"),
                (Identifier, "中文"),
                (Identifier, "ελληνικά"),
            ],
        );
    }

    #[test]
    fn non_english_identifier_in_declaration() {
        assert_clean(
            "café = 1",
            &[(Identifier, "café"), (Equals, "="), (Number, "1")],
        );
    }

    #[test]
    fn non_english_identifier_positions() {
        let src = "größe = π";
        assert_eq!(token(src, 0).span, span(0, 7));
        assert_eq!(token(src, 0).start, pos(1, 1));
        assert_eq!(token(src, 1).span, span(8, 9));
        assert_eq!(token(src, 1).start, pos(1, 7));
        assert_eq!(token(src, 2).span, span(10, 12));
        assert_eq!(token(src, 2).start, pos(1, 9));
    }

    #[test]
    fn combining_marks_and_other_script_digits_continue_an_identifier() {
        // U+0301 COMBINING ACUTE ACCENT, U+0663 ARABIC-INDIC DIGIT THREE.
        assert_clean(
            "x\u{301}y x\u{663}",
            &[(Identifier, "x\u{301}y"), (Identifier, "x\u{663}")],
        );
    }

    #[test]
    fn combining_mark_cannot_start_an_identifier() {
        let src = "\u{301}x";
        assert_tokens(src, &[(Error, "\u{301}"), (Identifier, "x")]);
        assert_eq!(diag_kinds(src), vec![DiagnosticKind::UnexpectedCharacter]);
    }

    #[test]
    fn other_script_digit_cannot_start_a_token() {
        // Number literals remain ASCII-digit only.
        let src = "\u{663}";
        assert_tokens(src, &[(Error, "\u{663}")]);
        assert_eq!(diag_kinds(src), vec![DiagnosticKind::UnexpectedCharacter]);
    }

    #[test]
    fn non_letter_symbols_are_rejected() {
        for src in ["∑", "→", "©", "€", "×", "«"] {
            assert_tokens(src, &[(Error, src)]);
            assert_eq!(
                diag_kinds(src),
                vec![DiagnosticKind::UnexpectedCharacter],
                "diagnostics for {src:?}"
            );
        }
    }

    #[test]
    fn rejected_symbol_after_identifier_recovers() {
        assert_tokens("a∑b", &[(Identifier, "a"), (Error, "∑"), (Identifier, "b")]);
    }

    #[test]
    fn decomposed_identifier_is_nfc_normalized() {
        // "cafe" + U+0301 normalizes to "caf" + U+00E9; the span still covers the raw source.
        let src = "cafe\u{301} x";
        let ident = token(src, 0);
        assert_eq!(ident.kind, Identifier);
        assert_eq!(ident.text, "caf\u{e9}");
        assert_eq!(ident.span, span(0, 6));
        assert_eq!(ident.start, pos(1, 1));
        let x = token(src, 1);
        assert_eq!(x.span, span(7, 8));
        assert_eq!(x.start, pos(1, 7));
    }

    #[test]
    fn precomposed_and_decomposed_spellings_have_equal_text() {
        assert_eq!(token("caf\u{e9}", 0).text, token("cafe\u{301}", 0).text);
        assert_eq!(
            token("\u{212b}ngstr\u{f6}m", 0).text,
            token("\u{c5}ngstr\u{f6}m", 0).text
        );
    }

    #[test]
    fn already_normalized_identifier_text_is_unchanged() {
        assert_eq!(token("caf\u{e9}", 0).text, "caf\u{e9}");
    }

    #[test]
    fn string_contents_are_not_normalized() {
        assert_clean("\"cafe\u{301}\"", &[(String, "\"cafe\u{301}\"")]);
    }

    #[test]
    fn emoji_identifier_is_rejected_for_now() {
        let src = "🚀 = 42";
        assert_tokens(src, &[(Error, "🚀"), (Equals, "="), (Number, "42")]);
        assert_eq!(diag_kinds(src), vec![DiagnosticKind::UnexpectedCharacter]);
    }

    #[test]
    fn composed_emoji_is_a_single_error_token() {
        let src = "👩‍💻 = 1";
        assert_tokens(src, &[(Error, "👩‍💻"), (Equals, "="), (Number, "1")]);
        assert_eq!(diag_kinds(src), vec![DiagnosticKind::UnexpectedCharacter]);
    }
}

mod numbers {
    use super::*;

    #[test]
    fn decimal_integers() {
        assert_clean(
            "0 42 1000",
            &[(Number, "0"), (Number, "42"), (Number, "1000")],
        );
    }

    #[test]
    fn decimal_fractions() {
        assert_clean(
            "3.14159 0.5 10.25",
            &[(Number, "3.14159"), (Number, "0.5"), (Number, "10.25")],
        );
    }

    #[test]
    fn digit_separators_between_digits() {
        assert_clean(
            "1_000 1_000_000 3.141_59 1_0.0_1",
            &[
                (Number, "1_000"),
                (Number, "1_000_000"),
                (Number, "3.141_59"),
                (Number, "1_0.0_1"),
            ],
        );
    }

    #[test]
    fn misplaced_digit_separators_are_errors() {
        for src in ["1__0", "1_", "1_.5", "1._5", "1.5_", "1_000_"] {
            assert_tokens(src, &[(Error, src)]);
            assert_eq!(
                diag_kinds(src),
                vec![DiagnosticKind::InvalidDigitSeparator],
                "diagnostics for {src:?}"
            );
        }
    }

    #[test]
    fn malformed_number_recovers() {
        assert_tokens(
            "x = 1__0 + y",
            &[
                (Identifier, "x"),
                (Equals, "="),
                (Error, "1__0"),
                (Plus, "+"),
                (Identifier, "y"),
            ],
        );
    }

    #[test]
    fn number_directly_followed_by_letters_is_one_error() {
        // Includes exponent and hex spellings, which are not approved number forms.
        for src in [
            "42abc", "1_a", "3.14x", "1e10", "0x1F", "7_000ms", "42é", "1__0abc",
        ] {
            assert_tokens(src, &[(Error, src)]);
            assert_eq!(
                diag_kinds(src),
                vec![DiagnosticKind::InvalidNumberSuffix],
                "diagnostics for {src:?}"
            );
        }
    }

    #[test]
    fn number_suffix_error_recovers() {
        let src = "x = 42abc + y";
        assert_tokens(
            src,
            &[
                (Identifier, "x"),
                (Equals, "="),
                (Error, "42abc"),
                (Plus, "+"),
                (Identifier, "y"),
            ],
        );
        let diags = tokenize(src).diagnostics;
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].span, span(4, 9));
        assert_eq!(diags[0].start, pos(1, 5));
    }

    #[test]
    fn non_identifier_character_after_number_is_not_a_suffix() {
        assert_tokens("1🚀", &[(Number, "1"), (Error, "🚀")]);
        assert_eq!(diag_kinds("1🚀"), vec![DiagnosticKind::UnexpectedCharacter]);
        assert_clean("1+x", &[(Number, "1"), (Plus, "+"), (Identifier, "x")]);
    }

    #[test]
    fn trailing_dot_is_not_part_of_number() {
        assert_clean("1.", &[(Number, "1"), (Dot, ".")]);
    }

    #[test]
    fn leading_dot_is_not_part_of_number() {
        assert_clean(".5", &[(Dot, "."), (Number, "5")]);
    }

    #[test]
    fn dot_followed_by_identifier_after_number() {
        assert_clean("1.foo", &[(Number, "1"), (Dot, "."), (Identifier, "foo")]);
    }

    #[test]
    fn minus_is_not_part_of_a_number() {
        assert_clean("-1", &[(Minus, "-"), (Number, "1")]);
        assert_clean("-3.5", &[(Minus, "-"), (Number, "3.5")]);
        assert_clean("1-2", &[(Number, "1"), (Minus, "-"), (Number, "2")]);
    }
}

mod strings {
    use super::*;

    #[test]
    fn simple_string_includes_quotes_in_text() {
        assert_clean("\"Alice\"", &[(String, "\"Alice\"")]);
    }

    #[test]
    fn empty_string() {
        assert_clean("\"\"", &[(String, "\"\"")]);
    }

    #[test]
    fn relative_import_paths() {
        assert_clean(
            "\"./math\" \"../lib/util.nat\" \"./data.json\"",
            &[
                (String, "\"./math\""),
                (String, "\"../lib/util.nat\""),
                (String, "\"./data.json\""),
            ],
        );
    }

    #[test]
    fn non_ascii_content_is_allowed() {
        assert_clean("\"héllo 🚀 👩‍💻\"", &[(String, "\"héllo 🚀 👩‍💻\"")]);
    }

    #[test]
    fn comment_markers_inside_string_are_content() {
        assert_clean(
            "\"// not /* a */ comment\"",
            &[(String, "\"// not /* a */ comment\"")],
        );
    }

    #[test]
    fn adjacent_strings_are_separate_tokens() {
        assert_clean("\"a\"\"b\"", &[(String, "\"a\""), (String, "\"b\"")]);
    }

    #[test]
    fn unterminated_string_at_eof() {
        let src = "\"abc";
        assert_tokens(src, &[(Error, "\"abc")]);
        assert_eq!(diag_kinds(src), vec![DiagnosticKind::UnterminatedString]);
    }

    #[test]
    fn unterminated_string_stops_at_newline_and_recovers() {
        let src = "\"abc\nx";
        assert_tokens(src, &[(Error, "\"abc"), (Newline, "\n"), (Identifier, "x")]);
        assert_eq!(diag_kinds(src), vec![DiagnosticKind::UnterminatedString]);
    }

    #[test]
    fn backslash_in_string_is_rejected() {
        let src = r#""a\nb""#;
        assert_tokens(src, &[(Error, r#""a\nb""#)]);
        let diags = tokenize(src).diagnostics;
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].kind, DiagnosticKind::EscapeNotSupported);
        assert_eq!(diags[0].span, span(2, 3));
        assert_eq!(diags[0].start, pos(1, 3));
    }

    #[test]
    fn backslash_does_not_escape_closing_quote() {
        let src = r#""a\" x"#;
        assert_tokens(src, &[(Error, r#""a\""#), (Identifier, "x")]);
        assert_eq!(diag_kinds(src), vec![DiagnosticKind::EscapeNotSupported]);
    }
}

mod comments {
    use super::*;

    #[test]
    fn line_comment_ends_before_newline() {
        assert_clean(
            "x // comment\ny",
            &[(Identifier, "x"), (Newline, "\n"), (Identifier, "y")],
        );
    }

    #[test]
    fn line_comment_at_eof() {
        assert_clean("x // comment", &[(Identifier, "x")]);
    }

    #[test]
    fn line_comment_only_source() {
        assert_clean("// just a comment", &[]);
    }

    #[test]
    fn comments_may_contain_non_ascii() {
        assert_clean("// 👩‍💻 café\n/* 🚀 é */", &[(Newline, "\n")]);
    }

    #[test]
    fn block_comment_inline() {
        assert_clean("x /* comment */ y", &[(Identifier, "x"), (Identifier, "y")]);
    }

    #[test]
    fn block_comment_without_spaces() {
        assert_clean("x/**/y", &[(Identifier, "x"), (Identifier, "y")]);
    }

    #[test]
    fn multiline_block_comment_emits_no_newlines() {
        // Provisional: open question LEX-COM-04.
        assert_clean("x /* a\nb */ y", &[(Identifier, "x"), (Identifier, "y")]);
    }

    #[test]
    fn nested_block_comments() {
        assert_clean(
            "x /* a /* b */ c */ y",
            &[(Identifier, "x"), (Identifier, "y")],
        );
    }

    #[test]
    fn deeply_nested_block_comments() {
        assert_clean("/* 1 /* 2 /* 3 */ 2 */ 1 */ y", &[(Identifier, "y")]);
    }

    #[test]
    fn line_comment_marker_inside_block_comment_is_ignored() {
        assert_clean("/* // */ x", &[(Identifier, "x")]);
    }

    #[test]
    fn unterminated_block_comment() {
        let src = "x /* a";
        assert_tokens(src, &[(Identifier, "x"), (Error, "/* a")]);
        let diags = tokenize(src).diagnostics;
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].kind, DiagnosticKind::UnterminatedBlockComment);
        assert_eq!(diags[0].span, span(2, 6));
        assert_eq!(diags[0].start, pos(1, 3));
    }

    #[test]
    fn unterminated_nested_block_comment() {
        let src = "/* /* */";
        assert_tokens(src, &[(Error, "/* /* */")]);
        assert_eq!(
            diag_kinds(src),
            vec![DiagnosticKind::UnterminatedBlockComment]
        );
    }

    #[test]
    fn stray_block_comment_close_is_not_a_comment() {
        assert_clean("*/", &[(Star, "*"), (Slash, "/")]);
    }
}

mod operators {
    use super::*;

    #[test]
    fn every_approved_operator_and_punctuation() {
        assert_clean(
            "= => : | & { } ( ) , . * + / as?",
            &[
                (Equals, "="),
                (FatArrow, "=>"),
                (Colon, ":"),
                (Pipe, "|"),
                (Ampersand, "&"),
                (LBrace, "{"),
                (RBrace, "}"),
                (LParen, "("),
                (RParen, ")"),
                (Comma, ","),
                (Dot, "."),
                (Star, "*"),
                (Plus, "+"),
                (Slash, "/"),
                (AsQuestion, "as?"),
            ],
        );
    }

    #[test]
    fn slash_is_a_token() {
        assert_clean(
            "A / B",
            &[(Identifier, "A"), (Slash, "/"), (Identifier, "B")],
        );
        assert_clean("A/B", &[(Identifier, "A"), (Slash, "/"), (Identifier, "B")]);
    }

    #[test]
    fn double_slash_and_slash_star_still_start_comments() {
        assert_clean("A //B", &[(Identifier, "A")]);
        assert_clean(
            "A /*B*/ / C",
            &[(Identifier, "A"), (Slash, "/"), (Identifier, "C")],
        );
        assert_clean(
            "A / / B",
            &[
                (Identifier, "A"),
                (Slash, "/"),
                (Slash, "/"),
                (Identifier, "B"),
            ],
        );
    }

    #[test]
    fn slash_ends_an_unexpected_character_run() {
        assert_tokens("@/", &[(Error, "@"), (Slash, "/")]);
        assert_eq!(diag_kinds("@/"), vec![DiagnosticKind::UnexpectedCharacter]);
    }

    #[test]
    fn as_question_is_one_token() {
        assert_clean(
            "x as? T",
            &[(Identifier, "x"), (AsQuestion, "as?"), (Identifier, "T")],
        );
        assert_clean(
            "x as?T",
            &[(Identifier, "x"), (AsQuestion, "as?"), (Identifier, "T")],
        );
        let toks = tokenize("x as? T").tokens;
        assert_eq!(toks[1].span, span(2, 5));
        assert_eq!(toks[1].start, pos(1, 3));
    }

    #[test]
    fn as_alone_is_an_identifier() {
        assert_clean(
            "as = 1",
            &[(Identifier, "as"), (Equals, "="), (Number, "1")],
        );
    }

    #[test]
    fn separated_as_and_question_mark_is_not_the_operator() {
        assert_tokens(
            "x as ?",
            &[(Identifier, "x"), (Identifier, "as"), (Error, "?")],
        );
        assert_eq!(
            diag_kinds("x as ?"),
            vec![DiagnosticKind::UnexpectedCharacter]
        );
    }

    #[test]
    fn as_question_requires_the_whole_word_as() {
        for (src, word) in [
            ("has?", "has"),
            ("as_?", "as_"),
            ("As?", "As"),
            ("AS?", "AS"),
        ] {
            assert_tokens(src, &[(Identifier, word), (Error, "?")]);
        }
        assert_tokens("unique?", &[(Unique, "unique"), (Error, "?")]);
        assert_tokens("alias?", &[(Identifier, "alias"), (Error, "?")]);
    }

    #[test]
    fn question_mark_after_as_question_is_unexpected() {
        assert_tokens("as??", &[(AsQuestion, "as?"), (Error, "?")]);
        assert_eq!(
            diag_kinds("as??"),
            vec![DiagnosticKind::UnexpectedCharacter]
        );
    }

    #[test]
    fn value_expression_operators() {
        assert_clean(
            "- ! < <= > >= == != && ||",
            &[
                (Minus, "-"),
                (Not, "!"),
                (LessThan, "<"),
                (LessThanOrEqual, "<="),
                (GreaterThan, ">"),
                (GreaterThanOrEqual, ">="),
                (EqualTo, "=="),
                (NotEqualTo, "!="),
                (And, "&&"),
                (Or, "||"),
            ],
        );
    }

    #[test]
    fn value_expression_operators_without_whitespace() {
        assert_clean(
            "!a&&b||-c<=d",
            &[
                (Not, "!"),
                (Identifier, "a"),
                (And, "&&"),
                (Identifier, "b"),
                (Or, "||"),
                (Minus, "-"),
                (Identifier, "c"),
                (LessThanOrEqual, "<="),
                (Identifier, "d"),
            ],
        );
        assert_clean(
            "a<b>c==d!=e>=f",
            &[
                (Identifier, "a"),
                (LessThan, "<"),
                (Identifier, "b"),
                (GreaterThan, ">"),
                (Identifier, "c"),
                (EqualTo, "=="),
                (Identifier, "d"),
                (NotEqualTo, "!="),
                (Identifier, "e"),
                (GreaterThanOrEqual, ">="),
                (Identifier, "f"),
            ],
        );
    }

    #[test]
    fn multi_character_operators_are_single_tokens() {
        for (src, kind) in [
            ("=>", FatArrow),
            ("<=", LessThanOrEqual),
            (">=", GreaterThanOrEqual),
            ("==", EqualTo),
            ("!=", NotEqualTo),
            ("&&", And),
            ("||", Or),
        ] {
            assert_clean(src, &[(kind, src)]);
            assert_eq!(token(src, 0).span, span(0, 2), "span for {src:?}");
        }
    }

    #[test]
    fn maximal_munch_takes_the_longest_operator_first() {
        assert_clean("=>=", &[(FatArrow, "=>"), (Equals, "=")]);
        assert_clean("===", &[(EqualTo, "=="), (Equals, "=")]);
        assert_clean("!==", &[(NotEqualTo, "!="), (Equals, "=")]);
        assert_clean("<=>", &[(LessThanOrEqual, "<="), (GreaterThan, ">")]);
        assert_clean(">==", &[(GreaterThanOrEqual, ">="), (Equals, "=")]);
        assert_clean("&&&", &[(And, "&&"), (Ampersand, "&")]);
        assert_clean("|||", &[(Or, "||"), (Pipe, "|")]);
        assert_clean("!!", &[(Not, "!"), (Not, "!")]);
        assert_clean("--", &[(Minus, "-"), (Minus, "-")]);
        assert_clean("=<", &[(Equals, "="), (LessThan, "<")]);
        assert_clean("->", &[(Minus, "-"), (GreaterThan, ">")]);
    }

    #[test]
    fn separated_characters_do_not_form_an_operator() {
        assert_clean("= >", &[(Equals, "="), (GreaterThan, ">")]);
        assert_clean("= =", &[(Equals, "="), (Equals, "=")]);
        assert_clean("< =", &[(LessThan, "<"), (Equals, "=")]);
        assert_clean("> =", &[(GreaterThan, ">"), (Equals, "=")]);
        assert_clean("! =", &[(Not, "!"), (Equals, "=")]);
        assert_clean("& &", &[(Ampersand, "&"), (Ampersand, "&")]);
        assert_clean("| |", &[(Pipe, "|"), (Pipe, "|")]);
        assert_clean("=/**/=", &[(Equals, "="), (Equals, "=")]);
        assert_clean(
            "=\n>",
            &[(Equals, "="), (Newline, "\n"), (GreaterThan, ">")],
        );
    }

    #[test]
    fn value_operator_ends_an_unexpected_character_run() {
        for (src, op, kind) in [
            ("@-", "-", Minus),
            ("@!", "!", Not),
            ("@<", "<", LessThan),
            ("@>", ">", GreaterThan),
            ("@==", "==", EqualTo),
        ] {
            assert_tokens(src, &[(Error, "@"), (kind, op)]);
            assert_eq!(diag_kinds(src), vec![DiagnosticKind::UnexpectedCharacter]);
        }
    }

    #[test]
    fn operators_without_whitespace() {
        assert_clean(
            "x=>x*x+1",
            &[
                (Identifier, "x"),
                (FatArrow, "=>"),
                (Identifier, "x"),
                (Star, "*"),
                (Identifier, "x"),
                (Plus, "+"),
                (Number, "1"),
            ],
        );
    }

    #[test]
    fn unsupported_ascii_characters_are_rejected() {
        for src in [
            ";", "@", "#", "?", "[", "]", "%", "'", "`", "\\", "$", "~", "^",
        ] {
            assert_tokens(src, &[(Error, src)]);
            assert_eq!(
                diag_kinds(src),
                vec![DiagnosticKind::UnexpectedCharacter],
                "diagnostics for {src:?}"
            );
        }
    }

    #[test]
    fn adjacent_unexpected_characters_form_one_error() {
        assert_tokens("@#$", &[(Error, "@#$")]);
        assert_eq!(diag_kinds("@#$"), vec![DiagnosticKind::UnexpectedCharacter]);
    }

    #[test]
    fn lexing_recovers_after_unexpected_characters() {
        assert_tokens(
            "x ; y",
            &[(Identifier, "x"), (Error, ";"), (Identifier, "y")],
        );
        assert_tokens("a@b", &[(Identifier, "a"), (Error, "@"), (Identifier, "b")]);
    }
}

mod whitespace {
    use super::*;

    #[test]
    fn spaces_are_skipped() {
        assert_clean("  a   b  ", &[(Identifier, "a"), (Identifier, "b")]);
    }

    #[test]
    fn lf_newline() {
        assert_clean(
            "a\nb",
            &[(Identifier, "a"), (Newline, "\n"), (Identifier, "b")],
        );
    }

    #[test]
    fn crlf_is_a_single_newline() {
        assert_clean(
            "a\r\nb",
            &[(Identifier, "a"), (Newline, "\r\n"), (Identifier, "b")],
        );
    }

    #[test]
    fn each_line_break_is_a_newline_token() {
        assert_clean(
            "a\n\n\nb",
            &[
                (Identifier, "a"),
                (Newline, "\n"),
                (Newline, "\n"),
                (Newline, "\n"),
                (Identifier, "b"),
            ],
        );
    }

    #[test]
    fn trailing_newline() {
        assert_clean("a\n", &[(Identifier, "a"), (Newline, "\n")]);
    }

    #[test]
    fn leading_indentation_is_not_tokenized() {
        assert_clean(
            "a\n    b",
            &[(Identifier, "a"), (Newline, "\n"), (Identifier, "b")],
        );
        assert_eq!(token("a\n    b", 2).start, pos(2, 5));
    }
}

mod positions {
    use super::*;

    #[test]
    fn spans_and_positions_on_one_line() {
        let src = "area = PI";
        assert_eq!(token(src, 0).span, span(0, 4));
        assert_eq!(token(src, 0).start, pos(1, 1));
        assert_eq!(token(src, 1).span, span(5, 6));
        assert_eq!(token(src, 1).start, pos(1, 6));
        assert_eq!(token(src, 2).span, span(7, 9));
        assert_eq!(token(src, 2).start, pos(1, 8));
    }

    #[test]
    fn positions_across_lines() {
        let src = "x = 1\ny = 2";
        let newline = token(src, 3);
        assert_eq!(newline.kind, Newline);
        assert_eq!(newline.span, span(5, 6));
        assert_eq!(newline.start, pos(1, 6));
        let y = token(src, 4);
        assert_eq!(y.text, "y");
        assert_eq!(y.span, span(6, 7));
        assert_eq!(y.start, pos(2, 1));
    }

    #[test]
    fn crlf_positions() {
        let src = "a\r\nb";
        assert_eq!(token(src, 1).span, span(1, 3));
        assert_eq!(token(src, 2).span, span(3, 4));
        assert_eq!(token(src, 2).start, pos(2, 1));
    }

    #[test]
    fn columns_count_unicode_scalars_not_bytes() {
        let src = "\"🚀\" x";
        assert_eq!(token(src, 0).span, span(0, 6));
        assert_eq!(token(src, 0).start, pos(1, 1));
        assert_eq!(token(src, 1).span, span(7, 8));
        assert_eq!(token(src, 1).start, pos(1, 5));
    }

    #[test]
    fn positions_after_multiline_block_comment() {
        let src = "x /* a\nb */ y";
        assert_eq!(token(src, 1).text, "y");
        assert_eq!(token(src, 1).span, span(12, 13));
        assert_eq!(token(src, 1).start, pos(2, 6));
    }

    #[test]
    fn error_token_and_diagnostic_positions_agree() {
        let src = "a 🚀 b";
        let error = token(src, 1);
        assert_eq!(error.kind, Error);
        assert_eq!(error.span, span(2, 6));
        assert_eq!(error.start, pos(1, 3));
        let b = token(src, 2);
        assert_eq!(b.span, span(7, 8));
        assert_eq!(b.start, pos(1, 5));
        let diag = &tokenize(src).diagnostics[0];
        assert_eq!(diag.span, error.span);
        assert_eq!(diag.start, error.start);
        assert!(!diag.message.is_empty());
    }

    #[test]
    fn diagnostics_are_reported_in_source_order() {
        assert_eq!(
            diag_kinds("@ 1__0 \"abc"),
            vec![
                DiagnosticKind::UnexpectedCharacter,
                DiagnosticKind::InvalidDigitSeparator,
                DiagnosticKind::UnterminatedString,
            ]
        );
    }

    #[test]
    fn unterminated_string_with_backslash_reports_both_in_source_order() {
        for src in ["\"a\\", "\"a\\\nx"] {
            let diags = tokenize(src).diagnostics;
            let kinds: Vec<_> = diags.iter().map(|d| d.kind).collect();
            assert_eq!(
                kinds,
                vec![
                    DiagnosticKind::UnterminatedString,
                    DiagnosticKind::EscapeNotSupported,
                ],
                "diagnostics for {src:?}"
            );
            assert_eq!(diags[0].span.start, 0);
            assert_eq!(diags[1].span, span(2, 3));
        }
    }

    #[test]
    fn diagnostic_starts_never_decrease() {
        for src in [
            "\"a\\",
            "@ \"x\\y\" /* open",
            "1__0 \"b\\ \n🚀 \"c\\\"",
            "∑ 1_ \"q\\",
        ] {
            let starts: Vec<_> = tokenize(src)
                .diagnostics
                .iter()
                .map(|d| d.span.start)
                .collect();
            assert!(
                starts.windows(2).all(|w| w[0] <= w[1]),
                "out of order for {src:?}: {starts:?}"
            );
        }
    }

    #[test]
    fn token_text_matches_its_span_for_nfc_source() {
        let src = "from \"./m\" import { größe = b }, M\r\n/* c */ x: T = 1_0.5 | y & z\n🚀 @ \"q\\\" // end";
        for t in tokenize(src).tokens {
            assert_eq!(t.text, &src[t.span.start..t.span.end], "token {t:?}");
        }
    }

    #[test]
    fn eof_on_empty_source() {
        let result = tokenize("");
        assert_eq!(result.tokens.len(), 1);
        assert_eq!(result.tokens[0].kind, Eof);
        assert_eq!(result.tokens[0].text, "");
        assert_eq!(result.tokens[0].span, span(0, 0));
        assert_eq!(result.tokens[0].start, pos(1, 1));
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn eof_position_at_end_of_input() {
        let ab = tokenize("ab");
        let eof = ab.tokens.last().unwrap();
        assert_eq!(eof.span, span(2, 2));
        assert_eq!(eof.start, pos(1, 3));

        let trailing = tokenize("a\n");
        let eof = trailing.tokens.last().unwrap();
        assert_eq!(eof.span, span(2, 2));
        assert_eq!(eof.start, pos(2, 1));
    }

    #[test]
    fn always_exactly_one_trailing_eof() {
        for src in [
            "", " ", "\n", "x", "\"", "\"abc", "/*", "/* /*", "@", "1_", "🚀", "=>", "//", "a\r\n",
            "\\",
        ] {
            assert_single_trailing_eof(src, &tokenize(src));
        }
    }
}

mod spec_examples {
    use super::*;

    #[test]
    fn module_with_import_declaration_and_export() {
        let src = "from \"./math\" import { PI }\n\narea = PI * 5 * 5\n\nexport { area }\n";
        assert_clean(
            src,
            &[
                (From, "from"),
                (String, "\"./math\""),
                (Import, "import"),
                (LBrace, "{"),
                (Identifier, "PI"),
                (RBrace, "}"),
                (Newline, "\n"),
                (Newline, "\n"),
                (Identifier, "area"),
                (Equals, "="),
                (Identifier, "PI"),
                (Star, "*"),
                (Number, "5"),
                (Star, "*"),
                (Number, "5"),
                (Newline, "\n"),
                (Newline, "\n"),
                (Export, "export"),
                (LBrace, "{"),
                (Identifier, "area"),
                (RBrace, "}"),
                (Newline, "\n"),
            ],
        );
    }

    #[test]
    fn import_forms() {
        assert_eq!(
            kinds("from \"./math\" import Math"),
            vec![From, String, Import, Identifier]
        );
        assert_eq!(
            kinds("from \"./math\" import { PI, square }"),
            vec![
                From, String, Import, LBrace, Identifier, Comma, Identifier, RBrace
            ]
        );
        assert_eq!(
            kinds("from \"./math\" import *"),
            vec![From, String, Import, Star]
        );
        assert_eq!(
            kinds("from \"./math\" import { PI }, Math"),
            vec![
                From, String, Import, LBrace, Identifier, RBrace, Comma, Identifier
            ]
        );
        assert_eq!(
            kinds("from \"./math\" import { localName = publicName }"),
            vec![
                From, String, Import, LBrace, Identifier, Equals, Identifier, RBrace
            ]
        );
        assert_eq!(
            kinds("from \"./math\" import { MATH_PI = PI }"),
            vec![
                From, String, Import, LBrace, Identifier, Equals, Identifier, RBrace
            ]
        );
    }

    #[test]
    fn export_forms() {
        assert_eq!(kinds("export {}"), vec![Export, LBrace, RBrace]);
        assert_eq!(kinds("export run"), vec![Export, Identifier]);
        assert_eq!(
            kinds("export { squareNumber = square }"),
            vec![Export, LBrace, Identifier, Equals, Identifier, RBrace]
        );
        assert_eq!(
            kinds("export { cheap = 42, expensive = expensiveCalculation() }"),
            vec![
                Export, LBrace, Identifier, Equals, Number, Comma, Identifier, Equals, Identifier,
                LParen, RParen, RBrace,
            ]
        );
    }

    #[test]
    fn declarations_with_constraints() {
        assert_clean(
            "x: number = 42",
            &[
                (Identifier, "x"),
                (Colon, ":"),
                (Identifier, "number"),
                (Equals, "="),
                (Number, "42"),
            ],
        );
        assert_eq!(kinds("MAX_SIZE = 100"), vec![Identifier, Equals, Number]);
        assert_eq!(kinds("PI = 3.14159"), vec![Identifier, Equals, Number]);
    }

    #[test]
    fn redeclarations_with_comments() {
        let src = "x = 10\nx = x + 5           // initializer sees previous x; new x = 15\nx: string = \"hi\"    // new binding; previous x remains immutable\n";
        assert_eq!(
            kinds(src),
            vec![
                Identifier, Equals, Number, Newline, Identifier, Equals, Identifier, Plus, Number,
                Newline, Identifier, Colon, Identifier, Equals, String, Newline,
            ]
        );
        assert!(tokenize(src).diagnostics.is_empty());
    }

    #[test]
    fn lambdas_and_calls() {
        assert_eq!(
            kinds("square = x => x * x"),
            vec![
                Identifier, Equals, Identifier, FatArrow, Identifier, Star, Identifier
            ]
        );
        assert_eq!(
            kinds("run = () => print(\"Hello\")"),
            vec![
                Identifier, Equals, LParen, RParen, FatArrow, Identifier, LParen, String, RParen
            ]
        );
        assert_eq!(
            kinds("getX = () => x"),
            vec![Identifier, Equals, LParen, RParen, FatArrow, Identifier]
        );
    }

    #[test]
    fn union_and_intersection_types() {
        assert_eq!(
            kinds("{ kind: \"left\", value: number }\n|\n{ kind: \"right\", value: string }"),
            vec![
                LBrace, Identifier, Colon, String, Comma, Identifier, Colon, Identifier, RBrace,
                Newline, Pipe, Newline, LBrace, Identifier, Colon, String, Comma, Identifier,
                Colon, Identifier, RBrace,
            ]
        );
        assert_eq!(kinds("A & B"), vec![Identifier, Ampersand, Identifier]);
    }

    #[test]
    fn object_literal_and_member_access() {
        assert_eq!(
            kinds("api = { PI = 3.14159, square = x => x * x }"),
            vec![
                Identifier, Equals, LBrace, Identifier, Equals, Number, Comma, Identifier, Equals,
                Identifier, FatArrow, Identifier, Star, Identifier, RBrace,
            ]
        );
        assert_eq!(
            kinds("Math.PI = 3.14"),
            vec![Identifier, Dot, Identifier, Equals, Number]
        );
    }
}
