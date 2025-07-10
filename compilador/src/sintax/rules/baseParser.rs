use crate::lexer::lexer_tokens::TokEnum;
use crate::sintax::rules::action_lr_table::ActionLrTable;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use strum::EnumCount;
use strum_macros::EnumCount;

#[derive(Debug, Clone)]
pub enum CLRActions {
    Shift,
    Reduce,
    Accept,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum ParserCharacters<T>
where
    T: Hash + Eq + PartialEq,
{
    Terminal(TokEnum),
    NonTerminal(T),
}

pub struct Parser<T>
where
    T: EnumCount + Hash + Eq + PartialEq,
{
    phantom_data: PhantomData<T>,
    pub errors: Vec<(String, i32, TokEnum, TokEnum)>,
}
impl<T> Parser<T>
where
    T: EnumCount + Hash + Eq + PartialEq + Clone,
{
    pub fn new() -> Self {
        Self {
            phantom_data: PhantomData,
            errors: vec![],
        }
    }
    pub fn add_new_error(
        &mut self,
        line: i32,
        token: TokEnum,
        expected: TokEnum,
        _message: String,
    ) {
        self.errors.push((_message, line, token, expected));
    }
    pub fn get_errors(&self) -> &Vec<(String, i32, TokEnum, TokEnum)> {
        &self.errors
    }
    /// run the parser algorithm for the rest of the lexemes available
    /// # Arguments
    ///
    /// * `lexemes`: Lexemes to analyze
    /// * `lr_table`: Rules for the LR(1) table, the is the hash map that
    /// contains the state, and all the actions to perform on that state
    ///
    /// returns: (h) TODO:  It must return the AST of the rule to parse
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ````
    pub fn run_parser(
        &self,
        lexemes: Vec<(String, TokEnum)>,
        lr_table: HashMap<i16, HashMap<String, ActionLrTable<T>>>,
    ) {
        
    }
}
