use anyhow::{Context, Error, Result};
use aoc2022::read_input;
use std::{cmp::Ordering, collections::HashSet, str::FromStr};

fn main() -> Result<()> {
    let input = read_input!()?;
    let motions = parse_input(&input)?;
    for len in [2, 10] {
        let mut rope = Rope::new(len);
        rope.apply_motions(&motions);
        println!("{}", rope.track.len());
    }
    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = match s {
            "R" => Direction::Right,
            "U" => Direction::Up,
            "L" => Direction::Left,
            "D" => Direction::Down,
            _ => return Err(Error::msg("invalid direction")),
        };
        Ok(dir)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn mov(&mut self, dir: Direction) {
        match dir {
            Direction::Right => self.x += 1,
            Direction::Up => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Down => self.y -= 1,
        }
    }

    fn max_dist(self, other: Position) -> u32 {
        self.x.abs_diff(other.x).max(self.y.abs_diff(other.y))
    }

    fn trail(&mut self, other: Position) -> bool {
        if self.max_dist(other) < 2 {
            false
        } else {
            match self.x.cmp(&other.x) {
                Ordering::Less => self.x += 1,
                Ordering::Equal => {}
                Ordering::Greater => self.x -= 1,
            }
            match self.y.cmp(&other.y) {
                Ordering::Less => self.y += 1,
                Ordering::Equal => {}
                Ordering::Greater => self.y -= 1,
            }
            true
        }
    }
}

struct Rope {
    knots: Vec<Position>,
    track: HashSet<Position>,
}

impl Rope {
    fn new(len: usize) -> Self {
        assert!(len >= 1);
        let knots = vec![Position::default(); len];
        let track = HashSet::<Position>::from_iter([*knots.last().unwrap()]);
        Self { knots, track }
    }

    fn mov(&mut self, dir: Direction) {
        self.knots[0].mov(dir);
        for i in 1..self.knots.len() {
            let front_knot = self.knots[i - 1];
            if !self.knots[i].trail(front_knot) {
                return;
            }
        }
        self.track.insert(*self.knots.last().unwrap());
    }

    fn apply_motions(&mut self, motions: &[(Direction, u32)]) {
        for &(dir, count) in motions {
            for _ in 0..count {
                self.mov(dir);
            }
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<(Direction, u32)>> {
    input
        .lines()
        .map(|line| {
            let (dir, count) = line
                .split_once(' ')
                .context("input line contains no space character")?;
            Ok((dir.parse()?, count.parse()?))
        })
        .collect()
}
