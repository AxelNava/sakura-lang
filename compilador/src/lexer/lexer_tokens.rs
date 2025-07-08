use strum_macros::{AsRefStr, Display, EnumString};

use super::lexer_cons::*;

/// Este enum es una representación de todos los tokens disponibles para el lenguaje y que serán reconocidos por el analizador
/// léxico
#[derive(EnumString, AsRefStr, PartialEq, Debug, Display, Clone, Hash, Eq)]
pub enum TokEnum {
    IDENTIFIER,
    PRIMITIVE,
    ///Open Round Brackets 
    ORB,
    ///Close Round Brackets
    CRB,
    ///Open Square Brackets
    OSB,
    ///Close Square Brackets
    CSB,
    ///Open Brackets
    OB,
    ///Close Brackets
    CB,  
    ASSIGNATION,
    ///SimpleComment
    SCMT, 
    ///Open the block comment "/*"
    OBlockComment,
    ///Close block comment "*/"
    CBlockComment,
    STRING,
    SEMICOLON,
    ///Double dot is ".." for a rest operator like \[other..] or ranges 8..20
    DoubleDot,
    Dot,
    MagicDoubleColon,
    COMA,
    COLON,
    INTEGER,
    FLOAT,
    BOOL,
    CHAR,
    ADD,
    SUBS,
    PLUS,
    MINUS,
    Asterisk,
    DIVIDE,
    SQR,
    POW,
    ENCAPSULATION,
    EQUAL,
    GREATER,
    MINOR,
    ///Equal or greater
    EG,
    ///Equal or minor
    EM,
    QuestionMark,
    ArrowEq,
    ArrowSingle,
    ///This means that can be an or operator or a empty closure params init, like '()' but '||'
    DoublePipe,
    AMPERSAND,
    AND,
    PIPE,
    BinaryAnd,
    WStatic,
    WIf,
    WElse,
    WWhile,
    WFor,
    WLoop,
    WReturn,
    WBreak,
    WContinue,
    WMatch,
    WPub,
    WFn,
    WLet,
    WSelfLower,
    WSelfUpper,
    WNew,
    WTrue,
    WFalse,
    WAs,
    WIs,
    WChar,
    WInter8,
    WInter16,
    WInter32,
    WInter64,
    WFloat32,
    WFloat64,
    WString,
    WEnum,
    WStruct,
    WUnion,
    WImpl,
    WTrait,
    EndToken,
}
pub enum TokTypeEnum {
    IDENTIFIER,
    INT,
    CHAR,
    STRING,
    FLOAT,
    BOOL,
    RESERVED,
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

    pub fn is_integer(lexeme: &str) -> bool {
        lexeme.parse::<i32>().is_ok()
    }

    pub fn is_operator(lexeme: &str) -> Option<TokEnum> {
        if lexeme == ((Cons::ADD as u8) as char).to_string() {
            return Some(TokEnum::ADD);
        }
        if lexeme == ((Cons::MINUS as u8) as char).to_string() {
            return Some(TokEnum::MINUS);
        }
        if lexeme == ((Cons::SLASH as u8) as char).to_string() {
            return Some(TokEnum::DIVIDE);
        }
        if lexeme == ((Cons::POW as u8) as char).to_string() {
            return Some(TokEnum::POW);
        }
        if lexeme == ((Cons::MULT as u8) as char).to_string() {
            return Some(TokEnum::Asterisk);
        }
        if lexeme == ((Cons::EQUALS as u8) as char).to_string() {
            return Some(TokEnum::ASSIGNATION);
        }
        let single_char = match lexeme {
            "|" => Some(TokEnum::PIPE),
            "&" => Some(TokEnum::BinaryAnd),
            ":" => Some(TokEnum::DoubleDot),
            "," => Some(TokEnum::COLON),
            "." => Some(TokEnum::Dot),
            _ => None,
        };

        single_char
    }

    ///Comprueba si es un '{' '}' o un ';'
    pub fn is_bracket_or_scn(chars: &[char]) -> Option<TokEnum> {
        let char = chars.get(0).unwrap();
        let convert_char = |char_ascii: u8| -> char { char_ascii as char };

        if char == &convert_char(Cons::SCN as u8) {
            return Some(TokEnum::SEMICOLON);
        }
        if char == &convert_char(Cons::OB as u8) {
            return Some(TokEnum::OB);
        }
        if char == &convert_char(Cons::CB as u8) {
            return Some(TokEnum::CB);
        }
        if char == &convert_char(Cons::OSB as u8) {
            return Some(TokEnum::OSB);
        }
        if char == &convert_char(Cons::CSB as u8) {
            return Some(TokEnum::CSB);
        }
        if char == &convert_char(Cons::ORB as u8) {
            return Some(TokEnum::ORB);
        }
        if char == &convert_char(Cons::CRB as u8) {
            return Some(TokEnum::CRB);
        }
        None
    }
    pub fn get_keyword(value: &str) -> Option<TokEnum> {
        match value {
            W_STATIC => Some(TokEnum::WStatic),
            W_IF => Some(TokEnum::WIf),
            W_ELSE => Some(TokEnum::WElse),
            W_WHILE => Some(TokEnum::WWhile),
            W_FOR => Some(TokEnum::WFor),
            W_LOOP => Some(TokEnum::WLoop),
            W_RETURN => Some(TokEnum::WReturn),
            W_BREAK => Some(TokEnum::WBreak),
            W_CONTINUE => Some(TokEnum::WContinue),
            W_MATCH => Some(TokEnum::WMatch),
            W_PUB => Some(TokEnum::WPub),
            W_FN => Some(TokEnum::WFn),
            W_LET => Some(TokEnum::WLet),
            W_SELFLOWER => Some(TokEnum::WSelfLower),
            W_SELFUPPER => Some(TokEnum::WSelfUpper),
            W_NEW => Some(TokEnum::WNew),
            W_TRUE => Some(TokEnum::WTrue),
            W_FALSE => Some(TokEnum::WFalse),
            W_AS => Some(TokEnum::WAs),
            W_IS => Some(TokEnum::WIs),
            W_CHAR => Some(TokEnum::WChar),
            W_INTER8 => Some(TokEnum::WInter8),
            W_INTER16 => Some(TokEnum::WInter16),
            W_INTER32 => Some(TokEnum::WInter32),
            W_INTER64 => Some(TokEnum::WInter64),
            W_FLOAT32 => Some(TokEnum::WFloat32),
            W_FLOAT64 => Some(TokEnum::WFloat64),
            W_STRING => Some(TokEnum::WString),
            W_ENUM => Some(TokEnum::WEnum),
            W_STRUCT => Some(TokEnum::WStruct),
            W_UNION => Some(TokEnum::WUnion),
            W_IMPL => Some(TokEnum::WImpl),
            W_TRAIT => Some(TokEnum::WTrait),

            _ => None,
        }
    }
}
