mod tasks;

use std::fs::read_to_string;

use clap::Parser;
use tasks::TASKS;

#[derive(Parser, Debug)]
struct Args {
    /// task
    task: String,

    /// input file
    #[arg(short, long)]
    input_file: Option<String>,

    /// input file type
    #[arg(short, long)]
    small: bool,
}

fn main() {
    let args = Args::parse();
    let selected_task = args.task;
    let input_file = args.input_file;

    let task = TASKS
        .iter()
        .find(|t| t.id == selected_task)
        .expect("invalid task id");

    let default_input_file = format!(
        "inputs/{}{}.txt",
        task.id,
        if args.small { "_small" } else { "" }
    );

    let input_file_path = match input_file {
        Some(fp) => fp,
        None => {
            println!("no input file provided, using {default_input_file}");
            default_input_file
        }
    };
    let input = read_to_string(input_file_path).expect("invalid input file");
    println!("executing task {}", task.id);
    let output = (task.func)(input);

    println!("output:\n{output}");
}
