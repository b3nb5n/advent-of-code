use std::{
    fs,
    io::{self, BufRead},
};

fn duplicate(a: &str, b: &str) -> Option<char> {
    for item in a.chars() {
        for comp_item in b.chars() {
            if item == comp_item {
                return Some(item);
            }
        }
    }

    None
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
    let reader = io::BufReader::new(file);
    let total_priority: i32 = reader
        .lines()
        .map(|read_result| {
            let line = read_result.expect("error reading line");
            let duplicate_item = duplicate(&line[..line.len() / 2], &line[(line.len() / 2)..])
                .expect("No duplicate found in line");

            priority(duplicate_item)
        })
        .collect::<Vec<_>>()
        .iter()
        .sum();

    println!("total priority: {}", total_priority)
}
