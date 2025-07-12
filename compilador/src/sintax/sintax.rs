use crate::lexer::lexer_tokens::TokEnum;

pub struct Sintax {
   pub global_stack_tokens: Vec<String>, 
    
}
impl Sintax {
    pub fn parse(lexemes: Result<Vec<(String, TokEnum)>, String>) -> bool {
        true
    }
}

fn test_sintax(){
}