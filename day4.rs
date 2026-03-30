use std::fs;

fn count_neighbours(grid: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let rows = grid.len();
    let cols = grid[0].len();
    (0..rows)
        .map(|r| {
            (0..cols)
                .map(|c| {
                    let mut count = 0u8;
                    for dr in -1i32..=1 {
                        for dc in -1i32..=1 {
                            if dr == 0 && dc == 0 {
                                continue;
                            }
                            let nr = r as i32 + dr;
                            let nc = c as i32 + dc;
                            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                                count += grid[nr as usize][nc as usize]
                            }
                        }
                    }
                    count
                })
                .collect()
        })
        .collect()
}

fn main() {
    let input = fs::read_to_string("day4-input").expect("Failed to read");
    let mut grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| if b == b'@' { 1 } else { 0 })
                .collect()
        })
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut total = 0;
    loop {
        let neighbours = count_neighbours(&grid);
        let a: usize = (0..rows)
            .flat_map(|r| (0..cols).map(move |c| (r, c)))
            .filter(|&(r, c)| grid[r][c] == 1 && neighbours[r][c] < 4)
            .count();
        if a == 0 {
            break;
        }
        grid = (0..rows)
            .map(|r| {
                (0..cols)
                    .map(|c| {
                        if grid[r][c] == 1 && neighbours[r][c] >= 4 {
                            1
                        } else {
                            0
                        }
                    })
                    .collect()
            })
            .collect();
        total += a;
    }

    println!("{}", total);
}
