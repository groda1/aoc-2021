use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Octopus {
    level: u32,
    flashed: bool,
}

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

pub fn main() -> io::Result<()> {
    let f = File::open("resources/day11")?;
    let f = BufReader::new(f);
    let lines: Vec<String> = f.lines().map(|l| l.unwrap()).collect();

    {
        let mut octos: Vec<Octopus> = lines
            .iter()
            .flat_map(|l| l.chars())
            .map(|c| Octopus {
                level: c.to_digit(10).unwrap(),
                flashed: false,
            })
            .collect();
        let mut flashes = 0;
        for _ in 0..100 {
            step(&mut octos, &mut flashes)
        }
        println!("pt1 {}", flashes);
    }

    {
        let mut octos: Vec<Octopus> = lines
            .iter()
            .flat_map(|l| l.chars())
            .map(|c| Octopus {
                level: c.to_digit(10).unwrap(),
                flashed: false,
            })
            .collect();
        let mut flashes = 0;
        let mut i = 0;
        loop {
            step(&mut octos, &mut flashes);
            i += 1;

            if octos.iter().all(|o| o.level == 0) {
                break;
            }
        }
        println!("pt2 {}", i);
    }

    Ok(())
}

fn step(octos: &mut Vec<Octopus>, flashes: &mut i32) {
    octos.iter_mut().for_each(|o| o.level += 1);
    loop {
        let mut done = true;
        for i in 0..WIDTH * HEIGHT {
            if octos[i].level > 9 && !octos[i].flashed {
                done = false;
                let left = i % WIDTH > 0;
                let right = i % WIDTH < (WIDTH - 1);
                let up = i / WIDTH > 0;
                let down = i / WIDTH < (HEIGHT - 1);

                if left {
                    octos[i - 1].level += 1;
                }
                if right {
                    octos[i + 1].level += 1;
                }
                if up {
                    octos[i - WIDTH].level += 1;
                }
                if down {
                    octos[i + WIDTH].level += 1;
                }
                if left && up {
                    octos[i - 1 - WIDTH].level += 1;
                }
                if left && down {
                    octos[i - 1 + WIDTH].level += 1;
                }
                if right && up {
                    octos[i + 1 - WIDTH].level += 1;
                }
                if right && down {
                    octos[i + 1 + WIDTH].level += 1;
                }
                *flashes += 1;
                octos[i].flashed = true;
            }
        }
        if done {
            octos
                .iter_mut()
                .filter(|o| o.level > 9)
                .for_each(|o| o.level = 0);
            octos.iter_mut().for_each(|o| o.flashed = false);
            break;
        }
    }
}
