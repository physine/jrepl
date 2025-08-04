use crate::command_interpreter::{
    command::{self, Command},
    eval::EvalError,
    types::{Expr, Referent},
};
use std::{clone, collections::HashMap, fmt::format, rc::Rc};

// -------------------------------- AppState -------------------------------- //

pub struct AppState {
    state: State,
}

impl AppState {
    pub fn new() -> AppState {
        AppState { state: State::new() }
    }

    pub fn set_next_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn resolve_symbol_to_terminal(&self, symbol: &str) -> Result<Expr, EvalError> {
        let symbol_table = &self.state.symbol_table;

        let referent = symbol_table
            .get(symbol)
            .ok_or_else(|| EvalError::UndefinedSymbol(format!("Undefined symbol: {symbol}.")))?;

        match referent {
            Referent::Command(_) => {
                // TODO: create a specific error enum for this error insted of just using UndefinedSymbol
                Err(EvalError::UndefinedSymbol(format!(
                    "Expected symbol '{symbol}' to resolve to a terminal but instead resolved to a command."
                )))
            }
            Referent::Expr(expr) => match expr {
                Expr::String(_) | Expr::Number(_) | Expr::Bool(_) | Expr::None | Expr::List(_) => Ok(expr.clone()),
                Expr::Symbol(symbol) => self.resolve_symbol_to_terminal(symbol),
            },
        }
    }

    pub fn get_command_from_symbol(&self, symbol: &str) -> Result<&Command, EvalError> {
        let symbol_table = &self.state.symbol_table;

        if symbol_table.is_empty() {
            // TODO: create a specific error enum for this error insted of just using UndefinedSymbol
            return Err(EvalError::UndefinedSymbol(format!("symbol_table is empty. 0.1")));
        }

        let referent = symbol_table
            .get(symbol)
            .ok_or_else(|| EvalError::UndefinedSymbol(format!("Undefined symbol: {symbol}. Expected a command.")))?;

        match referent {
            Referent::Command(command) => Ok(command),
            _ => Err(EvalError::UndefinedSymbol(format!(
                "Found symbol: '{symbol}' where a command was expected."
            ))), // TODO: create a specific error enum for this error insted of just using UndefinedSymbol
        }
    }

    pub fn get_commands(&self) -> &[Rc<Command>] {
        &self.state.commands
    }

    pub fn set_commands(&mut self, commands: Vec<Command>) {
        self.state.commands = commands.into_iter().map(Rc::new).collect();
        self.register_commands_with_symbol_table();
    }

    fn register_commands_with_symbol_table(&mut self) {
        let symbol_table = &mut self.state.symbol_table;
        for command in &self.state.commands {
            symbol_table.insert(command.symbol.clone(), Referent::Command(command.clone()));
        }
    }

    pub fn apply_action<F>(&self, action: F) -> State
    where
        F: FnOnce(&mut StateBuilder),
    {
        let mut builder = self.state.to_builder();
        action(&mut builder);
        builder.build()
    }

    pub fn should_exit(&self) -> bool {
        self.state.exit
    }
}

#[derive(Clone)]
pub struct State {
    // open_input_files: Vec<File>,
    // pub symbol_table: Map<String, >
    // pub json: Vec<Value>,
    // pub symbol_table: Map<String, Value>,
    editor: Editor,
    exit: bool,
    commands: Vec<Rc<Command>>,
    symbol_table: HashMap<String, Referent>,
}

impl State {
    pub fn new() -> State {
        State {
            editor: Editor::new(),
            exit: false,
            commands: Vec::new(),
            symbol_table: HashMap::new(),
        }
    }

    pub fn to_builder(&self) -> StateBuilder {
        StateBuilder {
            editor: self.editor.clone(),
            exit: self.exit,
            commands: self.commands.clone(),
            symbol_table: self.symbol_table.clone(),
        }
    }

    pub fn set_exit(&mut self) {
        self.exit = true;
    }

    pub fn commands_len(&self) -> usize {
        self.commands.len()
    }

    pub fn get_exit(&self) -> bool {
        self.exit
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Editor {
    pub prompt_symbol: String,
    // pub prompt_text: LinkedList<String>,
    pub prompt_text: String,
    pub cursor_pos: usize,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            prompt_symbol: ">".to_string(),
            prompt_text: "".to_string(),
            cursor_pos: 1,
        }
    }
}

pub struct StateBuilder {
    pub editor: Editor,
    pub exit: bool,
    pub commands: Vec<Rc<Command>>,
    pub symbol_table: HashMap<String, Referent>,
}

impl StateBuilder {
    pub fn build(self) -> State {
        State {
            editor: self.editor,
            exit: self.exit,
            commands: self.commands,
            symbol_table: self.symbol_table,
        }
    }
}

// impl AppState {
//     pub fn get_current_state(&self) -> &State {
//         self.state_timeline.get(self.current_state).unwrap()
//     }

//     pub fn set_next_state(&mut self, state: State) {
//         self.state_timeline.push(state);
//         self.current_state += 1;
//     }

//     // pub fn clone_current_state(&self) -> State {
//     //     self.state_timeline.get(self.current_state).unwrap().to_owned()
//     // }
//     pub fn set_commands(&mut self, commands: Vec<Command>) {
//         let commands: Vec<Rc<Command>> = commands.into_iter().map(Rc::new).collect();
//         self.state_timeline.get_mut(self.current_state).unwrap().commands = commands;
//         self.register_commands_with_symbol_table();
//     }

//     pub fn get_commands(&self) -> &Vec<Rc<Command>> {
//         &self.state_timeline.get(self.current_state).unwrap().commands
//     }

//     fn register_commands_with_symbol_table(&mut self) {
//         let state = self.state_timeline.get_mut(self.current_state).unwrap();
//         let commands = &state.commands;
//         let symbol_table = &mut state.symbol_table;
//         for command in commands {
//             symbol_table.insert(command.symbol.clone(), Referent::Command(command.clone()));
//         }
//     }

//     pub fn should_exit(&self) -> bool {
//         self.state_timeline.get(self.current_state).unwrap().exit
//     }
// }

// // -------------------------------- State -------------------------------- //

// // #[derive(Clone, Debug, PartialEq)]
// pub struct State {
//     // open_input_files: Vec<File>,
//     // pub symbol_table: Map<String, >
//     // pub json: Vec<Value>,
//     // pub symbol_table: Map<String, Value>,
//     editor: Editor,
//     exit: bool,
//     commands: Vec<Rc<Command>>,
//     symbol_table: HashMap<String, Referent>,
// }

// // TODO: register Commands symbols with AppState

// impl State {
//     fn new() -> State {
//         State {
//             editor: Editor {
//                 prompt_symbol: ">".into(),
//                 prompt_text: "()".into(),
//                 cursor_pos: 1, // 0 is where '(' is, and the cursor is just to the right of it
//             },
//             exit: false,
//             commands: Vec::new(),
//             symbol_table: HashMap::new(),
//         }
//     }
// }

// #[derive(Clone, Debug, PartialEq)]
// struct Editor {
//     pub prompt_symbol: String,
//     // pub prompt_text: LinkedList<String>,
//     pub prompt_text: String,
//     pub cursor_pos: usize,
// }
