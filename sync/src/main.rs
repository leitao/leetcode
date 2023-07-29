use std::thread;
use std::time::Duration;

fn main() {

    for i in 1..10 {
        let x = i.clone();
        thread::spawn(move || {
            println!("hi number {:?} from the spawned thread!", Duration::from_secs(10));
            thread::sleep(Duration::from_secs(10));
        });
    }

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_secs(10));
    }
}

