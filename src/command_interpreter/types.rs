#[derive(Debug, PartialEq)]
pub enum EvalValue {
    // ------------ Literals/Terminals ----------
    String(String),
    Number(f64),
    Bool(bool),
    None,
    // fn
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

pub struct Effect {
    pub value: EvalValue,
}

impl Effect {
    pub fn exe(&self) {}
}

#[derive(Debug, PartialEq)]
pub struct AST {
    pub expr: Expr,
}
