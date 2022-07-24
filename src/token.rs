#[derive(Clone, Debug)]
pub struct Token {
    pub tipo: i32,
    pub termo: String
}

impl Token {
    pub fn to_string(&self) {
        println!("Tipo: {}, Termo: {}", self.tipo, self.termo);
    }
}

