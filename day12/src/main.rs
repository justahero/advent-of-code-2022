//! Day 12: Hill Climbing Algorithm

struct Grid {
    cells: Vec<u8>,
}

impl Grid {
    pub fn new() -> Self {
        Self { cells: Vec::new() }
    }

    pub fn add_line(mut self, row: impl Iterator<Item = u8>) -> Self {
        self
    }
}

fn part1(grid: &Grid) -> u32 {
    0
}

fn parse(input: &str) -> Grid {
    let grid = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .fold(Grid::new(), |grid, line| {
            grid.add_line(
                line.chars()
                    .filter_map(|c| c.to_string().parse::<u8>().ok()),
            )
        });

    grid
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    "#;

    #[test]
    fn check_part1() {
        assert_eq!(31, part1(&parse(INPUT)));
    }
}
