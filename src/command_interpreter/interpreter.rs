use crate::appstate::AppState;
use crate::context::context::Context;

use crate::command_interpreter::types::{AST, Effect, Token};

pub fn interpret(state: &AppState, ctx: &Context, user_input: &str) -> Effect {
    let tokens = lexer(user_input);
    let ast = parse(tokens, ctx);
    eval(ast)
}

fn lexer(user_input: &str) -> Vec<Token> {
    vec![Token {}]
}

fn parse(tokens: Vec<Token>, ctx: &Context) -> AST {
    AST {}
}

fn eval(ast: AST) -> Effect {
    Effect {}
}
