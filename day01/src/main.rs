use itertools::Itertools;

/// Parse input & group list of successive calories to one elf.
fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(str::trim)
                .filter_map(|s| s.parse::<u32>().ok())
                .collect_vec()
        })
        .collect_vec()
}

/// Returns the summed calories for each elf, sorted by biggest sum first
fn sorted_sums(elves: &[Vec<u32>]) -> impl Iterator<Item = u32> + '_ {
    elves.iter().map(|elf| elf.iter().sum()).sorted().rev()
}

/// Return the highest calories sum of one elf
fn part1(elves: &[Vec<u32>]) -> u32 {
    sorted_sums(elves).next().unwrap()
}

/// Return the calories sums of th top three elves
fn part2(elves: &[Vec<u32>]) -> u32 {
    sorted_sums(elves).take(3).sum()
}

fn main() {
    let calories = parse(include_str!("input.txt"));
    println!("Max calories: {}", part1(&calories));
    println!("Top three calories: {}", part2(&calories));
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    const INPUT: &str = r#"
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    "#;

    #[test]
    fn test_part1() {
        assert_eq!(24_000, part1(&parse(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(45_000, part2(&parse(INPUT)));
    }
}
