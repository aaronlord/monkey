mod token;

use crate::lexer::token::{Token, Name, Literal};

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let literal = Literal::new(vec![self.ch]);

        let token = match self.ch {
            0 => Token::new(Name::EOF, literal),
            b'=' => match self.peek_char() {
                b'=' => Token::new(Name::EQ, Literal::new(vec![
                    self.ch,
                    self.read_char()
                ])),
                _ => Token::new(Name::ASSIGN, literal)
            },
            b'+' => Token::new(Name::PLUS, literal),
            b'-' => Token::new(Name::MINUS, literal),
            b'!' => match self.peek_char() {
                b'=' => Token::new(Name::NOTEQ, Literal::new(vec![
                    self.ch,
                    self.read_char()
                ])),
                _ => Token::new(Name::BANG, literal),
            },
            b'*' => Token::new(Name::ASTERISK, literal),
            b'/' => Token::new(Name::SLASH, literal),
            b'<' => Token::new(Name::LT, literal),
            b'>' => Token::new(Name::GT, literal),
            b'(' => Token::new(Name::LPAREN, literal),
            b')' => Token::new(Name::RPAREN, literal),
            b'{' => Token::new(Name::LBRACE, literal),
            b'}' => Token::new(Name::RBRACE, literal),
            b',' => Token::new(Name::COMMA, literal),
            b';' => Token::new(Name::SEMICOLON, literal),
            _ => match (is_letter(self.ch), is_digit(self.ch)) {
                (true, _) => {
                    let literal = self.read_chars(|ch| is_letter(ch));
                    let name = Token::lookup_ident(&literal);

                    return Token::new(name, literal);
                },
                (_, true) => {
                    let literal = self.read_chars(|ch| is_digit(ch));
                    let name = Token::int_literal(&literal);

                    return Token::new(name, literal);
                },
                (_, _) => Token::new(Name::ILLEGAL, literal),
            },
        };

        self.read_char();

        return token;
    }

    fn read_char(&mut self) -> u8 {
        self.ch = match self.read_position >= self.input.len() {
            true => 0,
            false => self.input.as_bytes()[self.read_position],
        };

        self.position = self.read_position;
        self.read_position += 1;

        return self.ch;
    }

    // Read chars until `f` returns false.
    fn read_chars<F>(&mut self, f: F) -> Literal
        where F: Fn(u8) -> bool
    {
        let mut vec = vec![];

        while f(self.ch) {
            vec.push(self.ch);

            self.read_char();
        }

        return Literal::new(vec);
    }

    fn peek_char(&mut self) -> u8 {
        return match self.read_position >= self.input.len() {
            true => 0,
            false => self.input.as_bytes()[self.read_position],
        };
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }
}

pub fn new(input: String) -> Lexer {
    let mut lexer = Lexer {
        input,
        position: 0,
        read_position: 0,
        ch: 0,
    };

    lexer.read_char();

    return lexer;
}

fn is_letter(ch: u8) -> bool {
    return b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_';
}

fn is_digit(ch: u8) -> bool {
    return b'0' <= ch && ch <= b'9';
}

#[cfg(test)]
mod tests {
    #[test]
    fn next_token() {
        let input = String::from("=+(){},;");

        let mut lexer = super::new(input);

        assert_eq!(super::token::Name::ASSIGN, lexer.next_token().name);
        assert_eq!(super::token::Name::PLUS, lexer.next_token().name);
        assert_eq!(super::token::Name::LPAREN, lexer.next_token().name);
        assert_eq!(super::token::Name::RPAREN, lexer.next_token().name);
        assert_eq!(super::token::Name::LBRACE, lexer.next_token().name);
        assert_eq!(super::token::Name::RBRACE, lexer.next_token().name);
        assert_eq!(super::token::Name::COMMA, lexer.next_token().name);
        assert_eq!(super::token::Name::SEMICOLON, lexer.next_token().name);
        assert_eq!(super::token::Name::EOF, lexer.next_token().name);
    }

    #[test]
    fn illegal_next_token() {
        let input = String::from("#");

        let mut lexer = super::new(input);

        assert_eq!(super::token::Name::ILLEGAL, lexer.next_token().name);
        assert_eq!(super::token::Name::EOF, lexer.next_token().name);
    }

    #[test]
    fn next_token_with_whitespace() {
        let input = String::from("
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
              x + y;
            };

            let result = add(five, ten);
        ");

        let mut lexer = super::new(input);

        assert_eq!(super::token::Name::LET, lexer.next_token().name);
        assert_eq!(super::token::Name::IDENT(String::from("five")), lexer.next_token().name);
        assert_eq!(super::token::Name::ASSIGN, lexer.next_token().name);
        assert_eq!(super::token::Name::INT(5), lexer.next_token().name);
        assert_eq!(super::token::Name::SEMICOLON, lexer.next_token().name);
        assert_eq!(super::token::Name::LET, lexer.next_token().name);
        assert_eq!(super::token::Name::IDENT(String::from("ten")), lexer.next_token().name);
        assert_eq!(super::token::Name::ASSIGN, lexer.next_token().name);
        assert_eq!(super::token::Name::INT(10), lexer.next_token().name);
        assert_eq!(super::token::Name::SEMICOLON, lexer.next_token().name);
        assert_eq!(super::token::Name::LET, lexer.next_token().name);
        assert_eq!(super::token::Name::IDENT(String::from("add")), lexer.next_token().name);
        assert_eq!(super::token::Name::ASSIGN, lexer.next_token().name);
        assert_eq!(super::token::Name::FUNCTION, lexer.next_token().name);
        assert_eq!(super::token::Name::LPAREN, lexer.next_token().name);
        assert_eq!(super::token::Name::IDENT(String::from("x")), lexer.next_token().name);
        assert_eq!(super::token::Name::COMMA, lexer.next_token().name);
        assert_eq!(super::token::Name::IDENT(String::from("y")), lexer.next_token().name);
        assert_eq!(super::token::Name::RPAREN, lexer.next_token().name);
        assert_eq!(super::token::Name::LBRACE, lexer.next_token().name);
        assert_eq!(super::token::Name::IDENT(String::from("x")), lexer.next_token().name);
        assert_eq!(super::token::Name::PLUS, lexer.next_token().name);
        assert_eq!(super::token::Name::IDENT(String::from("y")), lexer.next_token().name);
        assert_eq!(super::token::Name::SEMICOLON, lexer.next_token().name);
        assert_eq!(super::token::Name::RBRACE, lexer.next_token().name);
        assert_eq!(super::token::Name::SEMICOLON, lexer.next_token().name);
        assert_eq!(super::token::Name::LET, lexer.next_token().name);
        assert_eq!(super::token::Name::IDENT(String::from("result")), lexer.next_token().name);
        assert_eq!(super::token::Name::ASSIGN, lexer.next_token().name);
        assert_eq!(super::token::Name::IDENT(String::from("add")), lexer.next_token().name);
        assert_eq!(super::token::Name::LPAREN, lexer.next_token().name);
        assert_eq!(super::token::Name::IDENT(String::from("five")), lexer.next_token().name);
        assert_eq!(super::token::Name::COMMA, lexer.next_token().name);
        assert_eq!(super::token::Name::IDENT(String::from("ten")), lexer.next_token().name);
        assert_eq!(super::token::Name::RPAREN, lexer.next_token().name);
        assert_eq!(super::token::Name::SEMICOLON, lexer.next_token().name);
        assert_eq!(super::token::Name::EOF, lexer.next_token().name);
    }

    #[test]
    fn next_token_with_gibberish() {
        let input = String::from("
            !-/*5;
            5 < 10 > 5;
        ");

        let mut lexer = super::new(input);

        assert_eq!(super::token::Name::BANG, lexer.next_token().name);
        assert_eq!(super::token::Name::MINUS, lexer.next_token().name);
        assert_eq!(super::token::Name::SLASH, lexer.next_token().name);
        assert_eq!(super::token::Name::ASTERISK, lexer.next_token().name);
        assert_eq!(super::token::Name::INT(5), lexer.next_token().name);
        assert_eq!(super::token::Name::SEMICOLON, lexer.next_token().name);
        assert_eq!(super::token::Name::INT(5), lexer.next_token().name);
        assert_eq!(super::token::Name::LT, lexer.next_token().name);
        assert_eq!(super::token::Name::INT(10), lexer.next_token().name);
        assert_eq!(super::token::Name::GT, lexer.next_token().name);
        assert_eq!(super::token::Name::INT(5), lexer.next_token().name);
        assert_eq!(super::token::Name::SEMICOLON, lexer.next_token().name);
        assert_eq!(super::token::Name::EOF, lexer.next_token().name);
    }

    #[test]
    fn next_token_with_conditions() {
        let input = String::from("
            if (5 < 10) {
                return true;
            } else {
                return false;
            }
        ");

        let mut lexer = super::new(input);

        assert_eq!(super::token::Name::IF, lexer.next_token().name);
        assert_eq!(super::token::Name::LPAREN, lexer.next_token().name);
        assert_eq!(super::token::Name::INT(5), lexer.next_token().name);
        assert_eq!(super::token::Name::LT, lexer.next_token().name);
        assert_eq!(super::token::Name::INT(10), lexer.next_token().name);
        assert_eq!(super::token::Name::RPAREN, lexer.next_token().name);
        assert_eq!(super::token::Name::LBRACE, lexer.next_token().name);
        assert_eq!(super::token::Name::RETURN, lexer.next_token().name);
        assert_eq!(super::token::Name::TRUE, lexer.next_token().name);
        assert_eq!(super::token::Name::SEMICOLON, lexer.next_token().name);
        assert_eq!(super::token::Name::RBRACE, lexer.next_token().name);
        assert_eq!(super::token::Name::ELSE, lexer.next_token().name);
        assert_eq!(super::token::Name::LBRACE, lexer.next_token().name);
        assert_eq!(super::token::Name::RETURN, lexer.next_token().name);
        assert_eq!(super::token::Name::FALSE, lexer.next_token().name);
        assert_eq!(super::token::Name::SEMICOLON, lexer.next_token().name);
        assert_eq!(super::token::Name::RBRACE, lexer.next_token().name);
        assert_eq!(super::token::Name::EOF, lexer.next_token().name);
    }

    #[test]
    fn next_token_with_two_character_operators() {
        let input = String::from("
            10 == 10;
            10 != 9;
        ");

        let mut lexer = super::new(input);

        assert_eq!(super::token::Name::INT(10), lexer.next_token().name);
        assert_eq!(super::token::Name::EQ, lexer.next_token().name);
        assert_eq!(super::token::Name::INT(10), lexer.next_token().name);
        assert_eq!(super::token::Name::SEMICOLON, lexer.next_token().name);
        assert_eq!(super::token::Name::INT(10), lexer.next_token().name);
        assert_eq!(super::token::Name::NOTEQ, lexer.next_token().name);
        assert_eq!(super::token::Name::INT(9), lexer.next_token().name);
        assert_eq!(super::token::Name::SEMICOLON, lexer.next_token().name);
        assert_eq!(super::token::Name::EOF, lexer.next_token().name);
    }

    #[test]
    fn is_letter() {
        assert_eq!(true, super::is_letter(b'a'));
        assert_eq!(true, super::is_letter(b'b'));
        assert_eq!(true, super::is_letter(b'c'));
        assert_eq!(true, super::is_letter(b'y'));
        assert_eq!(true, super::is_letter(b'x'));
        assert_eq!(true, super::is_letter(b'z'));
        assert_eq!(true, super::is_letter(b'A'));
        assert_eq!(true, super::is_letter(b'B'));
        assert_eq!(true, super::is_letter(b'C'));
        assert_eq!(true, super::is_letter(b'X'));
        assert_eq!(true, super::is_letter(b'Y'));
        assert_eq!(true, super::is_letter(b'Z'));
        assert_eq!(true, super::is_letter(b'_'));
        assert_eq!(false, super::is_letter(b'0'));
        assert_eq!(false, super::is_letter(b'9'));
    }

    #[test]
    fn is_digit() {
        assert_eq!(true, super::is_digit(b'0'));
        assert_eq!(true, super::is_digit(b'1'));
        assert_eq!(true, super::is_digit(b'2'));
        assert_eq!(true, super::is_digit(b'3'));
        assert_eq!(true, super::is_digit(b'4'));
        assert_eq!(true, super::is_digit(b'5'));
        assert_eq!(true, super::is_digit(b'6'));
        assert_eq!(true, super::is_digit(b'7'));
        assert_eq!(true, super::is_digit(b'8'));
        assert_eq!(true, super::is_digit(b'9'));
        assert_eq!(false, super::is_digit(b'a'));
        assert_eq!(false, super::is_digit(b'z'));
        assert_eq!(false, super::is_digit(b'A'));
        assert_eq!(false, super::is_digit(b'Z'));
        assert_eq!(false, super::is_digit(b'_'));
    }
}
