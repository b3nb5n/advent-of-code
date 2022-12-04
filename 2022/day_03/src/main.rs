use std::{
    collections::{hash_map::RandomState, HashSet},
    fs,
    io::{self, BufRead},
    ops::BitAnd,
};

fn shared_char(strings: &[String]) -> Option<char> {
    let mut char_sets = strings
        .iter()
        .map(|string| HashSet::<_, RandomState>::from_iter(string.chars()))
        .collect::<Vec<_>>()
        .into_iter();

    let mut shared_chars = char_sets.next().unwrap();
    for set in char_sets {
        shared_chars = shared_chars.bitand(&set);
    }

    match shared_chars.iter().next() {
        Some(ch) => Some(*ch),
        None => None,
    }
}

fn priority(ch: char) -> i32 {
    if ch.is_ascii_lowercase() {
        return (ch as i32) - ('a' as i32) + 1;
    } else if ch.is_ascii_uppercase() {
        return (ch as i32) - ('A' as i32) + 27;
    }

    0
}

fn main() {
    let file = fs::File::open("./day_03/input_01.txt").expect("error reading file");
    let lines = io::BufReader::new(file).lines().map(|line| line.unwrap());

    let total_priority: i32 = lines
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|lines| shared_char(lines).expect("no shared characters"))
        .collect::<Vec<_>>()
        .iter()
        .map(|badge| priority(*badge))
        .sum();

    println!("total priority: {:?}", total_priority)
}
