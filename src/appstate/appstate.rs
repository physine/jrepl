use crate::command_interpreter::{
    command::{self, Command},
    eval::EvalError,
    types::{Expr, Referent},
};
use std::{clone, collections::HashMap, fmt::format, rc::Rc};

// -------------------------------- AppState -------------------------------- //

// #[derive(Debug, PartialEq)]
pub struct AppState {
    state_timeline: Vec<State>,
    current_state: usize,
}

impl AppState {
    pub fn new() -> AppState {
        let initial_state = State::new();
        AppState {
            state_timeline: vec![initial_state],
            current_state: 0,
        }
    }

    // pub fn state_builder(&self) -> StateBuilder {}

    pub fn get_current_state(&self) -> &State {
        self.state_timeline.get(self.current_state).unwrap()
    }

    pub fn set_next_state(&mut self, state: State) {
        self.state_timeline.push(state);
        self.current_state += 1;
    }

    // pub fn clone_current_state(&self) -> State {
    //     self.state_timeline.get(self.current_state).unwrap().to_owned()
    // }

    pub fn resolve_symbol_to_terminal(&self, symbol: &str) -> Result<Expr, EvalError> {
        let symbol_table = &self.state_timeline.get(self.current_state).unwrap().symbol_table;

        let referent_option = symbol_table.get(symbol);
        if let None = referent_option {
            return Err(EvalError::UndefinedSymbol(format!("Undefined symbol: {symbol}.")));
        }

        let referent = referent_option.unwrap();
        match referent {
            Referent::Command(_) => {
                // TODO: create a specific error enum for this error insted of just using UndefinedSymbol
                return Err(EvalError::UndefinedSymbol(format!(
                    "Expected symbol '{symbol}' to resolve to a terminal but insted resolved to a command."
                )));
            }
            Referent::Expr(expr) => match expr {
                Expr::String(_) | Expr::Number(_) | Expr::Bool(_) | Expr::None | Expr::List(_) => Ok(expr.clone()),
                Expr::Symbol(symbol) => self.resolve_symbol_to_terminal(symbol),
            },
        }
    }

    pub fn get_command_from_symbol(&self, symbol: &str) -> Result<&Command, EvalError> {
        let symbol_table = &self.state_timeline.get(self.current_state).unwrap().symbol_table;
        // println!("|------------------------------------------------|");
        // for (k, v) in symbol_table.iter() {
        //     println!("key: {:?}, value address: {:p}", k, v);
        // }
        if symbol_table.is_empty() {
            // TODO: create a specific error enum for this error insted of just using UndefinedSymbol
            return Err(EvalError::UndefinedSymbol(format!("symbol_table is empty. 0.1")));
        }

        let referent_option = symbol_table.get(symbol);
        if let None = referent_option {
            // TODO: create a specific error enum for this error insted of just using UndefinedSymbol
            return Err(EvalError::UndefinedSymbol(format!(
                "Undefined symbol: {symbol}. Expected to find a command 1."
            )));
        }

        let referent = referent_option.unwrap();
        match referent {
            Referent::Command(command) => Ok(command),
            Referent::Expr(_) => {
                // TODO: create a specific error enum for this error insted of just using UndefinedSymbol
                return Err(EvalError::UndefinedSymbol(format!(
                    "Found Expr where : {symbol}. Expected to find a command 2."
                )));
            }
        }
    }

    pub fn set_commands(&mut self, commands: Vec<Command>) {
        let commands: Vec<Rc<Command>> = commands.into_iter().map(Rc::new).collect();
        self.state_timeline.get_mut(self.current_state).unwrap().commands = commands;
        self.register_commands_with_symbol_table();
    }

    pub fn get_commands(&self) -> &Vec<Rc<Command>> {
        &self.state_timeline.get(self.current_state).unwrap().commands
    }

    fn register_commands_with_symbol_table(&mut self) {
        let state = self.state_timeline.get_mut(self.current_state).unwrap();
        let commands = &state.commands;
        let symbol_table = &mut state.symbol_table;
        for command in commands {
            symbol_table.insert(command.symbol.clone(), Referent::Command(command.clone()));
        }
    }

    pub fn should_exit(&self) -> bool {
        self.state_timeline.get(self.current_state).unwrap().exit
    }
}

// -------------------------------- State -------------------------------- //

// #[derive(Clone, Debug, PartialEq)]
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

// TODO: register Commands symbols with AppState

impl State {
    fn new() -> State {
        State {
            editor: Editor {
                prompt_symbol: ">".into(),
                prompt_text: "()".into(),
                cursor_pos: 1, // 0 is where '(' is, and the cursor is just to the right of it
            },
            exit: false,
            commands: Vec::new(),
            symbol_table: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Editor {
    pub prompt_symbol: String,
    // pub prompt_text: LinkedList<String>,
    pub prompt_text: String,
    pub cursor_pos: usize,
}
