use zenoh::prelude::*;

pub struct Config {
    pub locator: Locator,
    pub keyexpr: String,
}
