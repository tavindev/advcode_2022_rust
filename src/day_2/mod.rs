use crate::read_input_file;

#[derive(Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn get_points(&self) -> i16 {
        match *self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn get_losing_move(&self) -> Move {
        match *self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn get_winning_move(&self) -> Move {
        match *self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }
}

fn get_score(me: Move, oponent: Move) -> i16 {
    let match_points = match (&me, oponent) {
        (Move::Rock, Move::Scissors) => 6,
        (Move::Paper, Move::Rock) => 6,
        (Move::Scissors, Move::Paper) => 6,
        (Move::Rock, Move::Paper) => 0,
        (Move::Paper, Move::Scissors) => 0,
        (Move::Scissors, Move::Rock) => 0,
        _ => 3,
    };

    match_points + me.get_points()
}

fn get_move(input: &str) -> Move {
    match input {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissors,
        _ => panic!("Invalid move"),
    }
}

pub fn solve() {
    let input = read_input_file(2);
    let mut score_part = 0;
    let mut score_part_two = 0;

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let oponent = parts.next().unwrap();
        let me = parts.next().unwrap();
        let oponent_move = get_move(oponent);
        let my_move = get_move(me);

        score_part += get_score(my_move, oponent_move);

        let openent_move_copy = oponent_move.clone();
        // Part 2
        let my_move = match me {
            "X" => openent_move_copy.get_winning_move(),
            "Y" => openent_move_copy,
            "Z" => openent_move_copy.get_losing_move(),
            _ => panic!("Invalid move"),
        };

        score_part_two += get_score(my_move, get_move(oponent));
    }

    println!("Score part 1: {}", score_part);
    println!("Score part 2: {}", score_part_two);
}
