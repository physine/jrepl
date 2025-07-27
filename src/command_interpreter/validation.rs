pub fn verify_syntax(tokens: &[String]) -> Result<bool, SyntaxError> {
    // ...optional syntax checking for balanced parens...
    // let mut itr = tokens.clone().into_iter().enumerate();
    // while let Some(_tkn) = itr.next() {}
    Ok(true)
}

// pub fn verify_symbols() -> Result<bool, SyntaxError> {
//     Ok(true)
// }

#[derive(Debug, PartialEq)]
pub enum SyntaxError {
    InvalidSyntax(String),
    InvalidSymbol(String),
    // UnbalancedParentheses(String),
    // ... add more as needed
    // Custom(String),
}
