/*

    Check for balanced:
        ()      parens
        ""      quotes
        /**/    comment
        //      comment


    contexts (current_context):
        - in_singe_line_comment
        - in_multi_line_comment
        - in_string
        - in_list_operator_position
        - in_list_param_position
        - default/text

    initial context:
        default

    chars of interest:
        (
        )
        "
        /
        *
        other

    transition table:

        transition_function(context, char, char_history) => next_context, char_history
    
    ex:
        (exit)

*/

use std::collections::VecDeque;

enum Delimiter {
    open_paren,
    close_paren,
    double_quote,
    forward_slash,
    back_slash,
    asterisk,
}

impl Delimiter {
    pub fn is(token: &str) -> bool {
        matches!(token, "(" | ")" | "\"" | "/" | "\\" | "*")
    }
}

#[derive(Debug, Clone)]
enum Context {
    Init,
    Default,
    Single_line_comment,
    Multi_line_comment,
    String,
    Ls_cmd_pos,
    Ls_param_pos,
    Ls_after_cmd_pos,
}

#[derive(Debug)]
struct Accumulator {
    context_stack: Vec<Context>,
    token_history: VecDeque<String>,
    delimiter_stack: Vec<String>,
    // err // TODO: add an err fields as early exit is not possible with fold.
}

impl Accumulator {
    fn new() -> Accumulator {
        Accumulator {
            context_stack: vec![],
            token_history: VecDeque::new(),
            delimiter_stack: Vec::new(), // is there a better data structor?
        }
    }

    fn context_stack_push(&mut self, context: Context) {
        self.context_stack.push(context);
    }

    fn context_stack_pop(&mut self) {
        self.context_stack.pop();
    }

    fn context_stack_peek(&self) -> Context {
        if self.context_stack.is_empty() {
            return Context::Init;
        }
        self.context_stack.get(self.context_stack.len() - 1).unwrap().clone()
    }

    fn delimiter_stack_pop(&mut self) {
        self.delimiter_stack.pop();
    }
}

fn transition_table(mut acc: Accumulator, token: &str) -> Accumulator {
    match acc.context_stack_peek() {
        Context::Init => match token {
            "(" => {
                acc.context_stack_push(Context::Ls_cmd_pos);
                acc.token_history.push_front(token.to_string());
                acc.delimiter_stack.push(token.to_string());
                return acc;
            }
            _ => panic!("Context::default - unrecognized token: {}", token),
        },
        // For the moment assume its a symbol if its not a Delimiter.
        // It should also accept literals in place of the ls_cmd_pos, as literals can be evaluated to themselves.
        Context::Ls_cmd_pos => {
            if !Delimiter::is(token) {
                acc.context_stack_push(Context::Ls_after_cmd_pos);
                acc.token_history.push_front(token.to_string());
                return acc;
            }

            match token {
                ")" => {
                    acc.context_stack_pop();
                    acc.delimiter_stack_pop();
                    return acc;
                }
                _ => {
                    panic!("Context::ls_op_pos - unrecognized token: {}", token);
                }
            }
        }
        _ => acc,
    }
}

pub fn verify_syntax(tokens: &[String]) -> Result<(), SyntaxError> {
    let acc = tokens
        .iter()
        .fold(Accumulator::new(), |acc, token| transition_table(acc, token));

    if acc.delimiter_stack.is_empty() {
        return Ok(());
    }
    Err(SyntaxError::InvalidSyntax(
        "Unbalanced delimiters at end of input".to_string(),
    ))
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
    fn verify_help_command() {
        let result = verify_syntax(&vec!["(".to_string(), "help".to_string(), ")".to_string()]);
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

    #[test]
    fn verify_multiple_exprs() {
        let result = verify_syntax(&vec![
            "(".to_string(),
            "first".to_string(),
            ")".to_string(),
            "(".to_string(),
            "second".to_string(),
            ")".to_string(),
        ]);
        assert_eq!(Ok(()), result);
    }

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
    fn verify_with_comment() {
        let result = verify_syntax(&vec![
            "(".to_string(),
            "exit".to_string(),
            "//".to_string(),
            "this".to_string(),
            "is".to_string(),
            "a".to_string(),
            "comment".to_string(),
            ")".to_string(),
        ]);
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn verify_with_multiline_comment() {
        let result = verify_syntax(&vec![
            "(".to_string(),
            "exit".to_string(),
            "/*".to_string(),
            "multi".to_string(),
            "line".to_string(),
            "comment".to_string(),
            "*/".to_string(),
            ")".to_string(),
        ]);
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn verify_only_comment() {
        let result = verify_syntax(&vec![
            "//".to_string(),
            "full".to_string(),
            "line".to_string(),
            "comment".to_string(),
        ]);
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn verify_unclosed_string() {
        let result = verify_syntax(&vec![
            "(".to_string(),
            "print".to_string(),
            "\"unclosed_string".to_string(),
            ")".to_string(),
        ]);
        assert!(result.is_err());
    }
}
