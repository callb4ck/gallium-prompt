#[allow(unused_macros)]
pub mod warnig {
    #[macro_export]
    /// Emit a wraning message on stderr
    macro_rules! emit_warning {
        ($msg:literal) => {
            if SETTINGS.emit_warning {
                eprint!(concat!("[WARNING]: ", $msg, "\n\n"));
            }
        };

        // It's more optimized to evaluate $cond after SETTINGS.emit_warning
        (if $cond:expr; $msg:literal) => {
            if SETTINGS.emit_warning && $cond {
                eprint!(concat!("[WARNING]: ", $msg, "\n\n"));
            }
        };

        ($msg:literal, $($args:expr),*) => {
            if SETTINGS.emit_warning {
                eprint!(concat!("[WARNING]: ", $msg, "\n\n"), $($args),*);
            }
        };

        (if $cond:expr; $msg:literal, $($args:expr),*) => {
            if SETTINGS.emit_warning && $cond {
                eprint!(concat!("[WARNING]: ", $msg, "\n\n"), $($args),*);
            }
        };
    }
}
