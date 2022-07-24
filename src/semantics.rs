use std::f32::consts::E;

/**
 * Assuming correct syntax, semantics.rs code will calculate the values of the expressions
 * within the input file.
 */

use crate::{token::Token, lex_scanner::{OPERATOR, DIGIT, EXP1, NEWLINE, EOF}};

/**
 * Assuming values are organized in RPN, returns the expression value
 */
pub fn calculate_expr_rpn(tokens: Vec<Token>) -> Vec<Token> {
    let mut result_tokens: Vec<Token> = vec![];

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
                    
                    let operand_2 = &operand_2_token.termo.parse::<f32>().expect("Erro ao converter operando");
                    let operand_1 = &operand_1_token.termo.parse::<f32>().expect("Erro ao converter operando");

                    stack.push(Token{ termo: (operand_1 + operand_2).to_string(), tipo: DIGIT });
                },
                "-" => {
                    let operand_2_token = stack.pop().expect("Operando não encontrado");
                    let operand_1_token = stack.pop().expect("Operando não encontrado");
                    
                    let operand_2 = &operand_2_token.termo.parse::<f32>().expect("Erro ao converter operando");
                    let operand_1 = &operand_1_token.termo.parse::<f32>().expect("Erro ao converter operando");

                    stack.push(Token{ termo: (operand_1 - operand_2).to_string(), tipo: DIGIT });
                },
                "*" => {
                    let operand_2_token = stack.pop().expect("Operando não encontrado");
                    let operand_1_token = stack.pop().expect("Operando não encontrado");
                    
                    let operand_2 = &operand_2_token.termo.parse::<f32>().expect("Erro ao converter operando");
                    let operand_1 = &operand_1_token.termo.parse::<f32>().expect("Erro ao converter operando");

                    stack.push(Token{ termo: (operand_1 * operand_2).to_string(), tipo: DIGIT });
                },
                "/" => {
                    let operand_2_token = stack.pop().expect("Operando não encontrado");
                    let operand_1_token = stack.pop().expect("Operando não encontrado");
                    
                    let operand_2 = &operand_2_token.termo.parse::<f32>().expect("Erro ao converter operando");
                    let operand_1 = &operand_1_token.termo.parse::<f32>().expect("Erro ao converter operando");

                    stack.push(Token{ termo: (operand_1 / operand_2).to_string(), tipo: DIGIT });
                },
                "^" => {
                    let operand_2_token = stack.pop().expect("Operando não encontrado");
                    let operand_1_token = stack.pop().expect("Operando não encontrado");
                    
                    let operand_2 = &operand_2_token.termo.parse::<f32>().expect("Erro ao converter operando");
                    let operand_1 = &operand_1_token.termo.parse::<f32>().expect("Erro ao converter operando");

                    stack.push(Token{ termo: (operand_1.powf(*operand_2)).to_string(), tipo: DIGIT });
                },
                _ => todo!()
            }
        } else if token.tipo == EXP1 {
            let operand_1_token = stack.pop().expect("Operando não encontrado");
                    
            let operand_1 = &operand_1_token.termo.parse::<f32>().expect("Erro ao converter operando");

            stack.push(Token{ termo: (E.powf(*operand_1)).to_string(), tipo: DIGIT });
        } else if token.tipo == NEWLINE || token.tipo == EOF {
            result_tokens.push(stack.pop().unwrap())
        }
    }

    result_tokens

}