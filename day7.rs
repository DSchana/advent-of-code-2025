use std::fs;
use std::time::Instant;

macro_rules! time {
    ($name:ident, $body:expr) => {{
        let start = Instant::now();
        let res = $body;
        println!("{}: {:?} -> {}", stringify!($name), start.elapsed(), res);
    }};
}

fn part1(grid: &Vec<Vec<u8>>, laser_idx: &Vec<u8>) -> usize {
    grid.iter()
        .map(|row| {
            // TODO: Build this with a fold
            let mut new_laser_idx = vec!(0u8; laser_idx.len());
            row.iter()
                .enumerate()
                .map(move |(i, v)| {
                    if *v == 1 {
                        if let Some(e) = new_laser_idx.get_mut(i - 1) {
                            *e = 1;
                        }
                        if let Some(e) = new_laser_idx.get_mut(i + 1) {
                            *e = 1;
                        }
                    }
                })
        });

    laser_idx.iter().filter(|&x| *x == 1).count()
}

fn main() {
    let input = fs::read_to_string("day7-input").expect("Failed to read");
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| if b == b'^' { 1 } else if b == b'S' { 2 } else { 0 })
                .collect()
        }).collect();
    let mut laser_idx = vec!(0u8; grid[0].len());
    if let Some(pos) = grid.first().and_then(|row| row.iter().position(|&x| x == 2)) {
        laser_idx[pos] = 1;
    }

    time!(part1, part1(&grid, &mut laser_idx));
}

