use std::sync::mpsc;  // mpsc means "multiple producer, single consumer".
use std::thread;
use std::time::Duration;

fn main() {
    demo_send_receive_single_message();
    demo_send_receive_multiple_messages();
}

fn demo_send_receive_single_message() {

    println!("Send/receive single message");

    // Create a channel to which we can send messages, and from which we can receive those messages. 
    let (tx, rx) = mpsc::channel();

    // Spawn a thread to send a message to the channel. 
    let thread_handle = thread::spawn(move || {
        tx.send(String::from("Hei hei")).unwrap();
    });

    // On the main thread, receive the message from the channel.
    let received = rx.recv().unwrap();
    println!("Received: {}", received);

    thread_handle.join().unwrap();
}

fn demo_send_receive_multiple_messages() {

    println!("\nSend/receive multiple messages");

    // Create a channel to which we can send messages, and from which we can receive those messages. 
    let (tx, rx) = mpsc::channel();

    // Spawn a thread to send some messages to the channel. 
    let thread_handle = thread::spawn(move || {
        for i in 1..=10 {
            let s = std::format!("Hello this is message {}", i);
            tx.send(s).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // On the main thread, receive the messages from the channel.
    for received in rx {
        println!("Received: {}", received);
    }

    thread_handle.join().unwrap();
}
