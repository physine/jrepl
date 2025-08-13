#[derive(Debug, PartialEq)]
pub enum JreplErr {
    // -------------------- Interpreter Errors --------------------//
    UnbalancedDelimiter(String),
    LexificationErr(String),
    LexErr(String),
    VerifySyntaxErr(String),
    InvalidSymbol(String),
    InvalidSyntax(String),
    ParserErr(String),
    EvalErr(String),
    UndefinedSymbol(String),
    ArithmeticErr(String),
    OperatorFormatErr(String),
}
