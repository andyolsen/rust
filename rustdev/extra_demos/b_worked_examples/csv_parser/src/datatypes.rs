use chrono::NaiveDate;
use crate::util;

pub struct Visit {
    start_date: NaiveDate,
    end_date: NaiveDate,
    description: String 
}

impl Visit {

    pub fn from_string(s: String) -> Visit {
        let tokens: Vec<&str> = s.split(',').collect();
        Visit {
            start_date:  util::date_from_str(tokens[0]), 
            end_date:    util::date_from_str(tokens[1]),
            description: tokens[2].to_string()
        }
    }

    pub fn to_string(&self) -> String {
        std::format!("{},{},{}\n", self.start_date, self.end_date, self.description)
    }

    pub fn get_days_in_period(&self, window_start_date: NaiveDate, window_end_date: NaiveDate) -> i64 {
        if self.start_date > window_end_date || self.end_date < window_start_date {
            0
        }
        else {
            let start_effective = std::cmp::max(self.start_date, window_start_date);
            let end_effective   = std::cmp::min(self.end_date, window_end_date);
            end_effective.signed_duration_since(start_effective).num_days()
        }
    }
}