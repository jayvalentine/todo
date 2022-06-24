use std::{env, process, fs::File};
use std::io::{prelude::*, BufReader, LineWriter};
use core::str::FromStr;

enum TaskState {
    Open,
    Complete
}

// A task with a state and a unique identifier.
struct Task {
    // The state of the task - open, closed, incomplete, etc.
    state: TaskState,

    // A description of the task.
    description: String
}

enum TaskParseError {
    InvalidState,
    NoState
}

impl FromStr for Task {
    type Err = TaskParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split on whitespace.
        let mut split = s.split_whitespace();
        let state = match split.next() {
            Some(i) => i,
            None => return Err(TaskParseError::NoState)
        };

        let state = match state {
            "?" => TaskState::Open,
            "/" => TaskState::Complete,
            _ => return Err(TaskParseError::InvalidState)
        };

        let description: Vec<&str> = split.collect();
        let description: String = description.join(" ");

        return Ok(Task { state, description });
    }
}

struct Project(Vec<Task>);

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

fn do_list<T: Iterator<Item = String>>(project: &mut Project, args: T) {
    for s in &project.0 {
        println!("{}", s.description);
    }
}

fn do_new<T: Iterator<Item = String>>(project: &mut Project, args: T) {
    let description: Vec<String> = args.collect();
    let description = description.join(" ");

    let state = TaskState::Open;

    project.0.push(Task { state, description });
}

fn open_project() -> Project {
    // Open the project file.
    // Create the project file if it doesn't already exist.
    let project_file = match File::open(".todo") {
        Ok(f) => f,
        Err(_) => {
            {
                println!("Creating new project file");
                File::create(".todo").expect("Could not create new project file!");
            }
            File::open(".todo").expect("Could not open newly-created project file!")
        }
    };

    let reader = BufReader::new(project_file);
    let mut project = vec![];

    for line in reader.lines() {
        let line = line.expect("Invalid line in project!");

        if let Ok(v) = Task::from_str(line.trim()) {
            project.push(v);
        }
    }

    return Project(project);
}

fn main() {
    let mut project: Project = open_project();

    // Skip first argument - this will be the path to the executable.
    let mut args = env::args().skip(1);

    // We need at least one argument - this is the command.
    let command = match args.next() {
        Some(s) => s,
        None => invalid_usage()
    };

    // Match the command and perform the right function.
    match command.as_str() {
        "list" => do_list(&mut project, args),
        "new" => do_new(&mut project, args),
        _ => invalid_usage()
    };

    // Write back the project to the file.
    let f = File::create(".todo").expect("Could not open project file!");
    let mut w = LineWriter::new(f);

    for t in project.0 {
        let state = match t.state {
            TaskState::Complete => "/",
            TaskState::Open => "?"
        };

        w.write(state.as_bytes()).expect("Could not write task to project file!");
        w.write(" ".as_bytes()).expect("Could not write task to project file!");
        w.write(t.description.as_bytes()).expect("Could not write task to project file!");
        w.write("\n".as_bytes()).expect("Could not write task to project file!");
    }
}
