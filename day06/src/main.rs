//! Day 06: Tuning Trouble

/// Detects the start of packet marker, a four letter sequence where all letters are different
///
/// The first position of such a sequence is returned.
fn find_start_marker(datastream: &str) -> Option<u32> {
    None
}

fn parse(input: &str) {
    todo!("")
}

fn main() {
    let x = parse(include_str!("input.txt"));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn check_part1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(Some(7), find_start_marker(input));
    }
}
