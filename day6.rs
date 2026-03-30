use std::fs;
use std::time::Instant;

macro_rules! time {
    ($name:ident, $body:expr) => {{
        let start = Instant::now();
        let res = $body;
        println!("{}: {:?} --> {}", stringify!($name), start.elapsed(), res);
    }};
}

fn part1(num_arrays: &Vec<Vec<u64>>, ops: &Vec<&str>) -> u64 {
    num_arrays.iter()
        .enumerate()
        .map(|(i, nums)| {
            let op = ops[i];
            nums.iter().fold(if op == "*" { 1 } else { 0 }, |t, n| {
                if op == "*" { t * n } else { t + n }
            })
        })
        .sum()
}

fn part2(lines: &Vec<String>) -> u64 {
    let mut final_nums = vec![];
    let mut skip_blank_col = false;
    lines.last().unwrap().chars()
        .enumerate()
        .map(|(i, v)| {
            if skip_blank_col {
                skip_blank_col = false;
                return 0;
            }
            let mut p = 1u64;
            let num = lines[..lines.len() - 1].iter().rev()
                .fold(0, |num, line| {
                    let mut is_blank = false;
                    let digit = line.chars().nth(i)
                        .and_then(|c| c.to_digit(10))
                        .unwrap_or_else(|| {
                            is_blank = true;
                            0
                        }) as u64;
                    let new_num = num + digit * p;
                    if !is_blank {
                        p *= 10;
                    }
                    new_num
                });
            final_nums.push(num);

            match v {
                '+' => {
                    let result = final_nums.iter().fold(0, |t, n| t + n);
                    final_nums = vec![];
                    skip_blank_col = true;
                    result
                },
                '*' => {
                    let result = final_nums.iter().fold(1, |t, n| t * n);
                    final_nums = vec![];
                    skip_blank_col = true;
                    result
                },
                _ => 0,
            }
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("day6-input").expect("Failed to read");
    let lines: Vec<&str> = input.lines().collect();
    let ops: Vec<&str> = lines.last().copied().unwrap_or_default().split_whitespace().collect();
    let num_lines: Vec<&str> = lines[..lines.len() - 1].to_vec();
    let nums: Vec<Vec<u64>> = (0..ops.len())
        .map(|i| {
            num_lines
                .iter()
                .map(|row| {
                    row.split_whitespace().nth(i)
                        .expect("Col idx out of bounds")
                        .parse::<u64>()
                        .expect("Failed to parse num")
                }).collect()
        }).collect();

    time!(part1, part1(&nums, &ops));

    let rev_lines: Vec<String> = lines
        .iter()
        .map(|s| s.chars().rev().collect())
        .collect();
    time!(part2, part2(&rev_lines));
}
