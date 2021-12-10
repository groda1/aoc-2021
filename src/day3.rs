use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn main() -> io::Result<()> {
    let f = File::open("resources/day3")?;
    let f = BufReader::new(f);

    let lines: Vec<String> = f.lines().map(|l| l.unwrap()).collect();
    let fo = lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_digit(2).unwrap()).collect())
        .reduce(|mut acc: Vec<u32>, a| {
            for (i, val) in acc.iter_mut().enumerate() {
                *val += a[i];
            }
            acc
        })
        .unwrap();

    let pt1_gamma = fo
        .iter()
        .map(|d| {
            if *d as usize >= (lines.len() / 2) {
                1
            } else {
                0
            }
        })
        .fold(0, |acc, b| (acc << 1) + b as u32);

    println!("derp {:?}", pt1_gamma * (!pt1_gamma & 0xfff));

    Ok(())
}
