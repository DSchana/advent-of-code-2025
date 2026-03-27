use std::fs;

fn parse_input(input: &str) -> (Vec<std::ops::RangeInclusive<u64>>, Vec<u64>) {
  let parts: Vec<&str> = input.split("\n\n").collect();
  let fresh_ranges: Vec<std::ops::RangeInclusive<u64>> = parts[0]
    .lines()
    .map(|line| {
      let nums: Vec<u64> = line
        .split('-')
        .filter_map(|s| s.parse().ok())
        .collect();
      nums[0]..=nums[1]
    }).collect();

  let available_ids: Vec<u64> = parts[1]
    .lines()
    .filter_map(|l| l.parse::<u64>().ok())
    .collect();

  (fresh_ranges, available_ids)
}

fn main() {
  let input = fs::read_to_string("day5-input").expect("Failed to read");
  let (mut fresh_ranges, available_ids) = parse_input(&input);

  let fresh_id_count = available_ids
    .iter()
    .filter(|ai| fresh_ranges.iter().any(|r| r.contains(ai)))
    .count();
  
  fresh_ranges.sort_by_key(|r| *r.start());
  let mut fresh_range_count = 0u64;
  let mut c_start = fresh_ranges[0].start();
  let mut c_end = fresh_ranges[0].end();
  for r in &fresh_ranges[1..] {
    if r.start() <= c_end {
      c_end = c_end.max(r.end());
    } else {
      fresh_range_count += (c_end - c_start + 1) as u64;
      c_start = r.start();
      c_end = r.end();
    }
  }
  fresh_range_count += (c_end - c_start + 1) as u64;

  println!("{} {}", fresh_id_count, fresh_range_count);
}
