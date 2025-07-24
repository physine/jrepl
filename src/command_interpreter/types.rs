use crate::appstate::State;
use crate::command_interpreter::interpreter::InterpretErr;

#[derive(Debug, PartialEq)]
pub enum EvalValue {
    String(String),
    Number(f64),
    Bool(bool),
    None, // why do I have this?
          // fn - possible for a lambda in the furure
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    // ------------ Literals/Terminals ----------
    String(String),
    Number(f64),
    Bool(bool),
    None,
    // Command(String),
    // File(File),
    // State(AppState)
    // --------------- Expandables ---------------
    Symbol(String),
    List(Vec<Expr>), // e.g. (add 1 2)
}

#[derive(Debug, PartialEq)]
pub struct Effect {
    pub eval_value: Option<EvalValue>,
    pub next_state: Option<State>,
    pub user_feedback: Option<String>, // If a file is bound to a symbol with (load <file_path>) it might not have an eval_value, but the user should get feedback.
    pub err: Option<InterpretErr>,
    //     pub side_effect: Option<SideEffect>,   // IO, print, etc.
}

impl Effect {
    pub fn apply(&self) -> State {
        State { exit: false }
    }

    pub fn from_err(err: InterpretErr) -> Effect {
        Effect {
            eval_value: None,
            next_state: None,
            user_feedback: None,
            err: Some(err),
        }
    }
}

// impl Effect {
//     pub fn exe(&self) {}
// }

#[derive(Debug, PartialEq)]
pub struct AST {
    pub expr: Expr,
}
