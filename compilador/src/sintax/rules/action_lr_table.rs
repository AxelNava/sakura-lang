use std::fmt::Display;
use crate::lexer::lexer_tokens::TokEnum;
use crate::sintax::rules::baseParser::{CLRActions, TokenType};
use std::hash::Hash;
use std::iter::zip;
use strum::EnumCount;

///T es el tipo de los tokens no terminales que pueden generar una producción
#[derive(Debug, Clone)]
pub struct ActionLrTable<T>
where
    T: EnumCount + Clone + Hash + Eq ,
{
    pub action: CLRActions,
    pub goto_state: i16,
    rules: Option<Vec<TokenType<T>>>,
    production: Option<T>,
}
impl<T> ActionLrTable<T>
where
    T: EnumCount + Clone + Hash + Eq,
{
    pub fn new(
        action: CLRActions,
        goto_state: i16,
        rules: Option<Vec<TokenType<T>>>,
        production: Option<T>,
    ) -> Self {
        Self {
            action,
            goto_state,
            rules,
            production,
        }
    }
    /// Generate the production token of the vec of tokens
    /// 
    /// # Arguments 
    /// 
    /// * `tokens`: Una tupla estado-token, el estado representa el estado al que un token lleva (o al estado al que fue)
    /// 
    /// returns: Option<T> El token dado de la producción
    /// 
    /// # Examples 
    /// 
    /// ```
    /// 
    /// ```
    pub fn get_production(&self, tokens: &mut Vec<(i16, TokenType<T>)>) -> Option<T> {
        if tokens.is_empty() {
            return None;
        }
        //Compare production rules against the tokens given
        let iter_tokens = tokens.iter();
        let rules = &self.rules;
        let productions = rules.as_ref().unwrap().iter();
        let iter_tok_prod = zip(iter_tokens, productions);

        for (state_token, prod) in iter_tok_prod {
            if &state_token.1 != prod {
                return None;
            }
        }
        let len_production = rules.as_ref().unwrap().len();
        tokens.truncate(len_production);

        let prod = &self.production.as_ref().unwrap();
        Some((*prod).clone())
    }
}
