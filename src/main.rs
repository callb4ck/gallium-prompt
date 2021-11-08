use lazy_static::lazy_static;
use settings::Settings;
use task::start_tasks;

mod error_utils;
mod functions;
mod parser;
mod settings;
mod task;

lazy_static! {
    static ref SETTINGS: Settings = Settings::from_env();
}

fn main() {
    start_tasks();
}
