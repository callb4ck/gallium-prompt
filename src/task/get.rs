use std::env::args;
use std::process::exit;

pub fn from_arg() -> String {
    args()
        .collect::<Vec<String>>()
        .get(1)
        .unwrap_or_else(|| {
            eprintln!("[FATAL]: Not enough arguments supplied (expected 1)");
            exit(0);
        })
        .to_string()
}
