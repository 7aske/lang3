use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::iterator::{PeekableIterator, StringIterator};
use crate::source::SourceCodeLocation;
use crate::token::{Token, TokenKind};
use crate::util::{print_location, resolve_escape_sequence};

pub struct Lexer<'a> {
    iter: StringIterator<'a>,
    state: LexerState,
}


#[derive(Debug, Clone, PartialEq)]
pub enum LexerState {
    Ready,
    Lexing,
    Done,
}

impl Default for LexerState {
    fn default() -> Self {
        return LexerState::Ready;
    }
}

#[derive(Debug)]
pub struct LexerError {
    msg: String,
    location: Option<SourceCodeLocation>,
}

impl LexerError {
    pub fn from_indices(msg: String, text: &String, line: usize, start_char: usize, end_char: usize) -> Self {
        return LexerError {
            msg,
            location: Option::from(SourceCodeLocation::new(text.clone(), line, start_char, end_char)),
        };
    }

    pub fn from_location(msg: String, location: SourceCodeLocation) -> Self {
        return LexerError {
            msg,
            location: Some(location),
        };
    }

    pub fn invalid_escape_sequence(location: SourceCodeLocation) -> Self {
        return LexerError {
            msg: "Invalid escape sequence".to_string(),
            location: Some(location),
        };
    }
}

impl Error for LexerError {}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.location.is_some() {
            let location = self.location.as_ref().unwrap();

            print_location(&location.text, location.line, location.start_char, location.end_char);
        }

        return write!(f, "Lexer error: {}", self.msg);
    }
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a String) -> Self {
        return Lexer {
            iter: StringIterator::new(text),
            state: LexerState::default(),
        };
    }

    pub fn next_token(&mut self) -> Option<Result<Token, LexerError>> {
        if self.state == LexerState::Done {
            return None;
        }

        let c = match self.iter.peek() {
            Some(c) => c,
            None => {
                self.state = LexerState::Done;
                return None
            },
        };

        self.state = LexerState::Lexing;

        self.skip_whitespace();

        if self.is_start_of_block_comment(c) {
           self.parse_block_comment().err()?;
           return None;
        }

        if self.is_start_of_line_comment(c) {
            self.parse_line_comment().err()?;
            return None;
        }

        if self.is_start_of_string(c) {
            return Some(self.parse_string());
        }

        if self.is_start_of_char(c) {
            return Some(self.parse_char());
        }

        // TODO: parse number

        // TODO parse keyword

        let operator = self.parse_operator(c);
        if operator.is_none() {
            return Some(Err(LexerError::from_location("Invalid operator".to_string(),
                                               self.get_location())))
        }

        return Some(Ok(Token {
            kind: operator.unwrap(),
            lexeme: "".to_string(),
            line: self.iter.line(),
            start_char: self.iter.char(),
            end_char: self.iter.char(),
        }));
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.iter.peek() {
            if !c.is_whitespace() {
                break;
            }

            self._next();
        }
        {}}

    fn is_start_of_char(&self, c: char) -> bool {
        return c == '\'';
    }

    fn parse_char(&mut self) -> Result<Token, LexerError> {
        let mut string = String::new();
        let start_char = self.iter.char();
        let start_line = self.iter.line();

        self._next(); // skip the starting '

        let c = self._next().unwrap();

        if c == '\\' {
            let next = match self._next() {
                Some(c) => c,
                None => return Err(LexerError::invalid_escape_sequence(self.get_location())),
            };

            let resolved = match resolve_escape_sequence(next) {
                Some(c) => c,
                None => return Err(LexerError::invalid_escape_sequence(self.get_location())),
            };

            string.push(resolved);
        } else {
            string.push(c);
        }

        let next = self._next();
        if next.is_none() || !self.is_start_of_char(next.unwrap()) {
            let end_char = self.iter.char();
            return Err(LexerError::from_indices("Invalid char".to_string(),
                                                &self.text(),
                                                start_line,
                                                start_char,
                                                end_char));
        }

        return Ok(Token {
            kind: TokenKind::Char,
            lexeme: string.clone(),
            line: self.iter.line(),
            start_char,
            end_char: self.iter.char(),
        });
    }

    fn is_start_of_string(&self, c: char) -> bool {
        return c == '"';
    }

    fn parse_string(&mut self) -> Result<Token, LexerError> {
        let mut string = String::new();

        let start_line = self.iter.line();
        let start_char = self.iter.char();
        let mut terminated = false;

        self._next(); // skip start of string

        while let Some(c) = self._next() {
            if self.is_start_of_string(c) {
                terminated = true;
                break;
            }

            if c == '\\' {
                let next = match self._next() {
                    Some(c) => c,
                    None => return Err(LexerError::invalid_escape_sequence(self.get_location())),
                };

                let resolved = match resolve_escape_sequence(next) {
                    Some(c) => c,
                    None => return Err(LexerError::invalid_escape_sequence(self.get_location())),
                };

                string.push(resolved);
            } else {
                string.push(c);
            }
        }

        if !terminated {
            let end_char = self.iter.char();
            return Err(LexerError::from_indices("Unterminated string literal".to_string(),
                                                self.text(),
                                                start_line,
                                                start_char,
                                                end_char));
        }

        return Ok(Token {
            kind: TokenKind::String,
            lexeme: string.clone(),
            line: start_line,
            start_char,
            end_char: self.iter.char(),
        });
    }

    fn is_start_of_line_comment(&self, c: char) -> bool {
        return c == '/' && self._offset(1) == Option::from('/');
    }

    fn parse_line_comment(&mut self) -> Result<(), LexerError> {
        while let Some(c) = self._next() {
            if c == '\n' {
                break;
            }
        }
        return Ok(());
    }

    fn is_start_of_block_comment(&self, c: char) -> bool {
        return c == '/' && self._offset(1) == Option::from('*');
    }

    fn is_end_of_block_comment(&self, c: char) -> bool {
        return c == '*' && self._offset(1) == Option::from('/');
    }

    fn parse_block_comment(&mut self) -> Result<(), LexerError> {
        // Skip start of block comment
        self._skip(2);

        let mut depth = 1;

        while let Some(c) = self._next() {
            if self.is_end_of_block_comment(c) {
                self._next();
                depth -= 1;
            }

            if self.is_start_of_block_comment(c) {
                self._skip(2);
                depth += 1;
            }

            if depth == 0 {
                return Ok(());
            }
        }

        return Err(LexerError::from_location(
            "Unterminated block comment".to_string(),
            self.get_location()));
    }

    fn parse_operator(&mut self, c: char) -> Option<TokenKind> {
        self._next(); // skip c
        if c == '!' {
            return if self._peek() == Option::from('=') {
                self._next();
                Some(TokenKind::BangEqual)
            } else {
                Some(TokenKind::Bang)
            };
        }

        if c == '%' { return Some(TokenKind::Percent); }
        if c == '&' {
            return if self._peek() == Option::from('&') {
                self._next();
                Some(TokenKind::AmpersandAmpersand)
            } else {
                Some(TokenKind::Ampersand)
            };
        }

        if c == '(' {
            return Some(TokenKind::LeftParenthesis);
        }
        if c == ')' {
            return Some(TokenKind::RightParenthesis);
        }
        if c == '*' {
            return if self._peek() == Option::from('=') {
                self._next();
                Some(TokenKind::StarEqual)
            } else if self._peek() == Option::from('*') {
                self._next();
                Some(TokenKind::StarStar)
            } else {
                Some(TokenKind::Star)
            };
        }
        if c == '+' {
            return if self._peek() == Option::from('+') {
                self._next();
                Some(TokenKind::PlusPlus)
            } else if self._peek() == Option::from('=') {
                self._next();
                Some(TokenKind::PlusEqual)
            } else {
                Some(TokenKind::Plus)
            };
        }
        if c == ',' {
            return Some(TokenKind::Comma);
        }
        if c == '-' {
            return if self._peek() == Option::from('-') {
                self._next();
                Some(TokenKind::MinusMinus)
            } else if self._peek() == Option::from('=') {
                self._next();
                Some(TokenKind::MinusEqual)
            } else if self._peek() == Option::from('>') {
                self._next();
                Some(TokenKind::FatArrow)
            } else {
                Some(TokenKind::Minus)
            };
        }
        if c == '.' {
            return if self._peek() == Option::from('.') {
                self._next();
                Some(TokenKind::DotDot)
            } else {
                Some(TokenKind::Dot)
            };
        }
        if c == '/' {
            return if self._peek() == Option::from('=') {
                self._next();
                Some(TokenKind::SlashEqual)
            } else {
                Some(TokenKind::Slash)
            };
        }
        if c == ':' {
            return Some(TokenKind::Colon);
        }
        if c == ';' {
            return Some(TokenKind::Semicolon);
        }
        if c == '<' {
            return if self._peek() == Option::from('=') {
                self._next();
                Some(TokenKind::LessEqual)
            } else {
                Some(TokenKind::Less)
            };
        }
        if c == '=' {
            return if self._peek() == Option::from('=') {
                self._next();
                Some(TokenKind::EqualEqual)
            } else if self._peek() == Option::from('>') {
                self._next();
                Some(TokenKind::FatArrow)
            } else {
                Some(TokenKind::Equal)
            };
        }
        if c == '>' {
            return if self._peek() == Option::from('=') {
                self._next();
                Some(TokenKind::GreaterEqual)
            } else {
                Some(TokenKind::Greater)
            };
        }

        if c == '?' {
            return if self._peek() == Option::from('?') {
                self._next();
                Some(TokenKind::QuestionmarkQuestionmark)
            } else {
                Some(TokenKind::Questionmark)
            };
        }

        if c == '[' {
            return Some(TokenKind::LeftBracket);
        }
        if c == ']' {
            return Some(TokenKind::RightBracket);
        }
        if c == '{' {
            return Some(TokenKind::LeftBrace);
        }
        if c == '|' {
            return if self._peek() == Option::from('|') {
                self._next();
                Some(TokenKind::PipePipe)
            } else {
                Some(TokenKind::Pipe)
            };
        }
        if c == '}' {
            return Some(TokenKind::RightBrace);
        }

        return None;
    }

    #[inline(always)]
    fn _peek(&mut self) -> Option<char> {
        return self.iter.peek();
    }

    #[inline(always)]
    fn _next(&mut self) -> Option<char> {
        return self.iter.next();
    }

    fn _skip(&mut self, n: usize) {
        for _ in 0..n {
            self.iter.next();
        }
    }

    fn _offset(&self, num: usize) -> Option<char> {
        return self.iter.offset(num);
    }

    fn text(&mut self) -> &String {
        return self.iter.text();
    }

    fn get_location(&self) -> SourceCodeLocation {
        return SourceCodeLocation {
            text: self.iter.text().clone(),
            line: self.iter.line(),
            start_char: self.iter.char(),
            end_char: self.iter.char(),
        };
    }
}

#[cfg(test)]
mod lexer_tests {
    #[test]
    fn test_string_literal() {
        // given
        let code = String::from("\"Hello, World!\"");

        // when
        let mut lexer = super::Lexer::new(&code);
        let token = lexer.next_token().unwrap().unwrap();

        // then
        assert_eq!(token.kind, super::TokenKind::String);
        assert_eq!(token.lexeme, "Hello, World!");
    }

    #[test]
    fn test_string_literal_with_escape() {
        // given
        let code = String::from("\"Hello, \\\"World!\\\"\"");

        // when
        let mut lexer = super::Lexer::new(&code);
        let token = lexer.next_token().unwrap().unwrap();

        // then
        assert_eq!(token.kind, super::TokenKind::String);
        assert_eq!(token.lexeme, "Hello, \"World!\"");
    }

    #[test]
    fn test_string_literal_with_invalid_escape() {
        // given
        let code = String::from("\"Hello, \\World!\\\"\"");

        // when
        let mut lexer = super::Lexer::new(&code);
        let token = lexer.next_token();

        // then
        assert!(token.is_some());
        assert!(token.unwrap().is_err());
    }

    #[test]
    fn test_line_comment() {
        // given
        let code = String::from("// Hello, World!\n");

        // when
        let mut lexer = super::Lexer::new(&code);
        let token = lexer.next_token();

        // then
        assert!(token.is_none());
    }

    #[test]
    fn test_block_comment() {
        // given
        let code = String::from("/* Hello, World! */");

        // when
        let mut lexer = super::Lexer::new(&code);
        let token = lexer.next_token();

        // then
        assert!(token.is_none());
    }

    #[test]
    fn test_parse_operator() {
        // given
        let code = String::from("+-*/");

        // when
        let mut lexer = super::Lexer::new(&code);
        let token = lexer.next_token().unwrap().unwrap();

        // then
        assert_eq!(token.kind, super::TokenKind::Plus);

        // when
        let token = lexer.next_token().unwrap().unwrap();

        // then
        assert_eq!(token.kind, super::TokenKind::Minus);

        // when
        let token = lexer.next_token().unwrap().unwrap();

        // then
        assert_eq!(token.kind, super::TokenKind::Star);

        // when
        let token = lexer.next_token().unwrap().unwrap();

        // then
        assert_eq!(token.kind, super::TokenKind::Slash);
    }

    #[test]
    fn test_parse_char() {
        // given
        let code = String::from("'a'");

        // when
        let mut lexer = super::Lexer::new(&code);
        let token = lexer.next_token().unwrap().unwrap();

        // then
        assert_eq!(token.kind, super::TokenKind::Char);
        assert_eq!(token.lexeme, "a");
    }
}