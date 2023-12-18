use lazy_static::lazy_static;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and", TokenType::And);
        m.insert("class", TokenType::Class);
        m.insert("else", TokenType::Else);
        m.insert("false", TokenType::False);
        m.insert("for", TokenType::For);
        m.insert("fun", TokenType::Fun);
        m.insert("if", TokenType::If);
        m.insert("nil", TokenType::Nil);
        m.insert("or", TokenType::Or);
        m.insert("print", TokenType::Print);
        m.insert("return", TokenType::Return);
        m.insert("super", TokenType::Super);
        m.insert("this", TokenType::This);
        m.insert("true", TokenType::True);
        m.insert("var", TokenType::Var);
        m.insert("while", TokenType::While);
        m
    };
}

#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug)]
pub enum Literal {
    Empty,
    String(String),
    Float(f64),
}

#[derive(Debug)]
pub struct Token {
    r#type: TokenType,
    source: String,
    literal: Literal,
    line: usize,
}

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    chars: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            chars: source.chars().peekable(),
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        // TODO: use add_token
        self.tokens.push(Token {
            r#type: TokenType::Eof,
            source: String::from(""),
            literal: Literal::Empty,
            line: self.line,
        });

        self.tokens
    }

    pub fn scan_token(&mut self) {
        let c = self.advance().unwrap(); // TODO: handle None
        match c {
            '(' => self.add_token(TokenType::LeftParen, Literal::Empty),
            ')' => self.add_token(TokenType::RightParen, Literal::Empty),
            '{' => self.add_token(TokenType::LeftBrace, Literal::Empty),
            '}' => self.add_token(TokenType::RightBrace, Literal::Empty),
            ',' => self.add_token(TokenType::Comma, Literal::Empty),
            '.' => self.add_token(TokenType::Dot, Literal::Empty),
            '-' => self.add_token(TokenType::Minus, Literal::Empty),
            '+' => self.add_token(TokenType::Plus, Literal::Empty),
            ';' => self.add_token(TokenType::Semicolon, Literal::Empty),
            '*' => self.add_token(TokenType::Star, Literal::Empty),

            // Multiple char operators
            '!' => {
                let is_match = self.r#match('=');
                self.add_token(
                    if is_match {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    },
                    Literal::Empty,
                )
            }
            '=' => {
                let is_match = self.r#match('=');
                self.add_token(
                    if is_match {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    },
                    Literal::Empty,
                )
            }
            '<' => {
                let is_match = self.r#match('=');
                self.add_token(
                    if is_match {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    },
                    Literal::Empty,
                )
            }
            '>' => {
                let is_match = self.r#match('=');
                self.add_token(
                    if is_match {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    },
                    Literal::Empty,
                )
            }
            '/' => {
                if self.r#match('/') {
                    // this is a comment, ignore the rest of the line
                    while let Some(c) = self.peek() {
                        // We don't need to check for isAtEnd because peek returns None when we're at the end.
                        if c == '\n' {
                            break;
                        };
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, Literal::Empty);
                }
            }

            // Ignore whitespaces
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,

            // String literals
            '"' => {
                self.handle_string();
            }

            c => {
                if c.is_numeric() {
                    self.handle_number();
                } else if c.is_alphabetic() {
                    self.handle_identifier();
                } else {
                    // panic!("Unexpected character")
                }
            }
        }
    }

    fn handle_string(&mut self) {
        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            }
            if c == '\n' {
                self.line += 1
            }
            self.advance();
        }

        if self.is_at_end() {
            panic!("Unterminated string")
        }

        // Ending `"`
        self.advance();

        let slice_start = self.start + 1;
        let slice_end = self.current - 1;
        let value = &self.source[slice_start..slice_end];

        self.add_token(TokenType::String, Literal::String(String::from(value)))
    }

    fn handle_number(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_numeric() {
                break;
            }
            self.advance();
        }

        if let Some('.') = self.peek() {
            todo!("Numbers with decimals need a peekable with 2 or more lookahead items");
        }

        let str_literal = &self.source[self.start..self.current];
        self.add_token(
            TokenType::Number,
            Literal::Float(str_literal.parse::<f64>().unwrap()), // TODO: handle error
        )
    }

    fn handle_identifier(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_alphanumeric() {
                break;
            }
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let keyword_type = KEYWORDS.get(text);

        match keyword_type {
            // Keyword
            Some(token_type) => self.add_token(*token_type, Literal::Empty),

            // Identifier
            None => {
                self.add_token(TokenType::Identifier, Literal::Empty);
            }
        }
    }

    fn r#match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        // Unwrapping is safe because we check for is_at_end before this line.
        if self.peek().unwrap() != expected {
            return false;
        }

        self.advance();
        return true;
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().is_none()
    }

    fn peek(&mut self) -> Option<char> {
        match self.chars.peek() {
            Some(c) => Some(*c),
            None => None,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let next = self.chars.next()?;
        self.current += 1;
        Some(next)
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text = String::from(&self.source[self.start..self.current]);
        self.tokens.push(Token {
            r#type: token_type,
            source: text,
            literal,
            line: self.line,
        })
    }
}
