use crate::appstate::AppState;
use crate::command_interpreter::eval;
use crate::command_interpreter::lexer;
use crate::command_interpreter::parser::parse;
use crate::command_interpreter::types::Effect;

pub fn interpret(app_state: &AppState, user_input: &str) -> Effect {
    let lexer_result = lexer(user_input);
    let tokens = match lexer_result {
        Ok(tokens) => tokens,
        Err(err) => return Effect::from_err(err),
    };

    // parens are ballanced by now.
    // parse returns an AST.
    let expr = parse(&tokens);

    match eval(app_state, &expr) {
        Ok(effect) => return effect,
        Err(err) => return Effect::from_err(err),
    };
}

#[cfg(test)]
mod test {
    use crate::{command_interpreter::types::Expr, statics::commands::get_commands};

    use super::*;

    fn run(input: &str) -> Effect {
        let mut app_state = AppState::new();
        app_state.set_commands(get_commands());
        interpret(&app_state, input)
    }

    #[test]
    fn interpret_exit_sets_exit_flag_and_returns_next_state() {
        let effect = run("(exit)");
        let state = effect.next_state.expect("exit should produce a next_state");

        assert!(effect.err.is_none());
        assert!(effect.user_feedback.is_none());
        assert!(effect.eval_value.is_none());
        assert!(state.get_exit(), "next_state.exit should be true after (exit)");
    }

    #[test]
    fn interpret_empty_list_returns_none() {
        let effect = run("()");

        assert!(effect.err.is_none());
        assert!(effect.user_feedback.is_none());
        assert_eq!(effect.eval_value, Some(Expr::None));
    }

    #[test]
    fn interpret_help_contains_snippet() {
        let effect = run("(help)");
        let s = match effect.eval_value {
            Some(Expr::String(s)) => s,
            other => panic!("unexpected output: {:?}", other),
        };

        assert!(effect.err.is_none());
        assert!(effect.user_feedback.is_none());
        assert!(s.contains("display avalible options. Usage: (he"));
    }

    #[test]
    fn interpret_unknown_command_yields_error_effect() {
        let effect = run("(nope)");
        assert!(effect.err.is_some());
        assert!(effect.eval_value.is_none());
    }
}
