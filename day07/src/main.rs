//! Day 07:

/// A single file entry with name & size
#[derive(Debug)]
struct FileEntry {
    name: String,
    size: u64,
}

#[derive(Debug)]
struct DirEntry {
    name: String,
    entries: Vec<Box<Entry>>,
}

impl DirEntry {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            entries: Vec::new(),
        }
    }
}

#[derive(Debug)]
enum Entry {
    File(FileEntry),
    Directory(DirEntry),
}

impl Entry {
    pub fn root() -> Self {
        Entry::Directory(DirEntry::new("/"))
    }
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().map(str::trim).collect::<Vec<_>>()
}

fn main() {
    let lines = parse(include_str!("input.txt"));
    // println!("Part 1: {}", part1(&lines));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = r#"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    "#;

    #[test]
    fn check_part1() {
        let entry = parse(INPUT);
    }

    #[test]
    fn check_part2() {}
}
