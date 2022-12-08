use anyhow::Result;
use aoc2022::{read_input, Grid};

fn main() -> Result<()> {
    let input = read_input!()?;
    let mut trees = Trees::from_input(&input)?;
    trees.mark_visible();
    println!("{}", trees.count_visible());
    println!("{}", trees.max_scenic_score());
    Ok(())
}

struct Trees {
    heights: Grid<u8>,
    visible: Grid<bool>,
}

impl Trees {
    fn from_input(input: &str) -> Result<Self> {
        let heights = Grid::from_lines(input.lines(), |line| {
            line.as_bytes().iter().cloned().map(Ok)
        })?;
        let visible = Grid::new(heights.width(), heights.height());
        Ok(Self { heights, visible })
    }

    fn mark_visible_line(&mut self, mut pos: usize, stride: isize, count: usize) {
        let mut height = 0;
        for _ in 0..count {
            if self.heights[pos] > height {
                self.visible[pos] = true;
                height = self.heights[pos];
            }
            pos = (pos as isize + stride) as usize;
        }
    }

    fn mark_visible(&mut self) {
        let len = self.heights.len();
        let width = self.heights.width();
        let height = self.heights.height();
        for x in 0..width {
            self.mark_visible_line(x, width as _, height);
            self.mark_visible_line(len - 1 - x, -(width as isize), height);
        }
        for y in 0..height {
            self.mark_visible_line(y * width, 1, width);
            self.mark_visible_line(len - 1 - y * width, -1, width);
        }
    }

    fn count_visible(&self) -> usize {
        self.visible.iter().filter(|&&v| v).count()
    }

    fn viewing_distance(&self, mut pos: usize, stride: isize, max: usize) -> usize {
        let mut count = 0;
        let height = self.heights[pos];
        while count < max {
            pos = (pos as isize + stride) as usize;
            count += 1;
            if height <= self.heights[pos] {
                break;
            }
        }
        count
    }

    fn scenic_score(&self, pos: usize) -> usize {
        let width = self.heights.width();
        let height = self.heights.height();
        let (x, y) = (pos % width, pos / width);
        self.viewing_distance(pos, 1, width - 1 - x)
            * self.viewing_distance(pos, -1, x)
            * self.viewing_distance(pos, width as _, height - 1 - y)
            * self.viewing_distance(pos, -(width as isize), y)
    }

    fn max_scenic_score(&self) -> usize {
        (0..self.heights.len())
            .map(|pos| self.scenic_score(pos))
            .max()
            .unwrap_or_default()
    }
}
