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
    fn from_char(ch: &char) -> Result<Self, ()> {
        match ch {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

enum Outcome {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

impl Outcome {
    fn from_moves(mv: &Move, op_mv: &Move) -> Self {
        if *mv == *op_mv {
            return Self::Draw;
        }
        if *mv > *op_mv && !(*mv == Move::Rock && *op_mv == Move::Scissors) {
            return Self::Win;
        }

        return Self::Lose;
    }
}

fn score_round(mv: &Move, op_mv: &Move) -> i32 {
    (*mv as i32) + (Outcome::from_moves(mv, op_mv) as i32)
}

fn main() {
    let file = fs::File::open("./day_01/input_01.txt").expect("error reading file");
    let reader = io::BufReader::new(file);

    let x = reader.lines().map(|line| { 
        let mut chars = line.expect("error reading line").to_owned().chars();
        score_round(
            &Move::from_char(&chars.nth(0).expect("missing move")).expect("error parsing move"),
            &Move::from_char(&chars.nth(0).expect("missing move")).expect("error parsing move"),
        )

    }).collect::<Vec<_>>();

    let mut score = 0;
    for line in reader.lines() {
        let chars = match line {
            Ok(s) => s.to_owned().chars(),
            Err(_) => panic!("Error reading line"),
        };

        let mv_ch = chars.nth(0).unwrap();
        let mv = match Move::from_char(&mv_ch)  {
            Ok(mv) => mv,
            Err(_) => panic!("error parsing move: {}", mv_ch)
        };

        let op_mv_ch = chars.nth(2).unwrap();
        let op_mv = match Move::from_char(&op_mv_ch)  {
            Ok(mv) => mv,
            Err(_) => panic!("error parsing move: {}", mv_ch)
        };


        score += score_round(&mv, &op_mv);
    }

    println!("score: {}", score);
}
