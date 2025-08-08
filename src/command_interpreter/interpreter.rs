use crate::appstate::AppState;
use crate::command_interpreter::lexer;
use crate::command_interpreter::parser::parse;
use crate::command_interpreter::types::Effect;
use crate::command_interpreter::{eval, validation::verify_syntax};

pub fn interpret(app_state: &AppState, user_input: &str) -> Effect {
    let lexer_result = lexer(user_input);
    let tokens = match lexer_result {
        Ok(tokens) => tokens,
        Err(err) => return Effect::from_err(err),
    };

    let expr = parse(&tokens);

    // match eval(app_state, &expr) {
    //     Ok(effect) => return effect,
    //     Err(err) => return Effect::from_err(err),
    // };
    todo!()
}

// #[cfg(test)]
// mod test {
//     use super::*;
// }
