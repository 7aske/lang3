use std::fmt::{Debug, Display, Formatter};
use std::iter::{Iterator};
use std::str::FromStr;
use phf::{phf_map, Map};


#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub start_char: usize,
    pub end_char: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TokenKind {
    Invalid,
    Super,                     // super
    Class,                     // class
    This,                      // this
    While,                     // while
    If,                        // if
    Else,                      // else
    For,                       // for
    Foreach,                   // foreach
    In,                        // in
    Continue,                  // continue
    Break,                     // break
    True,                      // true
    False,                     // false
    Null,                      // null
    Import,                    // import
    Include,                   // include
    As,                        // as
    Fn,                        // fn
    Return,                    // return
    Let,                       // let
    Const,                     // const
    Print,                     // @temporary
    FatArrow,                  // =>
    ThinArrow,                 // ->
    Equal,                     // =
    QuestionmarkQuestionmark,  // ??
    Questionmark,              // ?
    Colon,                     // :
    Plus,                      // +
    Minus,                     // -
    Slash,                     // /
    Star,                      // *
    StarStar,                  // **
    Percent,                   // %
    Ampersand,                 // &
    AmpersandAmpersand,        // &&
    Caret,                     // ^
    Pipe,                      // |
    PipePipe,                  // ||
    Bang,                      // !
    EqualEqual,                // ==
    BangEqual,                 // !=
    GreaterEqual,              // >=
    LessEqual,                 // <=
    Greater,                   // >
    Less,                      // <
    LessLess,                  // <<
    GreaterGreater,            // >>
    Tilde,                     // ~
    PlusPlus,                  // ++
    MinusMinus,                // --
    MinusEqual,                // -=
    PlusEqual,                 // +=
    StarEqual,                 // *=
    SlashEqual,                // /=
    Dot,                       // .
    DotDot,                    // ..
    Comma,                     // ,
    Semicolon,                 // ;
    LeftParenthesis,           // (
    RightParenthesis,          // )
    LeftBrace,                 // {
    RightBrace,                // }
    LeftBracket,               // [
    RightBracket,              // ]
    Identifier,
    String,
    Char,
    Integer,
    Float,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TokenKind::Invalid => "<invalid>",
            TokenKind::Super => "super",
            TokenKind::Class => "class",
            TokenKind::This => "this",
            TokenKind::While => "while",
            TokenKind::If => "if",
            TokenKind::Else => "else",
            TokenKind::For => "for",
            TokenKind::Foreach => "foreach",
            TokenKind::In => "in",
            TokenKind::Continue => "continue",
            TokenKind::Break => "break",
            TokenKind::True => "true",
            TokenKind::False => "false",
            TokenKind::Null => "null",
            TokenKind::Import => "import",
            TokenKind::Include => "include",
            TokenKind::As => "as",
            TokenKind::Fn => "fn",
            TokenKind::Return => "return",
            TokenKind::Let => "let",
            TokenKind::Const => "const",
            TokenKind::Print => "@temporary",
            TokenKind::FatArrow => "=>",
            TokenKind::ThinArrow => "->",
            TokenKind::Equal => "=",
            TokenKind::QuestionmarkQuestionmark => "??",
            TokenKind::Questionmark => "?",
            TokenKind::Colon => ":",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Slash => "/",
            TokenKind::Star => "*",
            TokenKind::StarStar => "**",
            TokenKind::Percent => "%",
            TokenKind::Ampersand => "&",
            TokenKind::AmpersandAmpersand => "&&",
            TokenKind::Caret => "^",
            TokenKind::Pipe => "|",
            TokenKind::PipePipe => "||",
            TokenKind::Bang => "!",
            TokenKind::EqualEqual => "==",
            TokenKind::BangEqual => "!=",
            TokenKind::GreaterEqual => ">=",
            TokenKind::LessEqual => "<=",
            TokenKind::Greater => ">",
            TokenKind::Less => "<",
            TokenKind::LessLess => "<<",
            TokenKind::GreaterGreater => ">>",
            TokenKind::Tilde => "~",
            TokenKind::PlusPlus => "++",
            TokenKind::MinusMinus => "--",
            TokenKind::MinusEqual => "-=",
            TokenKind::PlusEqual => "+=",
            TokenKind::StarEqual => "*=",
            TokenKind::SlashEqual => "/=",
            TokenKind::Dot => ".",
            TokenKind::DotDot => "..",
            TokenKind::Comma => ",",
            TokenKind::Semicolon => ";",
            TokenKind::LeftParenthesis => "(",
            TokenKind::RightParenthesis => ")",
            TokenKind::LeftBrace => "{",
            TokenKind::RightBrace => "}",
            TokenKind::LeftBracket => "[",
            TokenKind::RightBracket => "]",
            TokenKind::Identifier => "<identifier>",
            TokenKind::String => "<string>",
            TokenKind::Char => "<char>",
            TokenKind::Integer => "<integer>",
            TokenKind::Float => "<float>",
        };

        write!(f, "{}", str)
    }
}

const TOKEN_KIND_MAP: Map<&'static str, TokenKind> = phf_map! {
    "super" => TokenKind::Super,
    "class" => TokenKind::Class,
    "this" => TokenKind::This,
    "while" => TokenKind::While,
    "if" => TokenKind::If,
    "else" => TokenKind::Else,
    "for" => TokenKind::For,
    "foreach" => TokenKind::Foreach,
    "in" => TokenKind::In,
    "continue" => TokenKind::Continue,
    "break" => TokenKind::Break,
    "true" => TokenKind::True,
    "false" => TokenKind::False,
    "null" => TokenKind::Null,
    "import" => TokenKind::Import,
    "include" => TokenKind::Include,
    "as" => TokenKind::As,
    "fn" => TokenKind::Fn,
    "return" => TokenKind::Return,
    "let" => TokenKind::Let,
    "const" => TokenKind::Const,
    "print" => TokenKind::Print,
    "=>" => TokenKind::FatArrow,
    "->" => TokenKind::ThinArrow,
    "=" => TokenKind::Equal,
    "??" => TokenKind::QuestionmarkQuestionmark,
    "?" => TokenKind::Questionmark,
    ":" => TokenKind::Colon,
    "+" => TokenKind::Plus,
    "-" => TokenKind::Minus,
    "/" => TokenKind::Slash,
    "*" => TokenKind::Star,
    "**" => TokenKind::StarStar,
    "%" => TokenKind::Percent,
    "&" => TokenKind::Ampersand,
    "&&" => TokenKind::AmpersandAmpersand,
    "^" => TokenKind::Caret,
    "|" => TokenKind::Pipe,
    "||" => TokenKind::PipePipe,
    "!" => TokenKind::Bang,
    "==" => TokenKind::EqualEqual,
    "!=" => TokenKind::BangEqual,
    ">=" => TokenKind::GreaterEqual,
    "<=" => TokenKind::LessEqual,
    ">" => TokenKind::Greater,
    "<" => TokenKind::Less,
    "<<" => TokenKind::LessLess,
    ">>" => TokenKind::GreaterGreater,
    "~" => TokenKind::Tilde,
    "++" => TokenKind::PlusPlus,
    "--" => TokenKind::MinusMinus,
    "-=" => TokenKind::MinusEqual,
    "+=" => TokenKind::PlusEqual,
    "*=" => TokenKind::StarEqual,
    "/=" => TokenKind::SlashEqual,
    "." => TokenKind::Dot,
    ".." => TokenKind::DotDot,
    "," => TokenKind::Comma,
    ";" => TokenKind::Semicolon,
    "(" => TokenKind::LeftParenthesis,
    ")" => TokenKind::RightParenthesis,
    "{" => TokenKind::LeftBrace,
    "}" => TokenKind::RightBrace,
    "[" => TokenKind::LeftBracket,
    "]" => TokenKind::RightBracket,
};

impl FromStr for TokenKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TOKEN_KIND_MAP.get(s).copied().ok_or(())
    }
}

impl TokenKind {
    pub fn to_str(&self) -> &'static str {
        return TOKEN_KIND_MAP.entries()
            .find(|&v| v.1 == self)
            .unwrap()
            .0;
    }

    pub fn parse_operator(c: char, c1: Option<char>) -> Option<Self> {
        if c == '!' {
            return if c1 == Option::from('=') {
                Some(TokenKind::BangEqual)
            } else {
                Some(TokenKind::Bang)
            };
        }

        if c == '%' {
            return Some(TokenKind::Percent);
        }

        if c == '&' {
            return if c1 == Option::from('&') {
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
            return if c1 == Option::from('=') {
                Some(TokenKind::StarEqual)
            } else if c1 == Option::from('*') {
                Some(TokenKind::StarStar)
            } else {
                Some(TokenKind::Star)
            };
        }
        if c == '+' {
            return if c1 == Option::from('+') {
                Some(TokenKind::PlusPlus)
            } else if c1 == Option::from('=') {
                Some(TokenKind::PlusEqual)
            } else {
                Some(TokenKind::Plus)
            };
        }
        if c == ',' {
            return Some(TokenKind::Comma);
        }
        if c == '-' {
            return if c1 == Option::from('-') {
                Some(TokenKind::MinusMinus)
            } else if c1 == Option::from('=') {
                Some(TokenKind::MinusEqual)
            } else if c1 == Option::from('>') {
                Some(TokenKind::FatArrow)
            } else {
                Some(TokenKind::Minus)
            };
        }
        if c == '.' {
            return if c1 == Option::from('.') {
                Some(TokenKind::DotDot)
            } else {
                Some(TokenKind::Dot)
            };
        }
        if c == '/' {
            return if c1 == Option::from('=') {
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
            return if c1 == Option::from('=') {
                Some(TokenKind::LessEqual)
            } else {
                Some(TokenKind::Less)
            };
        }
        if c == '=' {
            return if c1 == Option::from('=') {
                Some(TokenKind::EqualEqual)
            } else if c1 == Option::from('>') {
                Some(TokenKind::FatArrow)
            } else {
                Some(TokenKind::Equal)
            };
        }
        if c == '>' {
            return if c1 == Option::from('=') {
                Some(TokenKind::GreaterEqual)
            } else {
                Some(TokenKind::Greater)
            };
        }

        if c == '?' {
            return if c1 == Option::from('?') {
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
            return if c1 == Option::from('|') {
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
}