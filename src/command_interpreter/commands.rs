/*

Command:
    - symbol name (
        help,
        search,
        list,
        undo,
        quit,
        pritty print,
        load json file,
        transform data,
        create symbol,
        set symbol
        do [expr]
    )

    - expr (
        "(list <symbol>)"
    )

    fn eval(state: AppState) // destruct to whats only needed
    fn undo(state: AppState) // destruct to whats only needed

*/

/*

Symbol = command | data
command =

*/

pub struct Command {
    pub symbol: String,
    pub description: String,
    // pub param_format: String,

    fn eval(&AppState, &AST) -> EvalValue,
    // fn with trait eval and takes the args, and evaluates to a value
    // pub params: Vec<Expr>,
}

pub fn get_commands() -> Vec<Command> {
    vec![
        Command {
            symbol: String::from("help"),
            description: String::from(""),
            // param_format: "",
        },
        Command {
            symbol: String::from("search"),
            description: String::from(""),
            // param_format: [descripion="target-text" type="String" size="1"]
            //               [descripion="source to check against" type="File | String" size="1..n"]
        },
        Command {
            symbol: String::from("list"),
            description: String::from(""),
            // param_format: [description="list the avaliable 'things (symbols, commands, files, etc)'" type="String" size="1..n"]
        },
        Command {
            symbol: String::from("undo"),
            description: String::from(""),
            // param_format: ""
        },
        Command {
            symbol: String::from("quit"),
            description: String::from(""),
            // param_format: ""
        },
        Command {
            symbol: String::from("load"),
            description: String::from(""),
            // param_format: [description="files to load from disc" type="String" size="1..n"]
        },
        Command {
            symbol: String::from("def"),
            description: String::from(""),
            // param_format: [description="Symbol name" type="String" size="1"]
            //               [description="value to be bound to the symbol" type="Expr" size="1"]
        },
        Command {
            symbol: String::from("do"),
            description: String::from(""),
            // param_format: [description="Exprs to evaluate" type="Expr" size="1..n"]
        },
        Command {
            symbol: String::from("set"),
            description: String::from(""),
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

/*

kinds of symbols:
    File trait (input, output)
    Data (json from a file, user defined variable)
    Command

*/
