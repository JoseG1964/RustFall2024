use std::io::{self, Read, Write};
use std::fs::File;

struct Car {
   model: String,
   year: u32,
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