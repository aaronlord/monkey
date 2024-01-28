#[allow(dead_code)]
extern crate monkey;

use monkey::lexer::Lexer;
use monkey::parser::Parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let input = &args[1];

        let mut lexer = Lexer::new(input.to_string());

        loop {
            let token = lexer.next_token();

            println!("{:?}", token.name);

            if token.is_eof() {
                break;
            }
        }
    } else {
        let input = String::from(
            "
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ",
        );

        let lexer = Lexer::new(input);

        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        println!("{:?}", program);
    }
}
