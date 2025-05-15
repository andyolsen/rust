use chrono::{Duration, NaiveDate, Utc};
use csv_parser::{fh, util, datatypes::Visit};

struct Record {
    date: NaiveDate,
    days: i64
}

const PERIOD_OF_INTEREST: i64 = 180;

fn main() {
    let filename = util::parse_command_line().unwrap_or(String::from("data.txt"));
    let mut visits = fh::read_visits_from_file(&filename);
    
    if do_menu(&mut visits) {
        fh::write_visits_to_file(&filename, &visits);
    }
}

fn do_menu(visits: &mut Vec<Visit>) -> bool {

    let message = String::from(r"
Choose option:
1 Display all visits 
2 Calculate visit days within period of interest 
3 Add visit
4 Save and exit
5 Quit without saving
>>>");

    loop {
        let selection = util::prompt_for_string(&message);
        match selection.trim() {
            "1" => display_all_visits(visits),
            "2" => display_days_within_period_of_interest(visits),
            "3" => add_visit(visits),
            "4" => return true,
            "5" => return false,
            _   => println!("???")
        }
    }
}

fn display_all_visits(visits: &Vec<Visit>) {
    println!("All visits:");
    for v in visits {
        println!("{}", v.to_string());
    }
}

fn display_days_within_period_of_interest(visits: &Vec<Visit>) {
    let message = String::from("What reference date do you want to use? [yyyy-mm-dd, or ENTER for today]:");
    let mut date = util::prompt_for_date(&message);

    let mut records: Vec<Record> = Vec::new();
    while date >= Utc::now().naive_utc().date() {
        let days = get_days_prior_to(visits, date);
        records.push(Record{date, days});
        date = date - Duration::days(1)
    }

    println!("Reverse chronological order:");
    for r in records.iter() {
        println!("Days {}, date {}", r.days, r.date);
    }

    println!("\nReverse number of days order:");
    records.sort_by(|a, b| b.days.cmp(&a.days));
    for r in records.iter() {
        println!("Days {}, date {}", r.days, r.date)
    }
}

fn get_days_prior_to(visits: &Vec<Visit>, window_end_date: NaiveDate) -> i64{
    let window_start_date: NaiveDate = window_end_date - Duration::days(PERIOD_OF_INTEREST);

    let total_days: i64 = visits.iter()
                                .map(|e| e.get_days_in_period(window_start_date, window_end_date))
                                .sum();
    
    total_days
}

fn add_visit(visits: &mut Vec<Visit>) {
    let message = String::from("Enter start-date,end-date,description [yyyy-mm-dd format]:");
    let visit = util::prompt_for_string(&message).trim().to_string();
    let visit = Visit::from_string(visit);   
    
    visits.push(visit);
}