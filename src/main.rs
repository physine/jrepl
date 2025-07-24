use std::io::Write;

use clap::Parser;

mod context;
use context::context::Context;

mod appstate;

mod io;
use io::read::user_input;

mod command_interpreter;
use command_interpreter::interpreter::interpret;

use crate::appstate::AppState;

mod statics;
use statics::commands::get_commands;

fn main() {
    let args = Args::parse();
    // let json = extract_json(&args.input_files);

    let mut app_state = AppState::new();
    let mut ctx = Context::new();
    ctx.set_commands(get_commands());

    loop {
        print!(">");
        std::io::stdout().flush().expect("Error flushing stdout");

        let user_input = user_input();
        let effect = interpret(&app_state, &ctx, &user_input);

        let next_state = effect.apply();
        app_state.set_next_state(next_state);

        // could cause state change (ie. create a new symbol or change the value of a symbol or undo the last undoable command ('help' isnt undoable)),
        // could also just yeald a string to be printed to the UI (ie. the held command)

        // state.update(next_state);

        // update UI based on changed state

        if user_input == "q" {
            break;
        }

        // if app_state.should_exit() {
        //     break;
        // }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'j', long = "json", num_args = 1..)]
    input_files: Vec<String>,
}
