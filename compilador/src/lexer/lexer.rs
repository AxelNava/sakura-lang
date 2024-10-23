use std::{string, vec};

use super::{lexer_cons::*, lexer_tokens::*};

pub struct Lexer {
    pub lexemes: Vec<(String, (TokEnum, i32))>,
    num_line: i32,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            lexemes: Vec::new(),
            num_line: 0,
        }
    }

    /// Make the analizer
    /// ## asdffasdfasdfa
    pub fn analizer(&mut self, data: Vec<&str>) {
        let mut ascii = u8::MIN;
        let char_min_value = '\0';
        let mut word: String = String::new();
        self.num_line = 1;

        //Iterate per each line
        for line in data {
            //Reset the word
            word = String::new();

            // Convert the line into a vector of characters
            let chars: Vec<char> = line.chars().collect();
            let mut index: usize = 0;
            let mut letter: char;
            //Iterate per character in the line
            // for (mut index, &letter) in chars.iter().enumerate()
            while index < chars.len() {
                letter = chars[index];
                ascii = letter as u8;
                if letter == ')' {
                    print!("");
                }
                //Check if the ascii isn't chainable
                if index + 1 < chars.len() {
                    if ascii != Cons::CR as u8 && ascii != Cons::SPACE as u8 {
                        if ascii != Cons::STRING as u8
                            && ascii != Cons::CHAR as u8
                            && ascii != Cons::SLASH as u8
                            && (chars[index + 1] as u8) != Cons::SLASH as u8
                        {
                            //Bracket or Semicolon Lexeme
                            if self.is_bracket_or_scn(ascii) {
                                if word != String::new() {
                                    self.fill_lexemes(&word, self.num_line, None);
                                    word = String::new();
                                }
                                self.fill_lexemes(
                                    letter.to_string().as_str(),
                                    self.num_line + 1,
                                    None,
                                );
                                index += 1;
                                letter = chars[index];
                            }

                            //Numeric Lexeme
                            if self.is_numeric(letter.to_string().as_str()) {
                                let data = self.complete_numeric_element(index, chars.clone());

                                word += data.1.as_str();
                                index = data.0;

                                self.fill_lexemes(&word, self.num_line, None);
                                word = String::new();

                                letter = char_min_value;
                            }

                            //Chain Lexeme
                            if self.is_start_chain(letter as u8) {
                                if word != String::new() {
                                    word += letter.to_string().as_str();
                                    index += self.chain_lexemes(line, index.clone());
                                    word = String::new();
                                } else {
                                    index += self.chain_lexemes(line, index.clone());
                                }
                                letter = char_min_value;
                            }

                            //Lexeme Connected By Dot
                            if ascii == Cons::DOT as u8 {
                                if index - 1 > 0 && index + 1 < line.len() {
                                    let data = self.complete_dot_element(index, chars.clone());

                                    word += data.1.as_str();
                                    index = data.0;
                                }
                                self.fill_lexemes(&word, self.num_line, None);
                                word = String::new();
                            } else if letter != char_min_value {
                                word += letter.to_string().as_str();
                            }

                            //Space or Last Lexeme
                            if (ascii == Cons::SPACE as u8 || letter == *chars.last().unwrap())
                                && (word != String::new())
                            {
                                self.fill_lexemes(&word, self.num_line, None);
                                word = String::new();
                            }
                        } else {
                            if word != String::new() {
                                word += letter.to_string().as_str();
                                index += self.chain_lexemes(line, index.clone());
                                word = String::new();
                            } else {
                                index += self.chain_lexemes(line, index.clone());
                            }
                        }
                    } else {
                        if (ascii == Cons::SPACE as u8 || letter == *chars.last().unwrap())
                            && word != String::new()
                        {
                            self.fill_lexemes(&word, self.num_line, None);
                            word = String::new();
                        }
                    }
                } else {
                    if (ascii == Cons::SPACE as u8 || letter == *chars.last().unwrap())
                        && chars.len() - 1 == index
                        && chars.len() == 1
                    {
                        let token = self.lexeme_filter(letter.to_string().as_str());
                        self.fill_lexemes(letter.to_string().as_str(), self.num_line, Some(token));
                        index += 1;
                    }
                    if (ascii == Cons::SPACE as u8 || letter == *chars.last().unwrap())
                        && chars.len() - 1 == index
                    {
                        //Bracket or Semicolon Lexeme
                        if self.is_bracket_or_scn(ascii) {
                            if word != String::new() {
                                self.fill_lexemes(&word, self.num_line, None);
                                word = String::new();
                            }
                            self.fill_lexemes(
                                letter.to_string().as_str(),
                                self.num_line,
                                None,
                            );
                        }
                        else{
                            word += letter.to_string().as_str();
                            self.fill_lexemes(word.as_str(), self.num_line, None);
                            word = String::new();
                        }
                    }
                    if (ascii == Cons::SPACE as u8 || letter == *chars.last().unwrap())
                        && word != String::new()
                    {
                        self.fill_lexemes(&word, self.num_line, None);
                        word = String::new();
                    }
                }
                index += 1;
            }

            self.num_line += 1;
        }
    }

    fn chain_lexemes(&mut self, line: &str, index: usize) -> usize {
        let mut counter: usize = 0;
        let mut word = String::new();
        let mut line_copy = line.clone();
        let chars: Vec<char> = line.chars().collect();
        let first_letter = *chars.get(index).unwrap();
        for (index_line, &letter) in chars.iter().enumerate().skip(index) {
            if letter as u8 != Cons::SPACE as u8 {
                word += letter.to_string().as_str();
            }
            if index_line != index && letter == first_letter {
                if first_letter as u8 == Cons::SLASH as u8 {
                    //Remove all that is after those
                    line_copy = &line_copy[index_line..];
                    let trim_data = line_copy.trim_end();
                    //If is necesary that will remove the spaces at the end
                    let line_trimed = format!("/{trim_data}");

                    self.fill_lexemes(line_trimed.as_str(), self.num_line, Some(TokEnum::SCMT));
                    counter = chars.len();
                } else if letter as u8 == Cons::STRING as u8 {
                    self.fill_lexemes(&word, self.num_line, Some(TokEnum::STRING));
                } else if letter as u8 == Cons::CHAR as u8 {
                    self.fill_lexemes(&word, self.num_line, Some(TokEnum::CHAR));
                }
                break;
            } else {
                counter += 1;
            }
        }
        counter
    }

    fn lexeme_filter(&mut self, lexeme: &str) -> TokEnum {
        //Brackets or semicolon
        let mut token: (bool, TokEnum) = Tokens::is_bracket_or_scn(lexeme);
        if token.0 {
            return token.1;
        }

        //Numeric Types
        if Tokens::is_float(lexeme) {
            return TokEnum::FLOAT;
        } else if Tokens::is_integer(lexeme) {
            return TokEnum::INTEGER;
        }

        //Encapsulations
        if Tokens::is_encapsulation(lexeme) {
            return TokEnum::ENCAPSULATION;
        }

        //Reserved Words
        token = Tokens::is_reserved_word(lexeme);
        if token.0 {
            return token.1;
        }

        //Primitives
        if Tokens::is_primitive(lexeme) {
            return TokEnum::PRIMITIVE;
        }

        //Operators
        token = Tokens::is_operator(lexeme);
        if token.0 {
            return token.1;
        }

        //Default
        TokEnum::IDENTIFIER
    }

    fn is_numeric(&mut self, lexeme: &str) -> bool {
        Tokens::is_integer(lexeme) | Tokens::is_float(lexeme)
    }

    fn fill_lexemes(&mut self, lexeme: &str, line: i32, token: Option<TokEnum>) {
        let token = token.unwrap_or(TokEnum::IDENTIFIER);
        if token != TokEnum::IDENTIFIER {
            self.lexemes.push((lexeme.to_string(), (token, line)));
        } else {
            let lexeme_found = self.lexeme_filter(lexeme);
            self.lexemes
                .push((lexeme.to_string(), (lexeme_found, line)));
        }
    }

    fn is_bracket_or_scn(&mut self, ascii: u8) -> bool {
        ascii == Cons::ORB as u8
            || ascii == Cons::CRB as u8
            || ascii == Cons::OSB as u8
            || ascii == Cons::CSB as u8
            || ascii == Cons::OB as u8
            || ascii == Cons::CB as u8
            || ascii == Cons::SCN as u8
    }

    fn is_start_chain(&mut self, ascii: u8) -> bool {
        ascii == Cons::STRING as u8 || ascii == Cons::SLASH as u8 || ascii == Cons::CHAR as u8
    }

    fn complete_numeric_element(&mut self, mut index: usize, line: Vec<char>) -> (usize, String) {
        let mut second_mid_data = String::new();

        while index < line.len() {
            if self.is_numeric(line[index].to_string().as_str())
                || line[index] as u8 == Cons::DOT as u8
                || (index + 1 < line.len() && line[index + 1] as u8 == Cons::DOT as u8)
            {
                second_mid_data += line[index].to_string().as_str();
                index += 1;
            } else {
                break;
            }
        }

        (index - 1, second_mid_data)
    }

    fn complete_dot_element(&mut self, mut index: usize, line: Vec<char>) -> (usize, String) {
        let mut second_mid_data = String::new();
        let mut token: TokEnum = TokEnum::IDENTIFIER;
        while index < line.len()
            && (!self.is_bracket_or_scn(line[index] as u8)
                && !self.is_start_chain(line[index] as u8)
                && token != TokEnum::PRIMITIVE
                && token != TokEnum::ENCAPSULATION
                && !Tokens::is_operator(line[index].to_string().as_str()).0)
        {
            token = self.lexeme_filter(line[index].to_string().as_str());
            second_mid_data += line[index].to_string().as_str();
            index += 1;
        }

        (index - 1, second_mid_data)
    }
}
