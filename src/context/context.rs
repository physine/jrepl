use crate::command_interpreter::command::Command;

pub struct Context {
    pub commands: Vec<Command>,
}

impl Context {
    pub fn new() -> Context {
        // let commands = get_commands();
        Context { commands: Vec::new() }
    }

    pub fn set_commands(&mut self, commands: Vec<Command>) -> &mut Self {
        self.commands = commands;
        self
    }
}
