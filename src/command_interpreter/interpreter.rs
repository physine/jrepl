use std::collections::VecDeque;
use crate::appstate::AppState;
use crate::context::context::Context;
use crate::command_interpreter::types::{AST, Effect, EvalValue};
use crate::command_interpreter::types::Expr;

pub fn interpret(state: &AppState, ctx: &Context, user_input: &str) -> Effect {
    let tokens = lexer(user_input);
    verify(&tokens);
    let expr = parse(tokens.clone());
    let value = eval(state, &expr, ctx);
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
    */
    let mut itr = tokens.clone().into_iter().enumerate();
    while let Some(tkn) = itr.next() {}
}

fn parse(tokens: Vec<String>) -> Expr {
    /*
    // the load command used in an expression returns a symbol ($1) which evals to the file. So the $1 symbol is bound to the symbol visited_places.
    (do
        (def visited_places (load "places_ive_been.json"))
        (print (map to_upper_case visited_places)))
    */

    // ["(", search, "hi", file1, file2, ")"]
    
    let token = tokens.first().unwrap();
    let mut acc: Expr;

    match token {
        "(" => {
            acc = Expr::List(

            )
            parse(&tokens[1..]);
        }

        ")" => {
            return;
        }

        _ => {

        }
    }

}

fn eval(state: &AppState, ast: &Expr, ctx: &Context) -> EvalValue {
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
        vec![
            "(".into(),
            "help".into(),
            ")".into(), //
        ]
    }

    fn search_tokens() -> Vec<String> {
        // (search <type: String> [<type: File | String>])
        // (search "hi" file1 file2)

        // ["(", search, "hi", file1, file2, ")"]
        vec![
            "(".into(),
            "search".into(),
            "\"hi\"".into(),
            "file1".into(),
            "file2".into(),
            ")".into(),
        ]
    }

    // ------------------- Parser Tests ------------------- //

    #[test]
    fn no_depth_no_args_parse_help_command() {
        assert_eq!(
            parse(help_tokens()),
            Expr::Symbol("help".into())
        );
    }

    #[test]
    fn no_depth_with_args_parse_search_command() {
        // (search "hi" file1 file2)

        // ["(", search, "hi", file1, file2, ")"]

        assert_eq!(
            parse(search_tokens()),
            Expr::List(vec![
                    Expr::Symbol("search".into()),
                    Expr::String("hi".into()),
                    Expr::Symbol("file1".into()),
                    Expr::Symbol("file2".into()),
            ])
        );
    }

    // ------------------- Lexer Tests ------------------- //

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
