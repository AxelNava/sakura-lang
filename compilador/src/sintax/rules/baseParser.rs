use crate::lexer::lexer_tokens::TokEnum;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::thread::current;
use strum::EnumCount;
use strum_macros::EnumCount;

/// Enum that represent the state of the LR(1) table
/// The action variant is for terminal characters, the
/// goto variant is for non-terminal characters, this has to
/// be an enum with all the non-terminal characters
pub enum ParserState<T> {
    Action(TokEnum, CLRActions, i16),
    Goto(T, String, i16),
}

#[derive(Debug, Clone)]
pub enum CLRActions {
    Shift,
    Reduce,
    Accept,
}

pub enum TokenType {
    Terminal,
    NonTerminal,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum ParserCharacters<T>
where
    T: Hash + Eq + PartialEq,
{
    Terminal(TokEnum),
    NonTerminal(T),
}

pub struct AST {
    pub name: String,
    pub type_token: TokenType,
    pub children: Vec<AST>,
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
    T: EnumCount + Hash + Eq + PartialEq,
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
    ///
    ///
    /// # Arguments
    ///
    /// * `lexemes`: All lexemes to analyze
    /// * `lr_table`: HashMap of HashMap, the first level has the state and all the posibles values to do,
    /// the second level has the actions to execute and the rest of the data to do the action
    ///
    /// returns: () TODO: return the AST
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn run_parser(
        &self,
        lexemes: Vec<(String, TokEnum)>,
        lr_table: HashMap<i16, HashMap<ParserCharacters<T>, (CLRActions, i16)>>,
    ) {
        Self::run(lexemes, lr_table);
    }
}
impl<T> BaseParser<T> for Parser<T> where T: EnumCount + Hash + Eq + PartialEq {}

trait BaseParser<T>
where
    T: EnumCount + Hash + Eq + PartialEq,
{
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
    /// ```
    fn run(
        lexemes: Vec<(String, TokEnum)>,
        lr_table: HashMap<i16, HashMap<ParserCharacters<T>, (CLRActions, i16)>>,
    ) {
        let mut peekable_lexemes = lexemes.iter().peekable();
        let lexeme = peekable_lexemes.peek();
        let value_terminal_lexeme = *lexeme.unwrap();

        let mut current_state = 0;
        let mut last_token: ParserCharacters<T>;
        let mut accept = false;
        loop {
            peekable_lexemes.next();
            let value_state_lexeme = peekable_lexemes.peek();
            let lexeme = *value_state_lexeme.unwrap();
            let value_table_state = lr_table.get(&current_state).unwrap();
            let token_value = lexeme.1.clone();
            let value_state_action =
                value_table_state.get_key_value(&ParserCharacters::Terminal(token_value));

            if value_state_action.is_none() {
                //TODO: This have to change to continue the parser
                panic!("Error, no action for the token");
            }

            let action_token = value_state_action.unwrap().1;
            let action = &action_token.0;
            let next_state = action_token.1;
            let next_action = match action {
                CLRActions::Shift => {
                    current_state = next_state;
                }
                CLRActions::Reduce => {
                    
                }
                CLRActions::Accept => {
                    current_state = next_state;
                    accept = true;
                    break;
                }
            };
        }
        if accept {
            let last_state = lr_table.get(&current_state).unwrap();
        }
    }
}
