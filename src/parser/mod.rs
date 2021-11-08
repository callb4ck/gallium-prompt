use std::{process::exit, sync::{Arc, Mutex}, thread::{spawn, JoinHandle}};

use crate::{task::execute::execute_task, SETTINGS};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct TemplateParser;

/// Parses the input and executes the tasks
pub fn parse_and_exec(input: String) -> (String, Vec<(String, String)>) {
    let parser = TemplateParser::parse(Rule::line, &input);

    let mut task_result_list: Vec<(String, String)> = vec![];

    (
        input.clone(),
        if SETTINGS.single_thread {
            for mat in parser.unwrap().into_iter() {
                let sub = mat.as_span().as_str().to_string();

                let mut sub_result = String::new();
                for task in mat.into_inner() {
                    let task: Vec<_> = task.into_inner().collect();
                    let function = task.get(0).unwrap().clone().as_str().to_string();

                    let args: Option<Vec<String>> = match task.get(1) {
                        Some(arg_list) => {
                            let arg_list = arg_list.clone().into_inner();
                            let mut args: Vec<_> = vec![];

                            for arg in arg_list {
                                args.push(arg.as_span().as_str().to_string());
                            }
                            Some(args)
                        }

                        _ => None,
                    };

                    sub_result.push_str(&execute_task(&function, args))
                }

                task_result_list.push((sub_result, sub));
            }

            task_result_list
        } else {
            let mut first_threads: Vec<JoinHandle<_>> = vec![];

            let task_result_list_arc = Arc::new(Mutex::new(task_result_list));

            for mat in parser.unwrap_or_else(|_| {
                eprintln!("[FATAL]: Parsing error, perhaps you did (arg1 arg2) instead of (arg1, arg2)?\nIf you really wanted to put a whitespace please use \\s instead");
                exit(1)
            }).into_iter() {
                let mut second_threads: Vec<JoinHandle<_>> = vec![];
                let sub = mat.as_span().as_str().to_string();

                let sub_result = Arc::new(Mutex::new(vec![]));
                for task in mat.into_inner() {
                    let index = task.as_span().start();
                    let task: Vec<_> = task.into_inner().collect();
                    let function = task.get(0).unwrap().clone().as_str().to_string();

                    let args: Option<Vec<String>> = match task.get(1) {
                        Some(arg_list) => {
                            let arg_list = arg_list.clone().into_inner();
                            let mut args: Vec<_> = vec![];

                            for arg in arg_list {
                                args.push(arg.as_span().as_str().to_string().replace("\\s", " "));
                            }
                            Some(args)
                        }

                        _ => None,
                    };

                    let sub_result = sub_result.clone();
                    second_threads.push(spawn(move || {
                        sub_result
                            .lock()
                            .expect("[FATAL]: Sub result mutex panicked (push)")
                            .push((execute_task(&function, args), index));
                    }));
                }

                let task_result_list_arc = task_result_list_arc.clone();
                first_threads.push(spawn(move || {
                    for thread in second_threads {
                        thread.join().expect("[FATAL]: Secondary thread panicked");
                    }

                    let mut sub_result_string = String::new();
                    let mut sub_result = sub_result
                        .lock()
                        .expect("[FATAL]: Sub result mutex panicked (sort)")
                        .to_vec();

                    sub_result.sort_unstable_by_key(|(_, i)| *i);

                    for (task, _) in sub_result {
                        sub_result_string.push_str(&task);
                    }

                    task_result_list_arc
                        .lock()
                        .unwrap()
                        .push((sub_result_string, sub));
                }));
            }

            for thread in first_threads {
                thread.join().expect("[FATAL]: Primary thread panicked");
            }

            let task_results = task_result_list_arc.lock().unwrap().to_vec();

            task_results
        },
    )
}
