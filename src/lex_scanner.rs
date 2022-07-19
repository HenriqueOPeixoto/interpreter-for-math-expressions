use crate::token::Token;

pub const DIGIT: i32 = 1;
pub const NOT_DIGIT: i32 = 2;
pub const OPERATOR: i32 = 3;
pub const SPACE: i32 = 4;
pub const NOT_SPACE: i32 = 5;
pub const NEWLINE: i32 = 6;
pub const EXP1: i32 = 7;
pub const EXP2: i32 = 8;

pub const OPEN_PAR: i32 = 13;
pub const CLOSE_PAR: i32 = 15;

pub const OPEN_SQ_BRACKET: i32 = 17;
pub const CLOSE_SQ_BRACKET: i32 = 19;

pub const EOF: i32 = -1;

pub struct LexScanner {
    pub content: Vec<char>,
    pub state: i32,
    pub pos: usize,
}

impl LexScanner {
    /* pub fn prepare_scanner(&mut self, file: String) {
        self.content = file.chars().collect();
    } */

    pub fn next_token(&mut self) -> Token {
        if self.pos == self.content.len() {
            return Token {
                tipo: EOF,
                termo: "$".to_string(),
            };
        }

        if self.is_end(self.content[self.pos]) {
            return Token {
                tipo: EOF,
                termo: "$".to_string(),
            };
        }

        self.state = 0;

        let mut c: char;
        let mut buffer: String = String::from("");

        loop {
            c = self.content[self.pos];

            match self.state {
                0 => {
                    if self.is_digit(c) {
                        buffer.push(c);
                        self.state = DIGIT
                    } else if self.is_operator(c) {
                        buffer.push(c);
                        self.state = OPERATOR;
                    } else if self.is_space(c) {
                        buffer.push(c);
                        self.state = SPACE;
                    } else if self.is_e(c) {
                        buffer.push(c);
                        self.state = EXP1;
                    } else if self.is_open_par(c) {
                        buffer.push(c);
                        self.state = OPEN_PAR;
                    } else if self.is_close_par(c) {
                        buffer.push(c);
                        self.state = CLOSE_PAR;
                    } else if self.is_open_sq_bracket(c) {
                        buffer.push(c);
                        self.state = OPEN_SQ_BRACKET;
                    } else if self.is_close_sq_bracket(c) {
                        buffer.push(c);
                        self.state = CLOSE_SQ_BRACKET;
                    } else {
                        panic!("Token não reconhecido!")
                    }
                }
                DIGIT => match self.is_digit(c) {
                    true => {
                        buffer.push(c);
                        self.state = DIGIT
                    }
                    false => {
                        self.state = NOT_DIGIT;
                    }
                },
                NOT_DIGIT => {
                    self.pos -= 1;
                    return Token {
                        tipo: DIGIT,
                        termo: buffer.to_string(),
                    };
                }
                OPERATOR => {
                    return Token {
                        tipo: OPERATOR,
                        termo: buffer.to_string(),
                    };
                }
                SPACE => match self.is_space(c) {
                    true => {
                        buffer.push(c);
                        self.state = SPACE;
                    }
                    false => self.state = NOT_SPACE,
                },
                NOT_SPACE => {
                    self.pos -= 1;

                    if self.is_newline(c) {
                        return Token {
                            tipo: NEWLINE,
                            termo: "$".to_string()
                        }
                    } else {
                        return Token {
                            tipo: SPACE,
                            termo: buffer.to_string(),
                        };
                    }
                }
                EXP1 => match c {
                    'x' => {
                        buffer.push(c);
                        self.state = EXP2;
                    }
                    _ => panic!("Token não reconhecido!"),
                },
                EXP2 => match c {
                    'p' => {
                        buffer.push(c);
                        self.pos += 1;
                        return Token {
                            tipo: EXP1,
                            termo: buffer.to_string(),
                        };
                    }
                    _ => panic!("Token não reconhecido!"),
                },
                OPEN_PAR => {
                    return Token {
                        tipo: OPEN_PAR,
                        termo: buffer.to_string(),
                    }
                }
                CLOSE_PAR => {
                    return Token {
                        tipo: CLOSE_PAR,
                        termo: buffer.to_string(),
                    }
                }
                OPEN_SQ_BRACKET => {
                    return Token {
                        tipo: OPEN_SQ_BRACKET,
                        termo: buffer.to_string(),
                    }
                }
                CLOSE_SQ_BRACKET => {
                    return Token {
                        tipo: CLOSE_SQ_BRACKET,
                        termo: buffer.to_string(),
                    }
                }
                _ => (),
            };

            self.pos += 1;

            if self.is_end(c) {
                if self.state != OPERATOR {
                    self.state -= 1; // Sai do estado NOT_X para o estado X.
                }

                return Token {
                    tipo: self.state,
                    termo: buffer.to_string(),
                };
            }
        }
    }

    fn is_digit(&self, c: char) -> bool {
        match c {
            '0'..='9' => true,
            _ => false,
        }
    }

    fn is_operator(&self, c: char) -> bool {
        match c {
            '+' | '-' | '*' | '/' | '^' => true,
            _ => false,
        }
    }

    fn is_space(&self, c: char) -> bool {
        match c {
            ' ' | '\n' => true,
            _ => false,
        }
    }

    fn is_e(&self, c: char) -> bool {
        match c {
            'e' => true,
            _ => false,
        }
    }

    fn is_end(&self, c: char) -> bool {
        match c {
            '\0' => true,
            _ => false,
        }
    }

    fn is_open_par(&self, c: char) -> bool {
        match c {
            '(' => true,
            _ => false,
        }
    }
    fn is_close_par(&self, c: char) -> bool {
        match c {
            ')' => true,
            _ => false,
        }
    }

    fn is_open_sq_bracket(&self, c: char) -> bool {
        match c {
            '[' => true,
            _ => false,
        }
    }
    fn is_close_sq_bracket(&self, c: char) -> bool {
        match c {
            ']' => true,
            _ => false,
        }
    }

    fn is_newline(&self, c: char) -> bool {
        match c {
            '\n' => true,
            _ => false
        }
    }

}

pub fn is_sum_operator(token: &Token) -> bool {
    match token.termo.as_str() {
        "+" => true,
        _ => false
    }
}

pub fn is_sub_operator(token: &Token) -> bool {
    match token.termo.as_str() {
        "-" => true,
        _ => false
    }
}

pub fn is_mul_operator(token: &Token) -> bool {
    match token.termo.as_str() {
        "*" => true,
        _ => false
    }
}

pub fn is_div_operator(token: &Token) -> bool {
    match token.termo.as_str() {
        "/" => true,
        _ => false
    }
}

pub fn is_pow_operator(token: &Token) -> bool {
    match token.termo.as_str() {
        "^" => true,
        _ => false
    }
}
