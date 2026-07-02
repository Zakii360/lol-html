use crate::lol::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {

    Identifier,

    String,

    Number,

    Boolean,

    Colon,

    Comma,

    Dot,

    Equal,

    Arrow,

    LBrace,

    RBrace,

    LParen,

    RParen,

    LBracket,

    RBracket,

    Newline,

    Indent,

    Dedent,

    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {

    pub kind: TokenKind,

    pub lexeme: String,

    pub span: Span,
}

impl Token {

    pub fn new(
        kind: TokenKind,
        lexeme: impl Into<String>,
        span: Span,
    ) -> Self {

        Self {
            kind,
            lexeme: lexeme.into(),
            span,
        }

    }

}
