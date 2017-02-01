use std::path::Path;
use std::process::Command;

fn main() {
    let path = Path::new("");
    let working_directory = Path::new("");
    let child = Command::new(path)
        .current_dir(working_directory)
        .spawn()
        .expect("failed to start process");

    loop {

    }
}
