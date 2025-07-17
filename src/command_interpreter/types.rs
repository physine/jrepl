pub enum Expr {
    Symbol(String),
    List(Vec<Expr>),
    String(String),
    Number(f64),
}

pub struct Token {}

pub struct Effect {}

impl Effect {
    pub fn exe(&self) {}
}

pub struct AST {}
