fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret{
        return 0;
    } else if guess > secret {
        return 1;
    } else {
        return -1;
    }
}
fn main() {
    let mut secret = 35;

    let mut guesses = 0;

    let mut guess = 1;

    loop{

        guesses += 1;

        check_guess(guess, secret);

        if check_guess(guess, secret) == 0{
            println!("Correct");
            break;
        } else if check_guess(guess, secret) == 1{
            println!("Too High");
        } else{
            println!("Too Low");
        }

        guess += 1;

    }

    println!("It took you {} tries", guesses);

}
