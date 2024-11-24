use anyhow::Result;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref PLAYS_PART1: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();

        m.insert("A X", 4u32);
        m.insert("A Y", 8u32);
        m.insert("A Z", 3u32);
        m.insert("B X", 1u32);
        m.insert("B Y", 5u32);
        m.insert("B Z", 9u32);
        m.insert("C X", 7u32);
        m.insert("C Y", 2u32);
        m.insert("C Z", 6u32);
        m
    };
    static ref PLAYS_PART2: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();

        m.insert("A X", 3u32);
        m.insert("A Y", 4u32);
        m.insert("A Z", 8u32);
        m.insert("B X", 1u32);
        m.insert("B Y", 5u32);
        m.insert("B Z", 9u32);
        m.insert("C X", 2u32);
        m.insert("C Y", 6u32);
        m.insert("C Z", 7u32);
        m
    };
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./inputs/day02.txt").unwrap();
    let score = input.lines().flat_map(|x| PLAYS_PART1.get(x)).sum::<u32>();
    println!("Part 1: {}", score);

    let score = input.lines().flat_map(|x| PLAYS_PART2.get(x)).sum::<u32>();
    println!("Part 2: {}", score);
    Ok(())
}
