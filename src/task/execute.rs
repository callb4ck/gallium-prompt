use crate::{
    emit_warning,
    functions::git::branch,
    functions::system::{current_working_dir, get_host, get_user},
    functions::{
        color::{self, background_rgb, text_rgb},
        system::exec_cmd,
    },
    SETTINGS,
};

pub fn execute_task(function: &str, arg_list: Option<Vec<String>>) -> String {
    let arg_list = if let Some(args) = arg_list {
        if args == [""] {
            None
        } else {
            Some(args)
        }
    } else {
        None
    };

    match function.to_ascii_lowercase().as_ref() {
        "reset" => color::reset(),

        "text_color" | "color" => {
            if let Some(args) = arg_list {
                return color::text(&args[0]);
            }

            emit_warning!("Expected 1 argument for function 'text_color'");

            String::new()
        }

        "text_rgb" | "rgb" => {
            if let Some(args) = arg_list {
                emit_warning!(if args.len() < 3; "Expected at least 3 arguments for function 'text_rgb'");

                let r = args.get(0).unwrap_or(&"0".to_string()).to_string();
                let g = args.get(1).unwrap_or(&"0".to_string()).to_string();
                let b = args.get(2).unwrap_or(&"0".to_string()).to_string();
                let bold = args.get(3).unwrap_or(&"normal".to_string()).to_string();

                emit_warning!(if r.parse::<u8>().is_err();
                    "(1st arg) Argument 'r' not in range (0-255) in function 'text_rgb'");

                emit_warning!(if g.parse::<u8>().is_err();
                    "(2nd arg) Argument 'g' not in range (0-255) in function 'text_rgb'");

                emit_warning!(if b.parse::<u8>().is_err();
                    "(3rd arg) Argument 'b' not in range (0-255) in function 'text_rgb'");

                let bold = match bold.as_ref() {
                    "bold" | "true" => true,
                    "normal" | "false" => false,
                    _ => {
                        emit_warning!("(4th optarg) Optional argument 'bold' must be set to normal/false or bold/true");
                        false
                    }
                };
                text_rgb(&r, &g, &b, bold)
            } else {
                emit_warning!("Expected at least 3 arguments for function 'text_rgb'");
                String::new()
            }
        }

        "background_color" | "background" => {
            if let Some(args) = arg_list {
                return color::background(&args[0]);
            }

            emit_warning!("Expected 1 argument for function 'background_color'");

            String::new()
        }

        "background_rgb" => {
            if let Some(args) = arg_list {
                emit_warning!(if args.len() < 3; "Expected at least 3 arguments for function 'background_rgb'");

                let r = args.get(0).unwrap_or(&"0".to_string()).to_string();
                let g = args.get(1).unwrap_or(&"0".to_string()).to_string();
                let b = args.get(2).unwrap_or(&"0".to_string()).to_string();

                emit_warning!(if r.parse::<u8>().is_err();
                    "(1st arg) Argument 'r' not in range (0-255) in function 'background_rgb'");

                emit_warning!(if g.parse::<u8>().is_err();
                    "(2nd arg) Argument 'g' not in range (0-255) in function 'background_rgb'");

                emit_warning!(if b.parse::<u8>().is_err();
                    "(3rd arg) Argument 'b' not in range (0-255) in function 'background_rgb'");

                background_rgb(&r, &g, &b)
            } else {
                emit_warning!("Expected at least 3 arguments for function 'background_rgb'");
                String::new()
            }
        }

        "effect" => {
            if let Some(arg_list) = arg_list {
                let mut effect_list = String::new();

                for arg in arg_list {
                    effect_list.push_str(&color::effect(&arg))
                }

                return effect_list;
            }

            emit_warning!("Expected at least 1 argument for function 'effect'");
            String::new()

        }

        "get_user" | "user" | "username" => get_user(),

        "get_host" | "host" | "hostname" => get_host(),

        "current_working_dir" | "cwd" | "pwd" => current_working_dir(),

        "branch" | "git_branch" | "branch_name" | "git_branch_name" => match branch::get_name() {
            Some(branch_name) => match arg_list {
                Some(args) => {
                    let mut open = args.get(0).unwrap_or(&String::new()).to_string();

                    let close = args
                        .get(1)
                        .unwrap_or(
                            &match open.as_ref() {
                                "(" => ")",
                                "[" => "]",
                                "{" => "}",
                                "<" => ">",
                                _ => open.as_ref(),
                            }
                            .to_string(),
                        )
                        .to_string();

                    open.push_str(&branch_name);
                    open.push_str(&close);

                    open
                }
                _ => branch_name,
            },

            _ => String::new(),
        },

        "exec" => match arg_list {
            Some(arg_list) => {
                let cmd = &arg_list[0];
                let args = &arg_list[1..];
                exec_cmd(cmd, args)
            }

            _ => {
                emit_warning!("Expected at least 1 argument for function 'exec'");
                String::new()
            }
        },

        "exec_strip" | "exec_trim" => match arg_list {
            Some(arg_list) => {
                let cmd = &arg_list[0];
                let args = &arg_list[1..];
                exec_cmd(cmd, args).trim_end().to_string()
            }

            _ => {
                emit_warning!("Expected at least 1 argument for function 'exec'");
                String::new()
            }
        },

        _ => {
            emit_warning!("Function not found '{}'", function);

            String::new()
        }
    }
}

pub fn replace_output(input: &str, output_list: &[(String, String)]) -> String {
    let mut input = input.to_string();
    for (output, sub) in output_list {
        input = input.replacen(sub, output, 1);
    }

    input
}
