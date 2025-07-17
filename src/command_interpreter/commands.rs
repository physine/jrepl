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
    )

    - expr (
        "(list <symbol>)"
    )

    fn eval(state: AppState) // destruct to whats only needed
    fn undo(state: AppState) // destruct to whats only needed

*/

pub struct Command {
    pub symbol: String,
}

pub fn get_commands() -> Vec<Command> {
    vec![Command {
        symbol: String::from("help"),
    }]
}
