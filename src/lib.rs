use anyhow::{Context, Error, Result};
use std::{
    ops::{Deref, Index, IndexMut},
    path::PathBuf,
    str::FromStr,
};

pub fn read_input(bin_name: &str) -> std::io::Result<String> {
    if stdin_isatty() {
        let path = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "inputs", bin_name]);
        std::fs::read_to_string(path)
    } else {
        std::io::read_to_string(&mut std::io::stdin())
    }
}

#[macro_export]
macro_rules! read_input {
    () => {
        $crate::read_input(env!("CARGO_BIN_NAME"))
    };
}

pub fn stdin_isatty() -> bool {
    unsafe { libc::isatty(libc::STDIN_FILENO) != 0 }
}

pub fn parse_split<'a, T>(input: &'a str, sep: char) -> impl Iterator<Item = Result<T>> + 'a
where
    T: FromStr + 'a,
    Error: From<<T as FromStr>::Err>,
{
    input.trim().split(sep).map(|s| Ok(s.trim().parse()?))
}

pub fn parse_split_comma<'a, T>(line: &'a str) -> impl Iterator<Item = Result<T>> + 'a
where
    T: FromStr + 'a,
    Error: From<<T as FromStr>::Err>,
{
    parse_split(line, ',')
}

pub fn parse_split_space<'a, T>(line: &'a str) -> impl Iterator<Item = Result<T>> + 'a
where
    T: FromStr + 'a,
    Error: From<<T as FromStr>::Err>,
{
    parse_split(line, ' ')
}

pub fn parse_lines<'a, T>(line: &'a str) -> impl Iterator<Item = Result<T>> + 'a
where
    T: FromStr + 'a,
    Error: From<<T as FromStr>::Err>,
{
    parse_split(line, '\n')
}

#[derive(Clone, Debug)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Deref for Grid<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Default> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let mut data = vec![];
        data.resize_with(width * height, T::default);
        Self {
            data,
            width,
            height,
        }
    }
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn from_lines<'a, I, F, L>(lines: I, mut line_parser: F) -> Result<Self>
    where
        I: IntoIterator<Item = &'a str>,
        F: FnMut(&'a str) -> L,
        L: Iterator<Item = Result<T>> + 'a,
    {
        let mut data = Vec::new();
        let mut width = None;
        for line in lines {
            let start_index = data.len();
            for item in line_parser(line) {
                data.push(item?);
            }
            let line_len = data.len() - start_index;
            if *width.get_or_insert(line_len) != line_len {
                return Err(Error::msg("all lines must have the same length"));
            }
        }
        let width = width.context("no data in grid")?;
        let height = data.len() / width;
        Ok(Self {
            data,
            width,
            height,
        })
    }

    pub fn index(&self, x: usize, y: usize) -> Option<usize> {
        (x >= self.width || y >= self.height).then_some(self.width * x + y)
    }

    pub fn get_xy(&self, x: usize, y: usize) -> Option<&T> {
        self.index(x, y).map(|i| &self.data[i])
    }

    pub fn get_xy_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.index(x, y).map(|i| &mut self.data[i])
    }

    pub fn neighbours(&self, i: usize) -> impl Iterator<Item = usize> + '_ {
        (0..4).filter_map(move |j| match j {
            0 => i.checked_sub(self.width),
            1 if i % self.width > 0 => Some(i - 1),
            2 if (i + 1) % self.width > 0 => Some(i + 1),
            3 if i + self.width < self.data.len() => Some(i + self.width),
            _ => None,
        })
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
