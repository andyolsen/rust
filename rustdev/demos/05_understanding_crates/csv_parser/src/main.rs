use csv_parser::{ui, fh, util};

fn main() {
    
    let filename = util::parse_command_line().unwrap_or(String::from("data.txt"));
    let mut visits = fh::read_visits_from_file(&filename);
    
    if ui::do_menu(&mut visits) {
        fh::write_visits_to_file(&filename, &visits);
    }
}