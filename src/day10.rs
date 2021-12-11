use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn main() -> io::Result<()> {
    let f = File::open("resources/day10")?;
    let f = BufReader::new(f);

    let syntax = HashMap::from([('[', ']'), ('(', ')'), ('{', '}'), ('<', '>')]);
    let score = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let score_pt2 = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let lines: Vec<String> = f.lines().map(|l| l.unwrap()).collect();

    let pt1: u32 = lines
        .iter()
        .map(|l| {
            let mut stack = Vec::new();
            for c in l.chars().into_iter() {
                if syntax.contains_key(&c) {
                    stack.push(*syntax.get(&c).unwrap());
                } else if stack.pop().unwrap() == c {
                    continue;
                } else {
                    return *score.get(&c).unwrap();
                }
            }
            0
        })
        .sum();

    println!("pt1 {}", pt1);

    let mut pt2: Vec<u64> = lines
        .iter()
        .map(|l| {
            let mut stack = Vec::new();
            for c in l.chars().into_iter() {
                if syntax.contains_key(&c) {
                    stack.push(*syntax.get(&c).unwrap());
                } else if stack.pop().unwrap() == c {
                    continue;
                } else {
                    return 0;
                }
            }
            stack
                .iter()
                .rev()
                .map(|c| score_pt2.get(c).unwrap())
                .fold(0, |acc, n| acc * 5 + n)
        })
        .filter(|s| *s > 0)
        .collect();
    pt2.sort_unstable();

    println!("pt2 {}", pt2[pt2.len() / 2]);

    Ok(())
}
