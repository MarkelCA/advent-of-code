use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    match solve() {
        Ok(result) => println!("The result is: {}", result),
        Err(error) => println!("Error: {}", error)
    }
}

fn solve() -> std::io::Result<u32> {
    let file = File::open("../input.txt")?;
    let reader = BufReader::new(file);

    let mut current_cals = 0;
    let mut bigger_cals  = 0;

    for line in reader.lines() {
        let line = line?;

        if line == "" {
            if current_cals > bigger_cals {
                bigger_cals = current_cals;
            }
            current_cals = 0;
        } else {
            current_cals += line.parse::<u32>().unwrap();
        }
    }

    Ok(bigger_cals)
}
