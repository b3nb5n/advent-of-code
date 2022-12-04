use std::{
    fs,
    io::{self, BufRead},
};

fn main() {
    let file = fs::File::open("./day_04/input_01.txt").expect("error reading file");
    let lines = io::BufReader::new(file).lines().map(|line| line.unwrap());

    // let mut contained_count = 0;
    let mut overlapping_count = 0;

    for line in lines {
        let (range_1, range_2) = line.split_once(',').expect("couldn't split line at ' '");
        let (lower_1_str, upper_1_str) = range_1
            .split_once('-')
            .expect("couldn't split first range at '-'");
        let (lower_2_str, upper_2_str) = range_2
            .split_once('-')
            .expect("couldn't split first range at '-'");

        let lower_1 = lower_1_str
            .parse::<i32>()
            .expect("error parsing lower bound");
        let upper_1 = upper_1_str
            .parse::<i32>()
            .expect("error  parsing upper bound");

        let lower_2 = lower_2_str
            .parse::<i32>()
            .expect("error parsing lower bound");
        let upper_2 = upper_2_str
            .parse::<i32>()
            .expect("error parsing lower bound");

        // if (lower_1 <= lower_2 && upper_1 >= upper_2) || (lower_2 <= lower_1 && upper_2 >= upper_1) {
        //     contained_count += 1;
        // }

        if (lower_1 <= lower_2 && upper_1 >= lower_2) || (lower_2 <= lower_1 && upper_2 >= lower_1)
        {
            overlapping_count += 1;
        }
    }

    println!("overlapping ranges: {}", overlapping_count);
}
