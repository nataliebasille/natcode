//! Parser tests. Each test is cited as evidence by an item in
//! `docs/scorecards/parser.md`.

use natcode::lexer::Span;
use natcode::parser::{BinaryOp, DiagnosticKind, Expr, ExprKind, UnaryOp, parse_expression};

/// Renders a parsed expression as an S-expression, so that tests can state
/// the grouping explicitly: `a + b * c` renders as `(+ a (* b c))`.
fn sexpr(expr: &Expr) -> String {
    match &expr.kind {
        ExprKind::Identifier(name) => name.clone(),
        ExprKind::Number(text) => text.clone(),
        ExprKind::String(text) => format!("{text:?}"),
        ExprKind::Boolean(value) => value.to_string(),
        ExprKind::Paren(inner) => format!("(paren {})", sexpr(inner)),
        ExprKind::Member { object, name } => format!("(. {} {})", sexpr(object), name.text),
        ExprKind::Call { callee, args } => {
            let mut out = format!("(call {}", sexpr(callee));
            for arg in args {
                out.push(' ');
                out.push_str(&sexpr(arg));
            }
            out.push(')');
            out
        }
        ExprKind::Unary { op, operand } => {
            let op = match op {
                UnaryOp::Not => "!",
                UnaryOp::Negate => "neg",
            };
            format!("({op} {})", sexpr(operand))
        }
        ExprKind::Binary { op, left, right } => {
            format!("({} {} {})", binary_op(*op), sexpr(left), sexpr(right))
        }
        ExprKind::Lambda { params, body } => {
            let params: Vec<&str> = params.iter().map(|p| p.text.as_str()).collect();
            format!("(=> ({}) {})", params.join(" "), sexpr(body))
        }
    }
}

fn binary_op(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Multiply => "*",
        BinaryOp::Divide => "/",
        BinaryOp::Remainder => "%",
        BinaryOp::Add => "+",
        BinaryOp::Subtract => "-",
        BinaryOp::LessThan => "<",
        BinaryOp::LessThanOrEqual => "<=",
        BinaryOp::GreaterThan => ">",
        BinaryOp::GreaterThanOrEqual => ">=",
        BinaryOp::EqualTo => "==",
        BinaryOp::NotEqualTo => "!=",
        BinaryOp::And => "&&",
        BinaryOp::Or => "||",
    }
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
    assert_eq!(sexpr(&parse(src)), expected, "tree for {src:?}");
}

/// Asserts that `src` is rejected, with exactly one diagnostic of `kind`.
fn assert_rejected(src: &str, kind: DiagnosticKind) {
    let result = parse_expression(src);
    assert!(result.value.is_none(), "{src:?} should be rejected");
    let kinds: Vec<DiagnosticKind> = result.diagnostics.iter().map(|d| d.kind).collect();
    assert_eq!(kinds, vec![kind], "diagnostics for {src:?}");
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
        let ExprKind::Binary { left, right, .. } = expr.kind else {
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
}

mod prefix {
    use super::*;

    #[test]
    fn prefix_operators_bind_looser_than_postfix() {
        assert_parses("-x", "(neg x)");
        assert_parses("!ready", "(! ready)");
        assert_parses("-f(x)", "(neg (call f x))");
        assert_parses("!user.active", "(! (. user active))");
        assert_parses("!!x", "(! (! x))");
        assert_parses("- -x", "(neg (neg x))");
    }

    #[test]
    fn prefix_operators_bind_tighter_than_binary() {
        assert_parses("-a * b", "(* (neg a) b)");
        assert_parses("a - -b", "(- a (neg b))");
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
        assert_parses("(x => x)(1)", "(call (paren (=> (x) x)) 1)");
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
