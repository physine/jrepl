use serde_json::{Map, Number, Value};

#[derive(Debug)]
pub struct AppState {
    // open_input_files: Vec<File>,
    pub json: Vec<Value>,
    // pub symbol_table: Map<String, Value>,
    pub exit: bool,
}

impl AppState {
    pub fn from(json_data: Vec<Value>) -> AppState {
        AppState {
            json: json_data,
            exit: false,
        }
    }

    pub fn should_exit(&self) -> bool {
        self.exit
    }

    pub fn set_exit_flag(&mut self) {
        self.exit = true;
    }

    // pub fn new() -> AppState {
    //     AppState {
    //         // json: serde_json::Number(1),
    //         // symbol_table: Map::,
    //     }
    // }
}
