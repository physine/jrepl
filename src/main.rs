use std::io::Write;

use clap::Parser;

mod context;
use context::context::Context;

mod appstate;
use appstate::AppState;

mod io;
use io::read::extract_json;
use io::read::user_input;

mod command_interpreter;
use command_interpreter::commands::get_commands;
use command_interpreter::interpreter::interpret;

fn main() {
    let args = Args::parse();
    let json = extract_json(&args.input_files);
    // dbg!(&json);

    let mut app_state = AppState::from(json);
    // let app_state = AppState::new();
    // app_state.set_json(json);
    // app_state.set_symbol_table();

    let mut ctx = Context::new();
    let commands = get_commands();
    ctx.set_commands(commands);

    loop {
        print!(">");
        std::io::stdout().flush().expect("Error flushing stdout");

        let user_input = user_input();
        let effect = interpret(&app_state, &ctx, &user_input);
        effect.exe();

        // could cause state change (ie. create a new symbol or change the value of a symbol or undo the last undoable command ('help' isnt undoable)),
        // could also just yeald a string to be printed to the UI (ie. the held command)

        // state.update(next_state);

        // update UI based on changed state

        if user_input == "q" {
            app_state.set_exit_flag();
        }

        if app_state.should_exit() {
            break;
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'j', long = "json", num_args = 1..)]
    input_files: Vec<String>,
}
