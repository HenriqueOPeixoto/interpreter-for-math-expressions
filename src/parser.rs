use crate::{token::Token, lex_scanner::EOF};

struct Parser {
    tokens: Vec<Token>,
    pos: usize
}

impl Parser {
    pub fn parse_syntax(&mut self, parse_table: Vec<(char, &str)>) -> bool {
        
        let final_token: Token = Token { tipo: EOF, termo: "$".to_string() };

        while self.tokens[self.pos].termo != final_token.termo {
            todo!()
        }

        

        true
    }
}