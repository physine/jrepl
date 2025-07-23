pub fn verify_parens(tokens: &Vec<String>) {
    // ...optional syntax checking for balanced parens...
    let mut itr = tokens.clone().into_iter().enumerate();
    while let Some(_tkn) = itr.next() {}
}
