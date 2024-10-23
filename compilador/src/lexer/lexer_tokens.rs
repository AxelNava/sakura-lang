use strum_macros::{EnumString, AsRefStr};

use super::lexer_cons::*;

#[derive(EnumString, AsRefStr, PartialEq)]
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
    SEMICOLON, 
    COLON, 
    INTEGER, 
    FLOAT, 
    BOOL, 
    CHAR, 
    ADD, 
    SUBS, 
    PLUS, 
    MINUS, 
    MULT, 
    DIVIDE, 
    SQR, 
    POW, 
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
        if lexeme == ((Cons::ADD as u8) as char).to_string() {return (true, TokEnum::ADD);}
        if lexeme == ((Cons::MINUS as u8) as char).to_string() {return (true, TokEnum::MINUS);}
        if lexeme == ((Cons::SLASH as u8) as char).to_string() {return (true, TokEnum::DIVIDE);}
        if lexeme == ((Cons::POW as u8) as char).to_string() {return (true, TokEnum::POW);}
        if lexeme == ((Cons::MULT as u8) as char).to_string() {return (true, TokEnum::MULT);}
        if lexeme == ((Cons::EQUALS as u8) as char).to_string() {return (true, TokEnum::ASSIGNATION);}
        
        (false, TokEnum::IDENTIFIER)
    }

    pub fn is_reserved_word(lexeme: &str) -> (bool, TokEnum) {
        if lexeme == W_USING { return (true, TokEnum::USING);}
        if lexeme == W_VOID { return (true, TokEnum::VOID);}
        if lexeme == W_NAMESPACE { return (true, TokEnum::NAMESPACE);}
        if lexeme == W_CLASS { return (true, TokEnum::CLASS);}
        
        (false, TokEnum::IDENTIFIER)
    }
    
    pub fn is_bracket_or_scn(lexeme: &str) -> (bool, TokEnum) {
        if lexeme == ((Cons::SCN as u8) as char).to_string() {return (true, TokEnum::SEMICOLON);}
        if lexeme == ((Cons::OB as u8) as char).to_string() {return (true, TokEnum::OB);}
        if lexeme == ((Cons::CB as u8) as char).to_string() {return (true, TokEnum::CB);}
        if lexeme == ((Cons::OSB as u8) as char).to_string() {return (true, TokEnum::OSB);}
        if lexeme == ((Cons::CSB as u8) as char).to_string() {return (true, TokEnum::CSB);}
        if lexeme == ((Cons::ORB as u8) as char).to_string() {return (true, TokEnum::ORB);}
        if lexeme == ((Cons::CRB as u8) as char).to_string() {return (true, TokEnum::CRB);}
        (false, TokEnum::IDENTIFIER)
    }
    
    pub fn get_token_type(token: TokEnum) -> TokTypeEnum {
        match token {
            TokEnum::USING | TokEnum::VOID | TokEnum::ENCAPSULATION 
            | TokEnum::CLASS => TokTypeEnum::RESERVED,
            _ => TokTypeEnum::IDENTIFIER,
        }
    }

    pub fn is_encapsulation(lexeme: &str) -> bool {
        matches!(lexeme, W_PRIVATE | W_STATIC| W_PUBLIC 
            | W_INTERNAL)
    }
    
    pub fn is_primitive(lexeme: &str) -> bool {
        matches!(lexeme, W_STRING | W_CHAR 
            | W_INT | W_FLOAT 
            | W_BOOL)
    }
    
    pub fn item_addable(token: TokEnum) -> bool {
        matches!(token, TokEnum::USING | TokEnum::VOID 
            | TokEnum::ENCAPSULATION | TokEnum::CLASS 
            | TokEnum::IDENTIFIER)
    }

}
