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
    let mut passes = v / 100;
    let angle = v % 100;
    
    if c != 0 {
        passes += match d {
            'R' if c + angle >= 100 => 1,
            'L' if c - angle <= 0 => 1,
            _ => 0,
        };
    }
    
    let new_c = match d {
        'R' => (c + angle) % 100,
        'L' => (c - angle).rem_euclid(100),
        _ => c,
    };
    
    (new_c, passes)
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
