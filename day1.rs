use std::fs;

fn parse(s: &str) -> Option<(char, i32)> {
    let mut iter = s.chars();
    iter.next().and_then(|c| {
        if c.is_alphabetic() {
            iter.as_str().parse().ok().map(|i| {
                (c, i)
            })
        } else {
            None
        }
    })
}

fn turn_dial(c: i32, d: char, v: i32) -> (i32, i32) {
    let end = if d == 'R' { c + v } else { c - v };
    let crosses = if d == 'R' { end / 100 }
                else { end.abs() / 100 + (if end % 100 == 0 { 0 } else { 1 }) };
    let lands_on_zero = if end % 100 == 0 { 1 } else { 0 };

    (end.rem_euclid(100), crosses + lands_on_zero)
}

fn main() {
    let input = fs::read_to_string("day1-input").expect("Failed to read");
    let turns: Vec<(char, i32)> = input.lines()
        .map(|line| parse(line).unwrap())
        .collect();

    let mut c = 50;
    let mut a = 0;
    for (d, v) in turns {
        let (new_c, wrapped) = turn_dial(c, d, v);
        c = new_c;
        a += wrapped;
        //println!("{} {} -> {} {}", d, v, c, a);
    }
    println!("{}", a);
}
