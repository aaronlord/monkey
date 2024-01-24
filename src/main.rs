#[allow(dead_code)]

extern crate monkey;

use monkey::lexer;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let input = &args[1];

        let mut lexer = lexer::new(input.to_string());

        loop {
            let token = lexer.next_token();

            println!("{:?}", token);

            if token.is_eof() {
                break;
            }
        }
    } else {
        println!("Usage: monkey <input>")
    }
}
