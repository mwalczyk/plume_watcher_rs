use std::path::Path;
use std::process;
use std::process::Command;
use std::env;

/// If the user enters an invalid command, print some instructions.
fn help_dialog() {
    println!("usage:
    spectrum_watcher_rs path/to/program.exe");
}

/// Launches a child process at the specified path from the
/// specified directory.
fn launch_process(path: &str, current: Option<&str>) {
    print!("launching: {} from directory: {:?}", path, current);

    let mut command = Command::new(Path::new(path));

    if let Some(c) = current {
        command.current_dir(Path::new(c));
    }

    if let Ok(mut child) = command.spawn() {
        child.wait().expect("command wasn't running");
        println!("child process {} exited successfully", child.id());
    }
}

static MAX_THREADS: i32 = 10;

struct Config {
    program: String,
    working: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let program = args[1].clone();
        let working = args[2].clone();

        Ok(Config {
            program: program,
            working: working,
        })
    }
}

fn run(config: Config) {
    println!("starting program: {}", config.program);
    println!("working directory: {}", config.working);

    let mut command = Command::new(Path::new(&config.program));
    command.current_dir(Path::new(&config.working));

    loop {
        if let Ok(mut child) = command.spawn() {
            child.wait().expect("command wasn't running");
            println!("child process {} exited successfully", child.id());
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("problem parsing command line arguments: {}", e);
        process::exit(1);
    });

    run(config);
}
