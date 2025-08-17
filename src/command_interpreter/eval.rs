use crate::{appstate::AppState, command_interpreter::types::Effect};
use crate::{command_interpreter::types::Expr, errors::errors::JreplErr};

pub fn eval(app_state: &AppState, expr: &Expr) -> Result<Effect, JreplErr> {
    match expr {
        // terminals pass through
        Expr::File(_) | Expr::String(_) | Expr::Number(_) | Expr::Bool(_) | Expr::None => {
            Ok(Effect::from_eval_value(expr.clone()))
        }

        // resolve symbol to terminal
        Expr::Symbol(symbol) => {
            let v = app_state.resolve_symbol_value(symbol)?;
            Ok(Effect::from_eval_value(v))
        }

        // lists
        Expr::List(expr_list) => {
            if expr_list.is_empty() {
                return Ok(Effect::from_eval_value(Expr::None));
            }

            // single element: zero-arg command OR grouped expression
            if expr_list.len() == 1 {
                if let Expr::Symbol(s) = &expr_list[0] {
                    if let Ok(cmd) = app_state.get_command_from_symbol(s) {
                        return (cmd.eval_fn_ptr)(app_state, &[]);
                    }
                }
                let v = value_of(app_state, &expr_list[0])?;
                return Ok(Effect::from_eval_value(v));
            }

            // NEW: treat a list of literals as a literal value
            if expr.is_literal() {
                return Ok(Effect::from_eval_value(expr.clone()));
            }

            // otherwise must be a command form
            match &expr_list[0] {
                Expr::Symbol(symbol) => {
                    let command = app_state.get_command_from_symbol(symbol)?;
                    (command.eval_fn_ptr)(app_state, &expr_list[1..])
                }
                head => Err(JreplErr::OperatorFormatErr(format!(
                    "Invalid list: expected a command symbol at position 0, found {head:?}. \
                     Multi-element lists must start with a command (e.g., (+ 1 2)). \
                     To evaluate a literal, put it alone in the list or use a list of literals: (42), (true), (\"a\" \"b\")."
                ))),
            }
        }
    }
}

pub fn value_of(app_state: &AppState, expr: &Expr) -> Result<Expr, JreplErr> {
    if expr.is_literal() {
        return Ok(expr.clone());
    }
    match expr {
        Expr::Symbol(s) => app_state.resolve_symbol_value(s),
        Expr::List(_) => {
            let eff = eval(app_state, expr)?;
            eff.eval_value
                .ok_or_else(|| JreplErr::OperatorFormatErr("Subexpression returned no value".to_string()))
        }
        _ => unreachable!(),
    }
}

pub fn number_of(app_state: &AppState, expr: &Expr) -> Result<f64, JreplErr> {
    match value_of(app_state, expr)? {
        Expr::Number(n) => Ok(n),
        other => Err(JreplErr::UndefinedSymbol(format!(
            "Type error: expected Number, got {:?}",
            other
        ))),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::statics::commands::get_commands;

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
    fn eval_grouped_literal_list_ok() {
        let app_state = AppState::new();
        let ast = Expr::List(vec![Expr::Bool(true)]);
        let out = eval(&app_state, &ast).map(|e| e.eval_value).unwrap();
        assert_eq!(out, Some(Expr::Bool(true)));
    }

    #[test]
    fn eval_help_no_args() {
        let mut app_state = AppState::new();
        app_state.set_commands(get_commands());
        let ast = Expr::List(vec![sym("help")]);
        let out = eval(&app_state, &ast).map(|e| e.eval_value).unwrap();
        match out {
            Some(Expr::String(s)) => assert!(s.contains("display avalible options. Usage: (help)")),
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

    // ----- (+) -----

    #[test]
    fn eval_plus_simple() {
        let mut app_state = AppState::new();
        app_state.set_commands(get_commands());
        let ast = Expr::List(vec![sym("+"), Expr::Number(1.0), Expr::Number(2.0), Expr::Number(3.5)]);
        let out = eval(&app_state, &ast).map(|e| e.eval_value).unwrap();
        match out {
            Some(Expr::Number(n)) => assert_eq!(n, 6.5),
            other => panic!("{:?}", other),
        }
    }

    #[test]
    fn eval_plus_with_nested_minus() {
        let mut app_state = AppState::new();
        app_state.set_commands(get_commands());
        // (+ 1 (- 5 2)) => 4
        let ast = Expr::List(vec![
            sym("+"),
            Expr::Number(1.0),
            Expr::List(vec![sym("-"), Expr::Number(5.0), Expr::Number(2.0)]),
        ]);
        let out = eval(&app_state, &ast).map(|e| e.eval_value).unwrap();
        match out {
            Some(Expr::Number(n)) => assert_eq!(n, 4.0),
            other => panic!("{:?}", other),
        }
    }

    #[test]
    fn eval_plus_type_error_non_number_arg() {
        let mut app_state = AppState::new();
        app_state.set_commands(get_commands());
        let ast = Expr::List(vec![sym("+"), Expr::Number(1.0), Expr::String("a".into())]);
        let out = eval(&app_state, &ast);
        assert!(out.is_err());
    }

    // ----- (-) -----

    #[test]
    fn eval_minus_unary() {
        let mut app_state = AppState::new();
        app_state.set_commands(get_commands());
        let ast = Expr::List(vec![sym("-"), Expr::Number(5.0)]);
        let out = eval(&app_state, &ast).map(|e| e.eval_value).unwrap();
        match out {
            Some(Expr::Number(n)) => assert_eq!(n, -5.0),
            other => panic!("{:?}", other),
        }
    }

    #[test]
    fn eval_minus_multiple() {
        let mut app_state = AppState::new();
        app_state.set_commands(get_commands());
        // (- 10 1 2 3) => 4
        let ast = Expr::List(vec![
            sym("-"),
            Expr::Number(10.0),
            Expr::Number(1.0),
            Expr::Number(2.0),
            Expr::Number(3.0),
        ]);
        let out = eval(&app_state, &ast).map(|e| e.eval_value).unwrap();
        match out {
            Some(Expr::Number(n)) => assert_eq!(n, 4.0),
            other => panic!("{:?}", other),
        }
    }

    #[test]
    fn eval_minus_with_nested_plus() {
        let mut app_state = AppState::new();
        app_state.set_commands(get_commands());
        // (- (+ 10 5) 3) => 12
        let ast = Expr::List(vec![
            sym("-"),
            Expr::List(vec![sym("+"), Expr::Number(10.0), Expr::Number(5.0)]),
            Expr::Number(3.0),
        ]);
        let out = eval(&app_state, &ast).map(|e| e.eval_value).unwrap();
        match out {
            Some(Expr::Number(n)) => assert_eq!(n, 12.0),
            other => panic!("{:?}", other),
        }
    }

    #[test]
    fn eval_minus_type_error_non_number_arg() {
        let mut app_state = AppState::new();
        app_state.set_commands(get_commands());
        let ast = Expr::List(vec![sym("-"), Expr::Number(1.0), Expr::String("x".into())]);
        let out = eval(&app_state, &ast);
        assert!(out.is_err());
    }

    // ----- (*) -----

    #[test]
    fn eval_multiply_simple() {
        let mut app_state = AppState::new();
        app_state.set_commands(get_commands());
        // (* 2 3 4) => 24
        let ast = Expr::List(vec![sym("*"), Expr::Number(2.0), Expr::Number(3.0), Expr::Number(4.0)]);
        let out = eval(&app_state, &ast).map(|e| e.eval_value).unwrap();
        match out {
            Some(Expr::Number(n)) => assert_eq!(n, 24.0),
            other => panic!("{:?}", other),
        }
    }

    #[test]
    fn eval_multiply_with_nested_plus() {
        let mut app_state = AppState::new();
        app_state.set_commands(get_commands());
        // (* 2 (+ 3 1)) => 2 * 4 = 8
        let ast = Expr::List(vec![
            sym("*"),
            Expr::Number(2.0),
            Expr::List(vec![sym("+"), Expr::Number(3.0), Expr::Number(1.0)]),
        ]);
        let out = eval(&app_state, &ast).map(|e| e.eval_value).unwrap();
        match out {
            Some(Expr::Number(n)) => assert_eq!(n, 8.0),
            other => panic!("{:?}", other),
        }
    }

    #[test]
    fn eval_multiply_type_error_non_number_arg() {
        let mut app_state = AppState::new();
        app_state.set_commands(get_commands());
        let ast = Expr::List(vec![sym("*"), Expr::Number(2.0), Expr::String("x".into())]);
        let out = eval(&app_state, &ast);
        assert!(out.is_err());
    }

    // Optional: behavior for zero arguments to (*) => identity 1.0
    #[test]
    fn eval_multiply_zero_args_identity() {
        let mut app_state = AppState::new();
        app_state.set_commands(get_commands());
        let ast = Expr::List(vec![sym("*")]);
        let out = eval(&app_state, &ast).map(|e| e.eval_value).unwrap();
        match out {
            Some(Expr::Number(n)) => assert_eq!(n, 1.0),
            other => panic!("{:?}", other),
        }
    }

    // ----- (/) -----

    #[test]
    fn eval_divide_unary_reciprocal() {
        let mut app_state = AppState::new();
        app_state.set_commands(get_commands());
        // (/ 2) => 0.5
        let ast = Expr::List(vec![sym("/"), Expr::Number(2.0)]);
        let out = eval(&app_state, &ast).map(|e| e.eval_value).unwrap();
        match out {
            Some(Expr::Number(n)) => assert_eq!(n, 0.5),
            other => panic!("{:?}", other),
        }
    }

    #[test]
    fn eval_divide_multiple() {
        let mut app_state = AppState::new();
        app_state.set_commands(get_commands());
        // (/ 20 2 5) => 20 / 2 / 5 = 2
        let ast = Expr::List(vec![sym("/"), Expr::Number(20.0), Expr::Number(2.0), Expr::Number(5.0)]);
        let out = eval(&app_state, &ast).map(|e| e.eval_value).unwrap();
        match out {
            Some(Expr::Number(n)) => assert_eq!(n, 2.0),
            other => panic!("{:?}", other),
        }
    }

    #[test]
    fn eval_divide_with_nested_minus() {
        let mut app_state = AppState::new();
        app_state.set_commands(get_commands());
        // (/ 12 (- 5 2)) => 12 / 3 = 4
        let ast = Expr::List(vec![
            sym("/"),
            Expr::Number(12.0),
            Expr::List(vec![sym("-"), Expr::Number(5.0), Expr::Number(2.0)]),
        ]);
        let out = eval(&app_state, &ast).map(|e| e.eval_value).unwrap();
        match out {
            Some(Expr::Number(n)) => assert_eq!(n, 4.0),
            other => panic!("{:?}", other),
        }
    }

    #[test]
    fn eval_divide_by_zero_error() {
        let mut app_state = AppState::new();
        app_state.set_commands(get_commands());
        let ast = Expr::List(vec![sym("/"), Expr::Number(1.0), Expr::Number(0.0)]);
        let out = eval(&app_state, &ast);
        assert!(out.is_err());
    }

    #[test]
    fn eval_divide_type_error_non_number_arg() {
        let mut app_state = AppState::new();
        app_state.set_commands(get_commands());
        let ast = Expr::List(vec![sym("/"), Expr::Number(10.0), Expr::String("a".into())]);
        let out = eval(&app_state, &ast);
        assert!(out.is_err());
    }
}
