use crate::{token::Token, lex_scanner::{EOF, OPERATOR}, lex_scanner::{DIGIT, EXP1, CLOSE_PAR, OPEN_PAR, self}};

// Regras
const E: usize = 0;
const T: usize = 1;
const P: usize = 2;
const F: usize = 3;
const E1: usize = 4;
const T1: usize = 5;
const P1: usize = 6;

// Terminais
const T_EXP: usize= 0;
const T_OPEN_PAR: usize = 1;
const T_ID: usize = 2;
const T_SUM: usize = 3;
const T_END_OF_STACK: usize = 4;
const T_SUB: usize = 5;
const T_MUL: usize = 6;
const T_DIV: usize = 7;
const T_POW: usize = 8;
const T_CLOSE_PAR: usize = 10;

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
                    let mut derivation: Vec<&str> = parse_table[E][T_ID].split("|").collect();
                    derivation.reverse();
                    stack.append(&mut derivation);
                },
                "T" => {
                    stack.pop();
                    let mut derivation: Vec<&str> = parse_table[T][T_ID].split("|").collect();
                    derivation.reverse();
                    stack.append(&mut derivation);
                },
                "P" => {
                    stack.pop();
                    let mut derivation: Vec<&str> = parse_table[P][T_ID].split("|").collect();
                    derivation.reverse();
                    stack.append(&mut derivation);
                },
                "F" => {
                    stack.pop();
                    let mut derivation: Vec<&str> = parse_table[F][T_ID].split("|").collect();
                    derivation.reverse();
                    stack.append(&mut derivation);
                },
                "id" => {
                    stack.pop();
                    tokens.pop();
                }
                _ => panic!("Erro de sintaxe!")
            }
        } else if token_atual.tipo == OPERATOR {
            if lex_scanner::is_sum_operator(token_atual) {
                match stack.last().expect("Erro ao ler topo da pilha!").as_ref() {
                    "E1" => {
                        stack.pop();
                        let mut derivation: Vec<&str> = parse_table[E1][T_SUM].split("|").collect();
                        derivation.reverse();
                        stack.append(&mut derivation);
                    },
                    "T1" => {
                        stack.pop();
                    },
                    "P1" => {
                        stack.pop();
                    },
                    "+" => {
                        stack.pop();
                        tokens.pop();
                    }
                    _ => panic!("Erro de sintaxe")
                }
            } else if lex_scanner::is_sub_operator(token_atual) {
                match stack.last().expect("Erro ao ler topo da pilha!").as_ref() {
                    "E1" => {
                        stack.pop();
                        let mut derivation: Vec<&str> = parse_table[E1][T_SUB].split("|").collect();
                        derivation.reverse();
                        stack.append(&mut derivation);
                    },
                    "T1" => {
                        stack.pop();
                    },
                    "P1" => {
                        stack.pop();
                    },
                    "-" => {
                        stack.pop();
                        tokens.pop();
                    }
                    _ => panic!("Erro de sintaxe")
                }
            } else if lex_scanner::is_mul_operator(token_atual) {
                match stack.last().expect("Erro ao ler topo da pilha!").as_ref() {
                    "T1" => {
                        stack.pop();
                        let mut derivation: Vec<&str> = parse_table[T1][T_MUL].split("|").collect();
                        derivation.reverse();
                        stack.append(&mut derivation);
                    },
                    "P1" => {
                        stack.pop();
                    },
                    "*" => {
                        stack.pop();
                        tokens.pop();
                    }
                    _ => panic!("Erro de sintaxe")
                }
            } else if lex_scanner::is_div_operator(token_atual) {
                match stack.last().expect("Erro ao ler topo da pilha!").as_ref() {
                    "T1" => {
                        stack.pop();
                        let mut derivation: Vec<&str> = parse_table[T1][T_DIV].split("|").collect();
                        derivation.reverse();
                        stack.append(&mut derivation);
                    },
                    "P1" => {
                        stack.pop();
                    },
                    "/" => {
                        stack.pop();
                        tokens.pop();
                    }
                    _ => panic!("Erro de sintaxe")
                }
            } else if lex_scanner::is_pow_operator(token_atual) {
                match stack.last().expect("Erro ao ler topo da pilha!").as_ref() {
                    "P1" => {
                        stack.pop();
                        let mut derivation: Vec<&str> = parse_table[P1][T_POW].split("|").collect();
                        derivation.reverse();
                        stack.append(&mut derivation);
                    },
                    "^" => {
                        stack.pop();
                        tokens.pop();
                    }
                    _ => panic!("Erro de sintaxe")
                }
            }
        } else if token_atual.tipo == EXP1 {
            todo!()
        } else if token_atual.tipo == OPEN_PAR {
            match stack.last().expect("Erro ao ler topo da pilha!").as_ref() {
                "E" => {
                    stack.pop();
                    let mut derivation: Vec<&str> = parse_table[E][T_OPEN_PAR].split("|").collect();
                    derivation.reverse();
                    stack.append(&mut derivation);
                },
                "T" => {
                    stack.pop();
                    let mut derivation: Vec<&str> = parse_table[T][T_OPEN_PAR].split("|").collect();
                    derivation.reverse();
                    stack.append(&mut derivation);
                },
                "P" => {
                    stack.pop();
                    let mut derivation: Vec<&str> = parse_table[P][T_OPEN_PAR].split("|").collect();
                    derivation.reverse();
                    stack.append(&mut derivation);
                },
                "F" => {
                    stack.pop();
                    let mut derivation: Vec<&str> = parse_table[F][T_OPEN_PAR].split("|").collect();
                    derivation.reverse();
                    stack.append(&mut derivation);
                },
                "(" => {
                    stack.pop();
                    tokens.pop();
                },
                _ => panic!("Erro de sintaxe")
            }
        } else if token_atual.tipo == CLOSE_PAR {
            todo!()
        } 

        //pos -= 1;
    }

    

    true
}

pub fn prepare_parse_table() -> Vec<Vec<&'static str>> {
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