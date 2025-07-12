use crate::lexer::lexer_tokens::TokEnum;
use crate::sintax::rules::action_lr_table::ActionLrTable;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use strum::EnumCount;

#[derive(Debug, Clone)]
pub enum CLRActions {
    GoTo,
    Shift,
    Reduce,
    Accept,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum TokenType<T>
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
    /// * `current_stack_tokens`: El stack actual de los tokens analizados
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
        current_stack_tokens: &mut Vec<String>
    ) {
        let mut stack_tokens: Vec<(i16, TokenType<T>)> =
            vec![(0, TokenType::Terminal(TokEnum::EmptyToken))];
        let iter_lexemes = lexemes.iter().peekable();
        let mut current_state = stack_tokens.get(0).unwrap().0;
        
        for (lexeme, token) in iter_lexemes {
            let token_string = token.to_string();
            let state_actions = lr_table.get(&current_state).unwrap();
            let possible_action = state_actions.get(&token_string);
            if possible_action.is_none() {
                panic!("Error: No action found for token: {}", token_string);
            }
            let action = possible_action.unwrap();
            //TODO: Para realizar las acciones, se necesitan 2 pilas, una para los tokens globales, y otra pila de comprobación, en
            //la pila de comprobación se irán poniendo los tokens que se vayan comprobando, y en la global tanto los terminales como los no
            //terminales, en esta se pueden poner las reducciones, de esta forma se puede tener un estado y a la vez el siguiente token de entrada
            //para poder averiguar cual será el siguiente estado a donde ir, así siempre se irán poniendo de 2 en 2 los tokens
            //en la pila de comprobación y siempre se podrá tener el estado a donde debe de ir (o en donde debe de comprobar) el siguiente token
            match action.action {
                CLRActions::GoTo => {
                    stack_tokens.push((action.goto_state, TokenType::Terminal(token.clone())));
                }
                CLRActions::Shift => {
                    stack_tokens.push((action.goto_state, TokenType::Terminal(token.clone())));
                }
                CLRActions::Reduce => {
                    let token_reduce = action.get_production(&mut stack_tokens);
                    current_state = stack_tokens.get(0).unwrap().0;
                }
                CLRActions::Accept => {}
            }
        }
    }
}
