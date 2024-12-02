use std::thread;

fn main() {
    let mut items = vec![];

    for i in 1..=3 {
        let item = thread::spawn(move || {
            println!("Thread {}", i);
        });
        items.push(item);
    }

    for item in items {
        item.join().unwrap();
    }

    println!("All threads completed.");
}
