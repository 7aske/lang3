use std::str::FromStr;


#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub start_char: usize,
    pub end_char: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Invalid,
    Super(&'static str),       // super
    Class(&'static str),       // class
    This(&'static str),        // this
    While(&'static str),       // while
    If(&'static str),          // if
    Else(&'static str),        // else
    For(&'static str),         // for
    Foreach(&'static str),     // foreach
    In(&'static str),          // in
    Continue(&'static str),    // continue
    Break(&'static str),       // break
    True(&'static str),        // true
    False(&'static str),       // false
    Null(&'static str),        // null
    Import(&'static str),      // import
    Include(&'static str),     // include
    As(&'static str),          // as
    Fn(&'static str),          // fn
    Return(&'static str),      // return
    Let(&'static str),         // let
    Const(&'static str),       // const
    Print(&'static str),       // @temporary
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

impl FromStr for TokenKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "super" => Ok(TokenKind::Super("super")),
            "class" => Ok(TokenKind::Class("class")),
            "this" => Ok(TokenKind::This("this")),
            "while" => Ok(TokenKind::While("while")),
            "if" => Ok(TokenKind::If("if")),
            "else" => Ok(TokenKind::Else("else")),
            "for" => Ok(TokenKind::For("for")),
            "foreach" => Ok(TokenKind::Foreach("foreach")),
            "in" => Ok(TokenKind::In("in")),
            "continue" => Ok(TokenKind::Continue("continue")),
            "break" => Ok(TokenKind::Break("break")),
            "true" => Ok(TokenKind::True("true")),
            "false" => Ok(TokenKind::False("false")),
            "null" => Ok(TokenKind::Null("null")),
            "import" => Ok(TokenKind::Import("import")),
            "include" => Ok(TokenKind::Include("include")),
            "as" => Ok(TokenKind::As("as")),
            "fn" => Ok(TokenKind::Fn("fn")),
            "return" => Ok(TokenKind::Return("return")),
            "let" => Ok(TokenKind::Let("let")),
            "const" => Ok(TokenKind::Const("const")),
            "print" => Ok(TokenKind::Print("print")),
            _ => Ok(TokenKind::Identifier),
        }
    }
}