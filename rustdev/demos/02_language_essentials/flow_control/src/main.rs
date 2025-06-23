fn main() {
    demo_if();
    demo_match();
    demo_loops();
    demo_loops_break_continue();
}

fn demo_if() {

    let age = 59;
    if age > 50 {
        println!("You are old");
    }

    let height = 1.67;
    if height > 1.8 {
        println!("You are tall");
    }
    else {
        println!("You are not so tall");
    }

    let swans_games = 500;
    if swans_games > 300 {
        println!("You are a very loyal fan, we appreciate it dude");
    }
    else if swans_games > 100 {
        println!("You are a discerning fan");
    }
    else {
        println!("You are quite a new fan, welcome buddy");
    }

    let message = if age > 50 {"Hi oldie"} else {"Hi newbie"};
    println!("{}", message); 
}

fn demo_match() {

    let num = 100;
    
    println!("\nUsing a match to test an expression against patterns");
    match num {
        100 => println!("A hundred"),
        200 => println!("Two hundred"),
        _   => println!("Who cares")
    }

    match num {
        25..=50  => println!("25 to 50"),
        51..=100 => println!("51 to 100"),
        _        => println!("Who cares")
    }

    match num {
        25 | 50 | 75  => println!("25, 50, or 75"),
        100 | 200     => println!("100 or 200"),
        _             => println!("Who cares")
    }

    match num {
        x if x < 50  => println!("Less than 50"),
        x if x == 75 => println!("75"),
        _             => println!("Who cares (could be 100 maybe)")
    }
}

fn demo_loops() {

    // println!("\nUsing an infinite loop");
    // loop {
    //     println!("This loop will go on forever. Hit Ctrl-C to stop me!");
    //     std::thread::sleep(std::time::Duration::from_secs(1));
    // }

    println!("\nUsing a while loop");
    let mut i = 0;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }

    println!("\nUsing a for loop over a range (inclusive lower bound, exclusive upper bound)");
    for i in 0..10 {
        println!("{}", i);
    }

    println!("\nUsing a for loop over a range (inclusive lower bound, inclusive upper bound)");
    for i in 0..=10 {
        println!("{}", i);
    }

    println!("\nUsing a for loop over an array");
    let arr = [99, 55, 95, 100, 82];
    for elem in arr {
        println!("{}", elem);
    }
}

fn demo_loops_break_continue() {

    println!("\nDemo using break and continue");
    
    let arr = [99, 45, 85, 100, 82];

    println!("Demo break");
    for elem in arr {
        if elem == 100 {
            println!("Found 100, so break out of loop completely");
            break;
        } 
        println!("{}", elem);
    }

    println!("Demo continue");
    for elem in arr {
        if elem < 50 {
            println!("Found value less than 50, skip it and continue to next iteration");
            continue;
        }
        println!("{}", elem);
    }

    'outer: loop {
        println!("Entered the outer loop");
        loop {
            println!("Entered the inner loop");
            break 'outer;  // Break the outer loop.
        }
        // println!("This point will never be reached in this example");
    }
    println!("Exited the outer loop");

    println!("The End!");
}