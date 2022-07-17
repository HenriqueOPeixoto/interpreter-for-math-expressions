use crate::{token::Token, lex_scanner::EOF, lex_scanner::DIGIT};

use std::collections::HashMap;

// Regras
const E: usize = 0;
const T: usize = 1;
const P: usize = 2;
const F: usize = 3;
const E1: usize = 4;
const T1: usize = 5;
const P1: usize = 6;

// Terminais
const EXP: usize= 0;
const OPEN_PAR: usize = 1;
const ID: usize = 2;
const SUM: usize = 3;
const END_OF_STACK: usize = 4;
const SUB: usize = 5;
const MUL: usize = 6;
const DIV: usize = 7;
const POW: usize = 8;
const CLOSE_PAR: usize = 10;

// TODO: mapear os tokens com relação aos terminais

pub fn parse_syntax(mut tokens: Vec<Token>) -> bool {

    let parse_table = prepare_parse_table();
    
    let mut pos = tokens.len() - 1;
    let mut stack: Vec<&str> = vec!["$", "E"];

    println!("{:?}", stack);
    
    tokens.reverse();
    
    let final_token: Token = Token { tipo: EOF, termo: "$".to_string() };

    while tokens[pos].termo != final_token.termo {

        if tokens[pos].tipo == DIGIT {
            match stack.last().expect("Erro ao ler topo da pilha!").as_ref() { // gets &str from string
                "E" => {
                    stack.pop();
                    stack.push(parse_table[E][ID]);
                },
                _ => todo!()
            }
        }

        pos -= 1;
    }

    

    true
}

pub fn prepare_parse_table() -> Vec<Vec<&'static str>> {
        // let parse_table = HashMap::from([
        //     ("E".to_string(), "exp".to_string()),
        //     ("E".to_string(), "(".to_string()),
        //     ("E".to_string(), "id".to_string()),
        //     ("E".to_string(), "+".to_string()),
        //     ("E".to_string(), "$".to_string()),
        //     ("E".to_string(), "-".to_string()),
        //     ("E".to_string(), "*".to_string()),
        //     ("E".to_string(), "/".to_string()),
        //     ("E".to_string(), "^".to_string()),
        //     ("E".to_string(), ")".to_string()),
        //     ("T".to_string(), "exp".to_string()),
        //     ("T".to_string(), "(".to_string()),
        //     ("T".to_string(), "id".to_string()),
        //     ("T".to_string(), "+".to_string()),
        //     ("T".to_string(), "$".to_string()),
        //     ("T".to_string(), "-".to_string()),
        //     ("T".to_string(), "*".to_string()),
        //     ("T".to_string(), "/".to_string()),
        //     ("T".to_string(), "^".to_string()),
        //     ("T".to_string(), ")".to_string()),
        //     ("P".to_string(), "exp".to_string()),
        //     ("P".to_string(), "(".to_string()),
        //     ("P".to_string(), "id".to_string()),
        //     ("P".to_string(), "+".to_string()),
        //     ("P".to_string(), "$".to_string()),
        //     ("P".to_string(), "-".to_string()),
        //     ("P".to_string(), "*".to_string()),
        //     ("P".to_string(), "/".to_string()),
        //     ("P".to_string(), "^".to_string()),
        //     ("P".to_string(), ")".to_string()),
        //     ("F".to_string(), "exp".to_string()),
        //     ("F".to_string(), "(".to_string()),
        //     ("F".to_string(), "id".to_string()),
        //     ("F".to_string(), "+".to_string()),
        //     ("F".to_string(), "$".to_string()),
        //     ("F".to_string(), "-".to_string()),
        //     ("F".to_string(), "*".to_string()),
        //     ("F".to_string(), "/".to_string()),
        //     ("F".to_string(), "^".to_string()),
        //     ("F".to_string(), ")".to_string()),
        //     ("E1".to_string(), "exp".to_string()),
        //     ("E1".to_string(), "(".to_string()),
        //     ("E1".to_string(), "id".to_string()),
        //     ("E1".to_string(), "+".to_string()),
        //     ("E1".to_string(), "$".to_string()),
        //     ("E1".to_string(), "-".to_string()),
        //     ("E1".to_string(), "*".to_string()),
        //     ("E1".to_string(), "/".to_string()),
        //     ("E1".to_string(), "^".to_string()),
        //     ("E1".to_string(), ")".to_string()),
        //     ("T1".to_string(), "exp".to_string()),
        //     ("T1".to_string(), "(".to_string()),
        //     ("T1".to_string(), "id".to_string()),
        //     ("T1".to_string(), "+".to_string()),
        //     ("T1".to_string(), "$".to_string()),
        //     ("T1".to_string(), "-".to_string()),
        //     ("T1".to_string(), "*".to_string()),
        //     ("T1".to_string(), "/".to_string()),
        //     ("T1".to_string(), "^".to_string()),
        //     ("T1".to_string(), ")".to_string()),
        //     ("P1".to_string(), "exp".to_string()),
        //     ("P1".to_string(), "(".to_string()),
        //     ("P1".to_string(), "id".to_string()),
        //     ("P1".to_string(), "+".to_string()),
        //     ("P1".to_string(), "$".to_string()),
        //     ("P1".to_string(), "-".to_string()),
        //     ("P1".to_string(), "*".to_string()),
        //     ("P1".to_string(), "/".to_string()),
        //     ("P1".to_string(), "^".to_string()),
        //     ("P1".to_string(), ")".to_string())

        // ]);

    
    
        let parse_table: Vec<Vec<&'static str>> = vec![
            vec!["TE1", "TE1", "TE1", "", "", "", "", "", "", ""],
            vec!["PT1", "PT1", "PT1", "", "", "", "", "", "", ""],
            vec!["exp[F]", "FP1", "FP1", "", "", "", "", "", "", ""],
            vec!["", "(E)", "id", "", "", "", "", "", "", ""],
            vec!["", "", "", "+TE1", "epsilon", "-TE1", "", "", "", "epsilon"],
            vec!["", "", "", "epsilon", "epsilon", "epsilon", "*PT1", "/PT1", "", "epsilon"],
            vec!["", "", "", "epsilon", "epsilon", "epsilon", "epsilon", "epsilon", "^FP1", "epsilon"]
        ];

        parse_table
        
    }