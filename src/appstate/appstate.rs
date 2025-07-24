// -------------------------------- AppState -------------------------------- //

#[derive(Debug, PartialEq)]
pub struct AppState {
    state_timeline: Vec<State>,
    current_state: usize,
}

impl AppState {
    pub fn new() -> AppState {
        let state = State::new();
        AppState {
            state_timeline: vec![state],
            current_state: 0,
        }
    }

    pub fn get_current_state(&self) -> &State {
        self.state_timeline.get(self.current_state).unwrap()
    }

    pub fn set_next_state(&mut self, state: State) {
        self.state_timeline.push(state);
        self.current_state += 1;
    }

    // pub fn clone_current_state(&self) -> State {
    //     self.state_timeline.get(self.current_state).unwrap().to_owned()
    // }

    pub fn should_exit(&self) -> bool {
        self.state_timeline.get(self.current_state).unwrap().exit
    }
}

// -------------------------------- State -------------------------------- //

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    // open_input_files: Vec<File>,
    // pub symbol_table: Map<String, >
    // pub json: Vec<Value>,
    // pub symbol_table: Map<String, Value>,
    editor: Editor,
    exit: bool,
}

impl State {
    fn new() -> State {
        State {
            editor: Editor {
                prompt_symbol: ">".into(),
                prompt_text: "()".into(),
                cursor_pos: 1, // 0 is where '(' is, and the cursor is just to the right of it
            },
            exit: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Editor {
    pub prompt_symbol: String,
    pub prompt_text: String,
    pub cursor_pos: usize,
}
