use std::collections::HashMap;

use anyhow::{Context, Error, Result};
use aoc2022::read_input;

fn main() -> Result<()> {
    let input = read_input!()?;
    let mut fs = FileSystem::reconstruct(&input)?;
    fs.calculate_sizes();
    println!("{}", fs.total_size(100_000));
    println!("{}", fs.find_directory_to_delete(40_000_000)?);
    Ok(())
}

struct INode {
    mode: Mode,
    parent: usize,
    size: usize,
}

impl INode {
    fn directory(parent: usize) -> Self {
        Self {
            mode: Mode::Directory,
            parent,
            size: 0,
        }
    }

    fn file(parent: usize, size: usize) -> Self {
        Self {
            mode: Mode::File,
            parent,
            size,
        }
    }
}

#[derive(Eq, PartialEq)]
enum Mode {
    File,
    Directory,
}

struct FileSystem {
    inodes: Vec<INode>,
}

impl FileSystem {
    fn reconstruct(session: &str) -> Result<Self> {
        let mut inodes = vec![INode::directory(0)];
        let mut dirs: HashMap<(usize, &str), usize> = HashMap::new();
        let mut cwd = 0;
        for line in session.lines() {
            let mut tokens = line.split_whitespace();
            match tokens.next() {
                Some("$") => match tokens.next() {
                    Some("cd") => match tokens.next() {
                        Some("/") => cwd = 0,
                        Some("..") => cwd = inodes[cwd].parent,
                        Some(dir) => cwd = *dirs.get(&(cwd, dir)).context("directory not found")?,
                        None => return Err(Error::msg("expected directory name after cd")),
                    },
                    Some("ls") | None => {}
                    Some(_) => return Err(Error::msg("unexpected command")),
                },
                Some("dir") => {
                    let name = tokens.next().context("expected directory name")?;
                    dirs.insert((cwd, name), inodes.len());
                    inodes.push(INode::directory(cwd));
                }
                Some(size) => {
                    tokens.next().context("expected file name")?;
                    inodes.push(INode::file(cwd, size.parse()?));
                }
                None => {}
            }
            if tokens.next().is_some() {
                return Err(Error::msg("unexpected token"));
            }
        }
        Ok(Self { inodes })
    }

    fn calculate_sizes(&mut self) {
        for i in (1..self.inodes.len()).rev() {
            let parent = self.inodes[i].parent;
            let size = self.inodes[i].size;
            self.inodes[parent].size += size;
        }
    }

    fn total_size(&self, limit: usize) -> usize {
        self.inodes
            .iter()
            .filter(|inode| inode.mode == Mode::Directory && inode.size <= limit)
            .map(|inode| inode.size)
            .sum()
    }

    fn find_directory_to_delete(&self, max_used: usize) -> Result<usize> {
        let space_to_free = self.inodes[0]
            .size
            .checked_sub(max_used)
            .context("already enough free space")?;
        let min = self
            .inodes
            .iter()
            .filter(|inode| inode.mode == Mode::Directory && inode.size >= space_to_free)
            .map(|inode| inode.size)
            .min()
            .unwrap();
        Ok(min)
    }
}
