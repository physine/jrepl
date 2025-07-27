use crate::appstate::AppState;
use crate::command_interpreter::types::Effect;
use crate::command_interpreter::types::Expr;
use crate::{Context, command_interpreter::command};

pub fn eval(app_state: &AppState, expr: &Expr, ctx: &Context) -> Result<Expr, EvalError> {
    match expr {
        Expr::String(_) | Expr::Number(_) | Expr::Bool(_) | Expr::None => Ok(expr.clone()),
        // get the terminal at the end of the symbol chain.
        Expr::Symbol(symbol) => {
            let expr = app_state.resolve_symbol_to_terminal(symbol)?;
            eval(app_state, &expr, ctx)
        }
        Expr::List(expr_list) => {
            if expr_list.is_empty() {
                return Ok(Expr::None);
            }
            match &expr_list[0] {
                Expr::Symbol(symbol) => {
                    if let Some(command) = get_command_from_symbol(symbol) {
                        command.eval(&expr_list[1..])
                    } else {
                        panic!("Unknown symbol found in command position: {}", symbol);
                    }
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
        let app_state = AppState::new();
        let ctx = Context::from(get_commands());
        let ast = help_cmd_ast();

        assert_eq!(Ok(Expr::None), eval(&app_state, &ast, &ctx));
    }
}
