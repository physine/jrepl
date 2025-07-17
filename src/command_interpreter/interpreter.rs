use crate::appstate::AppState;
use crate::context::context::Context;

use crate::command_interpreter::types::{AST, Effect, Token};

pub fn interpret(state: &AppState, ctx: &Context, user_input: &str) -> Effect {
    let tokens = lexer(user_input);
    let ast = parse(tokens, ctx);
    eval(ast)
}

fn lexer(user_input: &str) -> Vec<String> {
    vec!["(".to_owned(), "help".to_owned(), ")".to_owned()]
}

fn parse(tokens: Vec<String>, ctx: &Context) -> AST {
    AST {}
}

fn eval(ast: AST) -> Effect {
    Effect {}
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn lexer_verify() {
        let input = "(help)";
        let tokens = lexer(input);
        assert_eq!(
            tokens,
            vec!["(".to_owned(), "help".to_owned(), ")".to_owned()]
        );
    }
}
