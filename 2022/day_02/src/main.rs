use std::{
    fs,
    io::{self, BufRead},
};

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper,
    Scissors,
}

impl Move {
    fn from_char(ch: char) -> Result<Self, String> {
        match ch {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(format!("unknow character: {}", ch).to_owned()),
        }
    }
}

enum Outcome {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

impl Outcome {
    fn from_moves(mv: Move, op_mv: Move) -> Self {
        // this implementation makes me feel very nooby
        // rust hurts me so much and its only day 2
        match mv {
            Move::Rock => match op_mv {
                Move::Rock => Outcome::Draw,
                Move::Paper => Outcome::Lose,
                Move::Scissors => Outcome::Win,
            },
            Move::Paper => match op_mv {
                Move::Rock => Outcome::Win,
                Move::Paper => Outcome::Draw,
                Move::Scissors => Outcome::Lose,
            },
            Move::Scissors => match op_mv {
                Move::Rock => Outcome::Lose,
                Move::Paper => Outcome::Win,
                Move::Scissors => Outcome::Draw,
            },
        }
    }
}

fn score(mv: Move, op_mv: Move) -> i32 {
    (mv as i32) + (Outcome::from_moves(mv, op_mv) as i32)
}

fn main() {
    let file = fs::File::open("./day_02/input_01.txt").expect("error reading file");
    let reader = io::BufReader::new(file);

    let score: i32 = reader
        .lines()
        .map(|read_result| {
            let line = read_result.expect("error reading line");
            let bytes = line.as_bytes();
            let mv = Move::from_char(bytes[2] as char).expect("error parsing move");
            let op_mv = Move::from_char(bytes[0] as char).expect("error parsing opponent move");

            score(mv, op_mv)
        })
        .collect::<Vec<_>>()
        .iter()
        .sum();

    println!("score: {}", score)
}
