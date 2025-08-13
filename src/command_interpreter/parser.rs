use crate::command_interpreter::{lexer::Token, types::Expr};
use regex::Regex;

pub fn parse(tokens: &[Token]) -> Expr {
    let (expr, _) = parse_helper(tokens, 0);
    expr
}

fn parse_helper(tokens: &[Token], mut i: usize) -> (Expr, usize) {
    use Token::*;

    match tokens.get(i) {
        // Start of a list: consume '(', parse until matching ')'
        Some(OpenParen(_)) => {
            i += 1; // skip '('
            let mut exprs = Vec::new();

            while i < tokens.len() {
                match tokens.get(i) {
                    // End of this list
                    Some(CloseParen(_)) => {
                        i += 1; // consume ')'
                        break;
                    }
                    // Ignore comments inside lists
                    Some(Comment(_)) => {
                        i += 1;
                    }
                    // Anything else: parse a sub-expression
                    _ => {
                        let (expr, next_i) = parse_helper(tokens, i);
                        exprs.push(expr);
                        i = next_i;
                    }
                }
            }

            (Expr::List(exprs), i)
        }

        // A stray ')' at this position: mirror old behavior by yielding None and consuming it.
        Some(CloseParen(_)) => (Expr::None, i + 1),

        // Skip comments in atom position and continue parsing the next thing.
        Some(Comment(_)) => parse_helper(tokens, i + 1),

        // Atom cases
        Some(StringLiteral(s)) => (Expr::String(s.clone()), i + 1),
        Some(NumberLiteral(s)) => {
            let n = s.parse::<f64>().expect("lexer produced invalid number literal");
            (Expr::Number(n), i + 1)
        }
        Some(BoolLiteral(s)) => (Expr::Bool(s == "true"), i + 1),
        Some(Symbol(s)) => (Expr::Symbol(s.clone()), i + 1),

        // End of input
        None => (Expr::None, i),
    }
}

// pub fn parse(tokens: &[String]) -> Expr {
//     let (expr, _) = parse_helper(tokens, 0);
//     expr
// }

// fn parse_helper(tokens: &[String], mut i: usize) -> (Expr, usize) {
//     match tokens.get(i).map(|s| s.as_str()) {
//         Some("(") => {
//             i += 1;
//             let mut exprs = Vec::new();
//             while i < tokens.len() && tokens[i] != ")" {
//                 let (expr, next_i) = parse_helper(tokens, i);
//                 exprs.push(expr);
//                 i = next_i;
//             }
//             (Expr::List(exprs), i + 1) // skip ')'
//         }
//         Some(")") => (Expr::None, i + 1),
//         Some(token) => (parse_case(token), i + 1),
//         None => (Expr::None, i),
//     }
// }

// fn parse_case(token: &str) -> Expr {
//     // String: starts and ends with quotes, not a number inside.
//     let string_re = Regex::new(r#"^"[^0-9][^"]*"$"#).unwrap();
//     // Symbol: not quoted, not starting with a number, no whitespace or quotes, not starting with a number, can start with letters, or +, -, *, /
//     let symbol_re = Regex::new(r#"^([A-Za-z_][A-Za-z0-9_\-+*/<>!=]*)|([+\-*/<>!=]{1,})$"#).unwrap();
//     // Bool: true or false
//     let bool_re = Regex::new(r"^(true|false)$").unwrap();
//     // Number: int or float, not quoted
//     let number_re = Regex::new(r"^-?\d+(\.\d+)?$").unwrap();

//     match token {
//         t if string_re.is_match(t) => {
//             // remove quotes, store as string
//             let inner = &t[1..t.len() - 1];
//             Expr::String(inner.to_string())
//         }
//         // bool needs to be checked before symbol to catch "true" and "false" from being returned as symbols
//         t if bool_re.is_match(t) => Expr::Bool(t == "true"),
//         t if symbol_re.is_match(t) => Expr::Symbol(t.to_string()),
//         t if number_re.is_match(t) => Expr::Number(t.parse::<f64>().unwrap()),
//         _ => panic!("Unexpected token not recognised as a terminal type: \"{}\"", token),
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_empty_expression() {
        let tokens = vec![Token::OpenParen("(".into()), Token::CloseParen(")".into())];
        assert_eq!(parse(&tokens), Expr::List(vec![]));
    }

    #[test]
    fn parse_string_literal() {
        let tokens = vec![
            Token::OpenParen("(".into()),
            Token::StringLiteral("i_am_a_string_literal".into()),
            Token::CloseParen(")".into()),
        ];
        assert_eq!(
            parse(&tokens),
            Expr::List(vec![Expr::String("i_am_a_string_literal".into())])
        );
    }

    #[test]
    fn parse_help_command() {
        let tokens = vec![
            Token::OpenParen("(".into()),
            Token::Symbol("help".into()),
            Token::CloseParen(")".into()),
        ];
        assert_eq!(parse(&tokens), Expr::List(vec![Expr::Symbol("help".into())]));
    }

    #[test]
    fn parse_search_command() {
        let tokens = vec![
            Token::OpenParen("(".into()),
            Token::Symbol("search".into()),
            Token::StringLiteral("hi".into()),
            Token::Symbol("file1".into()),
            Token::Symbol("file2".into()),
            Token::CloseParen(")".into()),
        ];
        assert_eq!(
            parse(&tokens),
            Expr::List(vec![
                Expr::Symbol("search".into()),
                Expr::String("hi".into()),
                Expr::Symbol("file1".into()),
                Expr::Symbol("file2".into()),
            ])
        );
    }

    #[test]
    fn parse_nested_command() {
        let tokens = vec![
            Token::OpenParen("(".into()),
            Token::Symbol("+".into()),
            Token::NumberLiteral("1".into()),
            Token::OpenParen("(".into()),
            Token::Symbol("-".into()),
            Token::NumberLiteral("10".into()),
            Token::OpenParen("(".into()),
            Token::Symbol("+".into()),
            Token::NumberLiteral("7".into()),
            Token::NumberLiteral("7".into()),
            Token::CloseParen(")".into()),
            Token::CloseParen(")".into()),
            Token::CloseParen(")".into()),
        ];
        assert_eq!(
            parse(&tokens),
            Expr::List(vec![
                Expr::Symbol("+".into()),
                Expr::Number(1.0),
                Expr::List(vec![
                    Expr::Symbol("-".into()),
                    Expr::Number(10.0),
                    Expr::List(vec![Expr::Symbol("+".into()), Expr::Number(7.0), Expr::Number(7.0),])
                ])
            ])
        );
    }

    #[test]
    fn parse_operator_symbol() {
        let tokens = vec![
            Token::OpenParen("(".into()),
            Token::Symbol("+".into()),
            Token::NumberLiteral("1".into()),
            Token::NumberLiteral("2".into()),
            Token::CloseParen(")".into()),
        ];
        assert_eq!(
            parse(&tokens),
            Expr::List(vec![Expr::Symbol("+".into()), Expr::Number(1.0), Expr::Number(2.0),])
        );
    }

    #[test]
    fn parse_quoted_operator_as_string() {
        let tokens = vec![
            Token::OpenParen("(".into()),
            Token::StringLiteral("+".into()),
            Token::CloseParen(")".into()),
        ];
        assert_eq!(parse(&tokens), Expr::List(vec![Expr::String("+".into())]));
    }

    #[test]
    fn parse_boolean_literals() {
        let tokens = vec![
            Token::OpenParen("(".into()),
            Token::BoolLiteral("true".into()),
            Token::BoolLiteral("false".into()),
            Token::CloseParen(")".into()),
        ];
        assert_eq!(parse(&tokens), Expr::List(vec![Expr::Bool(true), Expr::Bool(false)]));
    }

    #[test]
    fn parse_ignores_comments() {
        let tokens = vec![
            Token::OpenParen("(".into()),
            Token::Comment(" this is a comment ".into()),
            Token::Symbol("a".into()),
            Token::CloseParen(")".into()),
        ];
        assert_eq!(parse(&tokens), Expr::List(vec![Expr::Symbol("a".into())]));
    }

    #[test]
    fn parse_stray_close_paren_yields_none() {
        let tokens = vec![Token::CloseParen(")".into())];
        assert_eq!(parse(&tokens), Expr::None);
    }

    #[test]
    #[should_panic] // number parser will panic on invalid numeric literal from lexer
    fn parse_invalid_number_literal_should_panic() {
        let tokens = vec![Token::NumberLiteral("3.1.4".into())];
        let _ = parse(&tokens);
    }
}
