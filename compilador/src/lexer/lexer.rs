use super::{lexer_cons::*, lexer_tokens::*};

pub struct Lexer {
    pub lexemes:Vec<(String,(TokEnum, i32))>,
    num_line:i32,
}

impl Lexer {
    
    pub fn new()->Lexer {
        Lexer { 
            lexemes:Vec::new(), 
            num_line:0 
        }
    }
    
    /// Make the analizer
    /// ## asdffasdfasdfa
    pub fn analizer(&mut self, data:Vec<&str>){
        
        let mut ascii = u8::MAX;

        let mut word: String = String::new();
        self.num_line = 0;

        
        //Iterate per each line
        for line in data {
            //Reset the word
            word = String::new();

             // Convert the line into a vector of characters
            let chars: Vec<char> = line.chars().collect();

            //Iterate per character in the line
            for (mut index_line, &letter) in chars.iter().enumerate() {
                ascii = letter as u8;
                
                //Check if the ascii isn't chainable 
                if ascii != Cons::CR as u8 && ascii != Cons::SPACE as u8{
                    if ascii != Cons::STRING as u8 && ascii != Cons::CHAR as u8
                        && ascii != Cons::SLASH as u8 
                        && (chars[index_line + 1] as u8) != Cons::SLASH as u8{
                        
                        //Check if the character is a bracket or a semicolon
                        if self.is_bracket_or_scn(ascii){
                            if word != String::new() {
                                self.fill_lexemes(&word, self.num_line, None);
                                word = String::new();
                            }
                            self.fill_lexemes(letter.to_string().as_str(), self.num_line + 1, None);
                            continue;
                        }
                        if ascii != Cons::DOT as u8{
                            word += letter.to_string().as_str();
                        }
                        else {
                            if index_line - 1 > 0 && index_line + 1 < line.len() { 
                                if self.is_numeric((chars[index_line - 1] as u8).to_string().as_str()){
                                    word += letter.to_string().as_str();
                                    continue;
                                }
                            }
                            self.fill_lexemes(&word, self.num_line, None);
                            word = String::new();
                        }

                        if (ascii == Cons::SPACE as u8 || letter == *chars.last().unwrap()) 
                        &&(word != String::new()){
                            self.fill_lexemes(&word, self.num_line, None);
                            word = String::new();
                        }
                    }
                    else{
                        if word != String::new(){
                            self.fill_lexemes(&word, self.num_line, None);
                            word = String::new();
                        }
                        index_line += self.chain_lexemes(line, index_line.clone());
                    }
                }
                else {
                    if (ascii == Cons::SPACE as u8|| letter == *chars.last().unwrap()) 
                    && word != String::new(){
                        self.fill_lexemes(&word, self.num_line, None);
                        word = String::new();
                    }
                }
            }
            
            self.num_line += 1;
        
        }




    }


    fn chain_lexemes(&mut self, line:&str, index:usize) -> usize{
        12
    }


    fn lexeme_filter(&mut self, lexeme:&str)->TokEnum{
        return TokEnum::IDENTIFIER;
    }


    fn is_numeric(&mut self, lexeme:&str)->bool{
        lexeme.parse::<f64>().is_ok()
    }


    fn fill_lexemes(&mut self, lexeme:&str, line:i32, token:Option<TokEnum>){
        let token = token.unwrap_or(TokEnum::IDENTIFIER); 
        if token != TokEnum::IDENTIFIER {
            self.lexemes.push((lexeme.to_string(), (token, line)));
        }
        else {
            let lexeme_found = self.lexeme_filter(lexeme);
            self.lexemes.push((lexeme.to_string(), (lexeme_found, line)));
        }
    }


    fn is_bracket_or_scn(&mut self, ascii:u8)->bool{
        ascii == Cons::ORB as u8 || ascii == Cons::CRB as u8 || ascii == Cons::OSB as u8
        || ascii == Cons::CSB as u8 || ascii == Cons::OB as u8 || ascii == Cons::CB as u8
        || ascii == Cons::SCN as u8
    }

}


