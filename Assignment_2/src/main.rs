fn is_even(n: i32) -> bool{
    n % 2 == 0
}
fn main() {
    let numbers: [i32; 10] =[15, 45, 98, 190, 299, 230, 334, 453, 543, 789];
    for &n in numbers.iter(){
        if n % 3 == 0 && n % 5 == 0{
            println!("FizzBuzz");
        } else if n % 3 == 0{
            println!("Fizz");
        } else if n % 5 == 0{
            println!("Buzz");
        } else{
            if is_even(n){
                println!("{} is even", n);
            } else{
                println!("{} is odd", n);
            }
        }
    }

    let mut counter = 0;
    let mut x = 0;

    while counter < 10{

        x += numbers[counter];

        println!("{}", numbers[counter]);
        counter += 1;

        if counter == 10{
            println!("{} sum", x);
            break;
        }
    }

    let mut largest = numbers[0];
    let mut i = 0;
    loop{
        if numbers[i]>largest{
            largest = numbers[i];
        }

        i += 1;

        if i == numbers.len(){
            break;
        }
    }
    println!("Largest number in the array is {}", largest);
}
