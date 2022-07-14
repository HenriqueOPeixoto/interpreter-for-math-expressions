use crate::{token::Token, lex_scanner::EOF};

use std::collections::HashMap;

struct Parser {
    //tokens: Vec<Token>,
    //pos: usize,
    //parse_table: HashMap<String, String>
}

impl Parser {
    pub fn parse_syntax(&mut self, tokens: Vec<Token>) -> bool {

        let mut pos = 0;
        
        let final_token: Token = Token { tipo: EOF, termo: "$".to_string() };

        while tokens[pos].termo != final_token.termo {
            todo!()
        }

        

        true
    }

    
}

pub fn prepare_parse_table() {
        let parse_table = HashMap::from([
            ("E".to_string(), "exp".to_string()),
            ("E".to_string(), "(".to_string()),
            ("E".to_string(), "id".to_string()),
            ("E".to_string(), "+".to_string()),
            ("E".to_string(), "$".to_string()),
            ("E".to_string(), "-".to_string()),
            ("E".to_string(), "*".to_string()),
            ("E".to_string(), "/".to_string()),
            ("E".to_string(), "^".to_string()),
            ("E".to_string(), ")".to_string()),
            ("T".to_string(), "exp".to_string()),
            ("T".to_string(), "(".to_string()),
            ("T".to_string(), "id".to_string()),
            ("T".to_string(), "+".to_string()),
            ("T".to_string(), "$".to_string()),
            ("T".to_string(), "-".to_string()),
            ("T".to_string(), "*".to_string()),
            ("T".to_string(), "/".to_string()),
            ("T".to_string(), "^".to_string()),
            ("T".to_string(), ")".to_string()),
            ("P".to_string(), "exp".to_string()),
            ("P".to_string(), "(".to_string()),
            ("P".to_string(), "id".to_string()),
            ("P".to_string(), "+".to_string()),
            ("P".to_string(), "$".to_string()),
            ("P".to_string(), "-".to_string()),
            ("P".to_string(), "*".to_string()),
            ("P".to_string(), "/".to_string()),
            ("P".to_string(), "^".to_string()),
            ("P".to_string(), ")".to_string()),
            ("F".to_string(), "exp".to_string()),
            ("F".to_string(), "(".to_string()),
            ("F".to_string(), "id".to_string()),
            ("F".to_string(), "+".to_string()),
            ("F".to_string(), "$".to_string()),
            ("F".to_string(), "-".to_string()),
            ("F".to_string(), "*".to_string()),
            ("F".to_string(), "/".to_string()),
            ("F".to_string(), "^".to_string()),
            ("F".to_string(), ")".to_string()),
            ("E1".to_string(), "exp".to_string()),
            ("E1".to_string(), "(".to_string()),
            ("E1".to_string(), "id".to_string()),
            ("E1".to_string(), "+".to_string()),
            ("E1".to_string(), "$".to_string()),
            ("E1".to_string(), "-".to_string()),
            ("E1".to_string(), "*".to_string()),
            ("E1".to_string(), "/".to_string()),
            ("E1".to_string(), "^".to_string()),
            ("E1".to_string(), ")".to_string()),
            ("T1".to_string(), "exp".to_string()),
            ("T1".to_string(), "(".to_string()),
            ("T1".to_string(), "id".to_string()),
            ("T1".to_string(), "+".to_string()),
            ("T1".to_string(), "$".to_string()),
            ("T1".to_string(), "-".to_string()),
            ("T1".to_string(), "*".to_string()),
            ("T1".to_string(), "/".to_string()),
            ("T1".to_string(), "^".to_string()),
            ("T1".to_string(), ")".to_string()),
            ("P1".to_string(), "exp".to_string()),
            ("P1".to_string(), "(".to_string()),
            ("P1".to_string(), "id".to_string()),
            ("P1".to_string(), "+".to_string()),
            ("P1".to_string(), "$".to_string()),
            ("P1".to_string(), "-".to_string()),
            ("P1".to_string(), "*".to_string()),
            ("P1".to_string(), "/".to_string()),
            ("P1".to_string(), "^".to_string()),
            ("P1".to_string(), ")".to_string())
        ]);
    }