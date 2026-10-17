//! Lexer for Natcode. Item status lives in `docs/scorecards/lexer.md`.
//!
//! Implements the Implemented `LEX-*` items: UAX #31 identifiers (XID_Start
//! or `_`, then XID_Continue) normalized to NFC, the `from`/`import`/`export`
//! keywords, boolean literals, decimal numbers, escape-free double-quoted
//! strings, line/nested block comments, the core operator set, and newline
//! tokens. Emoji identifiers (LEX-ID-04) and indentation tokens (LEX-IND-01)
//! are open questions and not handled yet.

use unicode_ident::{is_xid_continue, is_xid_start};
use unicode_normalization::{UnicodeNormalization, is_nfc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Identifier,
    Number,
    String,
    True,
    False,
    From,
    Import,
    Export,
    /// Standalone `_`, reserved as a wildcard.
    Underscore,
    /// `=`
    Equals,
    /// `=>`
    FatArrow,
    /// `:`
    Colon,
    /// `|`
    Pipe,
    /// `&`
    Ampersand,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `*`
    Star,
    /// `+`
    Plus,
    Newline,
    Error,
    Eof,
}

/// Half-open byte range `[start, end)` into the source text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

/// 1-based line and column; columns count Unicode scalar values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    /// Source slice covered by the token (strings include their quotes).
    /// Identifier text is NFC-normalized, so it may differ from the raw
    /// bytes at `span`; all other token kinds hold the raw slice.
    pub text: String,
    pub span: Span,
    pub start: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticKind {
    UnexpectedCharacter,
    UnterminatedString,
    EscapeNotSupported,
    UnterminatedBlockComment,
    InvalidDigitSeparator,
    /// A number directly followed by identifier characters, such as `42abc`.
    InvalidNumberSuffix,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub message: String,
    pub span: Span,
    pub start: Position,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexResult {
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
}

pub fn tokenize(source: &str) -> LexResult {
    let mut lexer = Lexer {
        src: source,
        pos: 0,
        line: 1,
        column: 1,
        tokens: Vec::new(),
        diagnostics: Vec::new(),
    };
    lexer.run();
    LexResult {
        tokens: lexer.tokens,
        diagnostics: lexer.diagnostics,
    }
}

struct Lexer<'a> {
    src: &'a str,
    pos: usize,
    line: u32,
    column: u32,
    tokens: Vec<Token>,
    diagnostics: Vec<Diagnostic>,
}

impl Lexer<'_> {
    fn run(&mut self) {
        while let Some(c) = self.peek() {
            let start = self.pos;
            let start_pos = self.position();
            match c {
                ' ' => {
                    self.bump();
                }
                '\n' => {
                    self.bump();
                    self.push(TokenKind::Newline, start, start_pos);
                }
                '\r' if self.peek_second() == Some('\n') => {
                    self.bump();
                    self.bump();
                    self.push(TokenKind::Newline, start, start_pos);
                }
                '/' if self.peek_second() == Some('/') => self.line_comment(),
                '/' if self.peek_second() == Some('*') => self.block_comment(start, start_pos),
                '"' => self.string(start, start_pos),
                '0'..='9' => self.number(start, start_pos),
                c if is_identifier_start(c) => self.identifier(start, start_pos),
                '=' => {
                    self.bump();
                    if self.peek() == Some('>') {
                        self.bump();
                        self.push(TokenKind::FatArrow, start, start_pos);
                    } else {
                        self.push(TokenKind::Equals, start, start_pos);
                    }
                }
                c => match punctuation(c) {
                    Some(kind) => {
                        self.bump();
                        self.push(kind, start, start_pos);
                    }
                    None => self.unexpected(start, start_pos),
                },
            }
        }
        let end = self.pos;
        let end_pos = self.position();
        self.push(TokenKind::Eof, end, end_pos);
    }

    fn peek(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    fn peek_second(&self) -> Option<char> {
        let mut chars = self.src[self.pos..].chars();
        chars.next();
        chars.next()
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(c)
    }

    fn position(&self) -> Position {
        Position {
            line: self.line,
            column: self.column,
        }
    }

    fn at_line_break(&self) -> bool {
        match self.peek() {
            Some('\n') => true,
            Some('\r') => self.peek_second() == Some('\n'),
            _ => false,
        }
    }

    fn push(&mut self, kind: TokenKind, start: usize, start_pos: Position) {
        let text = self.src[start..self.pos].to_string();
        self.push_text(kind, text, start, start_pos);
    }

    fn push_text(&mut self, kind: TokenKind, text: String, start: usize, start_pos: Position) {
        self.tokens.push(Token {
            kind,
            text,
            span: Span {
                start,
                end: self.pos,
            },
            start: start_pos,
        });
    }

    fn diagnose(&mut self, kind: DiagnosticKind, message: String, span: Span, start: Position) {
        self.diagnostics.push(Diagnostic {
            kind,
            message,
            span,
            start,
        });
    }

    fn span_from(&self, start: usize) -> Span {
        Span {
            start,
            end: self.pos,
        }
    }

    fn line_comment(&mut self) {
        while self.peek().is_some() && !self.at_line_break() {
            self.bump();
        }
    }

    fn block_comment(&mut self, start: usize, start_pos: Position) {
        self.bump();
        self.bump();
        let mut depth = 1usize;
        while depth > 0 {
            match (self.peek(), self.peek_second()) {
                (None, _) => {
                    self.push(TokenKind::Error, start, start_pos);
                    self.diagnose(
                        DiagnosticKind::UnterminatedBlockComment,
                        "unterminated block comment; expected `*/`".to_string(),
                        self.span_from(start),
                        start_pos,
                    );
                    return;
                }
                (Some('/'), Some('*')) => {
                    self.bump();
                    self.bump();
                    depth += 1;
                }
                (Some('*'), Some('/')) => {
                    self.bump();
                    self.bump();
                    depth -= 1;
                }
                _ => {
                    self.bump();
                }
            }
        }
    }

    fn string(&mut self, start: usize, start_pos: Position) {
        self.bump();
        let mut backslash: Option<(usize, Position)> = None;
        let terminated = loop {
            match self.peek() {
                None => break false,
                Some(_) if self.at_line_break() => break false,
                Some('"') => {
                    self.bump();
                    break true;
                }
                Some('\\') => {
                    if backslash.is_none() {
                        backslash = Some((self.pos, self.position()));
                    }
                    self.bump();
                }
                Some(_) => {
                    self.bump();
                }
            }
        };

        let kind = if terminated && backslash.is_none() {
            TokenKind::String
        } else {
            TokenKind::Error
        };
        self.push(kind, start, start_pos);

        // The unterminated diagnostic starts at the opening quote, so it precedes the backslash.
        if !terminated {
            self.diagnose(
                DiagnosticKind::UnterminatedString,
                "unterminated string literal; expected closing `\"` on the same line".to_string(),
                self.span_from(start),
                start_pos,
            );
        }
        if let Some((offset, position)) = backslash {
            self.diagnose(
                DiagnosticKind::EscapeNotSupported,
                "escape sequences are not supported in string literals".to_string(),
                Span {
                    start: offset,
                    end: offset + 1,
                },
                position,
            );
        }
    }

    fn number(&mut self, start: usize, start_pos: Position) {
        self.digits();
        if self.peek() == Some('.')
            && matches!(self.peek_second(), Some(c) if c.is_ascii_digit() || c == '_')
        {
            self.bump();
            self.digits();
        }

        let digits_end = self.pos;
        while matches!(self.peek(), Some(c) if is_xid_continue(c)) {
            self.bump();
        }

        let text = &self.src[start..self.pos];
        if self.pos > digits_end {
            self.push(TokenKind::Error, start, start_pos);
            self.diagnose(
                DiagnosticKind::InvalidNumberSuffix,
                format!("invalid number `{text}`: a number cannot be directly followed by letters"),
                self.span_from(start),
                start_pos,
            );
        } else if has_valid_digit_separators(text) {
            self.push(TokenKind::Number, start, start_pos);
        } else {
            self.push(TokenKind::Error, start, start_pos);
            self.diagnose(
                DiagnosticKind::InvalidDigitSeparator,
                format!("invalid number `{text}`: `_` must appear between two digits"),
                self.span_from(start),
                start_pos,
            );
        }
    }

    fn digits(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_ascii_digit() || c == '_') {
            self.bump();
        }
    }

    fn identifier(&mut self, start: usize, start_pos: Position) {
        self.bump();
        while matches!(self.peek(), Some(c) if is_xid_continue(c)) {
            self.bump();
        }

        let raw = &self.src[start..self.pos];
        let text: String = if is_nfc(raw) {
            raw.to_string()
        } else {
            raw.nfc().collect()
        };
        let kind = match text.as_str() {
            "_" => TokenKind::Underscore,
            "from" => TokenKind::From,
            "import" => TokenKind::Import,
            "export" => TokenKind::Export,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            _ => TokenKind::Identifier,
        };
        self.push_text(kind, text, start, start_pos);
    }

    fn unexpected(&mut self, start: usize, start_pos: Position) {
        self.bump();
        while let Some(c) = self.peek() {
            if self.can_start_token(c) {
                break;
            }
            self.bump();
        }
        self.push(TokenKind::Error, start, start_pos);
        let text = &self.src[start..self.pos];
        let message = if text.chars().count() == 1 {
            format!("unexpected character `{text}`")
        } else {
            format!("unexpected characters `{text}`")
        };
        self.diagnose(
            DiagnosticKind::UnexpectedCharacter,
            message,
            self.span_from(start),
            start_pos,
        );
    }

    /// Whether lexing at `c` (the current character) would do anything other
    /// than report an unexpected character.
    fn can_start_token(&self, c: char) -> bool {
        match c {
            ' ' | '\n' | '"' | '=' | '0'..='9' => true,
            '\r' => self.peek_second() == Some('\n'),
            '/' => matches!(self.peek_second(), Some('/' | '*')),
            c => is_identifier_start(c) || punctuation(c).is_some(),
        }
    }
}

fn is_identifier_start(c: char) -> bool {
    c == '_' || is_xid_start(c)
}

fn punctuation(c: char) -> Option<TokenKind> {
    Some(match c {
        ':' => TokenKind::Colon,
        '|' => TokenKind::Pipe,
        '&' => TokenKind::Ampersand,
        '{' => TokenKind::LBrace,
        '}' => TokenKind::RBrace,
        '(' => TokenKind::LParen,
        ')' => TokenKind::RParen,
        ',' => TokenKind::Comma,
        '.' => TokenKind::Dot,
        '*' => TokenKind::Star,
        '+' => TokenKind::Plus,
        _ => return None,
    })
}

/// Every `_` in a numeric lexeme must sit between two ASCII digits.
fn has_valid_digit_separators(text: &str) -> bool {
    let bytes = text.as_bytes();
    bytes.iter().enumerate().all(|(i, &b)| {
        b != b'_'
            || (i > 0
                && bytes[i - 1].is_ascii_digit()
                && bytes.get(i + 1).is_some_and(u8::is_ascii_digit))
    })
}
