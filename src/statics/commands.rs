use clap::builder;

use crate::{
    appstate::AppState,
    command_interpreter::{eval::number_of, types::Effect},
};
use crate::{appstate::State, command_interpreter::command::Command};
use crate::{
    command_interpreter::{eval, types::Expr},
    errors::errors::JreplErr,
};

use crate::command_interpreter::eval::value_of;
use crate::command_interpreter::types::Referent;

pub fn get_commands() -> Vec<Command> {
    vec![
        Command {
            symbol: "help".to_string(),
            description: "help \t display avalible options. Usage: (help)".to_string(),
            eval_fn_ptr: Box::new(|app_state: &AppState, _exprs: &[Expr]| {
                // get_commands() returns something like &Vec<Rc<Command>>
                let mut cmds: Vec<&Command> = app_state
                    .get_commands()
                    .iter()
                    .map(|rc| rc.as_ref()) // &Rc<Command> -> &Command
                    .collect();

                // sort by symbol
                cmds.sort_by(|a, b| a.symbol.cmp(&b.symbol));

                // column width for alignment
                let col1 = cmds
                    .iter()
                    .map(|c| c.symbol.len())
                    .max()
                    .unwrap_or(6)
                    .max("Command".len());

                let sep1 = "-".repeat(col1);
                let sep2 = "-----------";

                let body = cmds
                    .into_iter()
                    .map(|cmd| {
                        // Strip duplicated symbol prefix from description if present
                        let mut desc = cmd.get_description().to_string();
                        if let Some(rest) = desc.strip_prefix(&cmd.symbol) {
                            desc = rest.trim_start_matches([' ', '\t']).to_string();
                        }
                        format!("  {:col1$}  {}", cmd.symbol, desc, col1 = col1)
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                let help_msg = format!(
                    "Available commands:\n\n  {:col1$}  {}\n  {:col1$}  {}\n{}\n",
                    "Command",
                    "Description",
                    sep1,
                    sep2,
                    body,
                    col1 = col1
                );

                Ok(Effect::from_eval_value(Expr::String(help_msg)))
            }),
        },
        Command {
            symbol: "exit".to_string(),
            description: "exit \t exit the interpreter. Usage: (exit)".to_string(),
            eval_fn_ptr: Box::new(|app_state: &AppState, _exprs: &[Expr]| {
                let next_state = app_state.apply_action(|state| {
                    state.exit = true;
                });
                Ok(Effect {
                    eval_value: None,
                    next_state: Some(next_state),
                    user_feedback: None,
                    err: None,
                })
            }),
        },
        Command {
            symbol: "+".to_string(),
            description: "+ \t Usage: (+ Number Number ...)".to_string(),
            eval_fn_ptr: Box::new(|app_state: &AppState, exprs: &[Expr]| {
                let sum = exprs
                    .iter()
                    .try_fold(0.0, |acc, e| Ok::<_, JreplErr>(acc + number_of(app_state, e)?))?;

                Ok(Effect::from_eval_value(Expr::Number(sum)))
            }),
        },
        Command {
            symbol: "-".to_string(),
            description: "- \t Usage: (- Number Number ...)".to_string(),
            eval_fn_ptr: Box::new(|app_state: &AppState, exprs: &[Expr]| {
                if exprs.is_empty() {
                    return Err(JreplErr::UndefinedSymbol(
                        "'-' expects at least one argument".to_string(),
                    ));
                }

                let first = number_of(app_state, &exprs[0])?;
                let result = if exprs.len() == 1 {
                    -first
                } else {
                    exprs[1..]
                        .iter()
                        .try_fold(first, |acc, e| Ok::<f64, JreplErr>(acc - number_of(app_state, e)?))?
                };

                Ok(Effect::from_eval_value(Expr::Number(result)))
            }),
        },
        Command {
            symbol: "*".to_string(),
            description: "* \t Usage: (* Number Number ...)".to_string(),
            eval_fn_ptr: Box::new(|app_state: &AppState, exprs: &[Expr]| {
                let product = exprs
                    .iter()
                    .try_fold(1.0, |acc, e| Ok::<f64, JreplErr>(acc * number_of(app_state, e)?))?;
                Ok(Effect::from_eval_value(Expr::Number(product)))
            }),
        },
        Command {
            symbol: "/".to_string(),
            description: "/ \t Usage: (/ Number Number ...)".to_string(),
            eval_fn_ptr: Box::new(|app_state: &AppState, exprs: &[Expr]| {
                if exprs.is_empty() {
                    return Err(JreplErr::OperatorFormatErr(
                        "'/‘ expects at least one argument".to_string(),
                    ));
                }

                let first = number_of(app_state, &exprs[0])?;
                let result = if exprs.len() == 1 {
                    if first == 0.0 {
                        return Err(JreplErr::ArithmeticErr("Division by zero".to_string()));
                    }
                    1.0 / first
                } else {
                    exprs[1..].iter().try_fold(first, |acc, e| {
                        let n = number_of(app_state, e)?;
                        if n == 0.0 {
                            return Err(JreplErr::ArithmeticErr("Division by zero".to_string()));
                        }
                        Ok::<f64, JreplErr>(acc / n)
                    })?
                };

                Ok(Effect::from_eval_value(Expr::Number(result)))
            }),
        },
        // Command {
        //     symbol: "search".into(),
        //     description: "".into(),
        //     // param_format: [descripion="target-text" type="String" size="1"]
        //     //               [descripion="source to check against" type="File | String" size="1..n"]
        // },
        // Command {
        //     symbol: "list".into(),
        //     description: "".into(),
        //     // param_format: [description="list the avaliable 'things (symbols, commands, files, etc)'" type="String" size="1..n"]
        // },
        // Command {
        //     symbol: "undo".into(),
        //     description: "".into(),
        //     // param_format: ""
        // },
        // Command {
        //     symbol: "quit".into(),
        //     description: "".into(),
        //     // param_format: ""
        // },
        Command {
            symbol: "defn".to_string(),
            description: "defn \t Define or redefine a symbol. Usage: (defn <name> <expr>)".to_string(),
            eval_fn_ptr: Box::new(|app_state: &AppState, exprs: &[Expr]| {
                if exprs.len() != 2 {
                    return Err(JreplErr::OperatorFormatErr(
                        "'defn' expects exactly 2 arguments: <name> <expr>".to_string(),
                    ));
                }

                let name = match &exprs[0] {
                    Expr::Symbol(s) => s.clone(),
                    other => {
                        return Err(JreplErr::OperatorFormatErr(format!(
                            "'defn' first argument must be a symbol name, got {:?}",
                            other
                        )));
                    }
                };

                // Evaluate the value expression
                let value = value_of(app_state, &exprs[1])?;

                // Only allow terminals to be bound, so later resolution works predictably.
                if !value.is_literal() {
                    return Err(JreplErr::UndefinedSymbol(format!(
                        "'defn' value must evaluate to a terminal (String|Number|Bool|None), got {:?}",
                        value
                    )));
                }

                let next_state = app_state.apply_action(|state| {
                    state.symbol_table.insert(name.clone(), Referent::Expr(value.clone()));
                });

                Ok(Effect {
                    eval_value: Some(value), // return the bound value
                    next_state: Some(next_state),
                    user_feedback: None,
                    err: None,
                })
            }),
        },
        // Command {
        //     symbol: "def".into(),
        //     description: "".into(),
        //     // param_format: [description="Symbol name" type="String" size="1"]
        //     //               [description="value to be bound to the symbol" type="Expr" size="1"]
        // },
        // Command {
        //     symbol: "do".into(),
        //     description: "".into(),
        //     // param_format: [description="Exprs to evaluate" type="Expr" size="1..n"]
        // },
        // Command {
        //     symbol: "set".into(),
        //     description: "".into(),
        //     // param_format: [description="existing symbol name" type="Expr" size="1"]
        //     // param_format: [description="expression to eval and bind to the symbol" type="Expr" size="1"]
        // },
        // Command {
        //     symbol: "print".into(),
        //     description: "".into(),
        //     // param_format: [description="existing symbol name" type="Expr" size="1"]
        //     // param_format: [description="expression to eval and bind to the symbol" type="Expr" size="1"]
        // },
        // Command {
        //     symbol: String::from("man"),
        //     description: String::from(""),
        //     // param_format: [description="" type="Expr" size="1..n"]
        // },

        // ---- Control Flow ------------------
        Command {
            symbol: "if".to_string(),
            description: "if\t Usage: (if <bool-expr> <then-expr> [<else-expr>])".to_string(),
            eval_fn_ptr: Box::new(|app_state: &AppState, exprs: &[Expr]| {
                use crate::command_interpreter::eval::{eval, value_of};

                if exprs.len() < 2 {
                    return Err(JreplErr::UndefinedSymbol(
                        "'if' expects at least 2 arguments: condition and then-branch".to_string(),
                    ));
                }

                let cond_ev = value_of(app_state, &exprs[0])?;
                let cond = match cond_ev {
                    Expr::Bool(b) => b,
                    other => {
                        return Err(JreplErr::UndefinedSymbol(format!(
                            "Type error in 'if': expected Bool for condition, got {:?}",
                            other
                        )));
                    }
                };

                if cond {
                    // evaluate only the then branch
                    eval(app_state, &exprs[1])
                } else if exprs.len() >= 3 {
                    // evaluate only the else branch
                    eval(app_state, &exprs[2])
                } else {
                    Ok(Effect::from_eval_value(Expr::None))
                }
            }),
        },
    ]
}
