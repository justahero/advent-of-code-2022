#[derive(Debug)]
struct Pair {
    left: String,
    right: String,
}

fn part1(pairs: &[Pair]) -> u32 {
    0
}

fn parse(input: &str) -> Vec<Pair> {
    Vec::new()
}

fn main() {
    let pairs = parse!(include_str!("input.txt"));
    println!("Part 1: {}", part1(&pairs));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"
        [1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
    "#;

    #[test]
    fn check_part1() {
        assert_eq!(13, part1(parse(INPUT)));
    }
}
