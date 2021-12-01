use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn main() -> io::Result<()> {
    let f = File::open("resources/day1")?;
    let f = BufReader::new(f);

    let mut prev = u32::MAX;
    let mut count: u32 = 0;
    let numbers: Vec<u32> = f
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .collect();

    for i in 0..(numbers.len() - 2) {
        let window = numbers[i] + numbers[i + 1] + numbers[i + 2];
        if window > prev {
            count += 1;
        }
        prev = window;
    }

    println!("increases: {}", count);
    Ok(())
}
