use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn main() -> io::Result<()> {
    let f = File::open("resources/day5")?;
    let f = BufReader::new(f);

    const WIDTH: usize = 1000;
    const HEIGHT: usize = 1000;

    let mut array = [0; WIDTH * HEIGHT];

    f.lines().for_each(|l| {
        let line = l.unwrap();
        let (p1_str, p2_str) = line.split_once(" -> ").unwrap();
        let (x0_str, y0_str) = p1_str.split_once(',').unwrap();
        let (x1_str, y1_str) = p2_str.split_once(',').unwrap();
        let x0: usize = x0_str.parse().unwrap();
        let y0: usize = y0_str.parse().unwrap();
        let y1: usize = y1_str.parse().unwrap();
        let x1: usize = x1_str.parse().unwrap();

        if y0 == y1 {
            // horizontal
            for i in x0.min(x1)..=x0.max(x1) {
                array[y0 * WIDTH + i] += 1;
            }
        } else if x0 == x1 {
            // vertical
            for i in y0.min(y1)..=y0.max(y1) {
                array[i * WIDTH + x0] += 1;
            }
        } else {
            // diagonal
            let (mut x, up) = if y0 > y1 {
                (x1, x0 > x1)
            } else {
                (x0, x1 > x0)
            };
            for i in y0.min(y1)..=y0.max(y1) {
                array[i * WIDTH + x] += 1;
                if up {
                    x += 1;
                } else {
                    x = x.wrapping_sub(1);
                }
            }
        }
    });

    let count = array.iter().filter(|p| **p >= 2).count();
    println!("count {}", count);

    Ok(())
}
