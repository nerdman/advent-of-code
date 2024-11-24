use std::collections::HashSet;

use anyhow::Result;

pub fn to_priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./inputs/day03.txt")?;
    let rucksacks = input.lines().collect();

    println!("Part 1: {:?}", part1(&rucksacks));

    println!("Part 2: {:?}", part2(&rucksacks));

    Ok(())
}

fn part1(rucksacks: &Vec<&str>) -> u32 {
    let priority_sums = rucksacks
        .iter()
        .map(|rucksack| {
            let half = rucksack.len() / 2;
            let (a, b) = rucksack.split_at(half);
            let a = a.chars().collect::<HashSet<_>>();
            b.chars()
                .find_map(|c| {
                    if a.contains(&c) {
                        Some(to_priority(c))
                    } else {
                        None
                    }
                })
                .unwrap_or(0)
        })
        .sum::<u32>();
    priority_sums
}

fn part2(rucksacks: &Vec<&str>) -> u32 {
    let badge_sums = rucksacks
        .chunks(3)
        .map(|group| find_group_badge(group))
        .sum::<u32>();

    badge_sums
}

fn find_group_badge(group: &[&str]) -> u32 {
    let a = group[0].chars().collect::<HashSet<_>>();
    let b = group[1].chars().collect::<HashSet<_>>();
    let badge = group[2]
        .chars()
        .find_map(|c| {
            if a.contains(&c) && b.contains(&c) {
                Some(to_priority(c))
            } else {
                None
            }
        })
        .unwrap();

    badge
}
