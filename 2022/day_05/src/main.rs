use std::{io::{self, BufRead}, fs};

fn main() {
    let file = fs::File::open("./day_05/input_01.txt").expect("error reading file");
    let lines = io::BufReader::new(file).lines().map(|line| line.unwrap()).collect::<Vec<_>>();
    let mut lines_iter = lines.iter();

    let mut stacks = Vec::new();
    for _ in 0..9 {
        stacks.push(Vec::<char>::new());
    }

    loop {
        let line = lines_iter.next().expect("error reading line ");

        if line.is_empty() { break; }
        for (i, container) in line.chars().collect::<Vec<_>>().chunks(4).enumerate() {
            if container[1].is_ascii_uppercase() {
                stacks[i].insert(0, container[1]);
            }
        }
    }

    for line in lines_iter {
        let segments = line.split_whitespace().collect::<Vec<_>>();
        let count = segments[1].parse::<usize>().expect("error parsing counts");
        let src = segments[3].parse::<usize>().expect("error parsing src index") - 1;
        let dst = segments[5].parse::<usize>().expect("error parsing dest index") - 1;

        let sub_stack = stacks[src][stacks[src].len() - count..].to_vec();
        for container in sub_stack.iter() {
            stacks[dst].push(*container);
            stacks[src].pop();
        }
    }

    for stack in stacks {
        print!("{:?}", stack.last());
    }
}
