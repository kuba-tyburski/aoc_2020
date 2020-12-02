use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    let data = read_file("./input/input.txt").unwrap();
    // data.sort();
    println!("{:?}", data);

    println!("{:?}", part1(data.clone()));
    println!("{:?}", part2(data.clone()));
}

fn part1(data: Vec<i64>) -> std::result::Result<i64, String> {
    // let mut processed_data = data.clone();
    // for (idx, val_a) in data.iter().enumerate() {
    //     let (_left, right) = processed_data.split_at_mut(idx);
    //     for val_b in right.iter() {
    //         if val_a + val_b > 2020 {
    //             break;
    //         }
    //         if val_a + val_b == 2020 {
    //             println!("found val_a: {:?} val_b: {:?}", val_a, val_b);
    //             return Ok(val_a * val_b);
    //         }
    //     }
    // }

    let mut set: HashSet<i64> = HashSet::new();
    for val_b in data.iter() {
        if set.contains(&(2020 - val_b)) {
            println!("found valA: {:?} valB: {:?}", (2020 - val_b), val_b);
            return Ok((2020 - val_b) * val_b);
        }
        set.insert(*val_b);
    }

    Err("Not found double".to_string())
}

fn part2(data: Vec<i64>) -> std::result::Result<i64, String> {
    let mut processed_data = data.clone();
    for (idx, val_a) in data.iter().enumerate() {
        let mut set: HashSet<i64> = HashSet::new();
        let sum = 2020 - val_a;
        let (_left, right) = processed_data.split_at_mut(idx);
        for val_b in right.iter() {
            if set.contains(&(sum - val_b)) {
                println!(
                    "found valA: {:?} val_b: {:?}, val_c: {:?}",
                    val_a,
                    val_b,
                    sum - val_b
                );
                return Ok(val_a * val_b * (sum - val_b));
            }
            set.insert(*val_b);
        }
    }
    Err("Not found triplet".to_string())
}

fn read_file<P>(filename: P) -> std::result::Result<Vec<i64>, ::std::num::ParseIntError>
where
    P: AsRef<Path>,
{
    fs::read_to_string(filename)
        .expect("Failed to load file")
        .lines()
        .flat_map(|l| l.split_whitespace())
        .map(|int| int.parse::<i64>())
        .collect()
}
