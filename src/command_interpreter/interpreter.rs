use crate::appstate::AppState;
use crate::command_interpreter::eval;
use crate::command_interpreter::eval::EvalError;
use crate::command_interpreter::lexer;
use crate::command_interpreter::parser::parse_top;
use crate::command_interpreter::syntax_validation::SyntaxError;
use crate::command_interpreter::syntax_validation::verify_syntax;
use crate::command_interpreter::types::Effect;
use crate::context::context::Context;

pub fn interpret(app_state: &AppState, ctx: &Context, user_input: &str) -> Effect {
    let tokens = lexer(user_input);

    let verify_syntax_result = verify_syntax(&tokens);
    if let Err(err) = verify_syntax_result {
        return Effect::from_err(InterpretErr::VerifySyntaxErr(err));
    }

    let expr = parse_top(&tokens);

    let expr = eval(app_state, &expr, ctx);
    Effect {
        eval_value: Some(expr),
        next_state: None,
        user_feedback: None,
        err: None,
    }
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
