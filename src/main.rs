use std::fs;

mod day_1;
mod day_2;

fn main() {
    day_1::solve();
    day_2::solve();
}

pub fn read_input_file(day: i16) -> String {
    fs::read_to_string(format!("src/day_{}/input.txt", day)).unwrap()
}
