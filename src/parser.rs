use crate::{token::Token, lex_scanner::EOF, lex_scanner::DIGIT};

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
    
    //let mut pos = tokens.len() - 1;
    let mut stack: Vec<&str> = vec!["$", "E"];

    println!("{:?}", stack);
    
    tokens.reverse();
    
    let final_token: Token = Token { tipo: EOF, termo: "$".to_string() };
 
    while tokens.last().expect("Erro ao ler o próximo token da pilha!").termo != final_token.termo {

        let token_atual = tokens.last().expect("Erro ao ler o próximo token da pilha!");

        if token_atual.tipo == DIGIT {
            match stack.last().expect("Erro ao ler topo da pilha!").as_ref() { // gets &str from string
                "E" => {
                    stack.pop();
                    let mut derivation: Vec<&str> = parse_table[E][ID].split("|").collect();
                    derivation.reverse();
                    stack.append(&mut derivation);
                },
                "T" => {
                    stack.pop();
                    let mut derivation: Vec<&str> = parse_table[T][ID].split("|").collect();
                    derivation.reverse();
                    stack.append(&mut derivation);
                },
                "P" => {
                    stack.pop();
                    let mut derivation: Vec<&str> = parse_table[P][ID].split("|").collect();
                    derivation.reverse();
                    stack.append(&mut derivation);
                },
                "F" => {
                    stack.pop();
                    let mut derivation: Vec<&str> = parse_table[F][ID].split("|").collect();
                    derivation.reverse();
                    stack.append(&mut derivation);
                },
                "id" => {
                    stack.pop();
                    tokens.pop();
                }
                _ => panic!("Erro de sintaxe!")
            }
        }

        //pos -= 1;
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

    
    
        // char '|' is the separator when appending the derivation to the stack
        // T|E1 turns into [T, E1]
        let parse_table: Vec<Vec<&'static str>> = vec![
            vec!["T|E1", "T|E1", "T|E1", "", "", "", "", "", "", ""],
            vec!["P|T1", "P|T1", "P|T1", "", "", "", "", "", "", ""],
            vec!["exp[|F|]", "F|P1", "F|P1", "", "", "", "", "", "", ""],
            vec!["", "(|E|)", "id", "", "", "", "", "", "", ""],
            vec!["", "", "", "+|T|E1", "epsilon", "-|T|E1", "", "", "", "epsilon"],
            vec!["", "", "", "epsilon", "epsilon", "epsilon", "*|P|T1", "/|P|T1", "", "epsilon"],
            vec!["", "", "", "epsilon", "epsilon", "epsilon", "epsilon", "epsilon", "^|F|P1", "epsilon"]
        ];

        parse_table
        
    }