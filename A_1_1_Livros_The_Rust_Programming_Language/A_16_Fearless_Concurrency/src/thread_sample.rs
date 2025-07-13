use std::thread;
use std::time::Duration;

pub fn sample() {
    thread_spawn_sample();
    thread_spawn_with_move_sample();
}

fn thread_spawn_sample() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn thread_spawn_with_move_sample() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    
    // drop(v);

    handle.join().unwrap();
}
