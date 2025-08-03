use crate::command_interpreter::command::Command;
use crate::command_interpreter::types::Expr;
use crate::{appstate::AppState, command_interpreter::types::Effect};

pub fn get_commands() -> Vec<Command> {
    vec![
        Command {
            symbol: "help".into(),
            description: "help — display the avalible options. Usage: (<help> [arguments])".into(),
            eval_fn_ptr: Box::new(|app_state: &AppState, expr: &[Expr]| {
                let help_msg = format!(
                    "\n{}\n",
                    app_state
                        .get_commands()
                        .iter()
                        .map(|cmd| cmd.get_description().to_string())
                        .collect::<Vec<String>>()
                        .join("\n")
                );
                Ok(Effect::from_eval_value(Expr::String(help_msg)))
            }),
        },
        // Command {
        //     symbol: "exit".into(),
        //     description: "exit — exit the interpreter. Usage: (exit)".into(),
        //     // param_format: "",
        //     eval_fn_ptr: Box::new(|app_state: &AppState, expr: &[Expr]| {
        //         app_state.state_builder();
        //         Ok(Effect::from_eval_value(Expr::None))
        //     }),
        // },
        // Command {
        //     symbol: "search".into(),
        //     description: "".into(),
        //     // param_format: [descripion="target-text" type="String" size="1"]
        //     //               [descripion="source to check against" type="File | String" size="1..n"]
        // },
        // Command {
        //     symbol: "list".into(),
        //     description: "".into(),
        //     // param_format: [description="list the avaliable 'things (symbols, commands, files, etc)'" type="String" size="1..n"]
        // },
        // Command {
        //     symbol: "undo".into(),
        //     description: "".into(),
        //     // param_format: ""
        // },
        // Command {
        //     symbol: "quit".into(),
        //     description: "".into(),
        //     // param_format: ""
        // },
        // Command {
        //     symbol: "load".into(),
        //     description: "".into(),
        //     // param_format: [description="files to load from disc" type="String" size="1..n"]
        // },
        // Command {
        //     symbol: "def".into(),
        //     description: "".into(),
        //     // param_format: [description="Symbol name" type="String" size="1"]
        //     //               [description="value to be bound to the symbol" type="Expr" size="1"]
        // },
        // Command {
        //     symbol: "do".into(),
        //     description: "".into(),
        //     // param_format: [description="Exprs to evaluate" type="Expr" size="1..n"]
        // },
        // Command {
        //     symbol: "set".into(),
        //     description: "".into(),
        //     // param_format: [description="existing symbol name" type="Expr" size="1"]
        //     // param_format: [description="expression to eval and bind to the symbol" type="Expr" size="1"]
        // },
        // Command {
        //     symbol: "print".into(),
        //     description: "".into(),
        //     // param_format: [description="existing symbol name" type="Expr" size="1"]
        //     // param_format: [description="expression to eval and bind to the symbol" type="Expr" size="1"]
        // },
        // Command {
        //     symbol: String::from("man"),
        //     description: String::from(""),
        //     // param_format: [description="" type="Expr" size="1..n"]
        // },
    ]
}
