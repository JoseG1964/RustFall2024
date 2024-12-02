use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut items = vec![];

    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let item = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
        });
        items.push(item);
    }

    for item in items {
        item.join().unwrap();
    }

    let result = *counter.lock().unwrap();
    println!("Final counter value: {}", result);
}
