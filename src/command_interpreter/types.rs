use std::rc::Rc;

use crate::{appstate::State, command_interpreter::command::Command, errors::errors::JreplErr};

#[derive(Clone)]
pub enum Referent {
    Command(Rc<Command>),
    Expr(Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    // ------------ Literals/Terminals ----------
    String(String),
    Number(f64),
    Bool(bool),
    None,
    // Operator(String),
    // Command(String),
    // File(File),
    // State(AppState)
    // --------------- Expandables ---------------
    Symbol(String),
    List(Vec<Expr>), // e.g. (add 1 2)
}

impl Expr {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Expr::String(_) | Expr::Number(_) | Expr::Bool(_) | Expr::None)
    }
    pub fn is_literal(&self) -> bool {
        match self {
            Expr::String(_) | Expr::Number(_) | Expr::Bool(_) | Expr::None => true,
            Expr::Symbol(_) => false,
            Expr::List(xs) => xs.iter().all(|e| e.is_literal()),
        }
    }
}

// #[derive(Debug, PartialEq)]
pub struct Effect {
    pub eval_value: Option<Expr>,
    pub next_state: Option<State>,
    pub user_feedback: Option<String>, // If a file is bound to a symbol with (load <file_path>) it might not have an eval_value, but the user should get feedback.
    pub err: Option<JreplErr>,
    //     pub side_effect: Option<SideEffect>,   // IO, print, etc.
}

impl Effect {
    pub fn from_eval_value(expr: Expr) -> Effect {
        Effect {
            eval_value: Some(expr),
            next_state: None,
            user_feedback: None,
            err: None,
        }
    }

    pub fn from_err(err: JreplErr) -> Effect {
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
