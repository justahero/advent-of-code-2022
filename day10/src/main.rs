//! Day 10:

fn parse(input: &str) {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty());
}

fn part1() -> usize {
    0
}

fn main() {
    let _ = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1());
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = r#"
    "#;

    #[test]
    fn check_part1() {
        let _ = parse(INPUT);
        assert_eq!(0, part1());
    }
}
