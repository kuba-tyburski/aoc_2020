use std::fs;
use std::path::Path;

fn main() {
    let data = read_file("./input/input.txt");
    // println!("{:?}", data);

    println!("{:?}", part1(data.clone()));
    println!("{:?}", part2(data.clone()));
}

fn part1(data: Vec<Vec<char>>) -> std::result::Result<usize, String> {
    let mut cnt: usize = 0;
    let mut col: usize = 0;
    for row in 1..data.len() {
        col = (col + 3) % data[row].len();
        if data[row][col] == '#' {
            cnt += 1;
        }
    }

    Ok(cnt)
}

fn part2(data: Vec<Vec<char>>) -> std::result::Result<usize, String> {
    let mut counters = Vec::new();
    let modificators = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    for (col_mod, row_mod) in modificators {
        println!("going for: ({}, {})", row_mod, col_mod);
        let rows_size = data.len();
        let mut cnt: usize = 0;
        let mut row: usize = row_mod;
        let mut col: usize = 0;
        while row < rows_size {
            col = (col + col_mod) % data[row].len();
            if data[row][col] == '#' {
                cnt += 1;
            }
            row += row_mod;
        }
        counters.push(cnt);
    }

    Ok(counters.iter().fold(1, |acc, x| acc * x))
}

fn read_file<P>(filename: P) -> Vec<Vec<char>>
where
    P: AsRef<Path>,
{
    fs::read_to_string(filename)
        .expect("Failed to load file")
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}
