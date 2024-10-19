use std::ops::Index;

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
        
        let mut ascii = u8::MIN;

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
        let mut counter:usize = 0;
        let mut word = String::new();
        let mut line_copy = line.clone();
        let chars: Vec<char> = line.chars().collect();
        let first_letter =  *chars.get(index).unwrap();
        for(index_line , &letter) in chars.iter().enumerate().skip(index){
            if letter as u8 != Cons::SPACE as u8{
                word += letter.to_string().as_str();
            }
            if index_line != index && letter == first_letter {
                if first_letter as u8 == Cons::SLASH as u8{
                        //Find the double slash position
                        let doble_slash = line_copy.find("//").unwrap();
                        
                        //Remove all that is after those
                        line_copy = &line_copy[index_line..];

                        //If is necesary that will remove the spaces at the end
                        let line_trimed = line_copy.trim_end();

                        self.fill_lexemes(line_trimed, self.num_line + 1, Some(TokEnum::SCMT));
                        counter = line.len() ;
                }
                else if letter as u8 == Cons::STRING as u8 {
                    self.fill_lexemes(&word, self.num_line + 1, Some(TokEnum::STRING));
                }
                else if letter as u8 == Cons::CHAR as u8 {
                    self.fill_lexemes(&word, self.num_line + 1, Some(TokEnum::CHAR));
                }
                break;
            }
            else { counter += 1; }
        }
        counter
    }


    fn lexeme_filter(&mut self, lexeme:&str)->TokEnum{
        
        //Brackets or semicolon
        let mut token:(bool,TokEnum) = Tokens::is_bracket_or_scn(lexeme);
        if token.0{
            return token.1;
        }
        
        //Numeric Types
        if Tokens::is_float(lexeme){
            return TokEnum::FLOAT;
        }
        else if Tokens::is_integer(lexeme){
            return TokEnum::INTEGER;
        }

        //Encapsulations
        if Tokens::is_encapsulation(lexeme){
            return TokEnum::ENCAPSULATION;
        }

        //Reserved Words
        token = Tokens::is_reserved_word(lexeme);
        if token.0{
            return token.1;
        }

        //Primitives
        if Tokens::is_primitive(lexeme){
            return TokEnum::PRIMITIVE;
        }

        //Operators
        token = Tokens::is_operator(lexeme);
        if token.0{
            return token.1;
        }

        //Default
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


