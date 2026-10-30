//! Parser tests. Each test is cited as evidence by an item in
//! `docs/scorecards/parser.md`.

use natcode::lexer::{Position, Span};
use natcode::parser::{DiagnosticKind, Expr, ExprKind, parse_expression};

/// The 1-based line and column of byte `offset`, counted independently of
/// the lexer; columns count Unicode scalar values.
fn position_of(src: &str, offset: usize) -> Position {
    let before = &src[..offset];
    let line_start = before.rfind('\n').map_or(0, |i| i + 1);
    Position {
        line: before.matches('\n').count() as u32 + 1,
        column: before[line_start..].chars().count() as u32 + 1,
    }
}

/// Multi-line source written indented in a test: drops the line after the
/// opening quote and the blank line before the closing quote, removes the
/// indentation shared by every non-blank line, and joins lines with `\n`.
fn dedent(text: &str) -> String {
    let lines: Vec<&str> = text.lines().skip(1).collect();
    let lines = match lines.last() {
        Some(last) if last.trim().is_empty() => &lines[..lines.len() - 1],
        _ => &lines[..],
    };
    let indent = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);
    lines
        .iter()
        .map(|line| line.get(indent..).unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n")
}

fn parse(src: &str) -> Expr {
    let result = parse_expression(src);
    assert!(
        result.diagnostics.is_empty(),
        "unexpected diagnostics for {src:?}: {:?}",
        result.diagnostics
    );
    result.value.expect("an expression")
}

fn assert_parses(src: &str, expected: &str) {
    // `Expr` displays as a fully parenthesized S-expression showing its grouping.
    assert_eq!(parse(src).to_string(), expected, "tree for {src:?}");
}

/// Asserts that `src` is rejected, with exactly one diagnostic of `kind`
/// whose line and column match the start of its span.
fn assert_rejected(src: &str, kind: DiagnosticKind) {
    let result = parse_expression(src);
    assert!(result.value.is_none(), "{src:?} should be rejected");
    let kinds: Vec<DiagnosticKind> = result.diagnostics.iter().map(|d| d.kind).collect();
    assert_eq!(kinds, vec![kind], "diagnostics for {src:?}");
    let diagnostic = &result.diagnostics[0];
    assert_eq!(
        diagnostic.start,
        position_of(src, diagnostic.span.start),
        "diagnostic position for {src:?}"
    );
}

mod diagnostics {
    use super::*;

    /// Asserts the span and the line and column of the only diagnostic.
    fn assert_located(
        src: &str,
        kind: DiagnosticKind,
        span: (usize, usize),
        line: u32,
        column: u32,
    ) {
        assert_rejected(src, kind);
        let diagnostic = &parse_expression(src).diagnostics[0];
        assert_eq!(
            diagnostic.span,
            Span {
                start: span.0,
                end: span.1
            },
            "span for {src:?}"
        );
        assert_eq!(
            diagnostic.start,
            Position { line, column },
            "position for {src:?}"
        );
    }

    #[test]
    fn positions_match_span_starts() {
        assert_located(
            "f(x) => y",
            DiagnosticKind::InvalidLambdaParameters,
            (0, 7),
            1,
            1,
        );
        assert_located(
            "a || b => c",
            DiagnosticKind::InvalidLambdaParameters,
            (0, 9),
            1,
            1,
        );
        assert_located("(x => x)(1)", DiagnosticKind::Unsupported, (0, 9), 1, 1);
        assert_located(
            "(x: Number) => x",
            DiagnosticKind::InvalidLambdaParameters,
            (0, 11),
            1,
            1,
        );
        assert_located(
            "g(1, (a b) => a)",
            DiagnosticKind::InvalidLambdaParameters,
            (5, 10),
            1,
            6,
        );
        assert_located(
            "/* c\n*/ f(x) => y",
            DiagnosticKind::InvalidLambdaParameters,
            (8, 15),
            2,
            4,
        );
    }
}

mod primaries {
    use super::*;

    #[test]
    fn literals_identifiers_and_grouping() {
        assert_parses("answer", "answer");
        assert_parses("1_000.5", "1_000.5");
        assert_parses("\"Ada\"", "\"Ada\"");
        assert_parses("true", "true");
        assert_parses("false", "false");
        assert_parses("(x)", "(paren x)");
        assert_parses("((x))", "(paren (paren x))");
    }

    #[test]
    fn spans_cover_the_whole_expression() {
        let expr = parse("(a + b) * c");
        assert_eq!(expr.span, Span { start: 0, end: 11 });
        let ExprKind::Binary { left, right, .. } = &expr.kind else {
            panic!("expected a binary expression");
        };
        assert_eq!(left.span, Span { start: 0, end: 7 });
        assert_eq!(right.span, Span { start: 10, end: 11 });
    }

    #[test]
    fn missing_or_trailing_tokens_are_rejected() {
        assert_rejected("", DiagnosticKind::UnexpectedToken);
        assert_rejected("a +", DiagnosticKind::UnexpectedToken);
        assert_rejected("(a", DiagnosticKind::UnexpectedToken);
        assert_rejected("a)", DiagnosticKind::UnexpectedToken);
        assert_rejected("a b", DiagnosticKind::UnexpectedToken);
    }

    #[test]
    fn type_operators_are_not_value_operators() {
        assert_rejected("a | b", DiagnosticKind::UnexpectedToken);
        assert_rejected("a & b", DiagnosticKind::UnexpectedToken);
        assert_rejected("a \\ b", DiagnosticKind::UnexpectedToken);
        assert_rejected("~a", DiagnosticKind::UnexpectedToken);
    }

    #[test]
    fn lexical_errors_are_reported_once() {
        let result = parse_expression("a + 1__0");
        assert!(result.value.is_none());
        let kinds: Vec<DiagnosticKind> = result.diagnostics.iter().map(|d| d.kind).collect();
        assert_eq!(
            kinds,
            vec![DiagnosticKind::Lexical(
                natcode::lexer::DiagnosticKind::InvalidDigitSeparator
            )]
        );
    }
}

mod postfix {
    use super::*;

    #[test]
    fn calls_use_parentheses() {
        assert_parses("f()", "(call f)");
        assert_parses("f(x)", "(call f x)");
        assert_parses("f(x, y + 1)", "(call f x (+ y 1))");
        assert_parses("f()()", "(call (call f))");
    }

    #[test]
    fn whitespace_application_is_rejected() {
        assert_rejected("f x", DiagnosticKind::UnexpectedToken);
        assert_rejected("f 1", DiagnosticKind::UnexpectedToken);
    }

    #[test]
    fn member_access_and_calls_chain_left_to_right() {
        assert_parses("foo.bar", "(. foo bar)");
        assert_parses("Option.Some(value)", "(call (. Option Some) value)");
        assert_parses("foo.bar(x).baz()", "(call (. (call (. foo bar) x) baz))");
        assert_parses("(a + b).c", "(. (paren (+ a b)) c)");
    }

    #[test]
    fn malformed_postfix_is_rejected() {
        assert_rejected("foo.", DiagnosticKind::UnexpectedToken);
        assert_rejected("foo.1", DiagnosticKind::UnexpectedToken);
        assert_rejected("f(x,)", DiagnosticKind::UnexpectedToken);
        assert_rejected("f(,)", DiagnosticKind::UnexpectedToken);
        assert_rejected("f(x", DiagnosticKind::UnexpectedToken);
    }

    #[test]
    fn labeled_arguments_are_not_parsed_yet() {
        assert_rejected("greet(last: \"Lovelace\")", DiagnosticKind::Unsupported);
    }

    #[test]
    fn keyword_spellings_are_member_names() {
        assert_parses("record.type", "(. record type)");
        assert_parses("record.when.where", "(. (. record when) where)");
        assert_parses("m.from(x).as", "(. (call (. m from) x) as)");
        for keyword in ["import", "export", "unique", "fn"] {
            assert_parses(&format!("a.{keyword}"), &format!("(. a {keyword})"));
        }
        assert_rejected("type", DiagnosticKind::UnexpectedToken);
        assert_rejected("a._", DiagnosticKind::UnexpectedToken);
    }
}

mod line_breaks {
    use super::*;

    /// Asserts that `src` is rejected exactly as `equivalent`, its form with
    /// the block comment replaced by an explicit line break.
    fn assert_rejected_like(src: &str, equivalent: &str) {
        assert_rejected(src, DiagnosticKind::UnexpectedToken);
        assert_rejected(equivalent, DiagnosticKind::UnexpectedToken);
        let messages = |s: &str| -> Vec<String> {
            parse_expression(s)
                .diagnostics
                .into_iter()
                .map(|d| d.message)
                .collect()
        };
        assert_eq!(messages(src), messages(equivalent), "messages for {src:?}");
    }

    #[test]
    fn explicit_line_breaks_end_the_expression() {
        assert_rejected("a\n+ b", DiagnosticKind::UnexpectedToken);
        assert_rejected("f\n(x)", DiagnosticKind::UnexpectedToken);
    }

    #[test]
    fn block_comment_line_breaks_are_not_joined() {
        assert_rejected_like("a /*\n*/ + b", "a\n+ b");
        assert_rejected_like("f /*\n*/ (x)", "f\n(x)");
        assert_rejected_like("a /* one\r\ntwo */ * b", "a\r\n* b");
        assert_rejected_like("a + /*\n*/ b", "a +\nb");
    }

    #[test]
    fn comment_line_break_position() {
        let result = parse_expression("ab /*\n*/ + c");
        let diagnostic = &result.diagnostics[0];
        assert_eq!(diagnostic.span, Span { start: 5, end: 6 });
        assert_eq!(diagnostic.start.line, 1);
        assert_eq!(diagnostic.start.column, 6);
    }

    #[test]
    fn single_line_block_comments_are_ignored() {
        assert_parses("a /* note */ + b", "(+ a b)");
    }

    #[test]
    fn unterminated_comment_reports_only_the_lexical_error() {
        let result = parse_expression("a + /*\n");
        assert!(result.value.is_none());
        let kinds: Vec<DiagnosticKind> = result.diagnostics.iter().map(|d| d.kind).collect();
        assert_eq!(
            kinds,
            vec![DiagnosticKind::Lexical(
                natcode::lexer::DiagnosticKind::UnterminatedBlockComment
            )]
        );
    }
}

mod prefix {
    use super::*;

    #[test]
    fn prefix_operators_bind_looser_than_postfix() {
        assert_parses("-x", "(- x)");
        assert_parses("!ready", "(! ready)");
        assert_parses("-f(x)", "(- (call f x))");
        assert_parses("!user.active", "(! (. user active))");
        assert_parses("!!x", "(! (! x))");
        assert_parses("- -x", "(- (- x))");
    }

    #[test]
    fn prefix_operators_bind_tighter_than_binary() {
        assert_parses("-a * b", "(* (- a) b)");
        assert_parses("a - -b", "(- a (- b))");
        assert_parses("!ready && enabled", "(&& (! ready) enabled)");
    }
}

mod binary {
    use super::*;

    #[test]
    fn multiplicative_binds_tighter_than_additive() {
        assert_parses("a + b * c", "(+ a (* b c))");
        assert_parses("a - b / c", "(- a (/ b c))");
        assert_parses("a % b + c", "(+ (% a b) c)");
        assert_parses("(a + b) * c", "(* (paren (+ a b)) c)");
    }

    #[test]
    fn arithmetic_is_left_associative() {
        assert_parses("a - b - c", "(- (- a b) c)");
        assert_parses("a + b - c", "(- (+ a b) c)");
        assert_parses("a * b / c", "(/ (* a b) c)");
        assert_parses("a / b % c", "(% (/ a b) c)");
    }

    #[test]
    fn comparisons_bind_looser_than_arithmetic() {
        assert_parses("a + 1 < b * 2", "(< (+ a 1) (* b 2))");
        assert_parses("a <= b", "(<= a b)");
        assert_parses("a > b", "(> a b)");
        assert_parses("a >= b", "(>= a b)");
        assert_parses("a == b", "(== a b)");
        assert_parses("a != b", "(!= a b)");
    }

    #[test]
    fn comparisons_are_non_associative() {
        assert_rejected("a < b < c", DiagnosticKind::ChainedComparison);
        assert_rejected("a == b == c", DiagnosticKind::ChainedComparison);
        assert_rejected("a < b == c", DiagnosticKind::ChainedComparison);
        assert_parses("a < b && b < c", "(&& (< a b) (< b c))");
        assert_parses("(a < b) == c", "(== (paren (< a b)) c)");
    }

    #[test]
    fn and_binds_tighter_than_or() {
        assert_parses("a || b && c", "(|| a (&& b c))");
        assert_parses("a && b || c", "(|| (&& a b) c)");
        assert_parses("a == b && c != d", "(&& (== a b) (!= c d))");
    }

    #[test]
    fn logical_operators_are_left_associative() {
        assert_parses("a && b && c", "(&& (&& a b) c)");
        assert_parses("a || b || c", "(|| (|| a b) c)");
    }
}

mod lambdas {
    use super::*;

    #[test]
    fn approved_parameter_forms() {
        assert_parses("x => x * 2", "(=> (x) (* x 2))");
        assert_parses("(x) => x * 2", "(=> (x) (* x 2))");
        assert_parses("(x, y) => x + y", "(=> (x y) (+ x y))");
        assert_parses("() => 42", "(=> () 42)");
    }

    #[test]
    fn arrow_is_right_associative_and_lowest() {
        assert_parses("x => y => x + y", "(=> (x) (=> (y) (+ x y)))");
        assert_parses("x => a || b && c", "(=> (x) (|| a (&& b c)))");
    }

    #[test]
    fn lambdas_as_operands_and_arguments() {
        assert_parses(
            "map(items, x => x + 1)",
            "(call map items (=> (x) (+ x 1)))",
        );
        assert_parses("a || (x => y)", "(|| a (paren (=> (x) y)))");
        assert_parses("make()(1)", "(call (call make) 1)");
    }

    #[test]
    fn immediately_invoked_lambdas_are_not_supported() {
        assert_rejected("(x => x)(1)", DiagnosticKind::Unsupported);
        assert_rejected("((x => x))(1)", DiagnosticKind::Unsupported);
        assert_rejected("(() => 42)()", DiagnosticKind::Unsupported);
    }

    #[test]
    fn nested_groups_and_lambda_heads() {
        assert_parses("((((x))))", "(paren (paren (paren (paren x))))");
        assert_parses("((x) => (y))", "(paren (=> (x) (paren y)))");
        assert_parses("f((a), (b) => b)", "(call f (paren a) (=> (b) b))");
    }
}

mod nesting {
    use super::*;

    const DEPTH: usize = 100_000;

    /// Parses, prints, and drops `src`, none of which may overflow the call
    /// stack.
    fn assert_deep_parse(src: &str) {
        let result = parse_expression(src);
        assert!(
            result.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            result.diagnostics.first()
        );
        let expr = result.value.expect("an expression");
        assert_eq!(expr.span.end, src.len());
        assert!(!format!("{expr:?}").is_empty());
    }

    #[test]
    fn deep_grouping() {
        assert_deep_parse(&format!("{}x{}", "(".repeat(DEPTH), ")".repeat(DEPTH)));
    }

    #[test]
    fn deep_prefix_chains() {
        assert_deep_parse(&format!("{}x", "!".repeat(DEPTH)));
        assert_deep_parse(&format!("{}x", "- ".repeat(DEPTH)));
    }

    #[test]
    fn deep_calls_and_lambdas() {
        assert_deep_parse(&format!("{}x{}", "f(".repeat(DEPTH), ")".repeat(DEPTH)));
        assert_deep_parse(&format!("{}x", "x => ".repeat(DEPTH)));
    }

    #[test]
    fn long_operator_and_postfix_chains() {
        assert_deep_parse(&format!("x{}", " - x".repeat(DEPTH)));
        assert_deep_parse(&format!("x{}", ".y".repeat(DEPTH)));
        assert_deep_parse(&format!("f{}", "()".repeat(DEPTH)));
    }

    #[test]
    fn deep_unclosed_grouping_is_rejected() {
        let result = parse_expression(&format!("{}x", "(".repeat(DEPTH)));
        assert!(result.value.is_none());
        assert_eq!(result.diagnostics.len(), 1);
    }

    #[test]
    fn non_parameter_left_operands_are_rejected() {
        assert_rejected("a || x => y", DiagnosticKind::InvalidLambdaParameters);
        assert_rejected("f(x) => y", DiagnosticKind::InvalidLambdaParameters);
        assert_rejected("1 => y", DiagnosticKind::InvalidLambdaParameters);
        assert_rejected("(a + b) => c", DiagnosticKind::InvalidLambdaParameters);
        assert_rejected("(x: Number) => x", DiagnosticKind::InvalidLambdaParameters);
        assert_rejected("(x,) => x", DiagnosticKind::InvalidLambdaParameters);
    }

    #[test]
    fn missing_body_is_rejected() {
        assert_rejected("x =>", DiagnosticKind::UnexpectedToken);
    }

    #[test]
    fn block_bodies_are_not_parsed_yet() {
        assert_rejected("x => { x }", DiagnosticKind::Unsupported);
    }
}

mod nominal_types {
    use super::*;
    use natcode::parser::{Declaration, DeclarationKind, TypeExprKind, parse_declaration};

    fn parse(src: &str) -> Declaration {
        let result = parse_declaration(src);
        assert!(
            result.diagnostics.is_empty(),
            "unexpected diagnostics for {src:?}: {:?}",
            result.diagnostics
        );
        result.value.expect("a declaration")
    }

    fn assert_parses(src: &str, expected: &str) {
        assert_eq!(parse(src).to_string(), expected, "tree for {src:?}");
    }

    fn assert_rejected(src: &str, kind: DiagnosticKind) {
        let result = parse_declaration(src);
        assert!(result.value.is_none(), "{src:?} should be rejected");
        let kinds: Vec<DiagnosticKind> = result.diagnostics.iter().map(|d| d.kind).collect();
        assert_eq!(kinds, vec![kind], "diagnostics for {src:?}");
        let diagnostic = &result.diagnostics[0];
        assert_eq!(
            diagnostic.start,
            position_of(src, diagnostic.span.start),
            "diagnostic position for {src:?}"
        );
    }

    #[test]
    fn positional_constructor_payloads() {
        assert_parses(
            "unique type UserId(Number)",
            "(unique-type UserId (Number))",
        );
        assert_parses(
            "unique type Point(Number, Number)",
            "(unique-type Point (Number Number))",
        );
        assert_parses(
            "unique type Sprint(Position, Color)",
            "(unique-type Sprint (Position Color))",
        );
        assert_parses(
            "unique type Point(number,number)",
            "(unique-type Point (number number))",
        );
        assert_parses("unique type Unit()", "(unique-type Unit ())");
        assert_parses(
            "unique /* c */ type Id( /* c */ Number )",
            "(unique-type Id (Number))",
        );
    }

    #[test]
    fn declaration_and_payload_spans() {
        let decl = parse("unique type Point(Number, Number)");
        assert_eq!(decl.span, Span { start: 0, end: 33 });
        let DeclarationKind::NominalType { name, payload } = &decl.kind;
        assert_eq!(name.text, "Point");
        assert_eq!(name.span, Span { start: 12, end: 17 });
        let spans: Vec<Span> = payload.iter().map(|t| t.span).collect();
        assert_eq!(
            spans,
            vec![Span { start: 18, end: 24 }, Span { start: 26, end: 32 }]
        );
        assert!(matches!(&payload[0].kind, TypeExprKind::Name(n) if n == "Number"));
    }

    #[test]
    fn malformed_declarations_are_rejected() {
        for src in [
            "",
            "unique",
            "unique Point(Number)",
            "unique type",
            "unique type (Number)",
            "unique type type(Number)",
            "unique type _(Number)",
            "unique type Point Number",
            "unique type Point(",
            "unique type Point(Number",
            "unique type Point(Number Number)",
            "unique type Point(Number,)",
            "unique type Point(,)",
            "unique type Point(,Number)",
            "unique type Point(Number) Number",
            "unique type Point(Number)(Number)",
            "unique type Point(Number)\n",
            "Point(Number)",
        ] {
            assert_rejected(src, DiagnosticKind::UnexpectedToken);
        }
    }

    #[test]
    fn reserved_none_literal_is_not_a_type_name() {
        assert_rejected("unique type none()", DiagnosticKind::UnexpectedToken);
        assert_rejected("unique type none(Number)", DiagnosticKind::UnexpectedToken);
        assert_rejected("type none = Number", DiagnosticKind::UnexpectedToken);
        assert_rejected("unique type Wrap(none)", DiagnosticKind::Unsupported);
        assert_rejected(
            "unique type Pair(Number, none)",
            DiagnosticKind::Unsupported,
        );
        assert_parses("unique type None()", "(unique-type None ())");
        assert_parses("unique type Wrap(None)", "(unique-type Wrap (None))");
    }

    #[test]
    fn bare_type_constructor_and_unique_alias_are_invalid() {
        assert_rejected(
            "type UserId(Number)",
            DiagnosticKind::InvalidTypeDeclaration,
        );
        assert_rejected(
            "unique type UserId = Number",
            DiagnosticKind::InvalidTypeDeclaration,
        );
    }

    #[test]
    fn forms_owned_by_other_items_are_not_parsed_yet() {
        for src in [
            "unique type Option",
            "type Identifier = UserId | ProductId",
            "unique type Range(Number, Number) where (start, end) => start < end",
            "unique type Port(Number where p => p > 0)",
            "unique type Pair(Number | String)",
            "unique type Pair(Number & String)",
            "unique type Pair(Number \\ String)",
            "unique type Pair(~Number)",
            "unique type Wrap(Value(\"one\"))",
            "unique type Wrap(Option.Some)",
            "unique type One(1)",
            "unique type Named(\"x\")",
            "unique type Flag(true)",
            "unique type Rec({ x: Number })",
            "unique type Grouped((Number))",
        ] {
            assert_rejected(src, DiagnosticKind::Unsupported);
        }
    }

    #[test]
    fn multiline_forms_are_not_parsed_yet() {
        let constructor_list = dedent(
            "
            unique type Option
                Some(Any)
                None
            ",
        );
        let payload_per_line = dedent(
            "
            unique type Point(
                Number,
                Number
            )
            ",
        );
        let continued_payload = dedent(
            "
            unique type Point(Number,
                Number)
            ",
        );
        for src in [constructor_list, payload_per_line, continued_payload] {
            assert_rejected(&src, DiagnosticKind::Unsupported);
        }
    }

    #[test]
    fn dedent_strips_the_common_indentation() {
        assert_eq!(
            dedent(
                "
                a(
                    b
                )
                ",
            ),
            "a(\n    b\n)"
        );
    }
}
