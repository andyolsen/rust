use rand::Rng;
use std::io;

pub fn hi_lo_game(upper_limit: u32) {

    let magic_number = rand::thread_rng().gen_range(1..=upper_limit);
    let mut guesses: Vec<u32> = Vec::new();

    println!("Guess a number between 1 and {}:", upper_limit);

    loop {
        print!("> ");

        io::Write::flush(&mut io::stdout()).ok(); 
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();

        match input.trim().parse::<u32>() {

            Ok(guess) => {
                guesses.push(guess);

                if guess > magic_number {
                    println!("Lower");
                } else if guess < magic_number {
                    println!("Higher");
                } else {
                    println!("Correct");
                    break;
                }
            },

            Err(_) => println!("Please enter an integer.")
        }
    }

    let num_guesses = guesses.len();
    if num_guesses == 1 {
        println!("You guessed the right number first time!");
    } else {
        let guess_list = guesses
            .iter()
            .map(|g| g.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("You took {} guesses: {}", num_guesses, guess_list);
    }
}