use crate::{token::Token, lex_scanner::{DIGIT, OPERATOR, OPEN_PAR, CLOSE_PAR}};

/**
 * Code that manipulates Reverse Poland Notation
 */

const MIN_PRIORITY: i32 = 0; // Used in + and -
const MED_PRIORITY: i32 = 1; // Used in * and /
const MAX_PRIORITY: i32 = 2; // Used in ^ and exp[]

pub fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {

    let mut out: Vec<Token> = vec![];
    let mut stack: Vec<Token> = vec![];

    for token in &tokens {
        if token.tipo == DIGIT {
            out.push(token.clone());
        } else if token.tipo == OPERATOR {
            match token.termo.as_str() {
                "+" | "-" => {
                    let precedence = match &stack.last() {
                        Some(x) => get_precedence(&x.termo),
                        None => -1
                    };

                    if precedence > MIN_PRIORITY {
                        while let Some(x) = stack.last() {
                            if get_precedence(&x.termo) <= MIN_PRIORITY { break; }

                            out.push(stack.pop().unwrap());
                        }
                    }
                    stack.push(token.clone());
                },
                "*" | "/" => {
                    let precedence = match &stack.last() {
                        Some(x) => get_precedence(&x.termo),
                        None => -1
                    };

                    if precedence > MED_PRIORITY {
                        while let Some(x) = stack.last() {
                            if get_precedence(&x.termo) <= MED_PRIORITY { break; }
                            
                            out.push(stack.pop().unwrap());
                        }
                    }
                    stack.push(token.clone());
                },
                "^" | "exp" => {
                    stack.push(token.clone());
                }
                _ => todo!()
            }
        } else if token.tipo == OPEN_PAR {
            stack.push(token.clone());
        } else if token.tipo == CLOSE_PAR {
            while let Some(x) = stack.last() {
                if x.tipo != OPEN_PAR { out.push(stack.pop().unwrap()); }
                else { 
                    stack.pop(); // discard (
                    break; 
                }
            }
        }

    }
    
    while !stack.is_empty() {
        out.push(stack.pop().unwrap());
    }

    out

}

fn get_precedence(op: &str) -> i32 {
    if op == "+" || op == "-" {
        return MIN_PRIORITY;
    } else if op == "*" || op == "/" {
        return MED_PRIORITY;
    } else if op == "^" || op == "exp" {
        return MAX_PRIORITY;
    } else {
        return -1;
    }
}