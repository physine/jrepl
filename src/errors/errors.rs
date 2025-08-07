#[derive(Debug, PartialEq)]
pub enum JreplErr {
    // -------------------- Interpreter Errors --------------------//
    LexificationErr(String),
    LexErr(String),
    VerifySyntaxErr(String),
    InvalidSymbol(String),
    InvalidSyntax(String),
    ParserErr(String),
    EvalErr(String),
    UndefinedSymbol(String),
}
