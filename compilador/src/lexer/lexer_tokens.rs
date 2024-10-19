use std::env::consts::FAMILY;

use super::lexer_cons::*;

#[derive(PartialEq)]
pub enum TokEnum {
    IDENTIFIER,
    PRIMITIVE,
    ORB, //Open Round Brackets
    CRB, //Close Round Brackets
    OSB, //Open Square Brackets
    CSB, //Close Square Brackets
    OB, //Open Brackets
    CB, //Close Brackets
    ASSIGNATION, 
    SCMT, //SimpleComment
    STRING,
    SEMICOLON, //PyC
    COLON, //dos puntos
    INTEGER, //int
    FLOAT, //float
    BOOL, //bool
    CHAR, //char
    ADD, //suma
    SUBS, //resta
    PLUS, //mas 
    MINUS, //menos
    MULT, //multiplicacion
    DIVIDE, //division
    SQR, //raiz
    POW, //exponente
    USING,
    NAMESPACE,
    CLASS,
    ENCAPSULATION,
    VOID
}
pub enum TokTypeEnum {
    IDENTIFIER,
    INT,
    CHAR,
    STRING,
    FLOAT,
    BOOL,
    RESERVED
}

pub struct Tokens;

impl Tokens {
    
    pub fn is_float(lexeme: &str) -> bool {
        // Intentamos parsear la cadena como un f64
        if let Ok(_float_value) = lexeme.parse::<f64>() {
            // Verificamos si contiene un punto
            return lexeme.contains('.');
        }
        false
    }

    pub fn is_integer(lexeme: &str) -> bool { lexeme.parse::<i32>().is_ok() }
    
    pub fn is_operator(lexeme: &str) -> (bool, TokEnum) {
        match lexeme {
            x if x == (Cons::ADD as u8).to_string() => return (true, TokEnum::ADD),
            x if x == (Cons::MINUS as u8).to_string() => return (true, TokEnum::ADD),
            x if x == (Cons::SLASH as u8).to_string() => return (true, TokEnum::ADD),
            x if x == (Cons::POW as u8).to_string() => return (true, TokEnum::ADD),
            x if x == (Cons::MULT as u8).to_string() => return (true, TokEnum::ADD),
            x if x == (Cons::EQUALS as u8).to_string() => return (true, TokEnum::ADD),
            _ => return (false, TokEnum::IDENTIFIER),
        }
    }

    pub fn is_reserved_word(lexeme: &str) -> (bool, TokEnum) {
        match lexeme {
            x if x == W_USING => return (true, TokEnum::USING),
            x if x == W_VOID => return (true, TokEnum::VOID),
            x if x == W_NAMESPACE => return (true, TokEnum::NAMESPACE),
            x if x == W_CLASS => return (true, TokEnum::CLASS),
            _ => return (false, TokEnum::IDENTIFIER),
        }
    }
    
    pub fn is_bracket_or_scn(lexeme: &str) -> (bool, TokEnum) {
        match lexeme {
            x if x == (Cons::SCN as u8).to_string() => return (true, TokEnum::ADD),
            x if x == (Cons::OB as u8).to_string() => return (true, TokEnum::ADD),
            x if x == (Cons::CB as u8).to_string() => return (true, TokEnum::ADD),
            x if x == (Cons::OSB as u8).to_string() => return (true, TokEnum::ADD),
            x if x == (Cons::ORB as u8).to_string() => return (true, TokEnum::ADD),
            x if x == (Cons::CRB as u8).to_string() => return (true, TokEnum::ADD),
            _ => return (false, TokEnum::IDENTIFIER),
        }
    }
    
    pub fn get_token_type(token: TokEnum) -> TokTypeEnum {
        match token {
            TokEnum::USING | TokEnum::VOID | TokEnum::ENCAPSULATION 
            | TokEnum::CLASS => TokTypeEnum::RESERVED,
            _ => TokTypeEnum::IDENTIFIER,
        }
    }

    pub fn is_encapsulation(lexeme: &str) -> bool {
        match lexeme {
            x if x == W_PRIVATE => return true,
            x if x == W_STATIC => return true,
            x if x == W_PUBLIC => return true,
            x if x == W_INTERNAL => return true,
            _ => return false,
        }
    }
    
    pub fn is_primitive(lexeme: &str) -> bool {
        match lexeme {
            x if x == W_STRING => return true,
            x if x == W_CHAR => return true,
            x if x == W_INT => return true,
            x if x == W_FLOAT=> return true,
            x if x == W_BOOL => return true,
            _ => return false,
        }
    }
    
    
    pub fn item_addable(token: TokEnum) -> bool {
        matches!(token, TokEnum::USING | TokEnum::VOID 
            | TokEnum::ENCAPSULATION | TokEnum::CLASS 
            | TokEnum::IDENTIFIER)
    }

}
