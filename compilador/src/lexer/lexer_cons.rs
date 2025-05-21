use std::path::Iter;

///El enum representa los valores ASCII de los tokens mencionados,
/// por ejemplo el token ';' tiene el valor ASCII 59.
/// Estos tokens son constantes, por lo que siempre serán los mismos independientemente del lenguaje utilizado
pub enum Cons {
    ///SEMICOLON
    SCN = 59, //SEMICOLON
    SPACE = 32,
    ///CARRIAGE RETURN
    CR = 13,
    DOT = 46,
    STRING = 34,
    CHAR = 39,
    ///OPEN ROUND BRACKET
    ORB = 40,
    ///CLOSE ROUND BRACKET
    CRB = 41,
    ///OPEN SQUARE BRACKET
    OSB = 91,
    ///CLOSE SQUARE BRACKET
    CSB = 93,
    ///OPEN BRACKET
    OB = 123,
    ///CLOSE BRACKET
    CB = 125,
    SLASH = 47,
    EQUALS = 61,
    NOT = 33,
    MULT = 42,
    POW = 94,
    MINUS = 45,
    ADD = 43,
}

pub const W_STATIC: &str = "static";
pub const W_IF: &str = "if";
pub const W_ELSE: &str = "else";
pub const W_WHILE: &str = "while";
pub const W_FOR: &str = "for";
pub const W_LOOP: &str = "loop";
pub const W_RETURN: &str = "return";
pub const W_BREAK: &str = "break";
pub const W_CONTINUE: &str = "continue";
pub const W_MATCH: &str = "match";
pub const W_PUB: &str = "pub";
pub const W_FN: &str = "fn";
pub const W_LET: &str = "let";
pub const W_SELFLOWER: &str = "selflower";
pub const W_SELFUPPER: &str = "selfupper";
pub const W_NEW: &str = "new";
pub const W_TRUE: &str = "true";
pub const W_FALSE: &str = "false";
pub const W_AS: &str = "as";
pub const W_IS: &str = "is";
pub const W_CHAR: &str = "char";
pub const W_INTER8: &str = "inter8";
pub const W_INTER16: &str = "inter16";
pub const W_INTER32: &str = "inter32";
pub const W_INTER64: &str = "inter64";
pub const W_FLOAT32: &str = "float32";
pub const W_FLOAT64: &str = "float64";
pub const W_STRING: &str = "string";
pub const W_ENUM: &str = "enum";
pub const W_STRUCT: &str = "struct";
pub const W_UNION: &str = "union";
pub const W_IMPL: &str = "impl";
pub const W_TRAIT: &str = "trait";
