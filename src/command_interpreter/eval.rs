use crate::command_interpreter::types::Expr;
use crate::{appstate::AppState, command_interpreter::types::Effect};

pub fn eval(app_state: &AppState, expr: &Expr) -> Result<Effect, EvalError> {
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

    // #[test]
    // fn eval_help_ast() {
    //     let mut app_state = AppState::new();
    //     app_state.set_commands(get_commands());
    //     let ast = help_cmd_ast();

    //     let result = eval(&app_state, &ast).map(|effect| effect.eval_value);

    //     assert_eq!(Ok(Some(Expr::String("<help command info>".to_string()))), result);
    // }
}
