use std::fs;

fn max_joltage(bb: &[u64], n_bat: u64) -> u64 {
    let mut mi = 0;
    let mut mj = 0;
    for rb in (0..n_bat).rev() {
        let end = bb.len() - rb as usize;
        let (n_mi, n_mj) = bb[mi..end].iter()
            .enumerate()
            .fold((0, &bb[mi]), |(max_i, max_v), (i, v)| {
                if v > max_v { (i, v) } else { (max_i, max_v) }
            });
        mi += n_mi + 1;
        mj += n_mj * 10_u64.pow(rb as u32);
    }
    mj
}

fn main() {
    let input = fs::read_to_string("day3-input").expect("Failed to read");
    let battery_bank: Vec<Vec<u64>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u64))
                .collect()
        })
        .collect();

    let total: u64 = battery_bank
        .iter()
        .map(|bb| max_joltage(&bb, 12))
        .sum();
    println!("{}", total);
}

