use crate::lexer::lexer_tokens::TokEnum;
use crate::sintax::rules::baseParser::{CLRActions, TokenType};
use std::fmt::Display;
use std::hash::Hash;
use std::iter::zip;
use strum::EnumCount;

///T es el tipo de los tokens no terminales que pueden generar una producción
#[derive(Debug, Clone)]
pub struct ActionLrTable<T>
where
    T: EnumCount + Clone + Hash + Eq + Display,
{
    pub action: CLRActions,
    pub goto_state: i16,
    rules: Option<Vec<TokenType<T>>>,
    production: Option<T>,
}
impl<T> ActionLrTable<T>
where
    T: EnumCount + Clone + Hash + Eq + Display,
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
    pub fn get_production(&self, tokens: &mut Vec<(i16, String)>) -> Option<T> {
        if tokens.is_empty() {
            return None;
        }
        //Compare production rules against the tokens given
        let iter_tokens = tokens.iter().rev();
        let rules = &self.rules;
        let productions = rules.as_ref().unwrap().iter();
        let iter_tok_prod = zip(iter_tokens, productions);

        for (state_token, prod) in iter_tok_prod {
            let string_prod = match prod {
                TokenType::Terminal(token_enum) => token_enum.to_string(),
                TokenType::NonTerminal(non_terminal) => non_terminal.to_string(),
            };

            let string_token = &state_token.1;
            if &string_prod != string_token {
                return None;
            }
        }
        let len_production = *&self.rules.as_ref().unwrap().len();
        for i in 0..len_production {
            tokens.pop();
        }

        let prod = &self.production.as_ref().unwrap();
        Some((*prod).clone())
    }
}
