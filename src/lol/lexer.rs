// this isn't complete yet, it'd be nice if someone could contribute to this and help build.
use crate::lol::{
    error::{LolError, Result},
    span::{Position, Span},
    token::{Token, TokenKind},
};

pub struct Lexer<'a> {
    source: &'a str,
    chars: Vec<char>,
    index: usize,

    line: usize,
    column: usize,

    indent_stack: Vec<usize>,
    pending: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().collect(),
            index: 0,

            line: 1,
            column: 1,

            indent_stack: vec![0],
            pending: Vec::new(),
        }
    }

    pub fn lex(mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while !self.eof() {
            if let Some(token) = self.pending.pop() {
                tokens.push(token);
                continue;
            }

            self.skip_whitespace();

            if self.eof() {
                break;
            }

            tokens.push(self.next_token()?);
        }

        while self.indent_stack.len() > 1 {
            self.indent_stack.pop();

            tokens.push(Token::new(
                TokenKind::Dedent,
                "",
                Span::empty(),
            ));
        }

        tokens.push(Token::new(
            TokenKind::EOF,
            "",
            Span::empty(),
        ));

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token> {
        let start = self.position();

        let c = match self.peek() {
            Some(c) => c,
            None => {
                return Ok(Token::new(
                    TokenKind::EOF,
                    "",
                    Span::empty(),
                ));
            }
        };

        match c {

            '{' => {
                self.advance();
                Ok(self.simple(TokenKind::LBrace, "{", start))
            }

            '}' => {
                self.advance();
                Ok(self.simple(TokenKind::RBrace, "}", start))
            }

            '(' => {
                self.advance();
                Ok(self.simple(TokenKind::LParen, "(", start))
            }

            ')' => {
                self.advance();
                Ok(self.simple(TokenKind::RParen, ")", start))
            }

            '[' => {
                self.advance();
                Ok(self.simple(TokenKind::LBracket, "[", start))
            }

            ']' => {
                self.advance();
                Ok(self.simple(TokenKind::RBracket, "]", start))
            }

            ':' => {
                self.advance();
                Ok(self.simple(TokenKind::Colon, ":", start))
            }

            ',' => {
                self.advance();
                Ok(self.simple(TokenKind::Comma, ",", start))
            }

            '.' => {
                self.advance();
                Ok(self.simple(TokenKind::Dot, ".", start))
            }

            '=' => {
                self.advance();
                Ok(self.simple(TokenKind::Equal, "=", start))
            }

            '-' => {
                if self.peek_next() == Some('>') {
                    self.advance();
                    self.advance();

                    Ok(Token::new(
                        TokenKind::Arrow,
                        "->",
                        Span::new(start, self.position()),
                    ))
                } else {
                    Err(LolError::Lexer {
                        message: "unexpected '-'".into(),
                        span: Span::new(start, start),
                    })
                }
            }

            '"' | '\'' => self.lex_string(),

            '0'..='9' => self.lex_number(),

            '#' => {
                self.skip_comment();
                self.next_token()
            }

            '\n' => {
                self.advance_line();

                Ok(Token::new(
                    TokenKind::Newline,
                    "\n",
                    Span::new(start, self.position()),
                ))
            }

            _ => {
                if Self::is_identifier_start(c) {
                    self.lex_identifier()
                } else {
                    Err(LolError::Lexer {
                        message: format!("unexpected character '{}'", c),
                        span: Span::new(start, start),
                    })
                }
            }
        }
    }

    fn simple(
        &self,
        kind: TokenKind,
        text: &str,
        start: Position,
    ) -> Token {
        Token::new(
            kind,
            text,
            Span::new(start, self.position()),
        )
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.index).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.index + 1).copied()
    }

    fn eof(&self) -> bool {
        self.index >= self.chars.len()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;

        self.index += 1;
        self.column += 1;

        Some(c)
    }

    fn advance_line(&mut self) {
        self.index += 1;
        self.line += 1;
        self.column = 1;
    }

    fn position(&self) -> Position {
        Position::new(
            self.line,
            self.column,
            self.index,
        )
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn skip_comment(&mut self) {
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }

            self.advance();
        }
    }

    fn is_identifier_start(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_identifier_continue(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_' || c == '-'
    }
      fn lex_identifier(&mut self) -> Result<Token> {
        let start = self.position();

        let mut text = String::new();

        while let Some(c) = self.peek() {
            if Self::is_identifier_continue(c) {
                text.push(c);
                self.advance();
            } else {
                break;
            }
        }

        let kind = match text.as_str() {
            "true" | "false" => TokenKind::Boolean,
            _ => TokenKind::Identifier,
        };

        Ok(Token::new(
            kind,
            text,
            Span::new(start, self.position()),
        ))
    }

    fn lex_number(&mut self) -> Result<Token> {
        let start = self.position();

        let mut text = String::new();
        let mut decimal = false;

        while let Some(c) = self.peek() {
            match c {
                '0'..='9' => {
                    text.push(c);
                    self.advance();
                }

                '.' if !decimal => {
                    decimal = true;
                    text.push('.');
                    self.advance();
                }

                '_' => {
                    self.advance();
                }

                _ => break,
            }
        }

        Ok(Token::new(
            TokenKind::Number,
            text,
            Span::new(start, self.position()),
        ))
    }

    fn lex_string(&mut self) -> Result<Token> {
        let start = self.position();

        let quote = self.advance().unwrap();

        let mut value = String::new();

        while let Some(c) = self.peek() {

            if c == quote {
                self.advance();

                return Ok(Token::new(
                    TokenKind::String,
                    value,
                    Span::new(start, self.position()),
                ));
            }

            if c == '\\' {
                self.advance();

                let escaped = match self.peek() {
                    Some('n') => '\n',
                    Some('r') => '\r',
                    Some('t') => '\t',
                    Some('"') => '"',
                    Some('\'') => '\'',
                    Some('\\') => '\\',
                    Some('0') => '\0',

                    Some(other) => other,

                    None => {
                        return Err(LolError::Lexer {
                            message: "unterminated escape sequence".into(),
                            span: Span::new(start, self.position()),
                        });
                    }
                };

                self.advance();

                value.push(escaped);

                continue;
            }

            if c == '\n' {
                return Err(LolError::Lexer {
                    message: "unterminated string literal".into(),
                    span: Span::new(start, self.position()),
                });
            }

            value.push(c);
            self.advance();
        }

        Err(LolError::Lexer {
            message: "unterminated string literal".into(),
            span: Span::new(start, self.position()),
        })
    }
}
