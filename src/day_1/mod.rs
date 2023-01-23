use std::fs;

fn read_input_file() -> String {
    fs::read_to_string("src/day_1/input.txt").unwrap()
}

fn get_elfs_calories(input: &str) -> Vec<i32> {
    let mut elfs_calories = Vec::new();

    for line in input.lines() {
        if line.len() == 0 {
            elfs_calories.push(0);
        } else {
            let cal = line.parse::<i32>().unwrap();
            elfs_calories.last_mut().map(|x| *x += cal);
        }
    }

    elfs_calories
}

pub fn solve() {
    let input = read_input_file();
    let mut elfs_calories = get_elfs_calories(input.as_str());
    elfs_calories.sort_by(|a, b| b.cmp(a));

    // Part 1
    let highest_calories = elfs_calories.first().unwrap();

    println!("Highest calories: {}", highest_calories);

    // Part 2

    let highest_three_calories = elfs_calories.iter().take(3).sum::<i32>();

    println!("Highest three calories: {}", highest_three_calories);
}
