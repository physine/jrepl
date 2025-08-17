mod appstate;
mod command_interpreter;
mod errors;
mod io;
mod statics;

use clap::Parser;
use command_interpreter::interpreter::interpret;
use rustyline::DefaultEditor;
use rustyline::Result;
use statics::commands::get_commands;

use crate::{
    appstate::AppState,
    command_interpreter::types::{Effect, Expr},
};

fn main() -> Result<()> {
    let args = Args::parse();
    // let json = extract_json(&args.input_files);

    let mut app_state = AppState::new();
    app_state.set_commands(get_commands());

    let mut tui = DefaultEditor::new()?;

    loop {
        let user_input = tui.readline(">")?;
        let effect = interpret(&app_state, &user_input);

        print_effect(&effect);

        if let Some(state) = effect.next_state {
            app_state.set_next_state(state);
        }

        // update UI based on changed state

        if app_state.should_exit() {
            break;
        }
    }

    Ok(())
}

pub fn print_effect(effect: &Effect) {
    // Value
    if let Some(ev) = &effect.eval_value {
        match ev {
            Expr::String(s) => print!("{}", s), // <-- preserves newlines
            other => println!("Evaluated value: {:?}", other),
        }
    }

    // Next state
    if let Some(next_state) = &effect.next_state {
        println!(
            "Next state: [state with {} commands, exit={}]",
            next_state.commands_len(),
            next_state.get_exit(),
        );
    }

    // Feedback
    if let Some(feedback) = &effect.user_feedback {
        println!("User feedback: {}", feedback);
    }

    // Error (keep debug unless you implement Display for JreplErr)
    if let Some(err) = &effect.err {
        eprintln!("Error: {:?}", err);
    }

    // Empty effect
    if effect.eval_value.is_none()
        && effect.next_state.is_none()
        && effect.user_feedback.is_none()
        && effect.err.is_none()
    {
        println!("Effect: empty (no value, feedback, state, or error)");
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'j', long = "json", num_args = 1..)]
    input_files: Vec<String>,
}
