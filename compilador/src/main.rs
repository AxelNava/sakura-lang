use lexer::lexer::Lexer;

mod lexer;

fn main() {
    let mut lex =  Lexer::new();
    lex.analizer(vec!["asdasd", "asdasd"]);
}
