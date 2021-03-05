// use crate::common::types::{BotState};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use std::sync::{Arc, Mutex};
const STATE_FILE: &'static str = "./src/state.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BotState {
    pub commands: CommandsState,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommandsState {
    pub first_list: Vec<String>,
}

impl BotState {
    pub fn add_visitor(&mut self, visitor: String) {
        self.commands.first_list.push(visitor);
        self.persist();
    }
    pub fn persist(&mut self) {
        let raw_data = serde_json::to_string_pretty(self).unwrap();
        fs::write(STATE_FILE, raw_data).expect("Failed to persist state");
    }
}
pub fn init_state() -> Result<Arc<Mutex<BotState>>> {
    let raw_data = fs::read_to_string(&STATE_FILE).expect("Unable to load state file");
    let state: BotState = serde_json::from_str(&raw_data)?;

    let mutable_state = Arc::new(Mutex::new(state));

    Ok(mutable_state)
}
