use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn main() -> io::Result<()> {
    let f = File::open("resources/day9")?;
    let f = BufReader::new(f);

    let lines: Vec<String> = f.lines().map(|l| l.unwrap()).collect();
    let arr_width = lines[0].len() as usize;
    let arr_height = lines.len() as usize;

    let height_map: Vec<u32> = lines
        .iter()
        .flat_map(|l| l.chars())
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let low_points: Vec<usize> = height_map
        .iter()
        .enumerate()
        .filter(|(i, height)| {
            let mut low = true;

            if i % arr_width > 0 {
                low &= height_map[i - 1] > **height;
            }
            if i % arr_width < (arr_width - 1) {
                low &= height_map[i + 1] > **height;
            }
            if i / arr_width > 0 {
                low &= height_map[i - arr_width] > **height;
            }
            if i / arr_width < (arr_height - 1) {
                low &= height_map[i + arr_width] > **height;
            }
            low
        })
        .map(|(i, _)| i)
        .collect();

    let pt1: u32 = low_points.iter().map(|l| height_map[*l] + 1).sum();
    println!("pt1 {:?}", pt1);

    let mut visited = HashSet::new();
    let mut pt2: Vec<u32> = low_points
        .iter()
        .map(|l| basin(&height_map, &mut visited, arr_width, arr_height, *l, 0))
        .collect();
    pt2.sort_by(|a, b| b.cmp(a));
    println!("pt2 {:?}", pt2[0] * pt2[1] * pt2[2]);

    Ok(())
}

fn basin(
    height_map: &[u32],
    visited: &mut HashSet<usize>,
    arr_width: usize,
    arr_height: usize,
    i: usize,
    last: u32,
) -> u32 {
    let cur = height_map[i];
    if visited.contains(&i) || cur == 9 || cur < last {
        return 0;
    }
    visited.insert(i);
    let mut sum = 1;
    if i % arr_width > 0 {
        sum += basin(height_map, visited, arr_width, arr_height, i - 1, cur);
    }
    if i % arr_width < (arr_width - 1) {
        sum += basin(height_map, visited, arr_width, arr_height, i + 1, cur);
    }
    if i / arr_width > 0 {
        sum += basin(
            height_map,
            visited,
            arr_width,
            arr_height,
            i - arr_width,
            cur,
        );
    }
    if i / arr_width < (arr_height - 1) {
        sum += basin(
            height_map,
            visited,
            arr_width,
            arr_height,
            i + arr_width,
            cur,
        );
    }
    sum
}
