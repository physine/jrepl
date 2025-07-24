use crate::command_interpreter::command::Command;

pub fn get_commands() -> Vec<Command> {
    vec![
        Command {
            symbol: "help".into(),
            description: "".into(),
            // param_format: "",
            // parse: |state: &AppState, ctx: &Context| -> Expr {
            //     //
            // },
        },
        Command {
            symbol: "search".into(),
            description: "".into(),
            // param_format: [descripion="target-text" type="String" size="1"]
            //               [descripion="source to check against" type="File | String" size="1..n"]
        },
        Command {
            symbol: "list".into(),
            description: "".into(),
            // param_format: [description="list the avaliable 'things (symbols, commands, files, etc)'" type="String" size="1..n"]
        },
        Command {
            symbol: "undo".into(),
            description: "".into(),
            // param_format: ""
        },
        Command {
            symbol: "quit".into(),
            description: "".into(),
            // param_format: ""
        },
        Command {
            symbol: "load".into(),
            description: "".into(),
            // param_format: [description="files to load from disc" type="String" size="1..n"]
        },
        Command {
            symbol: "def".into(),
            description: "".into(),
            // param_format: [description="Symbol name" type="String" size="1"]
            //               [description="value to be bound to the symbol" type="Expr" size="1"]
        },
        Command {
            symbol: "do".into(),
            description: "".into(),
            // param_format: [description="Exprs to evaluate" type="Expr" size="1..n"]
        },
        Command {
            symbol: "set".into(),
            description: "".into(),
            // param_format: [description="existing symbol name" type="Expr" size="1"]
            // param_format: [description="expression to eval and bind to the symbol" type="Expr" size="1"]
        },
        Command {
            symbol: "print".into(),
            description: "".into(),
            // param_format: [description="existing symbol name" type="Expr" size="1"]
            // param_format: [description="expression to eval and bind to the symbol" type="Expr" size="1"]
        },
        // Command {
        //     symbol: String::from("man"),
        //     description: String::from(""),
        //     // param_format: [description="" type="Expr" size="1..n"]
        // },
    ]
}
