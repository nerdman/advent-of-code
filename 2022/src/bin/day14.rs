use anyhow::Result;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
struct Sand {
    x: usize,
    y: usize,
    at_rest: bool,
}

impl Sand {
    fn new(x: usize, y: usize) -> Self {
        Sand {
            x,
            y,
            at_rest: false,
        }
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./inputs/day14.txt")?;

    let mut max_y: usize = usize::MIN;
    let rock_paths = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let mut coords = point
                        .split(',')
                        .map(|coord| coord.parse::<usize>().unwrap());

                    let x = coords.next().unwrap();
                    let y = coords.next().unwrap();

                    if y > max_y {
                        max_y = y;
                    }
                    (x, y)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut cave = HashSet::<(usize, usize)>::new();

    for rock_path in rock_paths.iter() {
        if rock_path.len() == 1 {
            let (x, y) = rock_path[0];
            cave.insert((x, y));
            continue;
        }

        for i in 1..rock_path.len() {
            let end = rock_path[i];
            let start = rock_path[i - 1];
            add_rocks(&mut cave, start, end);
        }
    }

    let cave_part2 = cave.clone();

    let sand_count_p1 = part1(max_y, cave);
    println!("====================");
    let sand_count_p2 = part2(max_y, cave_part2);
    println!("Part 1: {}", sand_count_p1);
    println!("Part 2: {}", sand_count_p2);
    Ok(())
}

fn part1(max_y: usize, mut cave: HashSet<(usize, usize)>) -> i32 {
    let mut sand_count = 0;

    while simulate_sand(max_y, &mut cave) {
        sand_count += 1;
    }
    sand_count
}

fn part2(max_y: usize, mut cave: HashSet<(usize, usize)>) -> i32 {
    let mut sand_count = 0;

    loop {
        let sand = simulate_sand2(max_y, &mut cave);
        println!("Adding sand at ({},{})", sand.x, sand.y);
        cave.insert((sand.x, sand.y));

        sand_count += 1;

        if sand.x == 500 && sand.y == 0 {
            break;
        }
    }

    draw_cave(max_y, &cave);
    sand_count
}

fn simulate_sand(max_y: usize, cave: &mut HashSet<(usize, usize)>) -> bool {
    println!("max_y: {}", max_y);

    let mut sand = Sand::new(500, 0);

    while sand.y < max_y {
        // check down (0, +1)
        if !cave.contains(&(sand.x, sand.y + 1)) {
            sand.y += 1;
            continue;
        }

        // check left (-1, +1)
        if !cave.contains(&(sand.x - 1, sand.y + 1)) {
            sand.x -= 1;
            sand.y += 1;
            continue;
        }

        // check right (+1, +1)
        if !cave.contains(&(sand.x + 1, sand.y + 1)) {
            sand.x += 1;
            sand.y += 1;
            continue;
        }

        cave.insert((sand.x, sand.y));
        return true;
    }
    false
}

fn simulate_sand2(max_y: usize, cave: &mut HashSet<(usize, usize)>) -> Sand {
    println!("max_y: {}", max_y);
    let mut sand = Sand::new(500, 0);

    if cave.contains(&(sand.x, sand.y)) {
        return sand;
    }

    while sand.y <= max_y {
        // check down (0, +1)
        if !cave.contains(&(sand.x, sand.y + 1)) {
            sand.y += 1;
            continue;
        }

        // check left (-1, +1)
        if !cave.contains(&(sand.x - 1, sand.y + 1)) {
            sand.x -= 1;
            sand.y += 1;
            continue;
        }
        // check right (+1, +1)
        if !cave.contains(&(sand.x + 1, sand.y + 1)) {
            sand.x += 1;
            sand.y += 1;
            continue;
        }
        break;
    }
    sand
}

fn add_rocks(cave: &mut HashSet<(usize, usize)>, start: (usize, usize), end: (usize, usize)) {
    if start.0 != end.0 {
        // draw horizontal wall
        for x in std::cmp::min(start.0, end.0)..=std::cmp::max(start.0, end.0) {
            cave.insert((x, start.1));
        }
    } else if start.1 != end.1 {
        // draw vertical wall
        for y in std::cmp::min(start.1, end.1)..=std::cmp::max(start.1, end.1) {
            cave.insert((start.0, y));
        }
    }
}

fn draw_cave(max_y: usize, cave: &HashSet<(usize, usize)>) {
    for y in 0..=max_y {
        for x in 0..=600 {
            if cave.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[allow(dead_code)]
const TEST_INPUT: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
