use crate::{appstate::AppState, command_interpreter::types::Effect};
use crate::{command_interpreter::types::Expr, errors::errors::JreplErr};

pub fn eval(app_state: &AppState, expr: &Expr) -> Result<Effect, JreplErr> {
    match expr {
        Expr::String(_) | Expr::Number(_) | Expr::Bool(_) | Expr::None => Ok(Effect::from_eval_value(expr.clone())),
        // get the terminal at the end of the symbol chain.
        Expr::Symbol(symbol) => {
            // this will never be a command which is bound to a symbol because those symbols are only found at the start of lists, which matches the case below
            let expr = app_state.resolve_symbol_to_terminal(symbol)?;
            Ok(Effect::from_eval_value(expr))
        }
        Expr::List(expr_list) => {
            if expr_list.is_empty() {
                return Ok(Effect::from_eval_value(Expr::None));
            }
            match &expr_list[0] {
                Expr::Symbol(symbol) => {
                    let command = app_state.get_command_from_symbol(symbol)?;
                    (command.eval_fn_ptr)(app_state, &expr_list[1..])
                }
                other => {
                    panic!("List does not start with Command. Found: {:?}", other);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::statics::commands::get_commands;

    use super::*;

    fn sym(s: &str) -> Expr {
        Expr::Symbol(s.into())
    }

    #[test]
    fn eval_empty_list_returns_none() {
        let app_state = AppState::new();
        let ast = Expr::List(vec![]);

        let result = eval(&app_state, &ast).map(|e| e.eval_value);

        assert_eq!(Ok(Some(Expr::None)), result);
    }

    #[test]
    #[should_panic(expected = "List does not start with Command")]
    fn eval_list_head_is_not_symbol_panics() {
        let app_state = AppState::new();
        let ast = Expr::List(vec![Expr::String("hello".into())]);

        let _ = eval(&app_state, &ast);
    }

    #[test]
    fn eval_help_no_args() {
        let mut app_state = AppState::new();
        app_state.set_commands(get_commands());

        let ast = Expr::List(vec![sym("help")]);

        let out = eval(&app_state, &ast).map(|e| e.eval_value).unwrap();
        match out {
            Some(Expr::String(s)) => {
                assert!(s.contains("display avalible options. Usage: (help)"));
            }
            other => panic!("Unexpected eval output: {:?}", other),
        }
    }

    #[test]
    fn eval_unknown_command_returns_err() {
        let mut app_state = AppState::new();
        app_state.set_commands(get_commands());

        let ast = Expr::List(vec![sym("nope")]);

        let out = eval(&app_state, &ast);
        assert!(out.is_err());
    }

    // #[test]
    // fn eval_command_with_args() {
    //     let mut app_state = AppState::new();
    //     app_state.set_commands(get_commands());

    //     let ast = Expr::List(vec![sym("echo"), Expr::String("hi".into()), Expr::Number(42.0)]);

    //     let out = eval(&app_state, &ast);
    //     assert!(out.is_ok());
    // }
}
