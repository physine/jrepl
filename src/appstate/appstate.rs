use crate::{
    command_interpreter::{
        command::Command,
        types::{Expr, Referent},
    },
    errors::errors::JreplErr,
};
use std::{collections::HashMap, rc::Rc};

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

    pub fn resolve_symbol_to_terminal(&self, symbol: &str) -> Result<Expr, JreplErr> {
        let referent = self
            .state
            .symbol_table
            .get(symbol)
            .ok_or_else(|| JreplErr::UndefinedSymbol(format!("Undefined symbol: {}.", symbol)))?;

        match referent {
            Referent::Command(_) => Err(JreplErr::UndefinedSymbol(format!(
                "Expected '{}' to resolve to a terminal, but resolved to a command.",
                symbol
            ))),
            Referent::Expr(expr) => {
                if expr.is_terminal() {
                    Ok(expr.clone())
                } else if let Expr::Symbol(s) = expr {
                    self.resolve_symbol_to_terminal(s)
                } else {
                    Err(JreplErr::UndefinedSymbol(format!(
                        "Expected '{}' to resolve to a terminal, but resolved to a non-terminal: {:?}",
                        symbol, expr
                    )))
                }
            }
        }
    }

    pub fn get_command_from_symbol(&self, symbol: &str) -> Result<&Command, JreplErr> {
        let symbol_table = &self.state.symbol_table;

        if symbol_table.is_empty() {
            return Err(JreplErr::UndefinedSymbol("symbol table is empty".to_string()));
        }

        let referent = symbol_table
            .get(symbol)
            .ok_or_else(|| JreplErr::UndefinedSymbol(format!("Undefined symbol: {}. Expected a command.", symbol)))?;

        match referent {
            Referent::Command(cmd) => Ok(cmd),
            Referent::Expr(expr) => Err(JreplErr::UndefinedSymbol(format!(
                "Found symbol '{}' but it resolves to a non-command: {:?}.",
                symbol, expr
            ))),
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
