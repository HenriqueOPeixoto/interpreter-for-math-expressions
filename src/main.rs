mod token;
mod lex_scanner;

use std::env;
use std::fs;

use lex_scanner::EOF;
use lex_scanner::LexScanner;
use token::Token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Um nome de arquivo deve ser fornecido!")
    }

    let filename = &args[1];

    let file = fs::read_to_string(filename)
        .expect("Não foi possível ler o arquivo!");

    let content = file.chars().collect();

    let mut lex = LexScanner {
        content: content,
        state: 0,
        pos: 0
    };

    let mut token: Token;
    loop {
        token = lex.next_token();
        
        if token.tipo == EOF { break; }
    }
    

}

