use crate::errors::errors::JreplErr;

pub fn lexer(user_input: &str) -> Result<Vec<String>, JreplErr> {
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
                let mut terminated = false;
                while let Some(&next_ch) = chars.peek() {
                    let next = chars.next().unwrap();
                    s.push(next);
                    if next == '"' {
                        terminated = true;
                        break;
                    }
                }
                if !terminated {
                    return Err(JreplErr::LexErr("Unterminated string literal".to_string()));
                }
                tokens.push(s);
            }
            '`' => {
                // backtick comment (including backticks in token)
                let mut s = String::new();
                s.push(chars.next().unwrap()); // opening backtick

                let mut terminated = false;
                while let Some(&next_ch) = chars.peek() {
                    let next = chars.next().unwrap();
                    s.push(next);
                    if next == '`' {
                        terminated = true;
                        break;
                    }
                }
                if !terminated {
                    return Err(JreplErr::LexErr("Unterminated backtick comment".to_string()));
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

    Ok(tokens)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lexer_string_literal() {
        assert_eq!(
            lexer("(\"i_am_a_string_literal\")").unwrap(),
            vec![
                "(".to_string(),
                "\"i_am_a_string_literal\"".to_string(),
                ")".to_string()
            ]
        );
    }

    #[test]
    fn lexer_string_literal_with_paren() {
        assert_eq!(
            lexer("(\"i_am_a_(string_literal\")").unwrap(),
            vec![
                "(".to_string(),
                "\"i_am_a_(string_literal\"".to_string(),
                ")".to_string()
            ]
        );
    }

    #[test]
    fn lexer_string_literal_with_backtick() {
        assert_eq!(
            lexer("(\"i_am_a_\\`string_literal\")").unwrap(),
            vec![
                "(".to_string(),
                "\"i_am_a_\\`string_literal\"".to_string(),
                ")".to_string()
            ]
        );
    }

    #[test]
    fn lexer_empty_command() {
        assert_eq!(lexer("()").unwrap(), vec!["(".to_string(), ")".to_string()]);
    }

    #[test]
    fn lexer_help_command() {
        assert_eq!(
            lexer("(help)").unwrap(),
            vec!["(".to_string(), "help".to_string(), ")".to_string()]
        );
    }

    #[test]
    fn lexer_white_space_in_command_allowed() {
        assert_eq!(
            lexer("( help   )").unwrap(),
            vec!["(".to_string(), "help".to_string(), ")".to_string()]
        );

        assert_eq!(
            lexer(" ( help   ) ").unwrap(),
            vec!["(".to_string(), "help".to_string(), ")".to_string()]
        );
    }

    #[test]
    fn lexer_numbers_and_symbols() {
        assert_eq!(
            lexer("(add 123 foo456)").unwrap(),
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
            lexer("(+ 1 (* 2 3))").unwrap(),
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
            lexer(" (   add    1    2  ) ").unwrap(),
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
            lexer("(help)(quit)").unwrap(),
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
            lexer("(search file1 file2 file3)").unwrap(),
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
            lexer("(+ 1 2)").unwrap(),
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
            lexer("(search \"target(-t)ext\" filename)").unwrap(),
            vec![
                "(".to_string(),
                "search".to_string(),
                "\"target(-t)ext\"".to_string(),
                "filename".to_string(),
                ")".to_string()
            ]
        );
    }

    #[test]
    fn lexer_handles_backtick_comment() {
        assert_eq!(
            lexer("(foo `this is a comment` bar)").unwrap(),
            vec![
                "(".to_string(),
                "foo".to_string(),
                "`this is a comment`".to_string(),
                "bar".to_string(),
                ")".to_string(),
            ]
        );
    }

    #[test]
    fn lexer_handles_multiline_backtick_comment() {
        assert_eq!(
            lexer("(foo `this is\na multiline\ncomment` bar)").unwrap(),
            vec![
                "(".to_string(),
                "foo".to_string(),
                "`this is\na multiline\ncomment`".to_string(),
                "bar".to_string(),
                ")".to_string(),
            ]
        );
    }

    #[test]
    fn lexer_handles_unterminated_backtick_comment() {
        assert_eq!(
            lexer("(foo `unterminated comment bar)").unwrap_err(),
            JreplErr::LexErr("Unterminated backtick comment".to_string())
        );
    }

    #[test]
    fn lexer_handles_unterminated_string_literal() {
        assert_eq!(
            lexer("(foo \"unterminated string bar)").unwrap_err(),
            JreplErr::LexErr("Unterminated string literal".to_string())
        );
    }
}
