use std::fs;

fn is_invalid(n: i64) -> bool {
    let s = n.to_string();
    let len = s.len();

    (1..=len / 2)
        .filter(|&chunk_size| len % chunk_size == 0)
        .any(|chunk_size| {
            s.as_bytes()
                .chunks(chunk_size)
                .all(|chunk| chunk == &s.as_bytes()[..chunk_size])
        })
}

fn main() {
    let input = fs::read_to_string("day2-input").expect("Failed to read");
    let ranges: Vec<(i64, i64)> = input.split(',')
        .filter_map(|s| {
            let mut parts = s.split('-');
            Some((
                    parts.next()?.parse().ok()?,
                    parts.next()?.parse().ok()?
                ))
        })
        .collect();

    let total: i64 = ranges
        .iter()
        .flat_map(|(l, h)| *l..=*h)
        .filter(|&n| is_invalid(n))
        .sum();
    println!("{}", total);
}
