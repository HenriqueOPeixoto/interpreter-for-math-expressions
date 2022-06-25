pub const DIGITO: i32 = 1;

pub struct Token {
    pub tipo: i32,
    pub termo: String
}

impl Token {
    fn to_string(&self) {
        println!("Tipo: {}, Termo: {}", self.tipo, self.termo);
    }
}

