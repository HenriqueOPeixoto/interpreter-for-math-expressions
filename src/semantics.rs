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
const OP_DIV: i32 = 4;

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
                    "/" => { operation = OP_DIV }
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
                    },
                    OP_DIV => {
                        expr_result /= token.termo.parse::<i32>().unwrap();
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

/**
 * Assuming values are organized in RPN, returns the expression value
 */
pub fn calculate_expr_rpn(tokens: Vec<Token>) -> Token {
    let mut expr_result = 0;

    let mut stack: Vec<Token> = vec![];

    for token in tokens {
        if token.tipo == DIGIT {
            stack.push(token.clone());
        } else if token.tipo == OPERATOR {
            match token.termo.as_str() {
                "+" => {
                    // In order for the operations to work, it is necessary to gather the second operand
                    // first, since I am using a stack.
                    let operand_2_token = stack.pop().expect("Operando não encontrado");
                    let operand_1_token = stack.pop().expect("Operando não encontrado");
                    
                    let operand_2 = &operand_2_token.termo.parse::<i32>().expect("Erro ao converter operando");
                    let operand_1 = &operand_1_token.termo.parse::<i32>().expect("Erro ao converter operando");

                    stack.push(Token{ termo: (operand_1 + operand_2).to_string(), tipo: DIGIT });
                },
                "-" => {
                    let operand_2_token = stack.pop().expect("Operando não encontrado");
                    let operand_1_token = stack.pop().expect("Operando não encontrado");
                    
                    let operand_2 = &operand_2_token.termo.parse::<i32>().expect("Erro ao converter operando");
                    let operand_1 = &operand_1_token.termo.parse::<i32>().expect("Erro ao converter operando");

                    stack.push(Token{ termo: (operand_1 - operand_2).to_string(), tipo: DIGIT });
                },
                "*" => {
                    let operand_2_token = stack.pop().expect("Operando não encontrado");
                    let operand_1_token = stack.pop().expect("Operando não encontrado");
                    
                    let operand_2 = &operand_2_token.termo.parse::<i32>().expect("Erro ao converter operando");
                    let operand_1 = &operand_1_token.termo.parse::<i32>().expect("Erro ao converter operando");

                    stack.push(Token{ termo: (operand_1 * operand_2).to_string(), tipo: DIGIT });
                },
                "/" => {
                    let operand_2_token = stack.pop().expect("Operando não encontrado");
                    let operand_1_token = stack.pop().expect("Operando não encontrado");
                    
                    let operand_2 = &operand_2_token.termo.parse::<i32>().expect("Erro ao converter operando");
                    let operand_1 = &operand_1_token.termo.parse::<i32>().expect("Erro ao converter operando");

                    stack.push(Token{ termo: (operand_1 / operand_2).to_string(), tipo: DIGIT });
                },
                _ => todo!()
            }
        }
    }

    stack.pop().unwrap()
}