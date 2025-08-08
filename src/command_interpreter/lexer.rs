use crate::errors::errors::JreplErr;

pub fn lexer(user_input: &str) -> Result<Vec<Token>, JreplErr> {
    let acc = user_input
        .chars()
        .into_iter()
        .try_fold(Accumulator::new(), |acc, c| transition_table(acc, c));

    match acc {
        Ok(acc) => {
            if acc.delimiter_balance == 0 {
                Ok(acc.tokens)
            } else {
                Err(JreplErr::UnbalancedDelimiter(String::from("Unbalanced parentheses.")))
            }
        }
        Err(err) => Err(err),
    }
}

fn transition_table(mut acc: Accumulator, c: char) -> Result<Accumulator, JreplErr> {
    match acc.context_stack_peek() {
        Context::Init => match c {
            '(' => {
                acc.context_stack.push(Context::List);
                acc.tokens.push(Token::OpenParen(c.to_string()));
                acc.delimiter_balance_inc();
                Ok(acc)
            }

            '`' => {
                acc.context_stack.push(Context::Comment);
                acc.memory.push(c);
                Ok(acc)
            }

            ' ' => Ok(acc),

            _ => {
                return Err(JreplErr::InvalidSymbol(format!(
                    "Found Invalid char at start of user_input: '{}'.",
                    c
                )));
            }
        },

        Context::List => match c {
            '(' => {
                acc.tokens.push(Token::OpenParen(c.to_string()));
                acc.context_stack.push(Context::List);
                acc.delimiter_balance_inc();
                Ok(acc)
            }

            ')' => {
                acc.tokens.push(Token::CloseParen(c.to_string()));
                acc.context_stack.pop();
                acc.delimiter_balance_dec()?;
                Ok(acc)
            }

            '"' => {
                acc.memory.push(c);
                acc.context_stack.push(Context::String);
                Ok(acc)
            }

            '`' => {
                acc.memory.push(c);
                acc.context_stack.push(Context::Comment);
                Ok(acc)
            }

            number if c.is_numeric() => {
                acc.memory.push(number);
                acc.context_stack.push(Context::Number);
                Ok(acc)
            }

            't' => {
                acc.memory.push(c);
                acc.context_stack.push(Context::SymbolOrTrue);
                Ok(acc)
            }

            'f' => {
                acc.memory.push(c);
                acc.context_stack.push(Context::SymbolOrFalse);
                Ok(acc)
            }

            _ if c.is_alphabetic() => {
                acc.memory.push(c);
                acc.context_stack.push(Context::Symbol);
                Ok(acc)
            }

            ' ' => Ok(acc),

            _ => Err(JreplErr::InvalidSymbol(format!(
                "Found Invalid char in user_input while tokenizing a Bool or Symbol. Char: {}.",
                c
            ))),
        },

        Context::Comment => match c {
            '`' /* make sure the previous char isnt escaping the ` */ => {
                acc.memory.push(c);
                acc.tokens.push(Token::Comment(acc.memory.clone()));
                acc.reset_memory();
                acc.context_stack.pop();
                Ok(acc)
            }

            // TODO: handel escaping

            _ => {
                acc.memory.push(c);
                Ok(acc)
            }
        },

        Context::String => match c {
            '"' /* make sure the previous char isnt escaping the " */ => {
                acc.memory.push(c);
                acc.tokens.push(Token::StringLiteral(String::from(acc.memory.clone())));
                acc.reset_memory();
                acc.context_stack.pop();
                Ok(acc)
            }

            // TODO: latter on support string interpolation.

            _ if c.is_ascii() => {
                acc.memory.push(c);
                Ok(acc)
            }

            _ => Err(JreplErr::InvalidSymbol(
                format!(
                    "Found Invalid char in user_input while tokenizing a String. Char: {}.",
                    c
                )
            )),
        },

        Context::Number => match c {
            ')' => {
                acc.tokens.push(Token::NumberLiteral(acc.memory.clone()));
                acc.reset_memory();
                acc.context_stack.pop();

                acc.tokens.push(Token::CloseParen(String::from(c)));
                acc.delimiter_balance_dec()?;

                Ok(acc)
            }

            _ if c.is_numeric() => {
                acc.memory.push(c);
                Ok(acc)
            }

            _ if c == '.' && !acc.memory.contains(".") => {
                acc.memory.push(c);
                Ok(acc)
            }

            ' ' => {
                if acc.memory.ends_with(".") {
                    return Err(JreplErr::InvalidSymbol(String::from(
                        "Found decimal point at end of number in user_input while tokenizing a Number. Number should end with a digit.",
                    )));
                }
                acc.context_stack.pop();
                acc.tokens.push(Token::NumberLiteral(String::from(acc.memory.clone())));
                acc.reset_memory();
                Ok(acc)
            }

            _ => Err(JreplErr::InvalidSymbol(format!(
                "Found Invalid char in user_input while tokenizing a Number. Char: '{}'.",
                c
            ))),
        },

        Context::SymbolOrTrue => match c {
            _ if acc.memory.ends_with("t") && c == 'r' => {
                acc.memory.push(c);
                Ok(acc)
            }

            _ if acc.memory.ends_with("r") && c == 'u' => {
                acc.memory.push(c);
                Ok(acc)
            }

            _ if acc.memory.ends_with("u") && c == 'e' => {
                acc.memory.push(c);
                Ok(acc)
            }

            _ if acc.memory.ends_with("e") && c == ' ' => {
                acc.tokens.push(Token::BoolLiteral(String::from("true")));
                acc.context_stack.pop();
                acc.reset_memory();
                Ok(acc)
            }

            _ if c.is_alphanumeric() => {
                acc.memory.push(c);
                acc.context_stack.pop();
                acc.context_stack.push(Context::Symbol);
                Ok(acc)
            }

            _ => Err(JreplErr::InvalidSymbol(format!(
                "Found Invalid char in user_input while tokenizing a Bool or Symbol. Char: {}.",
                c
            ))),
        },

        Context::SymbolOrFalse => match c {
            _ if acc.memory.ends_with("f") && c == 'a' => {
                acc.memory.push(c);
                Ok(acc)
            }

            _ if acc.memory.ends_with("l") && c == 's' => {
                acc.memory.push(c);
                Ok(acc)
            }

            _ if acc.memory.ends_with("s") && c == 'e' => {
                acc.memory.push(c);
                Ok(acc)
            }

            _ if acc.memory.ends_with("e") && c == ' ' => {
                acc.tokens.push(Token::BoolLiteral(String::from("false")));
                acc.context_stack.pop();
                acc.reset_memory();
                Ok(acc)
            }

            _ if c.is_alphanumeric() => {
                acc.memory.push(c);
                acc.context_stack.pop();
                acc.context_stack.push(Context::Symbol);
                Ok(acc)
            }

            _ => Err(JreplErr::InvalidSymbol(format!(
                "Found Invalid char in user_input while tokenizing a Bool or Symbol. Char: {}.",
                c
            ))),
        },

        Context::Symbol => match c {
            _ if c.is_alphanumeric() => {
                acc.memory.push(c);
                Ok(acc)
            }

            ' ' => {
                acc.tokens.push(Token::Symbol(String::from(acc.memory.clone())));
                acc.reset_memory();
                acc.context_stack.pop();
                Ok(acc)
            }

            ')' => {
                acc.tokens.push(Token::Symbol(String::from(acc.memory.clone())));
                acc.reset_memory();
                acc.context_stack.pop(); // leaving symbol
                acc.context_stack.pop(); // leaving list
                acc.tokens.push(Token::CloseParen(String::from(c)));
                acc.delimiter_balance_dec()?;
                Ok(acc)
            }

            _ => Err(JreplErr::InvalidSymbol(format!(
                "Found Invalid char in user_input while tokenizing a Symbol. Char: {}.",
                c
            ))),
        },
    }
}

struct Accumulator {
    tokens: Vec<Token>,
    context_stack: Vec<Context>,
    memory: String,
    delimiter_balance: i32,
}

impl Accumulator {
    fn new() -> Accumulator {
        Accumulator {
            tokens: Vec::new(),
            context_stack: Vec::new(),
            memory: String::from(""),
            delimiter_balance: 0,
        }
    }

    fn context_stack_peek(&self) -> Context {
        if self.context_stack.is_empty() {
            return Context::Init;
        }
        return self.context_stack.last().unwrap().clone();
    }

    fn reset_memory(&mut self) {
        self.memory = String::from("");
    }

    fn delimiter_balance_inc(&mut self) {
        self.delimiter_balance += 1;
    }

    fn delimiter_balance_dec(&mut self) -> Result<(), JreplErr> {
        self.delimiter_balance -= 1;
        if self.delimiter_balance < 0 {
            return Err(JreplErr::UnbalancedDelimiter(String::from("")));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    OpenParen(String),
    CloseParen(String),
    Comment(String),
    Symbol(String),

    StringLiteral(String),
    NumberLiteral(String),
    BoolLiteral(String),
}

#[derive(Clone)]
enum Context {
    Init,
    List,
    Comment,

    SymbolOrTrue,
    SymbolOrFalse,

    Symbol,
    String,
    Number,
}

#[cfg(test)]
mod test {
    use super::*;

    fn open() -> Token {
        Token::OpenParen(String::from("("))
    }
    fn close() -> Token {
        Token::CloseParen(String::from(")"))
    }
    fn symbol(s: &str) -> Token {
        Token::Symbol(s.to_string())
    }
    fn strlit(s: &str) -> Token {
        Token::StringLiteral(s.to_string())
    }
    fn comment(s: &str) -> Token {
        Token::Comment(s.to_string())
    }
    fn number(s: &str) -> Token {
        Token::NumberLiteral(s.to_string())
    }

    #[test]
    fn lexer_help_command() {
        let result = lexer("(help)").expect("[lexer_help_command] Produced an error.");
        assert_eq!(result, vec![open(), symbol("help"), close()]);
    }

    #[test]
    fn lexer_handles_backtick_comment() {
        let result = lexer("(foo `this is a comment` bar)").expect("[lexer_help_command] Produced an error.");
        assert_eq!(
            result,
            vec![
                open(),
                symbol("foo"),
                comment("`this is a comment`"),
                symbol("bar"),
                close()
            ]
        );
    }

    // #[test]
    // fn lexer_handles_single_char_operators_as_tokens() {
    //     let result = lexer("(+ 1 2)").expect("[lexer_handles_single_char_operators_as_tokens] Produced an error.");
    //     assert_eq!(result, vec![open(), symbol("+"), symbol("1"), symbol("2"), close()]);
    // }

    #[test]
    fn lexer_white_space_in_command_allowed() {
        assert_eq!(lexer("( help   )").unwrap(), vec![open(), symbol("help"), close()]);
    }

    #[test]
    fn lexer_numbers_and_symbols() {
        let result = lexer("(add 123 foo456)").expect("[lexer_numbers_and_symbols] Produced an error.");
        assert_eq!(
            result,
            vec![open(), symbol("add"), number("123"), symbol("foo456"), close()],
        );
    }

    // #[test]
    // fn lexer_nested_expressions() {
    //     let result = lexer("(+ 1 (* 2 3))").expect("[lexer_nested_expressions] Produced an error.");
    //     assert_eq!(
    //         result,
    //         vec![
    //             open(),
    //             symbol("+"),
    //             symbol("1"),
    //             open(),
    //             symbol("*"),
    //             symbol("2"),
    //             symbol("3"),
    //             close(),
    //             close()
    //         ]
    //     );
    // }

    #[test]
    fn lexer_ignores_extra_spaces() {
        let result = lexer(" (   add    1    2  ) ").expect("[lexer_ignores_extra_spaces] Produced an error.");
        assert_eq!(result, vec![open(), symbol("add"), number("1"), number("2"), close()]);
    }

    #[test]
    fn lexer_handles_multiple_expressions() {
        assert_eq!(
            lexer("(help)(quit)").unwrap(),
            vec![open(), symbol("help"), close(), open(), symbol("quit"), close()]
        );
    }

    #[test]
    fn lexer_parses_alpha_numeric_symbols() {
        assert_eq!(
            lexer("(search file1 file2 file3)").unwrap(),
            vec![
                open(),
                symbol("search"),
                symbol("file1"),
                symbol("file2"),
                symbol("file3"),
                close()
            ]
        );
    }

    #[test]
    fn lexer_parses_paren_in_string() {
        assert_eq!(
            lexer("(search \"target(-t)ext\" filename)").unwrap(),
            vec![
                open(),
                symbol("search"),
                strlit("\"target(-t)ext\""),
                symbol("filename"),
                close()
            ]
        );
    }

    #[test]
    fn lexer_handles_multiline_backtick_comment() {
        assert_eq!(
            lexer("(foo `this is\na multiline\ncomment` bar)").unwrap(),
            vec![
                open(),
                symbol("foo"),
                comment("`this is\na multiline\ncomment`"),
                symbol("bar"),
                close()
            ]
        );
    }

    #[test]
    fn lexer_empty_command() {
        let result = lexer("()").expect("[lexer_empty_command] Produced an error.");
        assert_eq!(result, vec![open(), close()]);
    }

    #[test]
    fn lexer_single_number() {
        let result = lexer("(42)").expect("[lexer_single_number] Produced an error.");
        assert_eq!(result, vec![open(), number("42"), close()]);
    }

    #[test]
    fn lexer_multiple_whitespace_everywhere() {
        let result =
            lexer(" (    add   1     foo   )  ").expect("[lexer_multiple_whitespace_everywhere] Produced an error.");
        assert_eq!(result, vec![open(), symbol("add"), number("1"), symbol("foo"), close()]);
    }

    #[test]
    fn lexer_comment_only_expr() {
        let result = lexer("(`just a comment`)").expect("[lexer_comment_only_expr] Produced an error.");
        assert_eq!(result, vec![open(), comment("`just a comment`"), close()]);
    }

    #[test]
    fn lexer_comment_at_start_middle_end() {
        let result =
            lexer("(`start` foo `middle` bar `end`)").expect("[lexer_comment_at_start_middle_end] Produced an error.");
        assert_eq!(
            result,
            vec![
                open(),
                comment("`start`"),
                symbol("foo"),
                comment("`middle`"),
                symbol("bar"),
                comment("`end`"),
                close()
            ]
        );
    }

    #[test]
    fn lexer_string_literal_only_expr() {
        let result = lexer("(\"hello world\")").expect("[lexer_string_literal_only_expr] Produced an error.");
        assert_eq!(result, vec![open(), strlit("\"hello world\""), close()]);
    }

    #[test]
    fn lexer_string_literal_and_number() {
        let result = lexer("(print \"num is\" 100)").expect("[lexer_string_literal_and_number] Produced an error.");
        assert_eq!(
            result,
            vec![open(), symbol("print"), strlit("\"num is\""), number("100"), close()]
        );
    }

    #[test]
    fn lexer_string_literal_with_spaces_and_comment() {
        let result = lexer("(echo \" a spaced string \" `a comment` foo)")
            .expect("[lexer_string_literal_with_spaces_and_comment] Produced an error.");
        assert_eq!(
            result,
            vec![
                open(),
                symbol("echo"),
                strlit("\" a spaced string \""),
                comment("`a comment`"),
                symbol("foo"),
                close()
            ]
        );
    }

    #[test]
    fn lexer_number_with_decimal() {
        let result = lexer("(add 3.14 2.71)").expect("[lexer_number_with_decimal] Produced an error.");
        assert_eq!(
            result,
            vec![open(), symbol("add"), number("3.14"), number("2.71"), close()]
        );
    }

    // #[test]
    // fn lexer_true_and_false_literals() {
    //     let result = lexer("(if true false)").expect("[lexer_true_and_false_literals] Produced an error.");
    //     assert_eq!(
    //         result,
    //         vec![
    //             open(),
    //             symbol("if"),
    //             symbol("true"), // or you could parse as Token::BoolLiteral("true"), depending on your actual logic!
    //             symbol("false"),
    //             close()
    //         ]
    //     );
    // }

    // Error test (unbalanced parens)
    #[test]
    fn lexer_unbalanced_open_paren_error() {
        let result = lexer("(foo bar");
        assert!(result.is_err(), "Expected an error for unbalanced open paren.");
    }

    #[test]
    fn lexer_unbalanced_close_paren_error() {
        let result = lexer("foo bar)");
        assert!(result.is_err(), "Expected an error for unbalanced close paren.");
    }

    // Error test (unterminated string)
    #[test]
    fn lexer_unterminated_string_literal_error() {
        let result = lexer("(print \"unterminated)");
        assert!(result.is_err(), "Expected an error for unterminated string literal.");
    }

    // Error test (unterminated comment)
    #[test]
    fn lexer_unterminated_comment_error() {
        let result = lexer("(foo `unterminated comment)");
        assert!(result.is_err(), "Expected an error for unterminated comment.");
    }
}
