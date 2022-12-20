//! Day 20: Grove Positioning System

use itertools::Itertools;

fn decrypt(encrypted_file: Vec<(usize, i64)>) -> Vec<i64> {
    let mut modified = encrypted_file.clone();

    for (id, value) in encrypted_file.iter() {
        if *value == 0 {
            continue;
        }
        println!("entry: {}", id);

        let index = modified
            .iter()
            .position(|state_value| state_value.0 == *id)
            .unwrap();

        println!("  index: {}", index);

        let current = modified.remove(index);

        let new_index = index as i64 + current.1;
        let new_index = new_index.rem_euclid(modified.len() as i64);

        modified.insert(new_index as usize, current);
    }

    modified.iter().map(|v| v.1).collect_vec()
}

/// Mix the encrypted file for all given entries
fn part1(encrypted_file: Vec<(usize, i64)>) -> i64 {
    let mixed = decrypt(encrypted_file);
    let zero_pos = mixed.iter().position(|p| *p == 0).unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|i| mixed[(i + zero_pos) % mixed.len()])
        .inspect(|entry| println!("COORDINATE: {}", entry))
        .sum::<i64>()
}

fn parse(input: &str) -> Vec<(usize, i64)> {
    input
        .lines()
        .map(str::trim)
        .filter_map(|line| line.parse::<i64>().ok())
        .enumerate()
        .collect_vec()
}

fn main() {
    let encrypted_file = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(encrypted_file));
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
    fn check_decrypt() {
        assert_eq!(vec![1, 2, -3, 4, 0, 3, -2], decrypt(parse(INPUT)),);
    }

    #[test]
    fn check_part1() {
        assert_eq!(3, part1(parse(INPUT)));
    }
}
