use chrono::{Utc, NaiveDate};

pub fn parse_command_line() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        Some(args[1].clone())
    } 
    else {
        None
    }
}

pub fn prompt_for_string(message: &String) -> String {
    println!("{}", message);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn prompt_for_date(message: &String) -> NaiveDate {
    let rd = prompt_for_string(message);
    let rd = rd.trim();
    if rd.len() == 0 {
        Utc::now().naive_utc().date()
    } 
    else {
        date_from_str(&rd)
    }
}

pub fn date_from_str(s: &str) -> NaiveDate {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
}