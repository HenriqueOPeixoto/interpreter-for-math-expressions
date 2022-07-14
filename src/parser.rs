use crate::{token::Token, lex_scanner::EOF, lex_scanner::DIGIT};

use std::collections::HashMap;

// Regras
const E: i32 = 0;
const T: i32 = 1;
const P: i32 = 2;
const F: i32 = 3;
const E1: i32 = 4;
const T1: i32 = 5;
const P1: i32 = 6;

// Terminais
const EXP: i32 = 0;
const OPEN_PAR: i32 = 1;
const ID: i32 = 2;
const SUM: i32 = 3;
const END_OF_STACK: i32 = 4;
const SUB: i32 = 5;
const MUL: i32 = 6;
const DIV: i32 = 7;
const POW: i32 = 8;
const CLOSE_PAR: i32 = 10;

pub fn parse_syntax(mut tokens: Vec<Token>) -> bool {

    let parse_table = prepare_parse_table();
    
    let mut pos = 0;
    let mut stack: Vec<&str> = vec!["$", "E"];

    println!("{:?}", stack);
    
    tokens.reverse();
    
    let final_token: Token = Token { tipo: EOF, termo: "$".to_string() };

    while tokens[pos].termo != final_token.termo {

        if tokens[pos].tipo == DIGIT {
            todo!()
        }

        pos += 1;
        todo!()
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