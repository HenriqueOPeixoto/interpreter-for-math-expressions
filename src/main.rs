mod token;
mod lex_scanner;
mod parser;
mod semantics;
mod utils {
    pub mod rpn;
}

use std::env;
use std::fs;

use lex_scanner::EOF;
use lex_scanner::LexScanner;
use token::Token;

use crate::lex_scanner::NEWLINE;
use crate::utils::rpn;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Um nome de arquivo deve ser fornecido!")
    }

    let filename = &args[1];

    println!("-> Lendo o arquivo {}...", filename);

    let file = fs::read_to_string(filename)
        .expect("Não foi possível ler o arquivo!");

    let mut content: Vec<char> = file.chars().collect();

    content.push('\0');

    println!("\n-> Iniciando análise léxica...");

    let mut lex = LexScanner {
        content: content,
        state: 0,
        pos: 0
    };

    let mut token: Token;
    let mut tokens: Vec<Token> = vec![lex.next_token()];//vec![Token { tipo: EOF, termo: "".to_string() }];
    loop {
        token = lex.next_token();
        
        if token.tipo == EOF { break; }

        tokens.push(token);
    }

    println!("Análise léxica concluída!");

/*     let parse_table = parser::prepare_parse_table();

    println!("{:?}", parse_table);

    println!("{}", parse_table[0][2]);
    println!("{}", parse_table[0][3]); */

    println!("\n-> Iniciando a análise sintática...");

    parser::parse_syntax(tokens.clone());

    println!("Análise sintática concluída!");

    println!("\n-> Convertendo entrada de valores infix para postfix...");
    let rpn_vec = rpn::shunting_yard(tokens.clone());

    println!("Conversão das operações para postfix:");
    for token in &rpn_vec {
        print!("{}, ", token.termo);
        if token.tipo == NEWLINE {
            println!();
        }
    }

    println!();

    println!("\n-> Iniciando análise semântica... Calculando o valor das expressões...");

    let result_tokens = semantics::calculate_expr_rpn(rpn_vec.clone());
    for token in &result_tokens {
        println!("{}", token.termo);
    }
    
    println!(">>> Finalizado com sucesso <<<");
    
    println!("\nSAÍDA DO PROGRAMA: ");
    let mut input_line: usize = 0;
    tokens.push(Token { tipo: EOF, termo: "$".to_string()} );
    for token in &tokens {

        print!("{}", token.termo);
        if token.tipo == NEWLINE || token.tipo == EOF {
            println!(" = {}", result_tokens[input_line].termo);
            input_line += 1;
        }
        
    }

}

