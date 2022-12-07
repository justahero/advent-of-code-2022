//! Day 07:

peg::parser! {
    grammar line_parser() for str {
        rule dots() -> String
            = ".." { String::from("..") }

        rule root() -> String
            = "/" { String::from("/") }

        rule label() -> String
            = l:$(['a'..='z']+) { l.to_string() }

        rule cd() -> Line
            = "$ cd " l:(root() / dots() / label()) { Line::Cd(l.into()) }

        rule ls() -> Line
            = "$ ls" { Line::Ls }

        rule filename() -> String
            = l:$(['a'..='z']+['.']?['a'..='z']*) { l.to_string() }

        rule file() -> Line
            = n:$(['0'..='9']+) " " l:filename() { Line::File(n.parse::<u64>().unwrap(), l.into()) }

        rule dir() -> Line
            = "dir " l:label() { Line::Dir(l.to_string()) }

        pub(crate) rule line() -> Line
            = cd() / ls() / file() / dir()
    }
}

#[derive(Debug, PartialEq)]
enum Line {
    Cd(String),
    Ls,
    Dir(String),
    File(u64, String),
}

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

fn parse(input: &str) -> Entry {
    let output = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();

    Entry::root()
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
    fn check_line_parser() {
        assert_eq!(Ok(Line::Cd("/".into())), line_parser::line("$ cd /"));
        assert_eq!(Ok(Line::Ls), line_parser::line("$ ls"));
        assert_eq!(Ok(Line::Dir("a".into())), line_parser::line("dir a"));
        assert_eq!(
            Ok(Line::File(14848514, "b.txt".into())),
            line_parser::line("14848514 b.txt")
        );
        assert_eq!(Ok(Line::File(584, "i".into())), line_parser::line("584 i"));
        assert_eq!(Ok(Line::Cd("..".into())), line_parser::line("$ cd .."));
    }

    #[test]
    fn check_part1() {
        let entry = parse(INPUT);
    }

    #[test]
    fn check_part2() {}
}
