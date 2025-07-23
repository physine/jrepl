use crate::appstate::AppState;
use crate::command_interpreter::eval;
use crate::command_interpreter::lexer;
use crate::command_interpreter::parser::parse_top;
use crate::command_interpreter::types::Effect;
use crate::context::context::Context;

pub fn interpret(state: &AppState, ctx: &Context, user_input: &str) -> Effect {
    let tokens = lexer(user_input);
    verify(&tokens);
    let expr = parse_top(&tokens);
    let value = eval(state, &expr, ctx);
    Effect { value }
}

fn verify(tokens: &Vec<String>) {
    // ...optional syntax checking for balanced parens...
    let mut itr = tokens.clone().into_iter().enumerate();
    while let Some(_tkn) = itr.next() {}
}

// #[cfg(test)]
// mod test {
//     use super::*;
// }
