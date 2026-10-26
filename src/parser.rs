//! Parser for Natcode. Item status lives in `docs/scorecards/parser.md`.
//!
//! Implements ordinary value expressions with the PAR-EXPR-01 precedence,
//! from highest to lowest: postfix member access and calls, prefix `!` and
//! `-`, multiplicative `*` `/` `%`, additive `+` `-`, non-associative
//! comparisons, `&&`, `||`, then right-associative `=>`.
//!
//! Not yet parsed, because they belong to other items: expression
//! continuation across line breaks (PAR-BLK-02), executable block bodies
//! (PAR-BLK-01), labeled call arguments (PAR-FN-01), immediately invoked
//! lambdas (PAR-FN-01), `as?` validation (PAR-VAL-01), `_` placeholders
//! (PAR-EXPR-02), object literals (PAR-OBJ-01), and `when`. A line break
//! inside a block comment ends a line like any other (LEX-COM-04). The parser
//! stops at the first syntax error; error recovery is PAR-ERR-01.

use crate::lexer::{self, Position, Span, Token, TokenKind, tokenize};

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Identifier(String),
    /// Source text of a number literal; numeric representation is TYP-BLT-05.
    Number(String),
    /// Contents of a string literal, without its quotes.
    String(String),
    Boolean(bool),
    /// Explicit grouping parentheses, kept so that formatting can preserve them.
    Paren(Box<Expr>),
    Member {
        object: Box<Expr>,
        name: Name,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Lambda {
        params: Vec<Name>,
        body: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    pub text: String,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Not,
    Negate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Multiply,
    Divide,
    Remainder,
    Add,
    Subtract,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    EqualTo,
    NotEqualTo,
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticKind {
    /// A lexical error reported by the lexer.
    Lexical(lexer::DiagnosticKind),
    UnexpectedToken,
    /// Comparisons are non-associative: `a < b < c` is invalid.
    ChainedComparison,
    /// The left side of `=>` is not a lambda parameter list.
    InvalidLambdaParameters,
    /// Syntax owned by a scorecard item that is not implemented yet.
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub message: String,
    pub span: Span,
    pub start: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseResult<T> {
    /// `None` when a lexical or syntax error was reported.
    pub value: Option<T>,
    pub diagnostics: Vec<Diagnostic>,
}

/// Parses `source` as one complete value expression.
pub fn parse_expression(source: &str) -> ParseResult<Expr> {
    let lexed = tokenize(source);
    let mut diagnostics: Vec<Diagnostic> = lexed
        .diagnostics
        .into_iter()
        .map(|d| Diagnostic {
            kind: DiagnosticKind::Lexical(d.kind),
            message: d.message,
            span: d.span,
            start: d.start,
        })
        .collect();
    let tokens = restore_comment_line_breaks(source, lexed.tokens);
    let closing_parens = match_parens(&tokens);
    let mut parser = Parser {
        tokens: &tokens,
        closing_parens,
        pos: 0,
        diagnostics: Vec::new(),
    };
    let value = parser.expression().ok().and_then(|expr| {
        if parser.at(TokenKind::Eof) {
            Some(expr)
        } else {
            parser.unexpected("end of expression").ok()
        }
    });
    let lexical_error = !diagnostics.is_empty();
    diagnostics.append(&mut parser.diagnostics);
    ParseResult {
        value: if lexical_error { None } else { value },
        diagnostics,
    }
}

/// The lexer emits no newline token for a line break inside a block comment
/// (LEX-COM-04), so this inserts one wherever two adjacent tokens are on
/// different lines. Removing comment text must never join two lines.
fn restore_comment_line_breaks(source: &str, tokens: Vec<Token>) -> Vec<Token> {
    let mut restored: Vec<Token> = Vec::with_capacity(tokens.len());
    for token in tokens {
        if let Some(prev) = restored.last()
            && prev.kind != TokenKind::Newline
            && token.kind != TokenKind::Newline
            && token.start.line > prev.start.line
            && let Some(found) = source[prev.span.end..token.span.start].find('\n')
        {
            // Only block comments span lines between tokens.
            let offset = prev.span.end + found;
            let start = if offset > 0 && source.as_bytes()[offset - 1] == b'\r' {
                offset - 1
            } else {
                offset
            };
            let column = prev.start.column + source[prev.span.start..start].chars().count() as u32;
            restored.push(Token {
                kind: TokenKind::Newline,
                text: source[start..offset + 1].to_string(),
                span: Span {
                    start,
                    end: offset + 1,
                },
                start: Position {
                    line: prev.start.line,
                    column,
                },
            });
        }
        restored.push(token);
    }
    restored
}

/// For each `(` token index, the index of its matching `)`, if any.
fn match_parens(tokens: &[Token]) -> Vec<Option<usize>> {
    let mut closing = vec![None; tokens.len()];
    let mut open = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        match token.kind {
            TokenKind::LParen => open.push(index),
            TokenKind::RParen => {
                if let Some(start) = open.pop() {
                    closing[start] = Some(index);
                }
            }
            _ => {}
        }
    }
    closing
}

/// A syntax error has already been recorded as a diagnostic.
struct Reported;

type Parsed<T> = Result<T, Reported>;

struct Parser<'a> {
    tokens: &'a [Token],
    closing_parens: Vec<Option<usize>>,
    pos: usize,
    diagnostics: Vec<Diagnostic>,
}

impl Parser<'_> {
    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn peek_kind_at(&self, offset: usize) -> TokenKind {
        self.tokens
            .get(self.pos + offset)
            .map_or(TokenKind::Eof, |t| t.kind)
    }

    fn at(&self, kind: TokenKind) -> bool {
        self.peek().kind == kind
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.pos];
        if token.kind != TokenKind::Eof {
            self.pos += 1;
        }
        token
    }

    fn expect(&mut self, kind: TokenKind, expected: &str) -> Parsed<Token> {
        if self.at(kind) {
            Ok(self.advance().clone())
        } else {
            self.unexpected(expected)
        }
    }

    fn error<T>(
        &mut self,
        kind: DiagnosticKind,
        message: String,
        span: Span,
        start: Position,
    ) -> Parsed<T> {
        self.diagnostics.push(Diagnostic {
            kind,
            message,
            span,
            start,
        });
        Err(Reported)
    }

    fn unexpected<T>(&mut self, expected: &str) -> Parsed<T> {
        let token = self.peek().clone();
        // The lexer has already reported the problem with an error token.
        if token.kind == TokenKind::Error {
            return Err(Reported);
        }
        let found = match token.kind {
            TokenKind::Eof => "end of input".to_string(),
            TokenKind::Newline => "line break".to_string(),
            _ => format!("`{}`", token.text),
        };
        self.error(
            DiagnosticKind::UnexpectedToken,
            format!("expected {expected}, found {found}"),
            token.span,
            token.start,
        )
    }

    /// `=>` has the lowest precedence and is right-associative.
    fn expression(&mut self) -> Parsed<Expr> {
        if let Some(params_end) = self.lambda_head_end() {
            return self.lambda(params_end);
        }
        let expr = self.or()?;
        if self.at(TokenKind::Arrow) {
            let arrow = self.peek().clone();
            return self.error(
                DiagnosticKind::InvalidLambdaParameters,
                "the left side of `=>` must be a lambda parameter list such as `x`, `(x, y)`, or `()`"
                    .to_string(),
                Span {
                    start: expr.span.start,
                    end: arrow.span.end,
                },
                arrow.start,
            );
        }
        Ok(expr)
    }

    /// If a lambda starts here, the index of the `=>` that ends its parameters.
    fn lambda_head_end(&self) -> Option<usize> {
        match self.peek().kind {
            TokenKind::Identifier if self.peek_kind_at(1) == TokenKind::Arrow => Some(self.pos + 1),
            TokenKind::LParen => {
                let close = self.closing_parens[self.pos]?;
                (self.tokens.get(close + 1)?.kind == TokenKind::Arrow).then_some(close + 1)
            }
            _ => None,
        }
    }

    fn lambda(&mut self, arrow_index: usize) -> Parsed<Expr> {
        let start = self.peek().span.start;
        let params = if self.at(TokenKind::Identifier) {
            vec![self.name()]
        } else {
            self.parenthesized_params(arrow_index)?
        };
        self.advance();
        if self.at(TokenKind::LBrace) {
            let brace = self.peek().clone();
            return self.error(
                DiagnosticKind::Unsupported,
                "lambda block bodies are not supported yet (PAR-BLK-01, PAR-BLK-02)".to_string(),
                brace.span,
                brace.start,
            );
        }
        let body = self.expression()?;
        let end = body.span.end;
        Ok(Expr {
            kind: ExprKind::Lambda {
                params,
                body: Box::new(body),
            },
            span: Span { start, end },
        })
    }

    /// `()`, `(x)`, or `(x, y, ...)`, ending just before `=>`.
    fn parenthesized_params(&mut self, arrow_index: usize) -> Parsed<Vec<Name>> {
        let open = self.advance().clone();
        let mut params = Vec::new();
        if !self.at(TokenKind::RParen) {
            loop {
                if !self.at(TokenKind::Identifier) {
                    return self.invalid_params(open.span.start, arrow_index);
                }
                params.push(self.name());
                if self.at(TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        if self.pos + 1 != arrow_index || !self.at(TokenKind::RParen) {
            return self.invalid_params(open.span.start, arrow_index);
        }
        self.advance();
        Ok(params)
    }

    fn invalid_params<T>(&mut self, start: usize, arrow_index: usize) -> Parsed<T> {
        let token = self.peek().clone();
        let end = self.tokens[arrow_index - 1].span.end;
        self.error(
            DiagnosticKind::InvalidLambdaParameters,
            "lambda parameters must be comma-separated names without type annotations, such as `(x, y)`"
                .to_string(),
            Span { start, end },
            token.start,
        )
    }

    fn name(&mut self) -> Name {
        let token = self.advance();
        Name {
            text: token.text.clone(),
            span: token.span,
        }
    }

    fn or(&mut self) -> Parsed<Expr> {
        let mut left = self.and()?;
        while self.at(TokenKind::Or) {
            self.advance();
            let right = self.and()?;
            left = binary(BinaryOp::Or, left, right);
        }
        Ok(left)
    }

    fn and(&mut self) -> Parsed<Expr> {
        let mut left = self.comparison()?;
        while self.at(TokenKind::And) {
            self.advance();
            let right = self.comparison()?;
            left = binary(BinaryOp::And, left, right);
        }
        Ok(left)
    }

    fn comparison(&mut self) -> Parsed<Expr> {
        let left = self.additive()?;
        let Some(op) = comparison_op(self.peek().kind) else {
            return Ok(left);
        };
        self.advance();
        let right = self.additive()?;
        if comparison_op(self.peek().kind).is_some() {
            let token = self.peek().clone();
            return self.error(
                DiagnosticKind::ChainedComparison,
                format!(
                    "comparison operators cannot be chained; `{}` follows another comparison (combine comparisons with `&&`)",
                    token.text
                ),
                token.span,
                token.start,
            );
        }
        Ok(binary(op, left, right))
    }

    fn additive(&mut self) -> Parsed<Expr> {
        let mut left = self.multiplicative()?;
        loop {
            let op = match self.peek().kind {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Subtract,
                _ => return Ok(left),
            };
            self.advance();
            let right = self.multiplicative()?;
            left = binary(op, left, right);
        }
    }

    fn multiplicative(&mut self) -> Parsed<Expr> {
        let mut left = self.unary()?;
        loop {
            let op = match self.peek().kind {
                TokenKind::Star => BinaryOp::Multiply,
                TokenKind::Divide => BinaryOp::Divide,
                TokenKind::Remainder => BinaryOp::Remainder,
                _ => return Ok(left),
            };
            self.advance();
            let right = self.unary()?;
            left = binary(op, left, right);
        }
    }

    fn unary(&mut self) -> Parsed<Expr> {
        let op = match self.peek().kind {
            TokenKind::Not => UnaryOp::Not,
            TokenKind::Minus => UnaryOp::Negate,
            _ => return self.postfix(),
        };
        let start = self.advance().span.start;
        let operand = self.unary()?;
        let end = operand.span.end;
        Ok(Expr {
            kind: ExprKind::Unary {
                op,
                operand: Box::new(operand),
            },
            span: Span { start, end },
        })
    }

    fn postfix(&mut self) -> Parsed<Expr> {
        let mut expr = self.primary()?;
        loop {
            match self.peek().kind {
                TokenKind::MemberAccess => {
                    self.advance();
                    // Keyword spellings are permitted as property names (lexical.md).
                    if !self.at(TokenKind::Identifier) && !is_keyword(self.peek().kind) {
                        return self.unexpected("a property name after `.`");
                    }
                    let name = self.name();
                    let span = Span {
                        start: expr.span.start,
                        end: name.span.end,
                    };
                    expr = Expr {
                        kind: ExprKind::Member {
                            object: Box::new(expr),
                            name,
                        },
                        span,
                    };
                }
                TokenKind::LParen => {
                    if is_lambda(&expr) {
                        let open = self.peek().clone();
                        return self.error(
                            DiagnosticKind::Unsupported,
                            "immediately invoked lambdas are not supported (PAR-FN-01)".to_string(),
                            Span {
                                start: expr.span.start,
                                end: open.span.end,
                            },
                            open.start,
                        );
                    }
                    self.advance();
                    let args = self.arguments()?;
                    let close =
                        self.expect(TokenKind::RParen, "`,` or `)` in the argument list")?;
                    let span = Span {
                        start: expr.span.start,
                        end: close.span.end,
                    };
                    expr = Expr {
                        kind: ExprKind::Call {
                            callee: Box::new(expr),
                            args,
                        },
                        span,
                    };
                }
                _ => return Ok(expr),
            }
        }
    }

    /// Comma-separated positional arguments, up to (not including) `)`.
    fn arguments(&mut self) -> Parsed<Vec<Expr>> {
        let mut args = Vec::new();
        if self.at(TokenKind::RParen) {
            return Ok(args);
        }
        loop {
            if self.at(TokenKind::Identifier) && self.peek_kind_at(1) == TokenKind::Colon {
                let label = self.peek().clone();
                return self.error(
                    DiagnosticKind::Unsupported,
                    "labeled arguments are not supported yet (PAR-FN-01)".to_string(),
                    label.span,
                    label.start,
                );
            }
            args.push(self.expression()?);
            if self.at(TokenKind::Comma) {
                self.advance();
            } else {
                return Ok(args);
            }
        }
    }

    fn primary(&mut self) -> Parsed<Expr> {
        let token = self.peek().clone();
        let kind = match token.kind {
            TokenKind::Identifier => ExprKind::Identifier(token.text.clone()),
            TokenKind::Number => ExprKind::Number(token.text.clone()),
            TokenKind::String => ExprKind::String(token.text[1..token.text.len() - 1].to_string()),
            TokenKind::True => ExprKind::Boolean(true),
            TokenKind::False => ExprKind::Boolean(false),
            TokenKind::LParen => {
                self.advance();
                let inner = self.expression()?;
                let close = self.expect(TokenKind::RParen, "`)`")?;
                return Ok(Expr {
                    kind: ExprKind::Paren(Box::new(inner)),
                    span: Span {
                        start: token.span.start,
                        end: close.span.end,
                    },
                });
            }
            TokenKind::Wildcard => {
                return self.error(
                    DiagnosticKind::Unsupported,
                    "`_` placeholders in expressions are not supported (PAR-EXPR-02)".to_string(),
                    token.span,
                    token.start,
                );
            }
            _ => return self.unexpected("an expression"),
        };
        self.advance();
        Ok(Expr {
            kind,
            span: token.span,
        })
    }
}

/// The hard-reserved keywords of `lexical.md`.
fn is_keyword(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::From
            | TokenKind::Import
            | TokenKind::Export
            | TokenKind::Unique
            | TokenKind::Type
            | TokenKind::Fn
            | TokenKind::When
            | TokenKind::Where
            | TokenKind::As
    )
}

/// A lambda, possibly wrapped in grouping parentheses.
fn is_lambda(expr: &Expr) -> bool {
    match &expr.kind {
        ExprKind::Lambda { .. } => true,
        ExprKind::Paren(inner) => is_lambda(inner),
        _ => false,
    }
}

fn comparison_op(kind: TokenKind) -> Option<BinaryOp> {
    Some(match kind {
        TokenKind::LessThan => BinaryOp::LessThan,
        TokenKind::LessThanOrEqual => BinaryOp::LessThanOrEqual,
        TokenKind::GreaterThan => BinaryOp::GreaterThan,
        TokenKind::GreaterThanOrEqual => BinaryOp::GreaterThanOrEqual,
        TokenKind::EqualTo => BinaryOp::EqualTo,
        TokenKind::NotEqualTo => BinaryOp::NotEqualTo,
        _ => return None,
    })
}

fn binary(op: BinaryOp, left: Expr, right: Expr) -> Expr {
    let span = Span {
        start: left.span.start,
        end: right.span.end,
    };
    Expr {
        kind: ExprKind::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        },
        span,
    }
}
