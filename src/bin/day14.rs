#![feature(array_windows)]

use anyhow::{Context, Error, Result};
use aoc2022::{parse_lines, read_input};
use std::{fmt::Display, str::FromStr};

const SOURCE_X: u32 = 500;

fn main() -> Result<()> {
    let input = read_input!()?;
    let paths = parse_lines(&input).collect::<Result<Vec<Path>>>()?;
    let mut map = Map::new(&paths)?;

    let (result1, count1) = map.trickle();
    if result1 != TrickleResult::Void {
        return Err(Error::msg("no sand reached the void"));
    }
    println!("{count1}");

    let y = map.height - 1;
    map.draw_rock_line(&[(map.left + 1, y), (map.left + map.width - 2, y)])?;
    let (result2, count2) = map.trickle();
    if result2 != TrickleResult::Blocked {
        return Err(Error::msg("the source did not become blocked"));
    }
    println!("{}", count1 + count2);

    Ok(())
}

#[derive(Clone)]
struct Map {
    tiles: Vec<Tile>,
    left: u32,
    width: u32,
    height: u32,
}

impl Map {
    fn new(paths: &[Path]) -> Result<Self> {
        let mut left = SOURCE_X;
        let mut right = SOURCE_X;
        let mut height = 0;
        for &(x, y) in paths.iter().flat_map(|p| &p.nodes) {
            left = left.min(x);
            right = right.max(x);
            height = height.max(y);
        }
        height += 3;
        if height > SOURCE_X {
            return Err(Error::msg(format!(
                "geometries with a height of more than {SOURCE_X} are not supported"
            )));
        }
        left = left.min(SOURCE_X - height);
        right = right.max(SOURCE_X + height);
        let width = right - left + 1;
        if width > 2 * SOURCE_X {
            return Err(Error::msg(format!(
                "geometries with a width of more than {} are not supported",
                2 * SOURCE_X
            )));
        }
        let mut map = Map {
            tiles: vec![Tile::Air; (width * height) as _],
            left,
            width,
            height,
        };
        for p in paths {
            for coords in p.nodes.array_windows() {
                map.draw_rock_line(coords)?;
            }
        }
        Ok(map)
    }

    fn draw_rock_line(&mut self, coords: &[(u32, u32); 2]) -> Result<()> {
        let &[(x0, y0), (x1, y1)] = coords;
        if x0 == x1 {
            for y in y0.min(y1)..=y0.max(y1) {
                self.set(x0, y, Tile::Rock);
            }
            Ok(())
        } else if y0 == y1 {
            for x in x0.min(x1)..=x0.max(x1) {
                self.set(x, y0, Tile::Rock);
            }
            Ok(())
        } else {
            Err(Error::msg(
                "only horizontal and vertical lines are supported",
            ))
        }
    }

    fn trickle_one(&mut self) -> TrickleResult {
        let mut x = SOURCE_X;
        let mut y = 0;
        if self.get(x, y) != Tile::Air {
            return TrickleResult::Blocked;
        }
        'outer: while y < self.height - 1 {
            let new_y = y + 1;
            for new_x in [x, x - 1, x + 1] {
                if self.get(new_x, new_y) == Tile::Air {
                    x = new_x;
                    y = new_y;
                    continue 'outer;
                }
            }
            self.set(x, y, Tile::Sand);
            return TrickleResult::Rest;
        }
        TrickleResult::Void
    }

    fn trickle(&mut self) -> (TrickleResult, u32) {
        let mut result;
        let mut count = 0;
        loop {
            result = self.trickle_one();
            match result {
                TrickleResult::Rest => count += 1,
                TrickleResult::Void => break,
                TrickleResult::Blocked => break,
            }
        }
        (result, count)
    }

    fn index(&self, x: u32, y: u32) -> usize {
        (y * self.width + (x - self.left)) as _
    }

    fn get(&self, x: u32, y: u32) -> Tile {
        self.tiles[self.index(x, y)]
    }

    fn set(&mut self, x: u32, y: u32, tile: Tile) {
        let index = self.index(x, y);
        self.tiles[index] = tile;
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.tiles.chunks(self.width as _) {
            for &tile in line {
                let c = match tile {
                    Tile::Air => ' ',
                    Tile::Rock => '█',
                    Tile::Sand => 'o',
                };
                c.fmt(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum TrickleResult {
    Rest,
    Void,
    Blocked,
}

struct Path {
    nodes: Vec<(u32, u32)>,
}

impl FromStr for Path {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let nodes = line
            .split(" -> ")
            .map(|s| {
                let (x, y) = s.split_once(',').context("invalid coordinates")?;
                Ok((x.parse()?, y.parse()?))
            })
            .collect::<Result<_>>()?;
        Ok(Self { nodes })
    }
}
