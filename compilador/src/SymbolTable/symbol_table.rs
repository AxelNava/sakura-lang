use crate::lexer::lexer_tokens::TokTypeEnum;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

pub enum TypeAmbit {
    Global,
    Function,
    AnonymousFunction,
}
pub struct IdentifierValues {
    pub parent_ambit: String,
    pub line: String,
    pub local_lines: String,
    pub lexeme: String,
    pub token_type: TokTypeEnum,
    pub value: String,
    pub memory_location: String,
}
pub struct SymbolTable {
    records: HashMap<u64, IdentifierValues>,
    pub type_ambit: TypeAmbit,
    pub has_parent: bool,
}
impl SymbolTable {
    pub fn initialize(type_ambit: TypeAmbit) -> SymbolTable {
        match type_ambit {
            TypeAmbit::Global => SymbolTable {
                records: HashMap::new(),
                type_ambit,
                has_parent: false,
            },
            _ => SymbolTable {
                records: HashMap::new(),
                type_ambit,
                has_parent: true,
            },
        }
    }
    fn hash_lexeme(lexeme: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        lexeme.hash(&mut hasher);
        hasher.finish()
    }
    pub fn add_identifier(mut self, identifier_values: IdentifierValues) -> bool {
        let hash = SymbolTable::hash_lexeme(&identifier_values.lexeme);
        self.records.insert(hash, identifier_values).is_some()
    }
    pub fn find_identifier(&self, lexeme: &str) -> bool {
        let hash = &SymbolTable::hash_lexeme(lexeme);
        self.records.contains_key(hash)
    }
}
