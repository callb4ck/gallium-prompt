use crate::SETTINGS;

#[inline]
pub fn reset() -> String {
    String::from("\x1b[0m")
}

pub fn text(color_str: &str) -> String {
    macro_rules! match_ansi_code {
            ($($($color:literal),* => $code:literal),*) => {
            match color_str.to_ascii_lowercase().as_ref() {
                $($($color)|* => concat!("\x1b[", $code, "m"),)*
                    _ => {
                        if SETTINGS.emit_warning {
                            eprintln!("[WARNING]: Color not found '{}'", color_str);
                        }
                        ""
                    }
            }
        };
    }

    (match_ansi_code! {
        "black"            => 30,
        "red"              => 31,
        "green"            => 32,
        "yellow"           => 33,
        "blue"             => 34,
        "magenta"          => 35,
        "cyan"             => 36,

        "bold_black",   "bright_black"   => "30;1",
        "bold_red",     "bright_red"     => "31;1",
        "bold_green",   "bright_green"   => "32;1",
        "bold_yellow",  "bright_yellow"  => "33;1",
        "bold_blue",    "bright_blue"    => "34;1",
        "bold_magenta", "bright_magenta" => "35;1",
        "bold_cyan",    "bright_cyan"    => "36;1"
    })
    .to_string()
}

pub fn text_rgb(r: &str, g: &str, b: &str, bold: bool) -> String {
    let mut color = String::with_capacity(20);

    if bold {
        color.push_str("\x1b[1;31m")
    }

    color.push_str("\x1b[38;2;");
    color.push_str(r);
    color.push(';');
    color.push_str(g);
    color.push(';');
    color.push_str(b);
    color.push('m');

    color
}

pub fn background(color_str: &str) -> String {
    macro_rules! match_ansi_code {
            ($($($color:literal),* => $code:literal),*) => {
            match color_str.to_ascii_lowercase().as_ref() {
                $($($color)|* => concat!("\x1b[", $code, "m"),)*
                    _ => {
                        if SETTINGS.emit_warning {
                            eprintln!("[WARNING]: Color not found '{}'", color_str);
                        }
                        ""
                    }
            }
        };
    }

    (match_ansi_code! {
        "black"          => 40,
        "red"            => 41,
        "green"          => 42,
        "yellow"         => 43,
        "blue"           => 44,
        "magenta"        => 45,
        "cyan"           => 46,

        "bold_black",   "bright_black"   => "40;1",
        "bold_red",     "bright_red"     => "41;1",
        "bold_green",   "bright_green"   => "42;1",
        "bold_yellow",  "bright_yellow"  => "43;1",
        "bold_blue",    "bright_blue"    => "44;1",
        "bold_magenta", "bright_magenta" => "45;1",
        "bold_cyan",    "bright_cyan"    => "46;1"
    })
    .to_string()
}

pub fn background_rgb(r: &str, g: &str, b: &str) -> String {
    let mut color = String::with_capacity(20);

    color.push_str("\x1b[48;2;");
    color.push_str(r);
    color.push(';');
    color.push_str(g);
    color.push(';');
    color.push_str(b);
    color.push('m');

    color
}

pub fn effect(effect_str: &str) -> String {
    macro_rules! match_ansi_code {
            ($($($color:literal),* => $code:literal),*) => {
            match effect_str.to_ascii_lowercase().as_ref() {
                $($($color)|* => concat!("\x1b[", $code, "m"),)*
                    _ => {
                        if SETTINGS.emit_warning {
                            eprintln!("[WARNING]: Effect not found '{}'", effect_str);
                        }
                        ""
                    }
            }
        };
    }

    (match_ansi_code! {
        "bold", "bright"     => 1,
        "faint"              => 2,
        "italic"             => 3,
        "underline"          => 4,
        "double_underline"   => 21,
        "slow_blink"         => 5,
        "fast_blink"         => 6,
        "reverse", "inverse" => 7,
        "erase", "invisible" => 8,
        "striketrough"       => 9
    })
    .to_string()
}
