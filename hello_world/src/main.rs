use std::fs::OpenOptions;
use std::io::Write;

use std::process::Command;

fn executing_os_commands_linux() {
    let output = Command::new("python3")
        .arg("my_script.py")
        .output()
        .expect("Failed to execute command");

    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
}

fn append_to_file() {
    let mut file = OpenOptions::new()
        .append(true)
        .open("example.txt")
        .unwrap();

    writeln!(file, "This line is appended to the file.").unwrap();
}

fn main() {
executing_os_commands_linux();  
}