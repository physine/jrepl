use crate::appstate::AppState;
use crate::command_interpreter::types::Expr;

pub fn eval(app_state: &AppState, expr: &Expr) -> Result<Expr, EvalError> {
    match expr {
        Expr::String(_) | Expr::Number(_) | Expr::Bool(_) | Expr::None => Ok(expr.clone()),
        // get the terminal at the end of the symbol chain.
        Expr::Symbol(symbol) => {
            // this will never be a command which is bound to a symbol because those symbols are only found at the start of lists, which matches the case below
            let expr = app_state.resolve_symbol_to_terminal(symbol)?;
            Ok(expr)
        }
        Expr::List(expr_list) => {
            if expr_list.is_empty() {
                return Ok(Expr::None);
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

#[derive(Debug, PartialEq)]
pub enum EvalError {
    UndefinedSymbol(String),
    InvalidSyntax(String), // unbalances '"' or '(' would have been caught by syntax_validation
    // TypeError(String),
    IOError(String),
    // ... add more as needed
    // Custom(String),
}

#[cfg(test)]
mod test {
    use crate::statics::commands::get_commands;

    use super::*;

    fn help_cmd_ast() -> Expr {
        Expr::List(vec![Expr::Symbol("help".into())])
    }

    #[test]
    fn eval_help_ast() {
        let mut app_state = AppState::new();
        app_state.set_commands(get_commands());
        let ast = help_cmd_ast();

        assert_eq!(
            Ok(Expr::String("<help command info>".to_string())),
            eval(&app_state, &ast)
        );
    }
}
