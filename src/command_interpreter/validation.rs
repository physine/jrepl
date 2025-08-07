/*

    Check for balanced:
        ()      parens

    contexts (current_context):
        - in_comment
        - in_string
        - in_list
        - in_list
        - init

    initial context:
        default

    chars of interest:
        (
        )

    transition table:

        transition_function(context, char, char_history) => next_context, char_history

    ex:
        (exit)

*/

enum Delimiter {}

impl Delimiter {
    pub fn is(token: &str) -> bool {
        matches!(token, "(" | ")" | "\"" | "\\")
    }
}

#[derive(Debug, Clone)]
enum Context {
    Init,
    List,
}

#[derive(Debug)]
struct Accumulator {
    context_stack: Vec<Context>,
    token_history: Vec<String>,
    delimiter_stack: Vec<String>,
    // err // TODO: add an err fields as early exit is not possible with fold.
}

impl Accumulator {
    fn new() -> Accumulator {
        Accumulator {
            context_stack: Vec::new(),
            token_history: Vec::new(),
            delimiter_stack: Vec::new(), // is there a better data structor?
        }
    }

    fn context_stack_peek(&self) -> Context {
        if self.context_stack.is_empty() {
            return Context::Init;
        }
        self.context_stack.get(self.context_stack.len() - 1).unwrap().clone()
    }
}

fn transition_table(mut acc: Accumulator, token: &str) -> Result<Accumulator, SyntaxError> {
    match acc.context_stack_peek() {
        Context::Init => match token {
            "(" => {
                // acc.context_stack_push(Context::List);
                acc.context_stack.push(Context::List);
                acc.token_history.push(token.to_string());
                acc.delimiter_stack.push(token.to_string());
                return Ok(acc);
            }

            _ => Err(SyntaxError::InvalidSyntax(
                "Context::Init - found token at start of expr which isnt a '(', token: {symbol}".to_string(),
            )),
        },
        // For the moment assume its a symbol if its not a Delimiter, a literal would be acceptable and not a symbol and not a Deliminiter.
        // It should also accept literals, as literals can be evaluated to themselves. Thus, ("string literal") or (4) is possible.
        Context::List => match token {
            "(" => {
                acc.context_stack.push(Context::List);
                acc.token_history.push(token.to_string());
                acc.delimiter_stack.push(token.to_string());
                return Ok(acc);
            }

            ")" => {
                acc.token_history.push(token.to_string());
                acc.context_stack.pop();
                acc.delimiter_stack.pop();
                return Ok(acc);
            }

            // this needs to exclude comments
            symbol if !Delimiter::is(token) => {
                acc.token_history.push(symbol.to_string());
                return Ok(acc);
            }

            // comment if isComment(token) => {
            //     // acc.context_stack.push(Context::Comment);
            //     acc.token_history.push(token.to_string());
            //     return Ok(acc);
            // }
            _ => Err(SyntaxError::InvalidSymbol(
                "Context::List - found token in list which isnt a '(', ')' or a symbol: {token}".to_string(),
            )),
        },

        _ => Ok(acc),
    }
}

pub fn verify_syntax(tokens: &[String]) -> Result<(), SyntaxError> {
    let acc = tokens
        .iter()
        .try_fold(Accumulator::new(), |acc, token| transition_table(acc, token));

    if let Err(err) = acc {
        return Err(err);
    }

    if let Ok(acc) = acc {
        if !acc.delimiter_stack.is_empty() {
            println!("acc.delimiter_stack: {:?}", acc.delimiter_stack);
            return Err(SyntaxError::InvalidSyntax(
                "Unbalanced delimiter somewhere in user input".to_string(),
            ));
        }
    }

    Ok(())
}
#[derive(Debug, PartialEq)]
pub enum SyntaxError {
    InvalidSyntax(String),
    InvalidSymbol(String),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_exit_command() {
        let result = verify_syntax(&vec!["(".to_string(), "exit".to_string(), ")".to_string()]);
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn verify_empty_expr() {
        let result = verify_syntax(&vec!["(".to_string(), ")".to_string()]);
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn verify_with_string_literal() {
        let result = verify_syntax(&vec![
            "(".to_string(),
            "search".to_string(),
            "\"symbol_name\"".to_string(),
            ")".to_string(),
        ]);
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn verify_nested_expr() {
        let result = verify_syntax(&vec![
            "(".to_string(),
            "outer".to_string(),
            "(".to_string(),
            "inner".to_string(),
            ")".to_string(),
            ")".to_string(),
        ]);
        assert_eq!(Ok(()), result);
    }

    // Error cases

    #[test]
    fn verify_unbalanced_open() {
        let result = verify_syntax(&vec!["(".to_string(), "unbalanced".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn verify_unbalanced_close() {
        let result = verify_syntax(&vec!["unbalanced".to_string(), ")".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn verify_extra_close() {
        let result = verify_syntax(&vec![
            "(".to_string(),
            "foo".to_string(),
            ")".to_string(),
            ")".to_string(),
        ]);
        assert!(result.is_err());
    }

    #[test]
    fn verify_unbalanced_nested() {
        let result = verify_syntax(&vec![
            "(".to_string(),
            "outer".to_string(),
            "(".to_string(),
            "inner".to_string(),
            ")".to_string(),
            // Missing outer ")"
        ]);
        assert!(result.is_err());
    }

    #[test]
    fn verify_invalid_symbol() {
        let result = verify_syntax(&vec!["$not_a_symbol".to_string()]);
        assert!(result.is_err());
    }

    // ---- Comment edge cases ----

    #[test]
    fn verify_backtick_comment_inside_expr() {
        let result = verify_syntax(&vec![
            "(".to_string(),
            "foo".to_string(),
            "`this is a comment`".to_string(),
            "bar".to_string(),
            ")".to_string(),
        ]);
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn verify_backtick_multiline_comment_inside_expr() {
        let result = verify_syntax(&vec![
            "(".to_string(),
            "foo".to_string(),
            "`this is\na multiline\ncomment`".to_string(),
            "bar".to_string(),
            ")".to_string(),
        ]);
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn verify_comment_only_expr() {
        let result = verify_syntax(&vec!["(".to_string(), "`just a comment`".to_string(), ")".to_string()]);
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn verify_comment_at_start_middle_end() {
        let result = verify_syntax(&vec![
            "(".to_string(),
            "`start`".to_string(),
            "foo".to_string(),
            "`middle`".to_string(),
            "bar".to_string(),
            "`end`".to_string(),
            ")".to_string(),
        ]);
        assert_eq!(Ok(()), result);
    }
}
