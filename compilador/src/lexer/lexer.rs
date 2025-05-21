use super::{lexer_cons::*, lexer_tokens::*};

pub struct Lexer {
    pub lexemes: Vec<(String, (TokEnum, i32))>,
    num_line: i32,
    pub errors: Vec<String>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            lexemes: Vec::new(),
            num_line: 0,
            errors: Vec::new(),
        }
    }
    fn is_newline(char: char) -> bool {
        char == '\n' || char == '\r'
    }
    pub fn analyze(&mut self, content: String) {
        let characters = content.as_str().chars().collect::<Vec<char>>();
        let mut tokens = Vec::<TokEnum>::new();
        let mut current_pos = 0;

        //Check if the first character is BOM(Byte Order Mark, for windows only)
        if characters.get(0) == Some(&'\u{feff}') {
            current_pos = current_pos + 1;
        };
        let total_length = content.len();
        while current_pos < total_length {
            let mut result_parse: (String, Result<TokEnum, &str>) =
                Self::parse_keywords_identifiers(&characters[current_pos..]);
            if let Err(error) = result_parse.1 {
                if !error.is_empty() {
                    self.errors.push(error.to_string());
                }
            } else {
                current_pos = current_pos + result_parse.0.len();
                tokens.push(result_parse.1.unwrap());
                continue;
            }

            result_parse = Self::parse_two_or_one_characters(&characters[current_pos..]);
            if result_parse.1.is_ok() {
                tokens.push(result_parse.1.unwrap());
                current_pos = current_pos + result_parse.0.len();
            }
            result_parse = Self::parse_one_character(&characters[current_pos..]);
            if result_parse.1.is_ok() {
                tokens.push(result_parse.1.unwrap());
                current_pos = current_pos + result_parse.0.len();
            }
        }
    }
    fn parse_one_character(chars: &[char]) -> (String, Result<TokEnum, &str>) {
        let mut binding = chars.iter().peekable();
        let char_to_check = binding.peek();
        if char_to_check.is_some() {
            let char = **char_to_check.unwrap();
            let token = Tokens::is_operator(&char.to_string());
            if token.is_some() {
                ()
            }
        }
        ("".to_string(), Err(""))
    }
    fn parse_keywords_identifiers(chars: &[char]) -> (String, Result<TokEnum, &str>) {
        let mut peekable_chars = chars.iter().peekable();
        let mut end_word = 0;
        while peekable_chars.peek().is_some() {
            let item = **peekable_chars.peek().unwrap();
            if item.is_ascii_whitespace() {
                break;
            };

            //Si el primer carácter es numérico, entonces devuelve un error
            if end_word == 0 && item.is_numeric() {
                return (
                    "".to_string(),
                    Err("No se pueden tener números al inicio del nombre de una variable"),
                );
            }

            if item.is_ascii_alphabetic() || item.eq(&'_') {
                end_word = end_word + 1;
                peekable_chars.next();
            }
        }
        if (end_word == 0) {
            return ("".to_string(), Err(""));
        }
        let string_identifier = chars.iter().take(end_word).collect::<String>();
        let token = Tokens::get_keyword(&string_identifier);
        if (token.is_none()) {
            return (string_identifier, Ok(TokEnum::IDENTIFIER));
        }
        (string_identifier, Ok(token.unwrap()))
    }
    fn parse_two_or_one_characters(chars: &[char]) -> (String, Result<TokEnum, &str>) {
        let num_chars = chars.len();
        if num_chars < 2 {
            return ("".to_string(), Err(""));
        }

        let empty_result = ("".to_string(), Err(""));
        match chars.get(0).unwrap() {
            '=' => match chars.get(1).unwrap() {
                '>' => ("=>".to_string(), Ok(TokEnum::ArrowEq)),
                _ => empty_result,
            },

            '<' => match chars.get(1).unwrap() {
                '=' => ("<=".to_string(), Ok(TokEnum::EM)),
                _ => empty_result,
            },
            '>' => match chars.get(1).unwrap() {
                '=' => (">=".to_string(), Ok(TokEnum::EG)),
                _ => empty_result,
            },
            '-' => match chars.get(1).unwrap() {
                '>' => ("->".to_string(), Ok(TokEnum::ArrowSingle)),
                _ => empty_result,
            },
            '&' => match chars.get(1).unwrap() {
                '&' => ("&&".to_string(), Ok(TokEnum::AND)),
                _ => empty_result,
            },
            '|' => match chars.get(1).unwrap() {
                '|' => ("||".to_string(), Ok(TokEnum::DoublePipe)),
                _ => ("".to_string(), Err("")),
            },
            ':' => match chars.get(1).unwrap() {
                ':' => ("".to_string(), Ok(TokEnum::MagicDoubleDot)),
                _ => empty_result,
            },
            '/' => match chars.get(1).unwrap() {
                '*' => ("/*".to_string(), Ok(TokEnum::OBlockComment)),
                '/' => ("//".to_string(), Ok(TokEnum::SCMT)),
                _ => empty_result,
            },
            '.' => match chars.get(1).unwrap() {
                '.' => ("".to_string(), Ok(TokEnum::Dot)),
                _ => empty_result,
            }
            _ => empty_result,
        }
    }

    /// Make the analizer
    pub fn analyzer(&mut self, data: Vec<&str>) {
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
                            // if self.is_start_chain(letter as u8) {
                            //     if word != String::new() {
                            //         word += letter.to_string().as_str();
                            //         index += self.chain_lexemes(line, index.clone());
                            //         word = String::new();
                            //     } else {
                            //         index += self.chain_lexemes(line, index.clone());
                            //     }
                            //     letter = char_min_value;
                            // }

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
                            self.fill_lexemes(letter.to_string().as_str(), self.num_line, None);
                        } else {
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
        let mut token = Tokens::is_bracket_or_scn(lexeme);

        if token.is_some() {
            return token.unwrap();
        }

        //Numeric Types
        if Tokens::is_float(lexeme) {
            return TokEnum::FLOAT;
        } else if Tokens::is_integer(lexeme) {
            return TokEnum::INTEGER;
        }

        //Primitives
        if Tokens::is_primitive(lexeme) {
            return TokEnum::PRIMITIVE;
        }

        //Operators
        token = Tokens::is_operator(lexeme);
        if token.is_some() {
            return token.unwrap();
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

    fn is_start_chain(ascii: u8) -> bool {
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
        // let mut second_mid_data = String::new();
        // let mut token: TokEnum = TokEnum::IDENTIFIER;
        // while index < line.len()
        //     && (!self.is_bracket_or_scn(line[index] as u8)
        //         && !self.is_start_chain(line[index] as u8)
        //         && token != TokEnum::PRIMITIVE
        //         && token != TokEnum::ENCAPSULATION
        //         && !Tokens::is_operator(line[index].to_string().as_str()).0)
        // {
        //     token = self.lexeme_filter(line[index].to_string().as_str());
        //     second_mid_data += line[index].to_string().as_str();
        //     index += 1;
        // }
        //
        // (index - 1, second_mid_data)
        (index - 1, "hola".to_string())
    }
}
