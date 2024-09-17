fn fahrenheit_to_celsius(f: f64) -> f64 {
    f - 32.0 * 5.0 / 9.0
    
}
fn celsius_to_fahrenheit(c: f64) -> f64{
    c * 9.0 / 5.0 + 32.0
}

fn main() {
    let mut fahrenheit = 90.0;

    println!("Fahrenheit to Celsius is {}°C", fahrenheit_to_celsius(fahrenheit));
    
    let mut counter = 0;

    loop{
        println!("Fahrenheit to Celsius is {}°C", fahrenheit_to_celsius(fahrenheit));
        counter += 1;
        fahrenheit += 1.0;
    
        if counter == 5{
            break;
        }
    }
    
    println!("Celsius to fahrenheit is {}°F", celsius_to_fahrenheit(90.0));
}