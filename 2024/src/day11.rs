use std::{collections::HashMap, fmt::Display};

pub fn part1(input: &str) -> impl Display {
    let mut hash: HashMap<usize, usize> = HashMap::new();
    input
        .split_ascii_whitespace()
        .for_each(|c| { 
            let val = c.parse::<usize>().unwrap(); 
            *hash.entry(val).or_insert(1) = 1;
        });
    process(&mut hash, &mut 25)
}

pub fn part2(input: &str) -> impl Display {
    let mut hash: HashMap<usize, usize> = HashMap::new();
    input
        .split_ascii_whitespace()
        .for_each(|c| { 
            let val = c.parse::<usize>().unwrap(); 
            *hash.entry(val).or_insert(1) = 1;
        });
    process(&mut hash, &mut 75)
}

fn process(nums: &mut HashMap<usize, usize>, iter_left: &mut usize) -> usize {
    if *iter_left == 0 {
        return nums.values().sum();
    }

    // Can't modify a map in place
    let mut changes: Vec<(usize, isize)> = vec![];
    for (key, val) in nums.iter() {
        let change = *val as isize;
        if *key == 0 {
            changes.push((1, change));
            changes.push((0, -change));
        } else if count_digits(*key as u64) % 2 == 0 {
            let (l, r) = split_number(*key);
            changes.push((l, change));
            changes.push((r, change));
            changes.push((*key, -change));
        } else {
            changes.push((*key * 2024, change));
            changes.push((*key, -change));
        }
    }

    // Apply changes after iteration
    for (k, delta) in changes {
        if delta > 0 {
            *nums.entry(k).or_insert(0) += delta as usize;
        } else if let Some(count) = nums.get_mut(&k) {
            *count = count.saturating_sub((-delta) as usize);
            if *count == 0 {
                nums.remove(&k);
            }
        }
    }

    *iter_left -= 1;
    process(nums, iter_left)
}

fn split_number(n: usize) -> (usize, usize) {
    let divisor = 10usize.pow(count_digits(n as u64) / 2);
    (n / divisor, n % divisor)
}

fn count_digits(n: u64) -> u32 {
    const POW10: [u64; 20] = [
        0,
        10,
        100,
        1_000,
        10_000,
        100_000,
        1_000_000,
        10_000_000,
        100_000_000,
        1_000_000_000,
        10_000_000_000,
        100_000_000_000,
        1_000_000_000_000,
        10_000_000_000_000,
        100_000_000_000_000,
        1_000_000_000_000_000,
        10_000_000_000_000_000,
        100_000_000_000_000_000,
        1_000_000_000_000_000_000,
        10_000_000_000_000_000_000,
    ];

    let t = ((64 - (n | 1).leading_zeros()) * 1233) >> 12;
    t - if n < POW10[t as usize] { 1 } else { 0 } + 1
}

/* #[cfg(test)]
mod tests {
    use std::fs;

    use super::*;   

    fn read_data_from_file(file_path: &str) -> String {
        fs::read_to_string(file_path)
            .unwrap_or_else(|_| panic!("Failed to read {}", file_path))   
    }

    #[test]
    fn real() {
        let input = read_data_from_file("input/2024/day11.txt");

        assert_eq!(part1(&input), 189167);
        assert_eq!(part2(&input), 225253278506288);
    }
} */