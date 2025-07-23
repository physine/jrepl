use crate::appstate::AppState;
use crate::command_interpreter::eval;
use crate::command_interpreter::lexer;
use crate::command_interpreter::parser::parse_top;
use crate::command_interpreter::syntax_validation::verify_parens;
use crate::command_interpreter::types::Effect;
use crate::context::context::Context;

pub fn interpret(state: &AppState, ctx: &Context, user_input: &str) -> Effect {
    let tokens = lexer(user_input);
    verify_parens(&tokens);
    let expr = parse_top(&tokens);
    let value = eval(state, &expr, ctx);
    Effect { value }
}

// #[cfg(test)]
// mod test {
//     use super::*;
// }
