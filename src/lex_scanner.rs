pub struct LexScanner {
    pub content: Vec<char>,
    pub state: i32,
    pub pos: usize
}

impl LexScanner {
    /* pub fn prepare_scanner(&mut self, file: String) {
        self.content = file.chars().collect();
    } */

    pub fn next_token(&mut self) {
        self.state = 0;
        self.pos = 0;

        let mut c: char;

        loop {
            c = self.content[self.pos];

            println!("{}", c);

            self.pos += 1;

            if self.pos == self.content.len() { break; }
        }
    }

    pub fn is_digit(&self, c: char) -> bool {
        if matches!(c, '0'..='9') {
            return true;
        }

        false
    }
}