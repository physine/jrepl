use std::io::Write;

use clap::Parser;

mod io;
use io::read::user_input;

mod command_interpreter;
use command_interpreter::interpreter::interpret;

mod appstate;
use crate::{
    appstate::AppState,
    command_interpreter::types::{Effect, Expr},
};

mod statics;
use statics::commands::get_commands;

fn main() {
    let args = Args::parse();
    // let json = extract_json(&args.input_files);

    let mut app_state = AppState::new();
    app_state.set_commands(get_commands());

    loop {
        print!(">");
        std::io::stdout().flush().expect("Error flushing stdout");

        let user_input = user_input();
        let effect = interpret(&app_state, &user_input);

        print_effect(&effect);

        if let Some(state) = effect.next_state {
            app_state.set_next_state(state);
        }

        // app_state.set_next_state(state);

        // let next_state = effect.apply();
        // app_state.set_next_state(next_state);

        // could cause state change (ie. create a new symbol or change the value of a symbol or undo the last undoable command ('help' isnt undoable)),
        // could also just yeald a string to be printed to the UI (ie. the held command)

        // state.update(next_state);

        // update UI based on changed state

        if app_state.should_exit() {
            break;
        }

        // if user_input == "q" {
        //     break;
        // }
    }
}

pub fn print_effect(effect: &Effect) {
    if let Some(eval_value) = &effect.eval_value {
        println!("----------------------------------");
        println!("Evaluated value: {:?}", eval_value);
    }
    if let Some(next_state) = &effect.next_state {
        println!("----------------------------------");
        println!(
            "Next state: [state with {} commands, exit={}]",
            next_state.commands_len(),
            next_state.get_exit(),
        );
    }
    if let Some(feedback) = &effect.user_feedback {
        println!("----------------------------------");
        println!("User feedback: {}", feedback);
    }
    if let Some(err) = &effect.err {
        println!("----------------------------------");
        println!("Error: {:?}", err);
    }
    // If nothing was Some, print that it's empty
    if effect.eval_value.is_none()
        && effect.next_state.is_none()
        && effect.user_feedback.is_none()
        && effect.err.is_none()
    {
        println!("----------------------------------");
        println!("Effect: empty (no value, feedback, state, or error)");
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'j', long = "json", num_args = 1..)]
    input_files: Vec<String>,
}
