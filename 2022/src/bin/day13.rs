use std::cmp::Ordering;

use anyhow::Result;
use serde_json::{json, Value};

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./inputs/day13.txt")?;

    println!("Part 1: {:?}", part1(&input).unwrap());
    println!("Part 2: {:?}", part2(&input).unwrap());

    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let packet_pairs = input
        .split("\n\n")
        .map(|line| {
            let (left, right) = line.split_once("\n").unwrap_or_default();
            (left, right)
        })
        .collect::<Vec<_>>();

    let mut sum_of_inorder = 0;
    for (i, packet) in packet_pairs.iter().enumerate() {
        let left: Value = serde_json::from_str(packet.0).expect("invalid json");
        let right: Value = serde_json::from_str(packet.1).expect("invalid json");

        if compare(left, right) == Ordering::Less {
            sum_of_inorder += i + 1;
        }
    }
    Ok(sum_of_inorder)
}

fn part2(input: &str) -> Result<usize> {
    let mut packets: Vec<Value> = vec![];

    for line in input.lines() {
        if !line.is_empty() {
            packets.push(serde_json::from_str(line).expect("invalid json"));
        }
    }
    // Add the special dividers
    let divider_one: Value = serde_json::from_str("[[2]]").expect("invalid json");
    let divider_two: Value = serde_json::from_str("[[6]]").expect("invalid json");
    packets.push(divider_one.clone());
    packets.push(divider_two.clone());

    packets.sort_by(|left, right| compare(left.clone(), right.clone()));

    let mut decoder_key = 1;
    for (i, packet) in packets.iter().enumerate() {
        if packet == &divider_one || packet == &divider_two {
            decoder_key *= i + 1;
        }
    }
    Ok(decoder_key)
}

fn compare(mut left: Value, mut right: Value) -> std::cmp::Ordering {
    if left.is_array() && right.is_number() {
        right = json!([&right]);
    }

    if left.is_number() && right.is_array() {
        left = json!([&left]);
    }

    if left.is_number() && right.is_number() {
        if left.as_u64().unwrap() < right.as_u64().unwrap() {
            return Ordering::Less;
        } else if left.as_u64().unwrap() == right.as_u64().unwrap() {
            return Ordering::Equal;
        }

        return Ordering::Greater;
    }

    if left.is_array() && right.is_array() {
        let mut i = 0;

        let l_len = left.as_array().unwrap().len();
        let r_len = right.as_array().unwrap().len();
        while i < l_len && i < r_len {
            let x = compare(left[i].clone(), right[i].clone());

            if x != Ordering::Equal {
                return x;
            }

            i += 1;
        }
        if i == l_len {
            if l_len == r_len {
                return Ordering::Equal;
            }
            return Ordering::Less;
        }
        return Ordering::Greater;
    }
    Ordering::Greater
}

#[allow(dead_code)]
const TEST_INPUT: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;
