use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub struct Board {
    winner: bool,
    values: Vec<(u32, bool)>,
}

pub fn main() -> io::Result<()> {
    let f = File::open("resources/day4")?;
    let f = BufReader::new(f);

    let mut lines = f.lines();
    let draw: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut boards = Vec::new();
    loop {
        let next_board = lines.next();
        if next_board.is_none() {
            break;
        }

        let mut board: Vec<(u32, bool)> = Vec::with_capacity(25);
        for _ in 0..5 {
            lines
                .next()
                .unwrap()
                .unwrap()
                .split(' ')
                .map(|n| n.parse::<u32>())
                .filter(|n| n.is_ok())
                .map(|n| (n.unwrap(), false))
                .for_each(|n| board.push(n));
        }
        boards.push(Board {
            winner: false,
            values: board,
        });
    }

    let mut winners = 0;
    let board_count = boards.len();
    for num in draw.iter() {
        for board in boards.iter_mut() {
            if board.winner {
                continue;
            }

            mark(&mut board.values, *num);
            let score = verify(&board.values);
            if score.is_some() {
                board.winner = true;
                winners += 1;
                if winners == board_count {
                    println!("LAST WINNER! {}", num * score.unwrap());
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}

pub fn mark(board: &mut Vec<(u32, bool)>, num: u32) {
    for i in board.iter_mut() {
        if i.0 == num {
            i.1 = true;
            return;
        }
    }
}

pub fn verify(board: &[(u32, bool)]) -> Option<u32> {
    let mut complete = false;
    for i in 0..5 {
        complete |= board[i * 5].1 & board[i * 5 + 1].1 & board[i * 5 + 2].1 & board[i * 5 + 3].1 & board[i * 5 + 4].1;
    }
    if complete {
        return Some(board.iter().filter(|n| !n.1).map(|n| n.0).sum());
    }
    for i in 0..5 {
        complete |= board[i].1 & board[5 + i].1 & board[10 + i].1 & board[15 + i].1 & board[20 + i].1;
    }
    if complete {
        return Some(board.iter().filter(|n| !n.1).map(|n| n.0).sum());
    }
    None
}
