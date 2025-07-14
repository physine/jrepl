use serde_json::Value;

#[derive(Debug)]
pub struct AppState {
    // open_input_files: Vec<File>,
    json: Vec<Value>,
}

impl AppState {
    pub fn from(json_data: Vec<Value>) -> AppState {
        AppState { json: json_data }
    }
}
