use std::fs;

fn max_joltage(bb: &[u32], n_bat: u32) -> u32 {
    // TODO: Loop through n battries finding the largest number
    // from the previous index and keep a sum += m * 10^bat_pos


    let (m_i, m) = bb[..bb.len() - 1].iter()
        .enumerate()
        .fold((0, &bb[0]), |(max_i, max_v), (i, v)| {
            if v > max_v { (i, v) } else { (max_i, max_v) }
        });

    let l = bb[m_i + 1..].iter()
        .max()
        .unwrap();

    m * 10 + l
}

fn main() {
    let input = fs::read_to_string("day3-input").expect("Failed to read");
    let battery_bank: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect()
        })
        .collect();

    let total: u32 = battery_bank
        .iter()
        .map(|bb| max_joltage(&bb, 12))
        .sum();
    println!("{}", total);
}

