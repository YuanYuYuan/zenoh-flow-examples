use serde::{Serialize, Deserialize};
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub resolution: Option<String>,
    pub path: Option<String>,
    // pub delay: Duration,
    pub delay: u64,
}

pub static DEFAULT_RESOLUTION: &str = "800x600";

impl Default for Config {
    fn default() -> Self {
        Self {
            resolution: Some(DEFAULT_RESOLUTION.to_string()),
            path: None,
            // delay: Duration::from_millis(40),
            delay: 40,
        }
    }
}
