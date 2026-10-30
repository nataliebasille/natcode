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
//!
//! Also parses one nominal type declaration with a positional constructor,
//! `unique type Name(T1, ..., Tn)` (PAR-TYPE-04), whose payloads are type
//! names. Constructor lists (PAR-TYPE-06), transparent `type` declarations
//! (PAR-TYPE-05), `where` restrictions (PAR-REF-05), and other type
//! expressions are not parsed yet.
//!
//! Parsing, printing, and dropping trees use explicit stacks rather than
//! recursion, so deeply nested input cannot overflow the call stack.

use std::fmt;

use crate::lexer::{self, Position, Span, Token, TokenKind, tokenize};

pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

/// Writes the expression as a fully parenthesized S-expression that shows
/// its grouping, such as `(+ a (* b c))` for `a + b * c`. Uses an explicit
/// stack rather than recursion.
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        enum Item<'a> {
            Expr(&'a Expr),
            Text(&'a str),
        }
        let mut stack = vec![Item::Expr(self)];
        while let Some(item) = stack.pop() {
            let expr = match item {
                Item::Text(text) => {
                    f.write_str(text)?;
                    continue;
                }
                Item::Expr(expr) => expr,
            };
            match &expr.kind {
                ExprKind::Identifier(text) | ExprKind::Number(text) => f.write_str(text)?,
                ExprKind::String(text) => write!(f, "{text:?}")?,
                ExprKind::Boolean(value) => write!(f, "{value}")?,
                ExprKind::Paren(inner) => {
                    f.write_str("(paren ")?;
                    stack.extend([Item::Text(")"), Item::Expr(inner)]);
                }
                ExprKind::Member { object, name } => {
                    f.write_str("(. ")?;
                    stack.extend([
                        Item::Text(")"),
                        Item::Text(&name.text),
                        Item::Text(" "),
                        Item::Expr(object),
                    ]);
                }
                ExprKind::Call { callee, args } => {
                    f.write_str("(call ")?;
                    stack.push(Item::Text(")"));
                    for arg in args.iter().rev() {
                        stack.extend([Item::Expr(arg), Item::Text(" ")]);
                    }
                    stack.push(Item::Expr(callee));
                }
                ExprKind::Unary { op, operand } => {
                    write!(f, "({} ", op.symbol())?;
                    stack.extend([Item::Text(")"), Item::Expr(operand)]);
                }
                ExprKind::Binary { op, left, right } => {
                    write!(f, "({} ", op.symbol())?;
                    stack.extend([
                        Item::Text(")"),
                        Item::Expr(right),
                        Item::Text(" "),
                        Item::Expr(left),
                    ]);
                }
                ExprKind::Lambda { params, body } => {
                    f.write_str("(=> (")?;
                    for (index, param) in params.iter().enumerate() {
                        if index > 0 {
                            f.write_str(" ")?;
                        }
                        f.write_str(&param.text)?;
                    }
                    f.write_str(") ")?;
                    stack.extend([Item::Text(")"), Item::Expr(body)]);
                }
            }
        }
        Ok(())
    }
}

/// Same as `Display`; a derived `Debug` would recurse through the tree.
impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Drop for Expr {
    fn drop(&mut self) {
        let mut pending = Vec::new();
        detach_children(&mut self.kind, &mut pending);
        while let Some(mut expr) = pending.pop() {
            detach_children(&mut expr.kind, &mut pending);
        }
    }
}

/// Moves the direct subexpressions of `kind` onto `pending`, leaving leaves
/// in their place, so that dropping `kind` does not recurse.
fn detach_children(kind: &mut ExprKind, pending: &mut Vec<Expr>) {
    let mut detach = |child: &mut Box<Expr>| {
        let leaf = Expr {
            kind: ExprKind::Boolean(false),
            span: child.span,
        };
        pending.push(std::mem::replace(&mut **child, leaf));
    };
    match kind {
        ExprKind::Paren(child)
        | ExprKind::Member { object: child, .. }
        | ExprKind::Unary { operand: child, .. }
        | ExprKind::Lambda { body: child, .. } => detach(child),
        ExprKind::Binary { left, right, .. } => {
            detach(left);
            detach(right);
        }
        ExprKind::Call { callee, args } => {
            detach(callee);
            pending.append(args);
        }
        ExprKind::Identifier(_)
        | ExprKind::Number(_)
        | ExprKind::String(_)
        | ExprKind::Boolean(_) => {}
    }
}

#[derive(Debug)]
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
pub struct Declaration {
    pub kind: DeclarationKind,
    pub span: Span,
}

/// Writes the declaration as an S-expression, such as
/// `(unique-type Point (Number Number))`.
impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            DeclarationKind::NominalType { name, payload } => {
                write!(f, "(unique-type {} (", name.text)?;
                for (index, ty) in payload.iter().enumerate() {
                    if index > 0 {
                        f.write_str(" ")?;
                    }
                    match &ty.kind {
                        TypeExprKind::Name(text) => f.write_str(text)?,
                    }
                }
                f.write_str("))")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclarationKind {
    /// `unique type Name(T1, ..., Tn)` with `n >= 0`: a fresh nominal type
    /// whose one constructor, also called `Name`, takes positional payloads.
    NominalType { name: Name, payload: Vec<TypeExpr> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeExpr {
    pub kind: TypeExprKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeExprKind {
    /// A type referenced by name, such as `Number` or `Position`.
    Name(String),
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

impl UnaryOp {
    pub fn symbol(self) -> &'static str {
        match self {
            UnaryOp::Not => "!",
            UnaryOp::Negate => "-",
        }
    }
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

impl BinaryOp {
    pub fn symbol(self) -> &'static str {
        match self {
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
    /// `type Name(...)` or `unique type Name = expression` (PAR-TYPE-03).
    InvalidTypeDeclaration,
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
    run(source, |parser| parser.parse())
}

/// Parses `source` as one complete declaration. So far this is only a
/// nominal type with a positional constructor (PAR-TYPE-04).
pub fn parse_declaration(source: &str) -> ParseResult<Declaration> {
    run(source, |parser| parser.declaration())
}

fn run<T>(
    source: &str,
    parse: impl for<'a> FnOnce(&mut Parser<'a>) -> Parsed<T>,
) -> ParseResult<T> {
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
    let value = parse(&mut parser).ok();
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

/// Binding strength of pending operators; higher binds tighter.
const PREFIX_PRECEDENCE: u8 = 6;
const COMPARISON_PRECEDENCE: u8 = 3;

fn precedence(op: BinaryOp) -> u8 {
    match op {
        BinaryOp::Multiply | BinaryOp::Divide | BinaryOp::Remainder => 5,
        BinaryOp::Add | BinaryOp::Subtract => 4,
        BinaryOp::LessThan
        | BinaryOp::LessThanOrEqual
        | BinaryOp::GreaterThan
        | BinaryOp::GreaterThanOrEqual
        | BinaryOp::EqualTo
        | BinaryOp::NotEqualTo => COMPARISON_PRECEDENCE,
        BinaryOp::And => 2,
        BinaryOp::Or => 1,
    }
}

enum Operator {
    Prefix { op: UnaryOp, start: usize },
    Binary(BinaryOp),
}

impl Operator {
    fn precedence(&self) -> u8 {
        match self {
            Operator::Prefix { .. } => PREFIX_PRECEDENCE,
            Operator::Binary(op) => precedence(*op),
        }
    }
}

/// What encloses an expression, and so which token ends it.
enum Context {
    /// The whole input, ended by end of input.
    Top,
    /// Grouping parentheses opened at `start`, ended by `)`.
    Paren { start: usize },
    /// One argument of a call, ended by `,` or `)`.
    Call { callee: Expr, args: Vec<Expr> },
    /// A lambda body, ended by whatever ends the enclosing expression.
    LambdaBody { params: Vec<Name>, start: usize },
}

/// One expression being parsed with operator-precedence state. Nested
/// expressions push frames instead of recursing, so nesting depth is
/// limited only by memory, never by the call stack.
struct Frame {
    context: Context,
    /// Byte offset where this frame's expression begins.
    start: usize,
    operands: Vec<Expr>,
    operators: Vec<Operator>,
}

impl Frame {
    fn new(context: Context, start: usize) -> Frame {
        Frame {
            context,
            start,
            operands: Vec::new(),
            operators: Vec::new(),
        }
    }

    fn at_start(&self) -> bool {
        self.operands.is_empty() && self.operators.is_empty()
    }

    /// Applies pending operators that bind at least as tightly as `min`.
    fn reduce(&mut self, min: u8) {
        while self
            .operators
            .last()
            .is_some_and(|op| op.precedence() >= min)
        {
            let operator = self.operators.pop().expect("a pending operator");
            let right = self.operands.pop().expect("an operand for each operator");
            let expr = match operator {
                Operator::Prefix { op, start } => Expr {
                    span: Span {
                        start,
                        end: right.span.end,
                    },
                    kind: ExprKind::Unary {
                        op,
                        operand: Box::new(right),
                    },
                },
                Operator::Binary(op) => {
                    let left = self.operands.pop().expect("a left operand");
                    binary(op, left, right)
                }
            };
            self.operands.push(expr);
        }
    }

    fn finish(&mut self) -> Expr {
        self.reduce(0);
        self.operands.pop().expect("a complete expression")
    }
}

enum Step {
    ExpectOperand,
    AfterOperand,
    Done(Expr),
}

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

    /// Records a diagnostic whose line and column are those of `span.start`.
    fn error<T>(&mut self, kind: DiagnosticKind, message: String, span: Span) -> Parsed<T> {
        let start = self.position_at(span.start);
        self.diagnostics.push(Diagnostic {
            kind,
            message,
            span,
            start,
        });
        Err(Reported)
    }

    /// The line and column of byte `offset`, which every diagnostic span
    /// starts at: the start of some token.
    fn position_at(&self, offset: usize) -> Position {
        let index = self.tokens.partition_point(|t| t.span.start < offset);
        let token = &self.tokens[index.min(self.tokens.len() - 1)];
        debug_assert_eq!(token.span.start, offset, "spans start at a token");
        token.start
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
        )
    }

    /// Parses one complete expression followed by end of input.
    fn parse(&mut self) -> Parsed<Expr> {
        let mut frames = vec![Frame::new(Context::Top, self.peek().span.start)];
        let mut step = Step::ExpectOperand;
        loop {
            step = match step {
                Step::ExpectOperand => self.operand(&mut frames)?,
                Step::AfterOperand => self.after_operand(&mut frames)?,
                Step::Done(expr) => return Ok(expr),
            };
        }
    }

    /// Handles the token where an operand must start.
    fn operand(&mut self, frames: &mut Vec<Frame>) -> Parsed<Step> {
        let token = self.peek().clone();
        let frame = frames.last_mut().expect("an open frame");
        if frame.at_start() {
            if matches!(frame.context, Context::Call { .. })
                && token.kind == TokenKind::Identifier
                && self.peek_kind_at(1) == TokenKind::Colon
            {
                return self.error(
                    DiagnosticKind::Unsupported,
                    "labeled arguments are not supported yet (PAR-FN-01)".to_string(),
                    token.span,
                );
            }
            if let Some(arrow_index) = self.lambda_head_end() {
                let params = self.lambda_params(arrow_index)?;
                let body_start = self.peek().span.start;
                frames.push(Frame::new(
                    Context::LambdaBody {
                        params,
                        start: token.span.start,
                    },
                    body_start,
                ));
                return Ok(Step::ExpectOperand);
            }
        }
        let kind = match token.kind {
            TokenKind::Not | TokenKind::Minus => {
                let op = if token.kind == TokenKind::Not {
                    UnaryOp::Not
                } else {
                    UnaryOp::Negate
                };
                self.advance();
                frame.operators.push(Operator::Prefix {
                    op,
                    start: token.span.start,
                });
                return Ok(Step::ExpectOperand);
            }
            TokenKind::LParen => {
                self.advance();
                let inner_start = self.peek().span.start;
                frames.push(Frame::new(
                    Context::Paren {
                        start: token.span.start,
                    },
                    inner_start,
                ));
                return Ok(Step::ExpectOperand);
            }
            TokenKind::Identifier => ExprKind::Identifier(token.text.clone()),
            TokenKind::Number => ExprKind::Number(token.text.clone()),
            TokenKind::String => ExprKind::String(token.text[1..token.text.len() - 1].to_string()),
            TokenKind::True => ExprKind::Boolean(true),
            TokenKind::False => ExprKind::Boolean(false),
            TokenKind::Wildcard => {
                return self.error(
                    DiagnosticKind::Unsupported,
                    "`_` placeholders in expressions are not supported (PAR-EXPR-02)".to_string(),
                    token.span,
                );
            }
            _ => return self.unexpected("an expression"),
        };
        self.advance();
        frame.operands.push(Expr {
            kind,
            span: token.span,
        });
        Ok(Step::AfterOperand)
    }

    /// Handles the token following a complete operand: a postfix or binary
    /// operator continues the expression; anything else ends the frame.
    fn after_operand(&mut self, frames: &mut Vec<Frame>) -> Parsed<Step> {
        let token = self.peek().clone();
        let frame = frames.last_mut().expect("an open frame");
        match token.kind {
            TokenKind::MemberAccess => {
                self.advance();
                // Keyword spellings are permitted as property names (lexical.md).
                if !self.at(TokenKind::Identifier) && !is_keyword(self.peek().kind) {
                    return self.unexpected("a property name after `.`");
                }
                let name = self.name();
                let object = frame.operands.pop().expect("a member object");
                let span = Span {
                    start: object.span.start,
                    end: name.span.end,
                };
                frame.operands.push(Expr {
                    kind: ExprKind::Member {
                        object: Box::new(object),
                        name,
                    },
                    span,
                });
                Ok(Step::AfterOperand)
            }
            TokenKind::LParen => {
                let callee = frame.operands.pop().expect("a callee");
                if is_lambda(&callee) {
                    return self.error(
                        DiagnosticKind::Unsupported,
                        "immediately invoked lambdas are not supported (PAR-FN-01)".to_string(),
                        Span {
                            start: callee.span.start,
                            end: token.span.end,
                        },
                    );
                }
                self.advance();
                if self.at(TokenKind::RParen) {
                    let end = self.advance().span.end;
                    let span = Span {
                        start: callee.span.start,
                        end,
                    };
                    frame.operands.push(Expr {
                        kind: ExprKind::Call {
                            callee: Box::new(callee),
                            args: Vec::new(),
                        },
                        span,
                    });
                    return Ok(Step::AfterOperand);
                }
                let arg_start = self.peek().span.start;
                frames.push(Frame::new(
                    Context::Call {
                        callee,
                        args: Vec::new(),
                    },
                    arg_start,
                ));
                Ok(Step::ExpectOperand)
            }
            TokenKind::Arrow => self.error(
                DiagnosticKind::InvalidLambdaParameters,
                "the left side of `=>` must be a lambda parameter list such as `x`, `(x, y)`, or `()`"
                    .to_string(),
                Span {
                    start: frame.start,
                    end: token.span.end,
                },
            ),
            kind => match binary_op(kind) {
                Some(op) => {
                    let level = precedence(op);
                    frame.reduce(level + 1);
                    if frame
                        .operators
                        .last()
                        .is_some_and(|top| top.precedence() == level)
                    {
                        if level == COMPARISON_PRECEDENCE {
                            return self.error(
                                DiagnosticKind::ChainedComparison,
                                format!(
                                    "comparison operators cannot be chained; `{}` follows another comparison (combine comparisons with `&&`)",
                                    token.text
                                ),
                                token.span,
                            );
                        }
                        frame.reduce(level);
                    }
                    frame.operators.push(Operator::Binary(op));
                    self.advance();
                    Ok(Step::ExpectOperand)
                }
                None => self.close_frame(frames),
            },
        }
    }

    /// Completes the innermost frame at a token that cannot continue it.
    fn close_frame(&mut self, frames: &mut Vec<Frame>) -> Parsed<Step> {
        let mut frame = frames.pop().expect("an open frame");
        let expr = frame.finish();
        let completed = match frame.context {
            Context::Top => {
                return if self.at(TokenKind::Eof) {
                    Ok(Step::Done(expr))
                } else {
                    self.unexpected("end of expression")
                };
            }
            Context::Paren { start } => {
                if !self.at(TokenKind::RParen) {
                    return self.unexpected("`)`");
                }
                let end = self.advance().span.end;
                Expr {
                    kind: ExprKind::Paren(Box::new(expr)),
                    span: Span { start, end },
                }
            }
            Context::Call { callee, mut args } => {
                args.push(expr);
                match self.peek().kind {
                    TokenKind::Comma => {
                        self.advance();
                        let arg_start = self.peek().span.start;
                        frames.push(Frame::new(Context::Call { callee, args }, arg_start));
                        return Ok(Step::ExpectOperand);
                    }
                    TokenKind::RParen => {
                        let end = self.advance().span.end;
                        let span = Span {
                            start: callee.span.start,
                            end,
                        };
                        Expr {
                            kind: ExprKind::Call {
                                callee: Box::new(callee),
                                args,
                            },
                            span,
                        }
                    }
                    _ => return self.unexpected("`,` or `)` in the argument list"),
                }
            }
            // The token that ended the body also ends the enclosing frame,
            // so it is left for that frame to handle.
            Context::LambdaBody { params, start } => {
                let end = expr.span.end;
                Expr {
                    kind: ExprKind::Lambda {
                        params,
                        body: Box::new(expr),
                    },
                    span: Span { start, end },
                }
            }
        };
        frames
            .last_mut()
            .expect("an enclosing frame")
            .operands
            .push(completed);
        Ok(Step::AfterOperand)
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

    /// Parses lambda parameters and the `=>` at `arrow_index`.
    fn lambda_params(&mut self, arrow_index: usize) -> Parsed<Vec<Name>> {
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
            );
        }
        Ok(params)
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
        let end = self.tokens[arrow_index - 1].span.end;
        self.error(
            DiagnosticKind::InvalidLambdaParameters,
            "lambda parameters must be comma-separated names without type annotations, such as `(x, y)`"
                .to_string(),
            Span { start, end },
        )
    }

    fn name(&mut self) -> Name {
        let token = self.advance();
        Name {
            text: token.text.clone(),
            span: token.span,
        }
    }

    fn unsupported<T>(&mut self, message: &str) -> Parsed<T> {
        let span = self.peek().span;
        self.error(DiagnosticKind::Unsupported, message.to_string(), span)
    }

    /// Parses one complete declaration followed by end of input.
    fn declaration(&mut self) -> Parsed<Declaration> {
        let start = self.peek().span.start;
        match self.peek().kind {
            TokenKind::Unique => {
                self.advance();
                if !self.at(TokenKind::Type) {
                    return self.unexpected("`type` after `unique`");
                }
                self.advance();
            }
            TokenKind::Type => return self.transparent_type(start),
            _ => return self.unexpected("a declaration"),
        }
        if !self.at(TokenKind::Identifier) {
            return self.unexpected("a type name");
        }
        let name = self.name();
        match self.peek().kind {
            TokenKind::LParen => {}
            TokenKind::Assign => {
                let end = self.peek().span.end;
                return self.error(
                    DiagnosticKind::InvalidTypeDeclaration,
                    "`unique type Name = expression` is invalid; use `type Name = expression` for a transparent name (PAR-TYPE-03)".to_string(),
                    Span { start, end },
                );
            }
            TokenKind::Newline | TokenKind::Eof => {
                return self.unsupported(
                    "constructor lists after `unique type Name` are not supported yet (PAR-TYPE-06)",
                );
            }
            _ => return self.unexpected("`(` after the type name"),
        }
        let (payload, end) = self.payload_types()?;
        match self.peek().kind {
            TokenKind::Eof => {}
            TokenKind::Where => {
                return self.unsupported(
                    "`where` restrictions on a constructor are not supported yet (PAR-REF-05)",
                );
            }
            _ => return self.unexpected("end of declaration"),
        }
        Ok(Declaration {
            kind: DeclarationKind::NominalType { name, payload },
            span: Span { start, end },
        })
    }

    /// `type Name ...`, whose `type` keyword starts at `start`.
    fn transparent_type(&mut self, start: usize) -> Parsed<Declaration> {
        self.advance();
        if !self.at(TokenKind::Identifier) {
            return self.unexpected("a type name");
        }
        self.advance();
        match self.peek().kind {
            TokenKind::LParen => {
                let end = self.peek().span.end;
                self.error(
                    DiagnosticKind::InvalidTypeDeclaration,
                    "`type Name(...)` is invalid; declare a nominal constructor with `unique type Name(...)` (PAR-TYPE-03)".to_string(),
                    Span { start, end },
                )
            }
            TokenKind::Assign => self
                .unsupported("transparent `type Name = expression` declarations are not supported yet (PAR-TYPE-05)"),
            _ => self.unexpected("`=` after the type name"),
        }
    }

    /// `()` or `(T1, ..., Tn)`, returning the payload types and the end of `)`.
    fn payload_types(&mut self) -> Parsed<(Vec<TypeExpr>, usize)> {
        self.advance();
        let mut payload = Vec::new();
        if self.at(TokenKind::RParen) {
            return Ok((payload, self.advance().span.end));
        }
        loop {
            payload.push(self.payload_type()?);
            match self.peek().kind {
                TokenKind::Comma => {
                    self.advance();
                }
                TokenKind::RParen => return Ok((payload, self.advance().span.end)),
                TokenKind::Newline => return self.multiline_payload(),
                TokenKind::Where => {
                    return self.unsupported(
                        "`where` restrictions on a payload type are not supported yet (PAR-REF-05)",
                    );
                }
                TokenKind::MemberAccess
                | TokenKind::LParen
                | TokenKind::Union
                | TokenKind::Intersection
                | TokenKind::SetDifference => return self.unsupported_type_expression(),
                kind if binary_op(kind).is_some() => return self.unsupported_type_expression(),
                _ => return self.unexpected("`,` or `)` in the payload list"),
            }
        }
    }

    fn payload_type(&mut self) -> Parsed<TypeExpr> {
        match self.peek().kind {
            TokenKind::Identifier => {
                let name = self.name();
                Ok(TypeExpr {
                    kind: TypeExprKind::Name(name.text),
                    span: name.span,
                })
            }
            TokenKind::Number
            | TokenKind::String
            | TokenKind::True
            | TokenKind::False
            | TokenKind::LParen
            | TokenKind::LBrace
            | TokenKind::Minus
            | TokenKind::Not
            | TokenKind::SetComplement => self.unsupported_type_expression(),
            TokenKind::Newline => self.multiline_payload(),
            _ => self.unexpected("a payload type"),
        }
    }

    fn unsupported_type_expression<T>(&mut self) -> Parsed<T> {
        self.unsupported("payload types other than a type name are not supported yet (PAR-TYPE-01, PAR-TYPE-07 to PAR-TYPE-14)")
    }

    fn multiline_payload<T>(&mut self) -> Parsed<T> {
        self.unsupported("payload lists spanning lines are not supported yet (PAR-BLK-02)")
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
fn is_lambda(mut expr: &Expr) -> bool {
    loop {
        match &expr.kind {
            ExprKind::Lambda { .. } => return true,
            ExprKind::Paren(inner) => expr = inner,
            _ => return false,
        }
    }
}

fn binary_op(kind: TokenKind) -> Option<BinaryOp> {
    Some(match kind {
        TokenKind::Star => BinaryOp::Multiply,
        TokenKind::Divide => BinaryOp::Divide,
        TokenKind::Remainder => BinaryOp::Remainder,
        TokenKind::Plus => BinaryOp::Add,
        TokenKind::Minus => BinaryOp::Subtract,
        TokenKind::LessThan => BinaryOp::LessThan,
        TokenKind::LessThanOrEqual => BinaryOp::LessThanOrEqual,
        TokenKind::GreaterThan => BinaryOp::GreaterThan,
        TokenKind::GreaterThanOrEqual => BinaryOp::GreaterThanOrEqual,
        TokenKind::EqualTo => BinaryOp::EqualTo,
        TokenKind::NotEqualTo => BinaryOp::NotEqualTo,
        TokenKind::And => BinaryOp::And,
        TokenKind::Or => BinaryOp::Or,
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
