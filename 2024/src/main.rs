use std::{fs, time::Instant};
use cs_aoc_2024::day_11;

pub fn read_data_from_file(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Failed to read {}", file_path))   
}

fn main() {    
    let input = read_data_from_file("input/2024/day11.txt");

    let mut start = Instant::now();
    let silver = day_11::part1(&input);
    let silver_duration = start.elapsed();

    start = Instant::now();
    let gold = day_11::part2(&input);
    let gold_duration = start.elapsed();

    println!("Silver: {}, Gold: {}", silver, gold);
    println!("Silver: {:?}, Gold: {:?}", silver_duration, gold_duration);

    
}
