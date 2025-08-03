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

use crate::{
    appstate::AppState,
    command_interpreter::{
        eval::EvalError,
        types::{Effect, Expr},
    },
};

// #[derive(Clone)]
pub struct Command {
    pub symbol: String,
    pub description: String,
    pub eval_fn_ptr: Box<dyn Fn(&AppState, &[Expr]) -> Result<Effect, EvalError>>,
}

impl Command {
    pub fn get_description(&self) -> &str {
        &self.description
    }
}

/*

kinds of symbols:
    File trait (input, output)
    Data (json from a file, user defined variable)
    Command

*/
