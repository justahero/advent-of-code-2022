//! Day 20: Grove Positioning System

use itertools::Itertools;

const DECRYPTION_KEY: i64 = 811589153;

/// NOTE this algorithm does not fully work with the sample data and returns a different
/// list of entries.

fn decrypt(encrypted_file: Vec<(usize, i64)>, rounds: usize) -> Vec<i64> {
    let file_length = encrypted_file.len();
    let mut modified = encrypted_file.clone();

    for _ in 0..rounds {
        for (id, value) in encrypted_file.iter() {
            if *value == 0 {
                continue;
            }

            let index = modified
                .iter()
                .position(|state_value| state_value.0 == *id)
                .unwrap();

            let current = modified.remove(index);

            let new_index = index as i64 + current.1;
            let new_index = new_index.rem_euclid(file_length as i64 - 1);

            modified.insert(new_index as usize, current);
        }
    }

    modified.iter().map(|v| v.1).collect_vec()
}

fn calculate_coordinates(mixed: &[i64]) -> i64 {
    let zero_pos = mixed.iter().position(|p| *p == 0).unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|i| mixed[(i + zero_pos) % mixed.len()])
        .sum::<i64>()
}

/// Mix the encrypted file for all given entries
fn part1(encrypted_file: Vec<i64>) -> i64 {
    // we need to enumerate here to know the original index after moving entries
    // the original file does not hold unique entries, but numbers can appear more than once.
    let encrypted_file = encrypted_file.into_iter().enumerate().collect_vec();
    let mixed = decrypt(encrypted_file, 1);

    calculate_coordinates(&mixed)
}

fn part2(encrypted_file: Vec<i64>) -> i64 {
    let encrypted_file = encrypted_file
        .iter()
        .map(|value| value * DECRYPTION_KEY)
        .enumerate()
        .collect_vec();

    let mixed = decrypt(encrypted_file, 10);
    calculate_coordinates(&mixed)
}

fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(str::trim)
        .filter_map(|line| line.parse::<i64>().ok())
        .collect_vec()
}

fn main() {
    let encrypted_file = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(encrypted_file.clone()));
    println!("Part 2: {}", part2(encrypted_file.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        1
        2
        -3
        3
        -2
        0
        4
    ";

    #[test]
    fn check_part1() {
        assert_eq!(3, part1(parse(INPUT)));
    }

    #[test]
    fn check_part2() {
        assert_eq!(1623178306, part2(parse(INPUT)));
    }
}
