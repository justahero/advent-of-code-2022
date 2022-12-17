//! Day 17: Pyroclastic Flow

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
}

impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug)]
enum Dir {
    Left = 0,
    Right,
}

impl Dir {
    pub fn dir(&self) -> Pos {
        match self {
            Dir::Left => Pos::new(-1, 0),
            Dir::Right => Pos::new(1, 0),
        }
    }
}

impl From<char> for Dir {
    fn from(c: char) -> Self {
        match c {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("Unknown char found"),
        }
    }
}

struct Shape {
    rows: Vec<u8>,
    height: u32,
    width: u32,
}

impl Shape {
    pub fn new(bits: [u8; 4]) -> Self {
        let width = bits.iter().map(|row| row.count_ones()).max().unwrap();
        let height = bits.iter().filter(|&row| row.count_ones() > 1).sum::<u8>() as u32;

        Self {
            rows: bits.to_vec(),
            height,
            width,
        }
    }
}

fn part1(directions: Vec<Dir>) -> usize {
    // X bits are given in reverse order
    //
    // `0b111100` -> represents '@@@@' 2 units from left side
    //    543210  <- bits indices
    let shapes: Vec<Shape> = vec![
        Shape::new([0b000000, 0b000000, 0b000000, 0b111100]),
        Shape::new([0b000000, 0b001000, 0b011100, 0b001000]),
        Shape::new([0b000000, 0b000100, 0b000100, 0b011100]),
        Shape::new([0b000100, 0b000100, 0b000100, 0b000100]),
        Shape::new([0b000000, 0b000000, 0b001100, 0b001100]),
    ];

    0
}

fn parse(input: &str) -> Vec<Dir> {
    input.chars().map(|c| c.into()).collect()
}

fn main() {
    //    let pipes = parse(include_str!("input.txt"));
    //    println!("Part 1: {}", part1(pipes.clone()));
    //    println!("Part 2: {}", part2(pipes));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn check_part1() {
        assert_eq!(3068, part1(parse(INPUT)));
    }

    #[test]
    fn check_part2() {
        // assert_eq!(1707, part2(parse(INPUT)));
    }
}
