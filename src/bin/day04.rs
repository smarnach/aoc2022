use anyhow::{Context, Error, Result};
use aoc2022::read_input;
use std::str::FromStr;

fn main() -> Result<()> {
    let input = read_input!()?;
    let pairs = input
        .lines()
        .map(parse_pair)
        .collect::<Result<Vec<[Range; 2]>>>()?;
    println!("{}", count(&pairs, Range::containing));
    println!("{}", count(&pairs, Range::overlapping));
    Ok(())
}

fn count<F>(pairs: &[[Range; 2]], predicate: F) -> usize
where
    F: Fn(&Range, &Range) -> bool,
{
    pairs.iter().filter(|&p| predicate(&p[0], &p[1])).count()
}

#[derive(Clone, Copy, Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn containing(&self, other: &Self) -> bool {
        (self.start <= other.start && other.end <= self.end)
            || (other.start <= self.start && self.end <= other.end)
    }

    fn overlapping(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').context("range spec contains no hyphen")?;
        Ok(Self {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

fn parse_pair(line: &str) -> Result<[Range; 2]> {
    let (first, second) = line.split_once(',').context("line contains no comma")?;
    Ok([first.parse()?, second.parse()?])
}
