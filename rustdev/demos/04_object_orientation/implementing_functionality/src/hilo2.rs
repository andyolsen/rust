use rand::Rng;
use std::io::{self};

struct NumberGenerator;

impl NumberGenerator {
    fn generate_random_number(upper_limit: u32) -> u32 {
        rand::thread_rng().gen_range(1..=upper_limit)
    }
}

struct ConsoleInput;

impl ConsoleInput {

    fn get_str(prompt: &str) -> String {

        print!("{}", prompt);

        io::Write::flush(&mut io::stdout()).ok();
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();

        input.trim().to_string()
    }

    fn get_yes_no(prompt: &str) -> bool {
        let input = Self::get_str(prompt);
        input.to_lowercase().starts_with('y')
    }

    fn get_int(prompt: &str) -> u32 {
        loop {
            let input = Self::get_str(prompt);
            match input.parse::<u32>() {
                Ok(n) => return n,
                Err(_) => println!("Please enter an integer."),
            }
        }
    }
}

pub struct HiLoGame {
    name: String,
    upper_limit: u32,
    guesses: Vec<u32>,
    magic_number: u32,
}

impl HiLoGame {
    
    pub fn new(upper_limit: u32) -> Self {
        let name = ConsoleInput::get_str("Please enter your name: ");
        Self {
            name,
            upper_limit,
            guesses: Vec::new(),
            magic_number: 0, // Will be set in `play`
        }
    }

    pub fn play(&mut self) -> bool {
        self.guesses.clear();
        self.magic_number = NumberGenerator::generate_random_number(self.upper_limit);
        println!("Guess a number between 1 and {}:", self.upper_limit);

        loop {
            let guess = ConsoleInput::get_int("> ");
            self.guesses.push(guess);
            if self.examine_guess(guess) == "Correct" {
                break;
            }
        }

        self.display_result();
        ConsoleInput::get_yes_no("\nPlay again [y/n]? ")
    }

    fn examine_guess(&self, guess: u32) -> String {
        let message = if guess > self.magic_number {
            "Lower"
        } else if guess < self.magic_number {
            "Higher"
        } else {
            "Correct"
        };
        println!("{}", message);
        message.to_string()
    }

    fn display_result(&self) {
        let num_guesses = self.guesses.len();
        if num_guesses == 1 {
            println!("Wow {}, you guessed the right number first time!", self.name);
        } else {
            let guess_str: String = self.guesses.iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            println!("{}: you took {} guesses: {}", self.name, num_guesses, guess_str);
        }
    }
} 