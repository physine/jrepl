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

pub struct Command {
    pub symbol: String,
    pub description: String,
    /*
        Function pointers for:
            eval() -> Value
    */
    // fn eval(&AppState, &AST) -> EvalValue,
    // fn with trait eval and takes the args, and evaluates to a value
}

/*

kinds of symbols:
    File trait (input, output)
    Data (json from a file, user defined variable)
    Command

*/
