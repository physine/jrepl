use clap::Parser;

mod appstate;
use appstate::AppState;

mod file_io;
use file_io::extract_json;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'j', long = "json", num_args = 1..)]
    input_files: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let json = extract_json(&args.input_files);

    let app_state = AppState::from(json);

    dbg!(&app_state);

    // loop {
    //     // read - get user input
    //         // parse user input into command
    //     // eval - run command
    //     // print
    // }
}
