use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
struct Entry {
    min: usize,
    max: usize,
    look_up_char: char,
    password: String,
}

impl FromStr for Entry {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d+)-(\d+)\s+([a-zA-Z]):\s+(\w+)$").expect("Failed to parse line!");
        let caps = re.captures(input).unwrap();
        let min = caps.get(1).unwrap().as_str().parse::<usize>().expect("NaN");
        let max = caps.get(2).unwrap().as_str().parse::<usize>().expect("NaN");
        let look_up_char = caps.get(3).unwrap().as_str().chars().next().unwrap();
        let password = caps.get(4).unwrap().as_str().into();
        Ok(Entry {
            min,
            max,
            look_up_char,
            password,
        })
    }
}

fn main() {
    let data = read_file("./input/input.txt");

    println!("{:?}", part1(data.clone()));
    println!("{:?}", part2(data.clone()));
}

fn part1(data: Vec<Entry>) -> std::result::Result<usize, String> {
    Ok(data
        .into_iter()
        .filter(|entry| {
            let split_cnt = entry.password.split(entry.look_up_char).count() - 1;
            // println!("{:?}", split_cnt);
            split_cnt >= entry.min && split_cnt <= entry.max
        })
        .count())
}

fn part2(data: Vec<Entry>) -> std::result::Result<usize, String> {
    Ok(data
        .into_iter()
        .filter(|entry| {
            let pass: Vec<char> = entry.password.chars().collect();
            pass[entry.min - 1].eq(&entry.look_up_char)
                ^ pass[entry.max - 1].eq(&entry.look_up_char)
        })
        .count())
}

fn read_file<P>(filename: P) -> Vec<Entry>
where
    P: AsRef<Path>,
{
    fs::read_to_string(filename)
        .expect("Failed to load file")
        .lines()
        .flat_map(|l| Entry::from_str(l))
        .collect()
}
