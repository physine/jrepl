use crate::Context;
use crate::appstate::AppState;
use crate::command_interpreter::types::Effect;
use crate::command_interpreter::types::Expr;

pub fn eval(app_state: &AppState, ast: Expr, ctx: &Context) -> Result<Effect, EvalError> {}

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
    use crate::command_interpreter::types::EvalValue;
    use crate::statics::commands::get_commands;

    use super::*;

    fn help_cmd_ast() -> Expr {
        Expr::List(vec![Expr::Symbol("help".into())])
    }

    #[test]
    fn eval_help_ast() {
        let app_state = AppState::new();
        let mut ctx = Context::new();
        &ctx.set_commands(get_commands());
        let ast = help_cmd_ast();

        assert_eq!(
            Effect {
                eval_value: Some(EvalValue::String("<print help string>".to_string())),
                next_state: None,
                user_feedback: None,
                err: None,
            },
            eval(&app_state, ast, &ctx).unwrap()
        );
    }
}
