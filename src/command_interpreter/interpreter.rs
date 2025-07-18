use crate::appstate::AppState;
use crate::context::context::Context;

use crate::command_interpreter::types::{AST, Effect, Token};

pub fn interpret(state: &AppState, ctx: &Context, user_input: &str) -> Effect {
    let tokens = lexer(user_input);
    let ast = parse(tokens, ctx);
    eval(ast)
}

fn lexer(user_input: &str) -> Vec<String> {
    let tokens: Vec<String> = Vec::new();
    for c in &user_input.as_bytes().iter().next() {
        println!("c: {}", c);
    }

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
    /*

    - (help)
    - (quit)
    - (undo)

    - (load <file_path.json>)

    - (search <symbol-name> <target-text>)
    - (tranform <symbol-name> <target-text>)

    - (do [<expr>])
    - (def <symbol-name> <expr>)

    */

    use super::*;

    #[test]
    fn lexer_simple_command() {
        assert_eq!(
            lexer("(help)"),
            vec!["(".to_owned(), "help".to_owned(), ")".to_owned()]
        );
    }

    #[test]
    fn lexer_white_space_in_command() {
        assert_eq!(
            lexer("( help )"),
            vec!["(".to_owned(), "undo".to_owned(), ")".to_owned()]
        );
    }
}
