const DIGITO: i32 = 0;
const ESPACO: i32 = 1;

pub struct Token {
    tipo: i32,
    termo: String
}

impl Token {
    fn to_string(&self) {
        println!("Tipo: {}, Termo: {}", self.tipo, self.termo);
    }
}

