pub enum Cons {
    SCN = 59, //SEMICOLON
    SPACE = 32,
    CR = 13, //CARRIAGUE RETURN
    DOT = 46,
    STRING = 34,
    CHAR = 39,
    ORB = 40, //OPEN ROUND BRACKET
    CRB = 41, //CLOSE ROUND BRACKET
    OSB = 91, //OPEN SQUARE BRACKET
    CSB = 93, //CLOSE SQUARE BRACKET
    OB = 123, //OPEN BRACKET
    CB = 125, //CLOSE BRACKET
    SLASH = 47,
    EQUALS = 61,
    NOT = 33,
    MULT = 42,
    POW = 94,
    MINUS = 45,
    ADD = 43,
}

pub const W_INTERNAL:&str = "internal";
pub const W_PUBLIC:&str = "public";
pub const W_PRIVATE:&str = "private";
pub const W_NAMESPACE:&str = "namespace";
pub const W_USING:&str = "using";
pub const W_CLASS:&str = "class";
pub const W_STATIC:&str = "static";

pub const W_VOID:&str = "void";
pub const W_STRING:&str = "string";
pub const W_CHAR:&str = "char";
pub const W_INT:&str = "int";
pub const W_FLOAT:&str = "float";
pub const W_BOOL:&str = "bool";