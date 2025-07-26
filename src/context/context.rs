use crate::command_interpreter::command::Command;

pub struct Context {
    pub commands: Vec<Command>,
}

impl Context {
    pub fn new() -> Context {
        // let commands = get_commands();
        Context { commands: Vec::new() }
    }

    pub fn from(commands: Vec<Command>) -> Context {
        Context { commands }
    }

    pub fn set_commands(&mut self, commands: Vec<Command>) -> &mut Self {
        self.commands = commands;
        self
    }

    pub fn is_command(&self, command: &str) -> bool {
        true
    }

    pub fn get_command(&self, symbol: &str) -> Option<&Command> {
        self.commands.iter().find(|command| command.symbol == symbol)
    }

    pub fn is_symbol(&self, symbol: &str) -> bool {
        self.commands.iter().filter(|command| command.symbol == symbol).count() != 0
    }

    // pub fn resolve_symbol(&self, symbol: &str) -> Expr {
    //     // self.commands.get(index)
    // }
}
