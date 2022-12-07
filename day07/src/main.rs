//! Day 07:

use anyhow::anyhow;

use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};

peg::parser! {
    grammar line_parser() for str {
        rule dots() -> String
            = ".." { String::from("..") }

        rule root() -> String
            = "/" { String::from("/") }

        rule label() -> String
            = l:$(['a'..='z']+) { l.to_string() }

        rule cd() -> Line
            = "$ cd " l:(root() / dots() / label()) { Line::Cmd(Command::Cd(l.into())) }

        rule ls() -> Line
            = "$ ls" { Line::Cmd(Command::Ls) }

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
enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug, PartialEq)]
enum Line {
    Cmd(Command),
    Dir(String),
    File(u64, String),
}

/// A single file entry with name & size
#[derive(Debug)]
struct FileEntry {
    name: String,
    size: u64,
}

impl FileEntry {
    pub fn new(name: &str, size: u64) -> Self {
        Self {
            name: name.to_string(),
            size,
        }
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}

impl Display for FileEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (file, size={})", self.name, self.size)
    }
}

#[derive(Debug)]
struct DirEntry {
    name: String,
    entries: BTreeMap<String, Box<Entry>>,
}

impl DirEntry {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            entries: BTreeMap::new(),
        }
    }

    pub fn add_dir(&mut self, name: &str) {
        self.entries
            .insert(name.to_string(), Box::new(Entry::dir(name)));
    }

    pub fn add_file(&mut self, name: &str, size: u64) {
        self.entries
            .insert(name.to_string(), Box::new(Entry::file(name, size)));
    }

    pub fn size(&self) -> u64 {
        self.entries.iter().map(|(_, entry)| entry.size()).sum()
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

    pub fn add_dir(&mut self, name: &str) -> anyhow::Result<()> {
        match self {
            Entry::Directory(dir) => dir.add_dir(name),
            Entry::File(_) => return Err(anyhow!("Not a Dir")),
        };
        Ok(())
    }

    pub fn add_file(&mut self, name: &str, size: u64) -> anyhow::Result<()> {
        match self {
            Entry::Directory(dir) => dir.add_file(name, size),
            Entry::File(_) => return Err(anyhow!("{:?} not a Dir", self)),
        }
        Ok(())
    }

    /// TODO is there a way to change the function signature?
    pub fn get_dir(&mut self, name: &str) -> Option<&mut Entry> {
        match self {
            Entry::Directory(dir) => dir.entries.get_mut(name).map(|b| b.as_mut()),
            Entry::File(_) => None,
        }
    }

    pub fn dir(name: &str) -> Self {
        Self::Directory(DirEntry::new(name))
    }

    pub fn file(name: &str, size: u64) -> Self {
        Self::File(FileEntry::new(name, size))
    }

    /// Returns the size of the directory, this needs to be a directory.
    pub fn size(&self) -> u64 {
        match self {
            Entry::Directory(dir) => dir.size(),
            Entry::File(file) => file.size(),
        }
    }

    /// Returns the directory sizes for all directories.
    pub fn sizes(&self, sizes: &mut Vec<u64>) {
        match self {
            Entry::Directory(dir) => {
                sizes.push(dir.size());
                for (_, entry) in dir.entries.iter() {
                    entry.sizes(sizes);
                }
            }
            Entry::File(_) => (),
        }
    }

    pub fn print(&self) {
        self.print_inner(0)
    }

    fn print_inner(&self, level: usize) {
        // `format!("{n:width$}", n = " ", width = level * 2)` does not work, will add a single ' '.
        let ws = " ".repeat(level * 2);

        match self {
            Entry::File(file) => println!("{ws}- {file}"),
            Entry::Directory(dir) => {
                println!("{ws}- {dir} (dir)", dir = dir.name);
                for (_, entry) in dir.entries.iter() {
                    entry.print_inner(level + 1);
                }
            }
        }
    }
}

fn build_hierarchy<'a>(
    parent: &mut Entry,
    lines: &mut impl Iterator<Item = &'a Line>,
) -> anyhow::Result<()> {
    loop {
        match lines.next() {
            Some(Line::Cmd(Command::Cd(dir))) => match dir.as_ref() {
                ".." => return Ok(()),
                dir => {
                    let entry = parent
                        .get_dir(dir)
                        .ok_or_else(|| anyhow!("Directory '{}' not found in parent", dir))?;
                    build_hierarchy(entry, lines)?;
                }
            },
            Some(Line::Cmd(Command::Ls)) => {
                // TODO do we need to do something here?
            }
            Some(Line::Dir(dir)) => {
                parent.add_dir(dir)?;
            }
            Some(Line::File(size, name)) => {
                parent.add_file(name, *size)?;
            }
            None => break,
        };
    }
    Ok(())
}

fn parse(input: &str) -> Entry {
    let lines = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .filter_map(|line| line_parser::line(line).ok())
        .collect::<Vec<_>>();

    let mut root = Entry::root();
    build_hierarchy(&mut root, &mut lines.iter().skip(1)).expect("Failed to build file hierarchy");

    root.print();
    root
}

/// Returns total size of all directories which contents are smaller than 100.000
fn part1(root: &Entry) -> u64 {
    let mut sizes = Vec::new();
    root.sizes(&mut sizes);
    sizes.iter().filter(|&&size| size < 100_0000).sum()
}

/// Check all directories, from all directories that free enough space to be above 30_000_000.
/// The one directory closest but above this threshold is the directory to be deleted. Its size
/// is returned as an answer
fn part2(root: &Entry) -> u64 {
    const TOTAL_DISK_SPACE: u64 = 70_000_000;
    const REQUIRED_DISK_SPACE: u64 = 30_000_000;

    let mut sizes = Vec::new();
    root.sizes(&mut sizes);

    // determine current available space
    let available_space = TOTAL_DISK_SPACE - root.size();

    let x = sizes
        .iter()
        .map(|&size| (size, size + available_space))
        .filter(|(_dir_size, available_space)| *available_space > REQUIRED_DISK_SPACE)
        .map(|(dir_space, _)| dir_space)
        .min();

    x.expect("Failed to find directory")
}

fn main() {
    let entry = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(&entry));
    println!("Part 2: {}", part2(&entry));
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
        assert_eq!(
            Ok(Line::Cmd(Command::Cd("/".into()))),
            line_parser::line("$ cd /")
        );
        assert_eq!(Ok(Line::Cmd(Command::Ls)), line_parser::line("$ ls"));
        assert_eq!(Ok(Line::Dir("a".into())), line_parser::line("dir a"));
        assert_eq!(
            Ok(Line::File(14848514, "b.txt".into())),
            line_parser::line("14848514 b.txt")
        );
        assert_eq!(Ok(Line::File(584, "i".into())), line_parser::line("584 i"));
        assert_eq!(
            Ok(Line::Cmd(Command::Cd("..".into()))),
            line_parser::line("$ cd ..")
        );
    }

    #[test]
    fn check_tree_sizes() {
        let mut entry = parse(INPUT);
        assert_eq!(94853, entry.get_dir("a").unwrap().size(),);
        assert_eq!(48381165, entry.size(),);
    }

    #[test]
    fn check_part1() {
        let entry = parse(INPUT);
        assert_eq!(95437, part1(&entry));
    }

    #[test]
    fn check_part2() {
        let entry = parse(INPUT);
        assert_eq!(24933642, part2(&entry));
    }
}
