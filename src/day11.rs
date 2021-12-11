use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Octopus {
    level: u32,
    flashed: bool,
}

const WIDTH: i32 = 10;
const HEIGHT: i32 = 10;

pub fn main() -> io::Result<()> {
    let f = File::open("resources/day11")?;
    let f = BufReader::new(f);
    let lines: Vec<String> = f.lines().map(|l| l.unwrap()).collect();

    {
        let mut octos: HashMap<(i32, i32), Octopus> = lines
            .iter()
            .flat_map(|l| l.chars())
            .enumerate()
            .map(|(idx, c)| {
                (
                    (idx as i32 % WIDTH, idx as i32 / WIDTH),
                    Octopus {
                        level: c.to_digit(10).unwrap(),
                        flashed: false,
                    },
                )
            })
            .collect();

        let mut flashes = 0;
        for _ in 0..100 {
            step(&mut octos, &mut flashes)
        }
        println!("pt1 {}", flashes);
    }

    {
        let mut octos: HashMap<(i32, i32), Octopus> = lines
            .iter()
            .flat_map(|l| l.chars())
            .enumerate()
            .map(|(idx, c)| {
                (
                    (idx as i32 % WIDTH, idx as i32 / WIDTH),
                    Octopus {
                        level: c.to_digit(10).unwrap(),
                        flashed: false,
                    },
                )
            })
            .collect();
        let mut flashes = 0;
        let mut i = 0;
        loop {
            step(&mut octos, &mut flashes);
            i += 1;

            if octos.values().all(|o| o.level == 0) {
                break;
            }
        }
        println!("pt2 {}", i);
    }

    Ok(())
}

fn step(octos: &mut HashMap<(i32, i32), Octopus>, flashes: &mut u32) {
    octos.values_mut().for_each(|o| o.level += 1);
    loop {
        let mut done = true;
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let mut flashed = false;
                if let Some(octo) = octos.get_mut(&(x, y)) {
                    if octo.level > 9 && !octo.flashed {
                        done = false;
                        *flashes += 1;
                        octo.flashed = true;
                        flashed = true;
                    }
                }
                if flashed {
                    for adjacent in [
                        (-1, 0),
                        (1, 0),
                        (0, -1),
                        (0, 1),
                        (-1, -1),
                        (1, 1),
                        (-1, 1),
                        (1, -1),
                    ] {
                        if let Some(adjacent_octo) =
                            octos.get_mut(&(x + adjacent.0, y + adjacent.1))
                        {
                            adjacent_octo.level += 1;
                        }
                    }
                }
            }
        }

        if done {
            octos
                .values_mut()
                .filter(|o| o.level > 9)
                .for_each(|o| o.level = 0);
            octos.values_mut().for_each(|o| o.flashed = false);
            break;
        }
    }
}
