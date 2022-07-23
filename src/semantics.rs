/**
 * Assuming correct syntax, semantics.rs code will calculate the values of the expressions
 * within the input file.
 */

use crate::{token::Token, lex_scanner::{OPEN_PAR, CLOSE_PAR, OPERATOR, DIGIT, SPACE}};

const OP_WAITING: i32 = -1; // waiting result from inner expression
const OP_NONE: i32 = 0;
const OP_SUM: i32 = 1;
const OP_SUB: i32 = 2;
const OP_MUL: i32 = 3;

pub fn calculate_expr(tokens: Vec<Token>, mut pos: usize) -> i32 {
    
    let mut expr_result: i32 = tokens[0].termo.parse::<i32>().unwrap();
    let mut operation: i32 = OP_NONE;

    for token in &tokens {
        
        if operation == OP_WAITING {
            pos += 1;

            if token.tipo == CLOSE_PAR {
                operation = OP_NONE;
            }

            continue;
        }

        match token.tipo {
            OPEN_PAR => {
                operation = OP_WAITING;
                expr_result += calculate_expr(tokens.clone()[pos + 1..tokens.len()].to_vec(), pos + 1);
            },
            CLOSE_PAR => {
                return expr_result;
            },
            OPERATOR => {
                match token.termo.as_str() {
                    "+" => { operation = OP_SUM },
                    "-" => { operation = OP_SUB },
                    "*" => { operation = OP_MUL },
                    _ => todo!()
                }
            },
            DIGIT => {
                match operation {
                    OP_NONE => {
                        pos += 1;
                        continue;
                    }
                    OP_SUM => {
                        expr_result += token.termo.parse::<i32>().unwrap();
                    },
                    OP_SUB => {
                        expr_result -= token.termo.parse::<i32>().unwrap();
                    },
                    OP_MUL => {
                        expr_result *= token.termo.parse::<i32>().unwrap();
                    }
                    _ => todo!()
                }
            },
            SPACE => { pos += 1; continue; }
            _ => todo!()
        }
        pos += 1;
    }

    expr_result
}