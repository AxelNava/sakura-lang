pub enum Cons {
    SCN = 59,
    SPACE = 32,
    CR = 13,
    DOT = 46,
    STRING = 34,
    CHAR = 39,
    ORB = 40,
    CRB = 41,
    OSB = 91,
    CSB = 93,
    OB = 123,
    CB = 125,
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