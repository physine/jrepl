use clap::builder::Str;

use crate::appstate::AppState;
use crate::context::context::Context;

use crate::command_interpreter::types::{AST, Effect, Token};

pub fn interpret(state: &AppState, ctx: &Context, user_input: &str) -> Effect {
    let tokens = lexer(user_input);
    let ast = parse(tokens, ctx);
    eval(ast)
}

fn lexer(user_input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut acc = String::from("");

    for char in user_input.chars() {
        if char == '(' || char == ')' {
            if !acc.is_empty() {
                tokens.push(acc.clone());
                acc.clear();
            }
            tokens.push(char.to_string());
        } else if char.is_alphanumeric() {
            acc.push(char);
        } else if char == ' ' {
            if !acc.is_empty() {
                tokens.push(acc.clone());
                acc.clear();
            }
        }
    }

    /*
        syntax varification
            - balanced brackets.
            - all "symbols" are real.

    */
    let mut itr = tokens.clone().into_iter().enumerate();
    while let Some(tkn) = itr.next() {}

    tokens
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

        assert_eq!(
            lexer("(undo)"),
            vec!["(".to_owned(), "undo".to_owned(), ")".to_owned()]
        );

        assert_eq!(
            lexer("(quit)"),
            vec!["(".to_owned(), "quit".to_owned(), ")".to_owned()]
        );
    }

    #[test]
    fn lexer_white_space_in_command_allowed() {
        assert_eq!(
            (lexer("( help   )")),
            vec!["(".to_owned(), "help".to_owned(), ")".to_owned()]
        );
    }
}
