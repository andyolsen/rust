use std::thread;
use std::time::Duration;

fn main() {
    demo_spawning_thread();
    demo_joining_thread();
    demo_implicitly_moving_data_into_closure();
    demo_explicitly_moving_data_into_closure();
}


fn demo_spawning_thread() {

    println!("Demo of spawning a thread");
 
    thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread {:?} displaying {}", thread::current().id(), i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 100..=105 {
        println!("Thread {:?} displaying {}", thread::current().id(), i);
        thread::sleep(Duration::from_secs(5));
    }
}


fn demo_joining_thread() {

    println!("\nDemo of joining a thread");
    
    let thread_handle = thread::spawn(|| {
         for i in 1..=5 {
             println!("Thread {:?} displaying {}", thread::current().id(), i);
             thread::sleep(Duration::from_secs(5));
         }
    });
 
    for i in 200..=205 {
        println!("Thread {:?} displaying {}", thread::current().id(), i);
        thread::sleep(Duration::from_secs(1));
    }

    thread_handle.join().unwrap();
}


fn demo_implicitly_moving_data_into_closure() {

    println!("\nDemo implicitly moving data into a closure");

    let v = vec![100, 101, 102];

    // The compiler infers how to capture v, depending on it's used in the closure. 
    // In this example, it decides a "move" is necessary. 
    let thread_handle = thread::spawn(|| {
        for item in v {
            println!("{}", item)
        }
    });

    // println!("You can't use v here, because it was moved into the closure. Try it and see! {:?}", v);

    thread_handle.join().unwrap();
}


fn demo_explicitly_moving_data_into_closure() {

    println!("\nDemo explicitly moving data into a closure");

    let v = vec![100, 101, 102];

    // The compiler infers how to capture v, depending on it's used in the closure. 
    // In this example, it would guess a "borrow" is sufficient, but that would give a compiler error... the closure might outlive the external code.
    // So we MUST use the move keyword, to explicitly tell Rust to move external data into the closure.
    let thread_handle = thread::spawn(move || {
        println!("{:?}", v)
    });

    // println!("You can't use v here, because it was moved into the closure. Try it and see! {:?}", v);

    thread_handle.join().unwrap();
}