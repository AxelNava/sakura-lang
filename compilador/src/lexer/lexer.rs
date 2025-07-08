use super::lexer_tokens::*;
use crate::lexer::print_elements_lexer::PrintElementsLexer;

pub struct Lexeme {
    pub word: String,
    pub token: TokEnum,
    pub line: i32,
}

impl Lexeme {
    pub fn new() -> Lexeme {
        Lexeme {
            word: String::new(),
            token: TokEnum::IDENTIFIER,
            line: 0,
        }
    }
}
pub struct Lexer {
    pub lexemes: Vec<(String, TokEnum)>,
    num_line: i32,
    pub errors_lexer: Vec<String>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            lexemes: Vec::new(),
            num_line: 0,
            errors_lexer: Vec::new(),
        }
    }
    fn is_newline(char: char) -> bool {
        char == '\n' || char == '\r'
    }
    pub fn analyze(&mut self, content: String) -> Result<Vec<(String, TokEnum)>, String> {
        let characters = content.as_str().chars().collect::<Vec<char>>();
        let mut tokens = Vec::<TokEnum>::new();
        let mut current_pos = 0;
        let mut num_lines = 0;

        //Check if the first character is BOM(Byte Order Mark, for windows only)
        if characters.get(0) == Some(&'\u{feff}') {
            current_pos = current_pos + 1;
        };
        let total_length = content.chars().count();
        while current_pos < total_length {
            if current_pos == 855 {
                println!("current_pos: {}", current_pos);
            }

            if Self::is_newline(characters[current_pos]) || characters[current_pos].is_whitespace()
            {
                current_pos = current_pos + 1;
                if Self::is_newline(characters[current_pos]) {
                    num_lines = num_lines + 1;
                }
                continue;
            }

            let mut result_parse: (String, Result<TokEnum, &str>) =
                Self::parse_two_or_one_characters(&characters[current_pos..]);

            if result_parse.1.is_ok() {
                let token = result_parse.1?;
                tokens.push(token.clone());
                self.lexemes
                    .push((result_parse.0.to_string(), token.clone()));
                current_pos = current_pos + result_parse.0.len();

                if token.eq(&TokEnum::OBlockComment) || token.eq(&TokEnum::SCMT) {
                    let is_block = token.eq(&TokEnum::OBlockComment);
                    let ignored_chars =
                        Self::ignore_block_comment(&characters[current_pos..], is_block);
                    current_pos = current_pos + ignored_chars as usize;
                }
                continue;
            }

            result_parse = Self::parse_one_character(&characters[current_pos..]);
            if result_parse.1.is_ok() {
                let token = result_parse.1?;
                tokens.push(token.clone());
                self.lexemes.push((result_parse.0.to_string(), token));
                current_pos = current_pos + result_parse.0.len();
                continue;
            }

            result_parse = Self::parse_number(&characters[current_pos..]);
            if result_parse.1.is_ok() {
                let token = result_parse.1?;
                tokens.push(token.clone());
                self.lexemes.push((result_parse.0.to_string(), token));
                current_pos = current_pos + result_parse.0.len();
                continue;
            }

            result_parse = Self::parse_keywords_identifiers(&characters[current_pos..]);

            if let Err(error) = result_parse.1 {
                if !error.is_empty() {
                    self.errors_lexer.push(error.to_string());
                    current_pos = current_pos + 1;
                    continue;
                }
            } else {
                current_pos = current_pos + result_parse.0.len();
                let token = result_parse.1?;
                tokens.push(token.clone());
                self.lexemes.push((result_parse.0.to_string(), token));
                continue;
            }

            let character_unreadable = characters[current_pos];
            self.errors_lexer.push(format!(
                "No se reconoce el carácter {character_unreadable}, en la línea: {num_lines}"
            ));
            current_pos = current_pos + 1;
        }
        println!("El último token es: {:?}", tokens[tokens.len() - 1]);
        let printer = PrintElementsLexer::new("lexer.txt".to_string());
        printer
            .print_elements(&self.lexemes)
            .expect("TODO: panic message");
        Ok(self.lexemes.clone())
    }
    fn parse_number(chars: &[char]) -> (String, Result<TokEnum, &str>) {
        let mut binding = chars.iter().peekable();
        let mut final_number: String = "".to_string();
        let mut is_decimal = false;
        loop {
            let char_to_check = binding.peek();
            if char_to_check.is_some() {
                let char = **char_to_check.unwrap();
                if char.is_numeric() {
                    binding.next();
                    final_number.push_str(&char.to_string());
                } else {
                    if char == '.' && is_decimal == false {
                        is_decimal = true;
                        final_number.push_str(&char.to_string());
                        continue;
                    }
                    if is_decimal == true {
                        return (
                            "".to_string(),
                            Err("No se puede tener más de un punto decimal en un número decimal"),
                        );
                    }
                    if (final_number.is_empty()) {
                        return ("".to_string(), Err(""));
                    }
                    return (
                        final_number.to_string(),
                        Ok(match is_decimal {
                            false => TokEnum::INTEGER,
                            true => TokEnum::FLOAT,
                        }),
                    );
                }
            }else{
                if (final_number.is_empty()) {
                    return ("".to_string(), Err(""));
                }
                return (
                    final_number.to_string(),
                    Ok(match is_decimal {
                        false => TokEnum::INTEGER,
                        true => TokEnum::FLOAT,
                    }),
                );
            }
        }
    }

    fn parse_one_character(chars: &[char]) -> (String, Result<TokEnum, &str>) {
        let mut binding = chars.iter().peekable();
        let char_to_check = binding.peek();
        if char_to_check.is_some() {
            let char = **char_to_check.unwrap();
            let string_char = char.to_string();
            let mut token = Tokens::is_operator(&string_char);
            if token.is_some() {
                return (string_char, Ok(token.unwrap()));
            } else {
                token = Tokens::is_bracket_or_scn(&[char]);
                if token.is_some() {
                    return (string_char, Ok(token.unwrap()));
                }
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

            if item.is_ascii_alphanumeric() || item.eq(&'_') {
                end_word = end_word + 1;
                peekable_chars.next();
            } else {
                break;
            }
        }
        if end_word == 0 {
            return ("".to_string(), Err(""));
        }
        let string_identifier = chars.iter().take(end_word).collect::<String>();
        let token = Tokens::get_keyword(&string_identifier);
        if token.is_none() {
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
                '=' => ("==".to_string(), Ok(TokEnum::EQUAL)),
                _ => ("=".to_string(), Ok(TokEnum::ASSIGNATION)),
            },

            '<' => match chars.get(1).unwrap() {
                '=' => ("<=".to_string(), Ok(TokEnum::EM)),
                _ => ("<".to_string(), Ok(TokEnum::MINOR)),
            },
            '>' => match chars.get(1).unwrap() {
                '=' => (">=".to_string(), Ok(TokEnum::EG)),
                _ => (">".to_string(), Ok(TokEnum::GREATER)),
            },
            '-' => match chars.get(1).unwrap() {
                '>' => ("->".to_string(), Ok(TokEnum::ArrowSingle)),
                _ => ("-".to_string(), Ok(TokEnum::MINUS)),
            },
            '&' => match chars.get(1).unwrap() {
                '&' => ("&&".to_string(), Ok(TokEnum::AND)),
                _ => ("&".to_string(), Ok(TokEnum::AMPERSAND)),
            },
            '|' => match chars.get(1).unwrap() {
                '|' => ("||".to_string(), Ok(TokEnum::DoublePipe)),
                _ => ("|".to_string(), Ok(TokEnum::PIPE)),
            },
            ':' => match chars.get(1).unwrap() {
                ':' => ("::".to_string(), Ok(TokEnum::MagicDoubleColon)),
                _ => (":".to_string(), Ok(TokEnum::COLON)),
            },
            '/' => match chars.get(1).unwrap() {
                '*' => ("/*".to_string(), Ok(TokEnum::OBlockComment)),
                '/' => ("//".to_string(), Ok(TokEnum::SCMT)),
                _ => ("/".to_string(), Ok(TokEnum::DIVIDE)),
            },
            '.' => match chars.get(1).unwrap() {
                '.' => ("..".to_string(), Ok(TokEnum::DoubleDot)),
                _ => (".".to_string(), Ok(TokEnum::Dot)),
            },
            ',' => (",".to_string(), Ok(TokEnum::COMA)),
            '?' => ("?".to_string(), Ok(TokEnum::QuestionMark)),
            '*' => match chars.get(1).unwrap() {
                '/' => ("*/".to_string(), Ok(TokEnum::CBlockComment)),
                _ => ("*".to_string(), Ok(TokEnum::Asterisk)),
            },
            '(' => ("(".to_string(), Ok(TokEnum::OB)),
            _ => empty_result,
        }
    }
    /// Get the num of character that are inside the comment
    ///
    /// # Arguments
    ///
    /// * `chars`:
    ///
    /// returns: i64
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn ignore_block_comment(chars: &[char], is_block: bool) -> i64 {
        let mut peekable_chars = chars.iter().peekable();
        let mut num_chars = 0;
        if is_block {
            while peekable_chars.peek().is_some() {
                let item = **peekable_chars.peek().unwrap();
                if item.eq(&'*') {
                    peekable_chars.next();
                    if peekable_chars.peek().is_some() && peekable_chars.peek().unwrap().eq(&&'/') {
                        return num_chars;
                    } else {
                        peekable_chars.next();
                        num_chars = num_chars + 1;
                    }
                } else {
                    peekable_chars.next();
                    num_chars = num_chars + 1;
                }
            }
        }
        while peekable_chars.peek().is_some() {
            let item = **peekable_chars.peek().unwrap();
            if item.eq(&'\n') {
                return num_chars;
            } else {
                peekable_chars.next();
                num_chars = num_chars + 1;
            }
        }
        0
    }
}
