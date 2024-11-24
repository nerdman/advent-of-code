use anyhow::Result;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/*
Only needed to implement ordering for the BinaryHeap, since tuples already have Ord implemented.
Could just use a regular tuple and the std::cmp::Reverse() function, but why not do extra work..
*/
#[derive(Eq)]
struct Loc(i32, (usize, usize));

impl Ord for Loc {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Loc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Loc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./inputs/day12.txt").unwrap();

    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut heightmap = vec![];

    for (row, line) in input.lines().enumerate() {
        heightmap.push(vec![]);
        for (col, c) in line.chars().enumerate() {
            // Convert chars to 0-25, with 'S' = 0 and 'E' = 25
            if c == 'S' {
                start = (row, col);
                heightmap[row].push('a' as i32 - 'a' as i32);
            } else if c == 'E' {
                end = (row, col);
                heightmap[row].push('z' as i32 - 'a' as i32);
            } else {
                heightmap[row].push(c as i32 - 'a' as i32);
            }
        }
    }

    let part1 = dijkstras(start, end, &heightmap, true);
    println!("Part 1: {}", part1);
    let part2 = dijkstras(start, end, &heightmap, false);
    println!("Part 2: {}", part2);
    Ok(())
}

fn neighbors(i: usize, j: usize, heightmap: &Vec<Vec<i32>>, is_part1: bool) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];

    for (di, dj) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)].into_iter() {
        let ni: i32 = i as i32 + di;
        let nj: i32 = j as i32 + dj;

        if ni < 0 || ni >= heightmap.len() as i32 || nj < 0 || nj >= heightmap[0].len() as i32 {
            continue;
        }

        // For part 1, we go from S -> E location.
        // For part 2 we go in reverse from E down to all locations with an elevation of 'a'
        if is_part1 {
            if heightmap[ni as usize][nj as usize] <= heightmap[i][j] + 1 {
                neighbors.push((ni as usize, nj as usize));
            }
        } else {
            if heightmap[ni as usize][nj as usize] >= heightmap[i][j] - 1 {
                neighbors.push((ni as usize, nj as usize));
            }
        }
    }
    neighbors
}

fn dijkstras(
    start: (usize, usize),
    end: (usize, usize),
    heightmap: &Vec<Vec<i32>>,
    is_part1: bool,
) -> i32 {
    let mut visited = vec![vec![false; heightmap[0].len()]; heightmap.len()];
    let mut pq = BinaryHeap::new();

    if is_part1 {
        pq.push(Loc(0, start));
    } else {
        pq.push(Loc(0, end));
    }

    while let Some(Loc(step_cnt, (i, j))) = pq.pop() {
        if visited[i][j] {
            continue;
        }
        visited[i][j] = true;

        if is_part1 {
            if (i, j) == end {
                return step_cnt;
            }
        } else {
            // for part 2 we want any location with an elevation of 'a' (0)
            if heightmap[i][j] == 0 {
                return step_cnt;
            }
        }
        for (ni, nj) in neighbors(i, j, heightmap, is_part1) {
            pq.push(Loc(step_cnt + 1, (ni, nj)));
        }
    }

    0
}

#[allow(dead_code)]
const TEST_INPUT: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;
