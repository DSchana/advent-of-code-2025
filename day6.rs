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
  lines.iter()
    .enumerate()
    .map(|(i, v)| {
      let p = 1;
      let mut res = 0u64;
      match lines.last()[i] {
        " " => 
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
