use crate::appstate::AppState;
use crate::command_interpreter::eval;
use crate::command_interpreter::eval::EvalError;
use crate::command_interpreter::lexer;
use crate::command_interpreter::parser::parse;
use crate::command_interpreter::types::Effect;
use crate::command_interpreter::validation::SyntaxError;
use crate::command_interpreter::validation::verify_syntax;

pub fn interpret(app_state: &AppState, user_input: &str) -> Effect {
    let tokens = lexer(user_input);

    let verify_syntax_result = verify_syntax(&tokens);
    if let Err(err) = verify_syntax_result {
        return Effect::from_err(InterpretErr::VerifySyntaxErr(err));
    }

    let expr = parse(&tokens);

    match eval(app_state, &expr) {
        Ok(effect) => return effect,
        Err(err) => return Effect::from_err(InterpretErr::EvalErr(err)),
    };
}

#[derive(Debug, PartialEq)]
pub enum InterpretErr {
    // LexerErr(String), // possible check for chars out of scope (ie: UTF-8)
    VerifySyntaxErr(SyntaxError),
    ParserErr(String),
    EvalErr(EvalError),
}

// #[cfg(test)]
// mod test {
//     use super::*;
// }
