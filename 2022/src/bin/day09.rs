use std::{collections::HashSet, str::FromStr};

use anyhow::Result;

#[allow(dead_code)]
const TEST_INPUT: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

#[allow(dead_code)]
const TEST_INPUT2: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

#[derive(Debug, Clone)]
pub struct Knot {
    pub x: i32,
    pub y: i32,
    pub history: HashSet<(i32, i32)>,
}

impl Knot {
    fn new() -> Self {
        let mut history = HashSet::new();
        history.insert((0, 0));
        Knot {
            x: 0,
            y: 0,
            history: history,
        }
    }
}
#[derive(Debug, PartialEq)]
struct Motion {
    dir: char,
    distance: usize,
}

impl FromStr for Motion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, distance) = s.split_once(" ").unwrap();
        let dir = dir.chars().next().unwrap();
        let distance = distance.parse::<usize>()?;
        Ok(Motion { dir, distance })
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./inputs/day09.txt").unwrap();
    let movements = input
        .lines()
        .map(|line| line.parse::<Motion>().unwrap())
        .collect::<Vec<_>>();

    let parts = solution(&movements);
    println!("Part 1: {:?}", parts.0);
    println!("Part 2: {:?}", parts.1);
    Ok(())
}

fn solution(movements: &Vec<Motion>) -> (usize, usize) {
    let part1_knots: Vec<Knot> = vec![Knot::new(), Knot::new()];

    let part2_knots: Vec<Knot> = vec![
        Knot::new(),
        Knot::new(),
        Knot::new(),
        Knot::new(),
        Knot::new(),
        Knot::new(),
        Knot::new(),
        Knot::new(),
        Knot::new(),
        Knot::new(),
    ];

    let part1 = move_knots(&movements, part1_knots);

    let part2 = move_knots(&movements, part2_knots);

    (part1, part2)
}

fn move_knots(movements: &Vec<Motion>, mut knots: Vec<Knot>) -> usize {
    for motion in movements {
        match motion.dir {
            'R' => {
                for _ in 0..motion.distance {
                    knots[0].x += 1; // head
                    for i in 0..knots.len() - 1 {
                        move_tail(i, i + 1, &mut knots);
                    }
                }
            }
            'L' => {
                for _ in 0..motion.distance {
                    knots[0].x -= 1; // head
                    for i in 0..knots.len() - 1 {
                        move_tail(i, i + 1, &mut knots);
                    }
                }
            }
            'U' => {
                for _ in 0..motion.distance {
                    knots[0].y += 1; // head
                    for i in 0..knots.len() - 1 {
                        move_tail(i, i + 1, &mut knots);
                    }
                }
            }
            'D' => {
                for _ in 0..motion.distance {
                    knots[0].y -= 1; // head
                    for i in 0..knots.len() - 1 {
                        move_tail(i, i + 1, &mut knots);
                    }
                }
            }
            _ => panic!("Unknown direction"),
        }
    }
    knots.last().unwrap().history.len()
}

fn move_tail(head: usize, tail: usize, knots: &mut Vec<Knot>) {
    if (knots[head].x - knots[tail].x).abs() > 1 {
        if knots[head].x > knots[tail].x {
            knots[tail].x += 1;

            if knots[head].y > knots[tail].y {
                knots[tail].y += 1;
            } else if knots[head].y < knots[tail].y {
                knots[tail].y -= 1;
            }

            let (x, y) = (knots[tail].x, knots[tail].y);
            knots[tail].history.insert((x, y));
        } else if knots[head].x < knots[tail].x {
            knots[tail].x -= 1;

            if knots[head].y > knots[tail].y {
                knots[tail].y += 1;
            } else if knots[head].y < knots[tail].y {
                knots[tail].y -= 1;
            }

            let (x, y) = (knots[tail].x, knots[tail].y);
            knots[tail].history.insert((x, y));
        }
    }

    if (knots[head].y - knots[tail].y).abs() > 1 {
        if knots[head].y > knots[tail].y {
            knots[tail].y += 1;

            if knots[head].x > knots[tail].x {
                knots[tail].x += 1;
            } else if knots[head].x < knots[tail].x {
                knots[tail].x -= 1;
            }

            let (x, y) = (knots[tail].x, knots[tail].y);
            knots[tail].history.insert((x, y));
        } else if knots[head].y < knots[tail].y {
            knots[tail].y -= 1;

            if knots[head].x > knots[tail].x {
                knots[tail].x += 1;
            } else if knots[head].x < knots[tail].x {
                knots[tail].x -= 1;
            }

            let (x, y) = (knots[tail].x, knots[tail].y);
            knots[tail].history.insert((x, y));
        }
    }
}
