#![feature(let_chains, map_many_mut)]

use anyhow::{Context, Error, Result};
use aoc2022::{parse_lines, read_input};
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::{
    collections::HashMap,
    str::{from_utf8, FromStr},
};

fn main() -> Result<()> {
    let (mut stacks, steps) = parse_input(&read_input!()?)?;
    let tops = stacks.clone().apply_all(&steps, CrateMover9000)?;
    println!("{}", from_utf8(&tops)?);
    let tops = stacks.apply_all(&steps, CrateMover9001)?;
    println!("{}", from_utf8(&tops)?);
    Ok(())
}

#[derive(Clone)]
struct Stacks {
    labels: Vec<u8>,
    crates: HashMap<u8, Vec<u8>>,
}

#[derive(Clone, Copy)]
enum CraneModel {
    CrateMover9000,
    CrateMover9001,
}

use CraneModel::*;

impl Stacks {
    fn apply(&mut self, step: &Step, model: CraneModel) -> Result<()> {
        let [from, to] = self
            .crates
            .get_many_mut([&step.from, &step.to])
            .context("invalid source or destination stack")?;
        let i = from
            .len()
            .checked_sub(step.count)
            .context("not enough crates on source stack")?;
        match model {
            CrateMover9000 => to.extend(from.drain(i..).rev()),
            CrateMover9001 => to.extend(from.drain(i..)),
        }
        Ok(())
    }

    fn apply_all(&mut self, steps: &[Step], model: CraneModel) -> Result<Vec<u8>> {
        for step in steps {
            self.apply(step, model)?;
        }
        Ok(self
            .labels
            .iter()
            .flat_map(|label| self.crates[label].last().cloned())
            .collect())
    }
}

impl FromStr for Stacks {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().rev();
        let stacks: Vec<(usize, u8)> = lines
            .next()
            .context("empty stack spec")?
            .bytes()
            .enumerate()
            .filter(|&(_, label)| label != b' ')
            .collect();
        let mut crates: HashMap<_, _> = stacks.iter().map(|&(_, label)| (label, vec![])).collect();
        for line in lines {
            let line = line.as_bytes();
            for &(i, label) in &stacks {
                if let Some(&c) = line.get(i) && c != b' ' {
                    crates.get_mut(&label).unwrap().push(c);
                }
            }
        }
        let labels = stacks.iter().map(|&(_, label)| label).collect();
        Ok(Self { labels, crates })
    }
}

struct Step {
    count: usize,
    from: u8,
    to: u8,
}

impl FromStr for Step {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (.) to (.)").unwrap();
        }
        let cap = RE
            .captures(s.as_bytes())
            .context("invalid crane instruction")?;
        Ok(Step {
            count: from_utf8(&cap[1]).unwrap().parse()?,
            from: cap[2][0],
            to: cap[3][0],
        })
    }
}

fn parse_input(input: &str) -> Result<(Stacks, Vec<Step>)> {
    let (stacks, steps) = input
        .split_once("\n\n")
        .context("input must contain empty line separating stacks and steps")?;
    Ok((stacks.parse()?, parse_lines(steps).collect::<Result<_>>()?))
}
