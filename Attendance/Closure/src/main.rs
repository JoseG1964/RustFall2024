use std::{thread, time::Duration};

struct ComputeCache<T>
where
    T: Fn() -> String,
{
    // Add fields here
    computation: T,
    result: Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        // Your implementation here
        Self {
            computation,
            result: None,
        }
    }

    fn get_result(&mut self) -> String {
        // Your implementation here
        if let Some(ref cached_result) = self.result{
            println!("Retrieved from cache instantly!");
            return cached_result.clone();
        }
        let result = (self.computation)();
        self.result = Some(result.clone());
        result
    }
}


fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        // Your implementation here
        tracker += 2;
        println!("Tracker: {}", tracker)
    };

    update();
    update();
}

fn process_vector_for_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for val in vec {
        result.push(f(val));
    }
    result
}

fn main() {
    // Write a closure named operation that multiplies two integers and returns the result. Test it with 10 * 5 and print the result
    let operation = |a: i32, b: i32| a * b;

    println!("Result: {}", operation(10, 5));

    // Write a closure named update inside a function track_changes. The closure should increment and print a counter each time it is called.

    track_changes();

    // Write a function process_vector that applies a closure to transform each element of a vector. Implement it in both ways:

    let numbers = vec![1, 2, 3];

    let doubled = process_vector_for_loop(numbers.clone(), |x| x * 2);

    let replaced = process_vector_for_loop(numbers, |x| if x > 2 { 0 } else { x });

    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);

    //Write a struct ComputeCache that accepts a closure during initialization. 
    //Cache the result after the first computation. Use thread::sleep to simulate an expensive computation.

    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take 2 seconds)...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());
    
    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}
