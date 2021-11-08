pub mod execute;
mod get;

use self::execute::replace_output;

use super::parser::parse_and_exec;

pub fn start_tasks() {
    let (input, outputs) = parse_and_exec(
        get::from_arg()
            .replace(r"\n", "\n")
            .replace(r"\r", "\r")
            .replace(r"\t", "\t"),
    );

    let final_result = replace_output(&input, &outputs);

    print!("{}", final_result)
}
