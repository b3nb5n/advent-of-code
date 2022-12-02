use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() -> io::Result<()> {
    let file = File::open("./day_01/input_01.txt")?;
    let reader = io::BufReader::new(file);

    let mut max = 0;
    let mut current_elf = 0;

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(s) => s,
            Err(_) => continue,
        };

        current_elf += match line.parse::<i32>() {
            Ok(x) => x,
            Err(_) => {
                if current_elf > max {
                    max = current_elf
                }

                current_elf = 0;
                continue;
            },
        };
    }

    println!("max calories: {}", max);
    Ok(())
}
