pub mod branch {
    use crate::{SETTINGS, emit_warning};
    use std::{env::current_dir, fs::read_to_string};
    /// Recursively scan for .git/HEAD
    fn scan_for_head() -> Option<String> {
        let dir = current_dir().unwrap().to_str().unwrap().to_string();

        let mut path: Vec<&str> = dir.split('/').collect();
        let mut file: Result<_, _>;

        while {
            file = read_to_string(path.join("/") + "/.git/HEAD");
            &file
        }
        .is_err()
        {
            path.pop();
            if path.is_empty() {
                return None;
            }
        }
        {
            Some(file.unwrap())
        }
    }

    /// Get name of the current git branch
    pub fn get_name() -> Option<String> {
        let contents = match scan_for_head() {
            Some(v) => v,
            _ => return None,
        };

        match contents
            .split('/')
            .collect::<Vec<&str>>()
            .get(2)
            .unwrap_or_else(|| {
                emit_warning!("Couldn't get branch name from '.git/HEAD'");
                &""
            }) {
                &"" => None,
                v => Some(v.trim_end().to_string()),
            }
    }
}
