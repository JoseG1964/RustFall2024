use std::io::{self, Read, Write};
use std::fs::File;
use std::process::Command;
use std::fs::OpenOptions;

struct Car {
   model: String,
   year: u32,
}

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

fn reading_from_console() {
    let mut buffer = String::new();
    let mut file = File::create("user_input.txt").unwrap();

    print!("What's the model of your car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let model = buffer.trim().to_string();
    writeln!(file, "{}", model).unwrap();
    buffer.clear();

    print!("What year is the car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year = buffer.trim().parse().unwrap();
    writeln!(file, "{}", year);

    let car = Car { model, year };
    println!("You have a {}, that is from {}!", car.model, car.year);

}
fn main(){
   reading_from_console();
}