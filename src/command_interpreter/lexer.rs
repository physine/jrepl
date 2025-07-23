pub fn lexer(user_input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut acc = String::from("");

    for char in user_input.chars() {
        if char == '(' || char == ')' {
            if !acc.is_empty() {
                tokens.push(acc.clone());
                acc.clear();
            }
            tokens.push(char.to_string());
        } else if char.is_alphanumeric() {
            acc.push(char);
        } else if char == ' ' {
            if !acc.is_empty() {
                tokens.push(acc.clone());
                acc.clear();
            }
        }
    }

    tokens
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lexer_empty_command() {
        assert_eq!(lexer("()"), vec!["(".to_owned(), ")".to_owned()]);
    }

    #[test]
    fn lexer_help_command() {
        assert_eq!(
            lexer("(help)"),
            vec!["(".to_owned(), "help".to_owned(), ")".to_owned()]
        );
    }

    #[test]
    fn lexer_white_space_in_command_allowed() {
        assert_eq!(
            lexer("( help   )"),
            vec!["(".to_owned(), "help".to_owned(), ")".to_owned()]
        );

        assert_eq!(
            lexer(" ( help   ) "),
            vec!["(".to_owned(), "help".to_owned(), ")".to_owned()]
        );
    }
}
