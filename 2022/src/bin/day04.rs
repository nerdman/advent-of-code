use std::ops::RangeInclusive;

use anyhow::Result;
use std::str::FromStr;

#[derive(Debug)]
struct Sections(RangeInclusive<usize>);

impl FromStr for Sections {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').unwrap();
        Ok(Sections(start.parse()?..=end.parse()?))
    }
}

impl Sections {
    fn fully_contains(&self, other: &Sections) -> bool {
        self.0.start() <= other.0.start() && self.0.end() >= other.0.end()
    }

    fn partially_contains(&self, other: &Sections) -> bool {
        self.0.contains(&other.0.start()) || self.0.contains(&other.0.end())
    }
}

fn part1(input: &String) -> Vec<(Sections, Sections)> {
    let pairs = input
        .lines()
        .filter_map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            let a: Sections = a.parse().unwrap();
            let b: Sections = b.parse().unwrap();
            if a.fully_contains(&b) || b.fully_contains(&a) {
                Some((a, b))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    pairs
}

fn part2(input: &String) -> Vec<(Sections, Sections)> {
    let pairs = input
        .lines()
        .filter_map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            let a: Sections = a.parse().unwrap();
            let b: Sections = b.parse().unwrap();
            if a.partially_contains(&b) || b.partially_contains(&a) {
                Some((a, b))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    pairs
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./inputs/day04.txt").unwrap();

    let part1 = part1(&input);

    println!("Part 1: {:?}", part1.len());

    let part2 = part2(&input);

    println!("Part 2: {:?}", part2.len());

    Ok(())
}
