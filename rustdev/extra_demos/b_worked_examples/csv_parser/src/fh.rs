use std::fs::File;
use std::io::{BufRead, BufReader, Write, BufWriter};

use crate::datatypes::Visit;

pub fn read_visits_from_file(filename: &String) -> Vec<Visit> {

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut visits: Vec<Visit> = Vec::new();
    for line in reader.lines() {
        
        let str = line.unwrap();
        
        if str.len() != 0 && !str.starts_with("#") {
            let visit = Visit::from_string(str);
            visits.push(visit);
        }
    }
    visits
}

pub fn write_visits_to_file(filename: &String, visits: &Vec<Visit>) {

    let file = File::create(filename).unwrap();
    let mut writer = BufWriter::new(file);

    for visit in visits {
        writer.write_all(visit.to_string().as_bytes()).unwrap();
    }
}