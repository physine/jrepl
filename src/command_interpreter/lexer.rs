pub fn lexer(user_input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = user_input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '(' | ')' => {
                tokens.push(ch.to_string());
                chars.next();
            }
            '"' => {
                // quoted string (including quotes in token)
                let mut s = String::new();
                s.push(chars.next().unwrap()); // opening quote
                while let Some(&next_ch) = chars.peek() {
                    s.push(chars.next().unwrap());
                    if next_ch == '"' {
                        break;
                    }
                }
                tokens.push(s);
            }
            c if c.is_whitespace() => {
                chars.next();
            }
            _ => {
                // accumulate a token until whitespace or paren
                let mut token = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_whitespace() || next_ch == '(' || next_ch == ')' {
                        break;
                    }
                    token.push(chars.next().unwrap());
                }
                tokens.push(token);
            }
        }
    }

    tokens
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lexer_expr_string_literal() {
        assert_eq!(
            lexer("()"),
            vec![
                "(".to_string(),
                "\"i_am_a_string_literal\"".to_string(),
                ")".to_string()
            ]
        );
    }

    #[test]
    fn lexer_empty_command() {
        assert_eq!(lexer("()"), vec!["(".to_string(), ")".to_string()]);
    }

    #[test]
    fn lexer_help_command() {
        assert_eq!(
            lexer("(help)"),
            vec!["(".to_string(), "help".to_string(), ")".to_string()]
        );
    }

    #[test]
    fn lexer_white_space_in_command_allowed() {
        assert_eq!(
            lexer("( help   )"),
            vec!["(".to_string(), "help".to_string(), ")".to_string()]
        );

        assert_eq!(
            lexer(" ( help   ) "),
            vec!["(".to_string(), "help".to_string(), ")".to_string()]
        );
    }

    #[test]
    fn lexer_numbers_and_symbols() {
        assert_eq!(
            lexer("(add 123 foo456)"),
            vec![
                "(".to_string(),
                "add".to_string(),
                "123".to_string(),
                "foo456".to_string(),
                ")".to_string()
            ]
        );
    }

    #[test]
    fn lexer_nested_expressions() {
        assert_eq!(
            lexer("(+ 1 (* 2 3))"),
            vec![
                "(".to_string(),
                "+".to_string(),
                "1".to_string(),
                "(".to_string(),
                "*".to_string(),
                "2".to_string(),
                "3".to_string(),
                ")".to_string(),
                ")".to_string()
            ]
        );
    }

    #[test]
    fn lexer_ignores_extra_spaces() {
        assert_eq!(
            lexer(" (   add    1    2  ) "),
            vec![
                "(".to_string(),
                "add".to_string(),
                "1".to_string(),
                "2".to_string(),
                ")".to_string()
            ]
        );
    }

    #[test]
    fn lexer_handles_multiple_expressions() {
        assert_eq!(
            lexer("(help)(quit)"),
            vec![
                "(".to_string(),
                "help".to_string(),
                ")".to_string(),
                "(".to_string(),
                "quit".to_string(),
                ")".to_string()
            ]
        );
    }

    #[test]
    fn lexer_parses_alpha_numeric_symbols() {
        assert_eq!(
            lexer("(search file1 file2 file3)"),
            vec![
                "(".to_string(),
                "search".to_string(),
                "file1".to_string(),
                "file2".to_string(),
                "file3".to_string(),
                ")".to_string()
            ]
        );
    }

    #[test]
    fn lexer_handles_single_char_operators_as_tokens() {
        assert_eq!(
            lexer("(+ 1 2)"),
            vec![
                "(".to_string(),
                "+".to_string(),
                "1".to_string(),
                "2".to_string(),
                ")".to_string()
            ]
        );
    }

    #[test]
    fn lexer_parses_paren_in_string() {
        assert_eq!(
            lexer("(search \"target(-t)ext\" filename)"),
            vec![
                "(".to_string(),
                "search".to_string(),
                "\"target(-t)ext\"".to_string(),
                "filename".to_string(),
                ")".to_string()
            ]
        );
    }
}
