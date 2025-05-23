use crate::lexer::lexer_tokens::TokEnum;
use std::fs::File;
use std::io;
use std::io::Write;

pub struct PrintElementsLexer {
    nombre_archivo: String,
}
impl PrintElementsLexer {
    pub fn new(nombre_archivo: String) -> Self {
        Self { nombre_archivo }
    }
    pub fn print_elements(&self, elements: &Vec<(String, TokEnum)>) -> io::Result<()> {
        let mut file = File::create(self.nombre_archivo.clone()).unwrap();
        let encabezado = format!("{:<15} | {:<25}", "Tipo", "Valor");
        let separador = "-".repeat(encabezado.len());
        writeln!(file, "{}", encabezado).unwrap();
        writeln!(file, "{}", separador).unwrap();
        for element in elements {
            let linea = format!("{:<15} | {:<25}", element.1, element.0);
            writeln!(file, "{}", linea)?;
        }
        println!("Tabla guardada éxitosamente en: {}", self.nombre_archivo);
        Ok(())
    }
}
