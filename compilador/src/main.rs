mod SymbolTable;
mod lexer;
// mod semantic;
mod sintax;

use lexer::lexer::*;
use std::{env, fs::read_to_string};
use compilador::lexer::lexer_tokens::TokEnum;
use crate::sintax::sintax::Sintax;

fn main() {
    let mut lex = Lexer::new();
    let current_dir = env::current_dir().unwrap();

    let file_path = current_dir.display().to_string() + "\\src\\resources\\first_example.txt";

    println!("Directorio actual: {}", file_path);
    // Leer el contenido del archivo
    match read_to_string(file_path) {
        Err(e) => {
            println!("Error al leer el archivo: {}", e);
        }
        Ok(content) => {
            let result_lexemes = lex.analyze(content);
            Sintax::parse(result_lexemes);
        }
    };
    ()
}
