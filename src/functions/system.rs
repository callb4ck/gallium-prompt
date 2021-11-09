use crate::{SETTINGS, emit_warning};
use std::{env::{current_dir, var}, fs::read_to_string, process::Command};

#[inline]
pub fn get_user() -> String {
    var("USER").unwrap_or_else(|_| {
        emit_warning!("USER environment variable is not set");

        String::new()
    })
}

#[inline]
pub fn get_host() -> String {
    read_to_string("/etc/hostname").unwrap_or_else(|e| {
        emit_warning!("Couldn't get hostname from /etc/hostname.\nError: {}", e.to_string());

        String::new()
    }).trim_end().to_string()
}

#[inline]
pub fn current_working_dir(expand: bool) -> String {
    let path = if let Ok(path) = current_dir() {
        path.to_str().unwrap_or("no_valid_unicode").to_string()
    } else {
        "unknown_path".to_string()
    };

    if !expand {
        if let Ok(user) = var("USER") {
            let homedir = "/home/".to_string() + &user;
            if path.starts_with(&homedir) {
                return path.replace(&homedir, "~");
            }
        }
    }

    path
}

#[inline]
pub fn exec_cmd(command: &str, args: &[String]) -> String {
    match Command::new(command).args(args).output() {
        Ok(output) => String::from_utf8(output.stdout).unwrap_or_else(|_| {
            let mut full_command = String::from(command);
            full_command.push(' ');
            full_command.push_str(&args.join(" "));

            emit_warning!("Output of '{}' is not valid utf-8", full_command);

            String::new()
        }),
        _ => {
            let mut full_command = String::from(command);
            full_command.push(' ');
            full_command.push_str(&args.join(" "));

            emit_warning!("Couldn't get output of '{}'", full_command);
            String::new()
        }
    }
}
