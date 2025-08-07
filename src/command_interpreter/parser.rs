use crate::command_interpreter::types::Expr;
use regex::Regex;

pub fn parse(tokens: &[String]) -> Expr {
    let (expr, _) = parse_helper(tokens, 0);
    expr
}

fn parse_helper(tokens: &[String], mut i: usize) -> (Expr, usize) {
    match tokens.get(i).map(|s| s.as_str()) {
        Some("(") => {
            i += 1;
            let mut exprs = Vec::new();
            while i < tokens.len() && tokens[i] != ")" {
                let (expr, next_i) = parse_helper(tokens, i);
                exprs.push(expr);
                i = next_i;
            }
            (Expr::List(exprs), i + 1) // skip ')'
        }
        Some(")") => (Expr::None, i + 1),
        Some(token) => (parse_case(token), i + 1),
        None => (Expr::None, i),
    }
}

fn parse_case(token: &str) -> Expr {
    // String: starts and ends with quotes, not a number inside
    let string_re = Regex::new(r#"^"[^0-9][^"]*"$"#).unwrap();
    // Symbol: not quoted, not starting with a number, no whitespace or quotes, not starting with a number, can start with letters, or +, -, *, /
    let symbol_re = Regex::new(r#"^([A-Za-z_][A-Za-z0-9_\-+*/<>!=]*)|([+\-*/<>!=]{1,})$"#).unwrap();
    // Bool: true or false
    let bool_re = Regex::new(r"^(true|false)$").unwrap();
    // Number: int or float, not quoted
    let number_re = Regex::new(r"^-?\d+(\.\d+)?$").unwrap();

    match token {
        t if string_re.is_match(t) => {
            // remove quotes, store as string
            let inner = &t[1..t.len() - 1];
            Expr::String(inner.to_string())
        }
        // bool needs to be checked before symbol to catch "true" and "false" from being returned as symbols
        t if bool_re.is_match(t) => Expr::Bool(t == "true"),
        t if symbol_re.is_match(t) => Expr::Symbol(t.to_string()),
        t if number_re.is_match(t) => Expr::Number(t.parse::<f64>().unwrap()),
        _ => panic!("Unexpected token not recognised as a terminal type: \"{}\"", token),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_empty_expression() {
        assert_eq!(parse(&vec!["(".to_string(), ")".to_string()]), Expr::List(vec![]));
    }

    #[test]
    fn parse_string_literal() {
        assert_eq!(
            parse(&vec![
                "(".to_string(),
                "\"i_am_a_string_literal\"".to_string(),
                ")".to_string()
            ]),
            Expr::List(vec![Expr::Symbol("help".to_string())])
        );
    }

    #[test]
    fn parse_help_command() {
        assert_eq!(
            parse(&vec!["(".to_string(), "help".to_string(), ")".to_string()]),
            Expr::List(vec![Expr::Symbol("help".to_string())])
        );
    }

    #[test]
    fn parse_search_command() {
        assert_eq!(
            parse(&vec![
                "(".to_string(),
                "search".to_string(),
                "\"hi\"".to_string(),
                "file1".to_string(),
                "file2".to_string(),
                ")".to_string(),
            ]),
            Expr::List(vec![
                Expr::Symbol("search".to_string()),
                Expr::String("hi".to_string()),
                Expr::Symbol("file1".to_string()),
                Expr::Symbol("file2".to_string()),
            ])
        );
    }

    #[test]
    fn parse_nested_command() {
        assert_eq!(
            parse(&vec![
                "(".to_string(),
                "+".to_string(),
                "1".to_string(),
                "(".to_string(),
                "-".to_string(),
                "10".to_string(),
                "(".to_string(),
                "+".to_string(),
                "7".to_string(),
                "7".to_string(),
                ")".to_string(),
                ")".to_string()
            ]),
            Expr::List(vec![
                Expr::Symbol("+".to_string()),
                Expr::Number(1.0),
                Expr::List(vec![
                    Expr::Symbol("-".to_string()),
                    Expr::Number(10.0),
                    Expr::List(vec![
                        Expr::Symbol("+".to_string()),
                        Expr::Number(7.0),
                        Expr::Number(7.0)
                    ])
                ])
            ])
        );
    }

    #[test]
    fn parse_operator_symbol() {
        assert_eq!(
            parse(&vec![
                "(".to_string(),
                "+".to_string(),
                "1".to_string(),
                "2".to_string(),
                ")".to_string()
            ]),
            Expr::List(vec![
                Expr::Symbol("+".to_string()),
                Expr::Number(1.0),
                Expr::Number(2.0),
            ])
        );
    }

    #[test]
    fn parse_quoted_operator_as_string() {
        assert_eq!(
            parse(&vec!["(".to_string(), "\"+\"".to_string(), ")".to_string()]),
            Expr::List(vec![Expr::String("+".to_string())])
        );
    }

    #[test]
    fn parse_boolean_literals() {
        assert_eq!(
            parse(&vec![
                "(".to_string(),
                "true".to_string(),
                "false".to_string(),
                ")".to_string()
            ]),
            Expr::List(vec![Expr::Bool(true), Expr::Bool(false),])
        );
    }

    #[test]
    #[should_panic(expected = "Unexpected token not recognised as a terminal type")]
    fn parse_terminal_invalid_symbol_starting_with_number() {
        parse_case("123abc"); // should panic
    }

    #[test]
    fn parse_terminal_string_valid() {
        let result = parse_case("\"hello\"");
        assert_eq!(result, Expr::String("hello".to_string()));
    }

    #[test]
    fn parse_terminal_string_with_spaces() {
        let result = parse_case("\"hello world\"");
        assert_eq!(result, Expr::String("hello world".to_string()));
    }

    #[test]
    #[should_panic]
    fn parse_terminal_string_numeric_only_should_panic() {
        parse_case("\"123\""); // Not matched by string_re (starts with digit)
    }

    #[test]
    fn parse_terminal_symbol_alpha() {
        let result = parse_case("foo");
        assert_eq!(result, Expr::Symbol("foo".to_string()));
    }

    #[test]
    fn parse_terminal_symbol_operator() {
        let result = parse_case("+");
        assert_eq!(result, Expr::Symbol("+".to_string()));
    }

    #[test]
    #[should_panic]
    fn parse_terminal_symbol_invalid_starting_with_number() {
        parse_case("123abc"); // should panic due to starting with a number
    }

    #[test]
    fn parse_terminal_number_integer() {
        let result = parse_case("42");
        assert_eq!(result, Expr::Number(42.0));
    }

    #[test]
    fn parse_terminal_number_float() {
        let result = parse_case("3.14");
        assert_eq!(result, Expr::Number(3.14));
    }

    #[test]
    fn parse_terminal_number_negative() {
        let result = parse_case("-10.5");
        assert_eq!(result, Expr::Number(-10.5));
    }

    #[test]
    fn parse_terminal_bool_true() {
        let result = parse_case("true");
        assert_eq!(result, Expr::Bool(true));
    }

    #[test]
    fn parse_terminal_bool_false() {
        let result = parse_case("false");
        assert_eq!(result, Expr::Bool(false));
    }

    #[test]
    #[should_panic]
    fn parse_terminal_unexpected_token_should_panic() {
        parse_case("@@invalid@@"); // should not match any regex
    }

    #[test]
    #[should_panic]
    fn parse_terminal_empty_string_should_panic() {
        parse_case("");
    }
}
