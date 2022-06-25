struct LexScanner {
    content: Vec<char>,
    state: i32,
    pos: usize
}

impl LexScanner {
    fn prepare_scanner(&mut self, file: String) {
        self.content = file.chars().collect();
    }

    fn next_token(&mut self) {
        self.state = 0;
        self.pos = 0;

        let mut c: char;

        loop {
            c = self.content[self.pos];

            println!("{}", c);

            self.pos += 1;
        }
    }

    fn isDigit(&self, c: char) -> bool {
        if matches!(c, '0'..='9') {
            return true;
        }

        false
    }
}