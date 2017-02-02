use std::path::Path;
use std::process::Command;
use std::env;

/// If the user enters an invalid command, print some instructions.
fn help_dialog() {
    println!("usage:
    spectrum_watcher_rs executable_path");
}

/// Launches a child process at the specified path from the
/// specified directory.
fn launch_process(path: &str, current: Option<&str>) {
    print!("launching: {} from directory: {:?}", path, current);

    // launch the child process
    let mut command = Command::new(Path::new(path));

    if let Some(c) = current {
        command.current_dir(Path::new(c));
    }

    if let Ok(mut child) = command.spawn() {
        child.wait().expect("command wasn't running");
        println!("child process {} exited successfully", child.id());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path;
    let current;

    // Remember: the first argument is always the executing process
    match args.len() {
        1 => {
            println!("spectrum_watcher_rs requires at least one command line argument");
        }
        2 => {
            path = &args[1];
            loop {
                launch_process(&path[..], None);
            }
        }
        3 => {
            path = &args[1];
            current = &args[2];
            loop {
                launch_process(&path[..], Some(&current[..]));
            }
        }
        _ => {
            println!("error: invalid command");
            help_dialog();
        }
    }


}
