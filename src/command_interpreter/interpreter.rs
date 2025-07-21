use crate::appstate::AppState;
use crate::context::context::Context;

use crate::command_interpreter::types::{AST, Effect, EvalValue};

use crate::command_interpreter::types::Expr;

pub fn interpret(state: &AppState, ctx: &Context, user_input: &str) -> Effect {
    let tokens = lexer(user_input);
    verify(&tokens);
    let ast = parse(&tokens, &ctx);
    let value = eval(&state, &ast, &ctx);
    Effect {
        value: EvalValue::None,
    }
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

    tokens
}

fn verify(tokens: &Vec<String>) {
    /*
        syntax varification
            - balanced brackets.
            - all "symbols" are real.

    */
    let mut itr = tokens.clone().into_iter().enumerate();
    while let Some(tkn) = itr.next() {}
}

fn parse(tokens: &Vec<String>, ctx: &Context) -> AST {
    AST {
        expr: Expr::Bool(true),
    }
}

fn eval(state: &AppState, ast: &AST) -> EvalValue {
    EvalValue::None
}

#[cfg(test)]
mod test {
    /*

    - (help)
    - (quit)
    - (undo)

    - (load <file_path.json>)

    - (search <target-text> [<symbol-nam>])
    - (tranform <symbol-name> <target-text>)

    - (do [<expr>])
    - (def <symbol-name> <expr>)

    */

    use crate::command_interpreter::commands::get_commands;

    use super::*;

    fn ctx() -> Context {
        let mut ctx = Context::new();
        let commands = get_commands();
        ctx.set_commands(commands);
        ctx
    }

    fn help_tokens() -> Vec<String> {
        vec!["(".to_owned(), "help".to_owned(), ")".to_owned()]
    }

    #[test]
    fn parse_simple_command() {
        assert_eq!(
            parse(&help_tokens(), &ctx()),
            AST {
                expr: Expr::Symbol(String::from("help"))
            }
        );
    }

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
