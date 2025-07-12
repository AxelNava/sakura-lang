use crate::lexer::lexer_tokens::TokEnum;
use crate::sintax::rules::action_lr_table::ActionLrTable;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::marker::PhantomData;
use strum::EnumCount;
use strum_macros::Display;

#[derive(Debug, Clone)]
pub enum CLRActions {
    GoTo,
    Shift,
    Reduce,
    Accept,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Display)]
pub enum TokenType<T>
where
    T: Hash + Eq + PartialEq + Display,
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
    T: EnumCount + Hash + Eq + PartialEq + Clone + Display,
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
        lr_table: HashMap<i16, HashMap<String, ActionLrTable<T>>>,
        current_stack_tokens: &mut Vec<String>,
    ) -> bool {
        let mut stack_tokens: Vec<(i16, String)> = vec![(0, "".to_string())];

        while current_stack_tokens.len() > 0 {
            let current_token = current_stack_tokens.pop();
            if current_token.is_none() {
                break;
            }
            let _unwrapped_token: &String = &current_token.unwrap();

            let current_stack_state = stack_tokens.get(stack_tokens.len() - 1).unwrap();
            let reference_state = current_stack_state.0;

            if !(lr_table.contains_key(&current_stack_state.0)
                && lr_table
                    .get(&current_stack_state.0)
                    .unwrap()
                    .contains_key(_unwrapped_token))
            {
                break;
            }

            let next_action = lr_table
                .get(&reference_state)
                .unwrap()
                .get(_unwrapped_token)
                .unwrap();

            match next_action.action {
                CLRActions::GoTo => {
                    stack_tokens.push((next_action.goto_state, _unwrapped_token.clone()));
                }
                CLRActions::Shift => {
                    stack_tokens.push((next_action.goto_state, _unwrapped_token.clone()));
                }
                CLRActions::Reduce => {
                    current_stack_tokens.push(_unwrapped_token.clone());
                    let production_token = next_action.get_production(&mut stack_tokens);
                    if production_token.is_none() {
                        panic!("No se ha podido obtener el token esperado");
                    }
                    let non_terminal_token = production_token.unwrap().to_string();
                    current_stack_tokens.push(non_terminal_token);
                }
                CLRActions::Accept => {
                    return true;
                }
            }
        }
        false

        //TODO: Para realizar las acciones, se necesitan 2 pilas, una para los tokens globales, y otra pila de comprobación, en
        //la pila de comprobación se irán poniendo los tokens que se vayan comprobando, y en la global tanto los terminales como los no
        //terminales, en esta se pueden poner las reducciones, de esta forma se puede tener un estado y a la vez el siguiente token de entrada
        //para poder averiguar cual será el siguiente estado a donde ir, así siempre se irán poniendo de 2 en 2 los tokens
        //en la pila de comprobación y siempre se podrá tener el estado a donde debe de ir (o en donde debe de comprobar) el siguiente token
    }
}
