/*

Command:
    - symbol name (
        help,
        list,
        undo,
        quit,
        search,
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

use clap::builder::Str;

/*

Symbol = command | data
command =

*/
pub struct Command {
    pub symbol: String,
    pub params: Vec<Expr>,
}

impl Command {
    pub fn parse(tokens: &Vec<String>) -> &Vec<String> {}
}

pub fn get_commands() -> Vec<Command> {
    vec![
        /*
        (help)
        */
        Command {
            symbol: String::from("help"),
            params: vec![],
        },
    ]
}
