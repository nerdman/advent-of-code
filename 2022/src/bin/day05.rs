use std::{collections::VecDeque, str::FromStr};

use anyhow::Result;

#[derive(Debug)]
struct Procedure {
    num_move: usize,
    from: usize,
    to: usize,
}

const MOVE_INDEX: usize = 1;
const FROM_INDEX: usize = 3;
const TO_INDEX: usize = 5;

impl FromStr for Procedure {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        let num_move = parts[MOVE_INDEX].parse()?;
        let from = parts[FROM_INDEX].parse::<usize>()? - 1;
        let to = parts[TO_INDEX].parse::<usize>()? - 1;

        Ok(Procedure { num_move, from, to })
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./inputs/day05.txt").unwrap();

    let (starting_stacks, procedures) = input.split_once("\n\n").unwrap();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    for (i, level) in starting_stacks.lines().rev().enumerate() {
        if i == 0 {
            level
                .split_ascii_whitespace()
                .for_each(|_| stacks.push(Vec::new()));
        } else {
            level.chars().enumerate().for_each(|(i, c)| {
                if c.is_ascii_alphabetic() {
                    stacks[i / 4].push(c);
                }
            });
        }
    }

    // Save starting crate stacks for use in part 2
    let mut part2_stacks = stacks.clone();

    procedures.lines().for_each(|line| {
        let procedure = line.parse::<Procedure>().unwrap();
        for _ in 0..procedure.num_move {
            let from = stacks[procedure.from].pop().unwrap();
            stacks[procedure.to].push(from);
        }
    });

    let part1 = stacks
        .iter()
        .peekable()
        .flat_map(|stack| stack.last())
        .collect::<String>();

    println!("Part 1: {}", part1);

    // Part 2

    procedures.lines().for_each(|line| {
        let procedure = line.parse::<Procedure>().unwrap();
        let mut temp = VecDeque::<char>::new();
        for _ in 0..procedure.num_move {
            let from = part2_stacks[procedure.from].pop().unwrap();
            temp.push_front(from);
        }
        part2_stacks[procedure.to].append(&mut temp.into());
    });

    let part2 = part2_stacks
        .iter()
        .peekable()
        .flat_map(|stack| stack.last())
        .collect::<String>();

    println!("Part 2: {}", part2);

    Ok(())
}
