use std::collections::HashMap;

use anyhow::Result;

#[derive(Debug, Default)]
struct Valve {
    label: String,
    rate: u32,
    neighbors: Vec<String>,
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./inputs/day16.txt")?;

    println!("Part 1: {}", part(&input.to_string(), 30, false));
    println!("Part 2: {}", part(&input.to_string(), 26, true));

    Ok(())
}

fn part(input: &String, time: u32, elephant: bool) -> u32 {
    let valves = input
        .lines()
        .map(|line| {
            let mut parts = line.split(" ");
            let label = parts.nth(1).unwrap().to_string();
            let rate = parts
                .nth(2)
                .unwrap()
                .split_once(";")
                .unwrap()
                .0
                .split_once("=")
                .unwrap()
                .1
                .parse::<u32>()
                .unwrap();

            let neighbors = parts
                .skip(4)
                .map(|s| s.to_string().replace(",", ""))
                .collect::<Vec<_>>();

            (
                label.clone(),
                Valve {
                    label,
                    rate,
                    neighbors,
                },
            )
        })
        .collect::<HashMap<String, Valve>>();

    // Create an hashmap of valve indices for only the valves that have a non-zero flow rate.
    // These indices are used for tracking which valves are open via a bitmask.
    let bitmask_index = valves
        .values()
        .filter_map(|v| {
            if v.label != "AA" && v.rate == 0 {
                None
            } else {
                Some(v.label.clone())
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v, i as u32))
        .collect::<HashMap<String, u32>>();

    println!("Mask index: {:?}", bitmask_index);

    let opened_bitmask = 0;
    let mut cache = HashMap::<(String, u32, u32, bool), u32>::new();

    solve(
        valves.get("AA").unwrap(),
        time,
        opened_bitmask,
        &mut cache,
        &valves,
        &bitmask_index,
        elephant,
    )
}

fn solve(
    valve: &Valve,
    time: u32,
    opened_bitmask: u32,
    cache: &mut HashMap<(String, u32, u32, bool), u32>,
    valves: &HashMap<String, Valve>,
    bitmask_index: &HashMap<String, u32>,
    elephant: bool,
) -> u32 {
    if time == 0 {
        if elephant {
            return solve(
                valves.get("AA").unwrap(),
                26,
                opened_bitmask,
                cache,
                valves,
                bitmask_index,
                false,
            );
        }
        return 0;
    }

    if let Some(&cached) = cache.get(&(valve.label.clone(), time, opened_bitmask, elephant)) {
        return cached;
    }

    let scores = valve
        .neighbors
        .iter()
        .map(|neighbor| {
            let neighbor_valve = valves.get(neighbor).unwrap();
            solve(
                neighbor_valve,
                time - 1,
                opened_bitmask,
                cache,
                valves,
                bitmask_index,
                elephant,
            )
        })
        .collect::<Vec<u32>>();

    let mut score = *scores.iter().max().unwrap();

    let bit = 1 << *bitmask_index.get(&valve.label).unwrap_or(&0);

    if valve.rate > 0 && (opened_bitmask & bit) == 0 {
        score = score.max(
            (time - 1) * valve.rate
                + solve(
                    valve,
                    time - 1,
                    opened_bitmask | bit,
                    cache,
                    valves,
                    bitmask_index,
                    elephant,
                ),
        );
    }

    cache.insert((valve.label.clone(), time, opened_bitmask, elephant), score);

    score
}

#[allow(dead_code)]
const TEST_INPUT: &str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;
