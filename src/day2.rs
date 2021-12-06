use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn main() -> io::Result<()> {
    let f = File::open("resources/day2")?;
    let f = BufReader::new(f);

    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;

    f.lines().for_each(|l| {
        let derp = l.unwrap();
        let mut split = derp.split(' ');
        let dir = split.next();
        let val: u32 = split.next().unwrap().parse().unwrap();

        match dir {
            None => {
                unreachable!()
            }
            Some("forward") => {
                forward += val;
                depth += aim * val;
            }
            Some("down") => {
                aim += val;
            }
            Some("up") => {
                aim -= val;
            }
            _ => {
                unreachable!()
            }
        }
    });

    println!("foo: {}", forward * depth);

    Ok(())
}
