use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use crate::Segment::{Bot, BotLeft, BotRight, Mid, Top, TopLeft, TopRight};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Segment {
    Top,
    TopLeft,
    TopRight,
    Mid,
    BotLeft,
    BotRight,
    Bot,
}

pub fn main() -> io::Result<()> {
    let f = File::open("resources/day8")?;
    let f = BufReader::new(f);

    let mut sum = 0;

    for l in f.lines() {
        let mut map = HashMap::new();
        let line = l.unwrap();

        let mut one: Option<HashSet<char>> = None;
        let mut four: Option<HashSet<char>> = None;

        let (input, digits) = line.split_once(" | ").unwrap();

        let mut segments = HashMap::new();
        let codes: Vec<String> = input.split(' ').map(|s| s.to_string()).collect();

        for w in codes.iter() {
            if w.len() == 2 {
                one = Some(w.chars().collect());
                map.insert(w.chars().collect::<String>(), 1);
            } else if w.len() == 4 {
                four = Some(w.chars().collect());
                map.insert(w.chars().sorted().collect::<String>(), 4);
            }
        }

        for c in ['a', 'b', 'c', 'd', 'e', 'f', 'g'] {
            let sum = codes
                .iter()
                .filter(|n| n.chars().any(|character| c == character))
                .count();

            match sum {
                4 => {
                    segments.insert(c, BotLeft);
                }
                9 => {
                    segments.insert(c, BotRight);
                }
                6 => {
                    segments.insert(c, TopLeft);
                }
                8 => {
                    if one.as_ref().unwrap().contains(&c) {
                        segments.insert(c, TopRight);
                    } else {
                        segments.insert(c, Top);
                    }
                }
                7 => {
                    if four.as_ref().unwrap().contains(&c) {
                        segments.insert(c, Mid);
                    } else {
                        segments.insert(c, Bot);
                    }
                }
                _ => unreachable!(),
            }
        }

        sum += map_number(digits, &segments);
    }

    println!("foo {}", sum);

    Ok(())
}

fn map_number(digits: &str, segments: &HashMap<char, Segment>) -> u32 {
    let mut number_str = String::new();
    for digit in digits.split(' ') {
        let mut active_segments = HashSet::new();
        for c in digit.chars() {
            active_segments.insert(*segments.get(&c).unwrap());
        }

        if active_segments.len() == 6 && !active_segments.contains(&Mid) {
            number_str.push('0');
        } else if active_segments.len() == 2 {
            number_str.push('1');
        } else if active_segments.len() == 5
            && !active_segments.contains(&TopLeft)
            && !active_segments.contains(&BotRight)
        {
            number_str.push('2');
        } else if active_segments.len() == 5
            && !active_segments.contains(&TopLeft)
            && !active_segments.contains(&BotLeft)
        {
            number_str.push('3');
        } else if active_segments.len() == 4 {
            number_str.push('4');
        } else if active_segments.len() == 5
            && !active_segments.contains(&TopRight)
            && !active_segments.contains(&BotLeft)
        {
            number_str.push('5');
        } else if active_segments.len() == 6 && !active_segments.contains(&TopRight) {
            number_str.push('6');
        } else if active_segments.len() == 3 {
            number_str.push('7');
        } else if active_segments.len() == 7 {
            number_str.push('8');
        } else if active_segments.len() == 6 && !active_segments.contains(&BotLeft) {
            number_str.push('9');
        }
    }
    number_str.parse().unwrap()
}
