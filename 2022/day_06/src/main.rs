use std::{fs, collections::{HashSet, hash_map::RandomState}};

fn main() {
    let signal_string = fs::read_to_string("./day_06/input_01.txt").expect("error reading input file");
    let signal = signal_string.as_str();

    let window_size = 14;
    for i in 0..(signal.len() - window_size - 1) {
        let char_set = HashSet::<_, RandomState>::from_iter(signal[i..i+window_size].chars());
        if char_set.len() == window_size {
            println!("{}", i + window_size);
            break;
        }
    }
}
