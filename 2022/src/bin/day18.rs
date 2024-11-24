use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./inputs/day18.txt")?;
    let cubes = input
        .lines()
        .map(|line| {
            let mut parts = line.split(",");
            let x = parts.next().unwrap().parse::<i32>().unwrap();
            let y = parts.next().unwrap().parse::<i32>().unwrap();
            let z = parts.next().unwrap().parse::<i32>().unwrap();
            (x, y, z)
        })
        .collect::<HashSet<_>>();

    let open_sides = part_one(&cubes);

    println!("Part 1: {}", open_sides);

    let part2_sides = part_two(&cubes);
    println!("Part 2: {}", part2_sides);

    Ok(())
}

fn part_one(cubes: &HashSet<(i32, i32, i32)>) -> i32 {
    let mut open_sides = 0;
    for cube in cubes.iter() {
        if !cubes.contains(&(cube.0 + 1, cube.1, cube.2)) {
            open_sides += 1;
        }

        if !cubes.contains(&(cube.0 - 1, cube.1, cube.2)) {
            open_sides += 1;
        }

        if !cubes.contains(&(cube.0, cube.1 + 1, cube.2)) {
            open_sides += 1;
        }

        if !cubes.contains(&(cube.0, cube.1 - 1, cube.2)) {
            open_sides += 1;
        }

        if !cubes.contains(&(cube.0, cube.1, cube.2 + 1)) {
            open_sides += 1;
        }

        if !cubes.contains(&(cube.0, cube.1, cube.2 - 1)) {
            open_sides += 1;
        }
    }
    open_sides
}

fn can_get_out(
    x: i32,
    y: i32,
    z: i32,
    cubes: &HashSet<(i32, i32, i32)>,
    min: &(i32, i32, i32),
    max: &(i32, i32, i32),
) -> bool {
    let mut queue = VecDeque::new();

    queue.push_back((x, y, z));
    let mut seen = HashSet::new();
    while !queue.is_empty() {
        let (x, y, z) = queue.pop_front().unwrap();
        if seen.contains(&(x, y, z)) {
            continue;
        }
        seen.insert((x, y, z));

        if cubes.contains(&(x, y, z)) {
            continue;
        }

        if (x > max.0 || x < min.0) || (y > max.1 || y < min.1) || (z > max.2 || z < min.2) {
            return true;
        }

        for (dx, dy, dz) in &[
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            queue.push_back((x + dx, y + dy, z + dz));
        }
    }

    false
}

fn part_two(cubes: &HashSet<(i32, i32, i32)>) -> i32 {
    let mut min_x = 0i32;
    let mut max_x = 0i32;
    let mut min_y = 0i32;
    let mut max_y = 0i32;
    let mut min_z = 0i32;
    let mut max_z = 0i32;
    for cube in cubes.iter() {
        min_x = min_x.min(cube.0);
        max_x = max_x.max(cube.0);
        min_y = min_y.min(cube.1);
        max_y = max_y.max(cube.1);
        min_z = min_z.min(cube.2);
        max_z = max_z.max(cube.2);
    }
    let mut cache = HashMap::new();
    let mut open_sides = 0;
    for cube in cubes.iter() {
        for (dx, dy, dz) in &[
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            let key = (cube.0 + dx, cube.1 + dy, cube.2 + dz);
            let can_escape = cache.entry(key).or_insert_with(|| {
                can_get_out(
                    cube.0 + dx,
                    cube.1 + dy,
                    cube.2 + dz,
                    cubes,
                    &(min_x, min_y, min_z),
                    &(max_x, max_y, max_z),
                )
            });

            if *can_escape {
                open_sides += 1;
            }
        }
    }

    open_sides
}

#[allow(dead_code)]
const TEST_INPUT: &str = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;
