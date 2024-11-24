use anyhow::Result;

use std::{collections::VecDeque, str::FromStr};

#[derive(Debug, Default, Clone)]
struct Operation {
    op: char,
    value: Option<u64>,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Operation, Self::Err> {
        let (op, value) = s.split_once(" ").unwrap();
        Ok(Operation {
            op: op.chars().next().unwrap(),
            value: match value.parse::<u64>() {
                Ok(v) => Some(v),
                Err(_) => None,
            },
        })
    }
}

#[derive(Debug, Default, Clone)]
struct Test {
    divisor: u32,
    true_target: usize,
    false_target: usize,
}

#[derive(Debug, Default, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    inspected: u64,
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Monkey, Self::Err> {
        let mut monkey = Monkey::default();

        for line in s.lines() {
            if line.starts_with("  Starting items: ") {
                monkey.items = line[18..]
                    .split(", ")
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect();
            }
            if line.starts_with("  Operation: new = old ") {
                monkey.operation = line[23..].parse().unwrap();
            }
            if line.starts_with("  Test: divisible by ") {
                monkey.test.divisor = line[21..].parse::<u32>().unwrap();
            }
            if line.starts_with("    If true: throw to monkey ") {
                monkey.test.true_target = line[29..].parse::<usize>().unwrap();
            }
            if line.starts_with("    If false: throw to monkey ") {
                monkey.test.false_target = line[30..].parse::<usize>().unwrap()
            }
        }

        Ok(monkey)
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./inputs/day11.txt").unwrap();
    let part1_monkeys = input
        .split("\n\n")
        .map(|monkey| monkey.parse::<Monkey>().unwrap())
        .collect::<Vec<_>>();
    let part2_monkeys = part1_monkeys.clone();

    let part1 = solution(part1_monkeys, 20, true);
    let part2 = solution(part2_monkeys, 10_000, false);
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
    Ok(())
}

fn solution(mut monkeys: Vec<Monkey>, rounds: u32, worry_divisor: bool) -> u64 {
    // Calculate the least common multiple of the test divisors or worry numbers go kaboom
    let mut lcm = 1;
    for i in 0..monkeys.len() {
        lcm = (lcm * monkeys[i].test.divisor) / num::integer::gcd(lcm, monkeys[i].test.divisor);
    }

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let mut item = monkeys[i].items.pop_front().unwrap();
                monkeys[i].inspected += 1;
                let worry = match monkeys[i].operation.value {
                    Some(v) => v,
                    None => item,
                };

                let new_item = match monkeys[i].operation.op {
                    '+' => item + worry,
                    '*' => item * worry,
                    _ => panic!("unknown operation"),
                };

                if worry_divisor {
                    item = new_item / 3;
                } else {
                    item = new_item % lcm as u64;
                }

                let to_monkey = if item % monkeys[i].test.divisor as u64 == 0 {
                    monkeys[i].test.true_target
                } else {
                    monkeys[i].test.false_target
                };

                monkeys[to_monkey].items.push_back(item);
            }
        }
    }

    let mut activity = monkeys
        .iter()
        .map(|monkey| monkey.inspected)
        .collect::<Vec<_>>();
    activity.sort_by(|a, b| b.cmp(a));

    activity[0] * activity[1]
}

#[allow(dead_code)]
const TEST_INPUT: &str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;
