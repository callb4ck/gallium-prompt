use std::env::var;

pub struct Settings {
    /// Default: true
    pub emit_warning: bool,
    /// Default: false
    pub single_thread: bool,
}

impl Settings {
    /// Get Settings from environment variables
    pub fn from_env() -> Settings {
        Settings {
            emit_warning: var("GALLIUMPROMPT_NO_WARNING").is_err(),
            single_thread: var("GALLIUMPROMPT_SINGLE_THREAD").is_ok(),
        }
    }
}
