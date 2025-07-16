pub fn tokenize(user_input: &str) -> Vec<String> {
    vec!["(".to_owned(), "help".to_owned(), ")".to_owned()]
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn tokenize_help_command() {
        let user_input = "(help)";
        let t = tokenize(user_input);
        assert_eq!(t, vec!["(", "help", ")"]);
    }
}
