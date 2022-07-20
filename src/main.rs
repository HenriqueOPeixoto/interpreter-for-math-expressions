mod token;
mod lex_scanner;
mod parser;

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

    let mut content: Vec<char> = file.chars().collect();

    content.push('\0');

    println!("{:?}", content);

    let mut lex = LexScanner {
        content: content,
        state: 0,
        pos: 0
    };

    let mut token: Token;
    let mut tokens: Vec<Token> = vec![lex.next_token()];//vec![Token { tipo: EOF, termo: "".to_string() }];
    loop {
        token = lex.next_token();
        
        token.to_string();
        if token.tipo == EOF { break; }

        tokens.push(token);
    }

    let parse_table = parser::prepare_parse_table();

    println!("{:?}", parse_table);

    println!("{}", parse_table[0][2]);
    println!("{}", parse_table[0][3]);

    println!("{}", parser::parse_syntax(tokens));

}

