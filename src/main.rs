use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Um nome de arquivo deve ser fornecido!")
    }

    let filename = &args[1];

    println!("{}", filename);

    let input_file = fs::read_to_string(filename)
        .expect("Algo deu errado ao ler o arquivo!");

    println!("{}", input_file);
    

}

