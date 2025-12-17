use std::fs;

fn max_joltage(bb: &[u32]) -> u32 {
    println!("{:?}", bb);
    let (m_i, m) = bb[..bb.len() - 1].iter()
        .enumerate()
        .max_by_key(|(_, &v)| v)
        .unwrap();


    let l = bb[m_i + 1..].iter()
        .max()
        .unwrap();


    println!("{}{}", m, l);
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
        .map(|bb| max_joltage(&bb))
        .sum();
    println!("{}", total);
}

