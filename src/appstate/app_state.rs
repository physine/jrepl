mod app_state;

use std::files::File;

pub struct AppState {
    open_input_files: Vec<File>,
}
