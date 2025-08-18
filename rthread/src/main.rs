use std::{thread, vec};
use std::sync::{Mutex,Arc};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Thread incremented count to {}", *num);
        });
        handlers.push(handler);
    } 
    for handler in handlers {
        handler.join().unwrap();
    }
    let result = counter.lock().unwrap();
    println!("Final count: {}", *result);
}
