mod semantic;
mod lexer;
mod sintax;

use std::{env, fs::read_to_string};
use lexer::lexer::Lexer;

fn main() {
    let mut lex =  Lexer::new();
    let current_dir = env::current_dir().unwrap();

    let file_path = current_dir.display().to_string() + "\\src\\resources\\test.txt";

    println!("Directorio actual: {}", file_path);
    // Leer el contenido del archivo
    match read_to_string(file_path) {
        Ok(content) => {

            lex.analizer(content.lines().collect());
            
            for lexeme in lex.lexemes {
                println!("*{}*: ({}: {})", lexeme.0, lexeme.1.0.as_ref(), lexeme.1.1)
            }

        }
        Err(e) => {
            println!("Error al leer el archivo: {}", e);
        }
    }

    
}
