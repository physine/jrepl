use crate::command_interpreter::interpreter_parts::commands::{self, Command};

pub struct InterpreterBuilder {
    commands: Vec<Command>,
}

impl InterpreterBuilder {
    pub fn new() -> Self {
        InterpreterBuilder { commands: vec![] }
    }

    pub fn set_command(&mut self, command: Command) -> &mut Self {
        &self.commands.push(command);
        self
    }

    pub fn build(&mut self) -> Interpreter {
        Interpreter::from(self)
    }
}

pub struct Interpreter {
    commands: Vec<Command>,
}

impl Interpreter {
    pub fn from(builder: &mut InterpreterBuilder) -> Interpreter {
        Interpreter {
            commands: builder.commands,
        }
    }

    pub fn set_commands(&mut self, commands: Vec<Command>) {
        &self.set_commands(commands);
    }

    pub fn parse() {}
    pub fn eval() {}
    pub fn apply() {}
}

pub fn interpret(app_state: &AppState, input: &str) -> () {}
