#[allow(dead_code)]

#[derive(Debug, PartialEq)]
pub enum Name {
    // Special
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT(String),
    INT(i64),

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    // Comparison
    EQ,
    NOTEQ,
    LT,
    GT,

    // Delimiters
    COMMA,
    SEMICOLON,

    // Braces
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

#[derive(Debug)]
pub struct Literal {
    pub chars: Vec<u8>,
}

impl Literal {
    pub fn new(chars: Vec<u8>) -> Literal {
        Literal {
            chars,
        }
    }

    pub fn to_string(&self) -> String {
        String::from_utf8(self.chars.clone()).unwrap()
    }
}

#[derive(Debug)]
pub struct Token {
    pub name: Name,
    pub literal: Literal,
}

impl Token {
    pub fn new(name: Name, literal: Literal) -> Token {
        Token {
            name,
            literal,
        }
    }

    pub fn lookup_ident(literal: &Literal) -> Name {
        let string = literal.to_string();

        match string.as_str() {
            "fn" => Name::FUNCTION,
            "let" => Name::LET,
            "true" => Name::TRUE,
            "false" => Name::FALSE,
            "if" => Name::IF,
            "else" => Name::ELSE,
            "return" => Name::RETURN,
            _ => Name::IDENT(string),
        }
    }

    pub fn int_literal(literal: &Literal) -> Name {
        let string = literal.to_string();

        match string.parse::<i64>() {
            Ok(int) => Name::INT(int),
            Err(_) => Name::ILLEGAL,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.name == Name::EOF
    }
}
