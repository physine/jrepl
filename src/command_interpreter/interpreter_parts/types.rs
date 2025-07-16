enum Expr {
    symbol(String),
    List(Vec<Expr>),
    String(String),
    Number(f64),
}
