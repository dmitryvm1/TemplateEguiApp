use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct AppState {
    pub recent_file: Option<String>,
}