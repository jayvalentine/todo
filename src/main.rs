use std::{env, process};

// Function to print invalid usage command
// and inform the user of available commands.
fn invalid_usage() -> ! {
    println!("Usage:");
    println!("    todo <command>");
    println!("");
    println!("Available commands:");
    println!("    list");

    process::exit(1);
}

fn do_list<T: Iterator<Item = String>>(project: Vec<&str>, args: T) {
    for s in project {
        println!("{}", s);
    }
}

fn main() {
    let project: Vec<&str> = vec![];

    // Skip first argument - this will be the path to the executable.
    let mut args = env::args().skip(1);

    // We need at least one argument - this is the command.
    let command = match args.next() {
        Some(s) => s,
        None => invalid_usage()
    };

    println!("{}", command);

    // Match the command and perform the right function.
    do_list(project, args);
}
