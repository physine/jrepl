use crate::appstate::AppState;
use crate::command_interpreter::types::Expr;
use crate::command_interpreter::types::{AST, Effect, EvalValue};
use crate::context::context::Context;
use regex::Regex;

pub fn interpret(state: &AppState, ctx: &Context, user_input: &str) -> Effect {
    let tokens = lexer(user_input);
    verify(&tokens);
    let expr = parse_top(&tokens); // Use parse_top here!
    let value = eval(state, &expr, ctx);
    Effect { value }
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
    // ...optional syntax checking for balanced parens...
    let mut itr = tokens.clone().into_iter().enumerate();
    while let Some(_tkn) = itr.next() {}
}

fn parse_top(tokens: &[String]) -> Expr {
    let (expr, _) = parse_expr(tokens, 0);
    expr
}

fn parse_expr(tokens: &[String], mut i: usize) -> (Expr, usize) {
    match tokens.get(i).map(|s| s.as_str()) {
        Some("(") => {
            i += 1;
            let mut exprs = Vec::new();
            while i < tokens.len() && tokens[i] != ")" {
                let (expr, next_i) = parse_expr(tokens, i);
                exprs.push(expr);
                i = next_i;
            }
            (Expr::List(exprs), i + 1) // skip ')'
        }
        Some(")") => (Expr::None, i + 1),
        Some(token) => (parse_terminal(token), i + 1),
        None => (Expr::None, i),
    }
}

fn parse_terminal(token: &str) -> Expr {
    // String: starts and ends with quotes, not a number inside
    let string_re = Regex::new(r#"^"[^0-9][^"]*"$"#).unwrap();
    // Symbol: not quoted, not starting with a number, no whitespace or quotes
    let symbol_re = Regex::new(r#"^[A-Za-z_][^\s"]*$"#).unwrap();
    // Bool: true or false
    let bool_re = Regex::new(r"^(true|false)$").unwrap();
    // Number: int or float, not quoted
    let number_re = Regex::new(r"^-?\d+(\.\d+)?$").unwrap();

    match token {
        t if string_re.is_match(t) => {
            // Remove quotes, store as string
            let inner = &t[1..t.len() - 1];
            Expr::String(inner.to_string())
        }
        t if symbol_re.is_match(t) => Expr::Symbol(t.to_string()),
        t if bool_re.is_match(t) => Expr::Bool(t == "true"),
        t if number_re.is_match(t) => Expr::Number(t.parse::<f64>().unwrap()),
        _ => panic!("Unexpected token: '{}'", token),
    }
}

fn eval(_state: &AppState, _ast: &Expr, _ctx: &Context) -> EvalValue {
    EvalValue::None
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::command_interpreter::commands::get_commands;

    fn ctx() -> Context {
        let mut ctx = Context::new();
        let commands = get_commands();
        ctx.set_commands(commands);
        ctx
    }

    fn help_tokens() -> Vec<String> {
        vec!["(".into(), "help".into(), ")".into()]
    }

    fn search_tokens() -> Vec<String> {
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
    fn parse_empty_expression() {
        assert_eq!(parse_top(&vec!["(".into(), ")".into()]), Expr::List(vec![]));
    }

    #[test]
    fn parse_help_command() {
        assert_eq!(
            parse_top(&help_tokens()),
            Expr::List(vec![Expr::Symbol("help".into())])
        );
    }

    #[test]
    fn parse_search_command() {
        assert_eq!(
            parse_top(&search_tokens()),
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
            lexer("( help   )"),
            vec!["(".to_owned(), "help".to_owned(), ")".to_owned()]
        );
    }
}
