use anyhow::{Context, Error, Result};
use aoc2022::{read_input, Grid};
use std::{cmp::Reverse, collections::BinaryHeap, str::FromStr};

fn main() -> Result<()> {
    let input = read_input!()?;
    let map: Map = input.parse()?;
    println!("{}", map.find_path(Map::is_start).context("no path to target found")?);
    println!("{}", map.find_path(Map::is_height_a).context("no path to target found")?);
    Ok(())
}

struct Map {
    heights: Grid<u8>,
    start: usize,
    end: usize,
}

impl Map {
    fn find_path<F>(&self, finished: F) -> Option<usize>
    where
        F: Fn(&Self, usize) -> bool,
    {
        let mut dist = vec![usize::MAX; self.heights.len()];
        dist[self.end] = 0;
        let mut queue = BinaryHeap::from([(Reverse(0), self.end)]);
        while let Some((Reverse(current_dist), index)) = queue.pop() {
            if finished(self, index) {
                return Some(current_dist);
            }
            let min_height = self.heights[index] - 1;
            let next_dist = current_dist + 1;
            for next in self.heights.neighbours(index) {
                if min_height <= self.heights[next] && next_dist < dist[next] {
                    dist[next] = next_dist;
                    queue.push((Reverse(next_dist), next));
                }
            }
        }
        None
    }

    fn is_start(&self, index: usize) -> bool {
        index == self.start
    }

    fn is_height_a(&self, index: usize) -> bool {
        self.heights[index] == b'a'
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut heights = Grid::from_lines(input.lines(), |line| {
            line.as_bytes().iter().cloned().map(Ok)
        })?;
        let start = heights
            .iter()
            .position(|&c| c == b'S')
            .context("no start position found")?;
        heights[start] = b'a';
        let end = heights
            .iter()
            .position(|&c| c == b'E')
            .context("no end position found")?;
        heights[end] = b'z';
        if !heights.iter().all(u8::is_ascii_lowercase) {
            Err(Error::msg("invalid character in height map"))
        } else {
            Ok(Self {
                start,
                end,
                heights,
            })
        }
    }
}
