use std::collections::HashMap;

use anyhow::Result;

const EMPTY: u8 = 0;
const FALLING: u8 = 1;
const STOPPED: u8 = 2;

#[derive(Debug)]
struct Rock {
    rows: usize,
    cols: usize,
    x: usize,
    y: usize,
    data: [[u8; 4]; 4],
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./inputs/day17.txt")?;
    let jets = input
        .lines()
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<char>>();
    // _
    let rock_one = Rock {
        rows: 1,
        cols: 4,
        data: [[1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        x: 0,
        y: 0,
    };

    // +
    let rock_two = Rock {
        rows: 3,
        cols: 3,
        data: [[0, 1, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
        x: 0,
        y: 0,
    };

    // J
    let rock_three = Rock {
        rows: 3,
        cols: 3,
        data: [[0, 0, 1, 0], [0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0]],
        x: 0,
        y: 0,
    };

    // I
    let rock_four = Rock {
        rows: 4,
        cols: 1,
        data: [[1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0]],
        x: 0,
        y: 0,
    };

    // o
    let rock_five = Rock {
        rows: 2,
        cols: 2,
        data: [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        x: 0,
        y: 0,
    };

    let mut rocks = vec![rock_one, rock_two, rock_three, rock_four, rock_five];

    let part1_highest = solution(&jets, &mut rocks, 2022u64);
    let part2_highest = solution(&jets, &mut rocks, 1_000_000_000_000u64);
    println!("Part 1: {}", part1_highest);
    println!("Part 2: {}", part2_highest);
    Ok(())
}

fn solution(jets: &Vec<char>, rocks: &mut Vec<Rock>, max_rocks: u64) -> u64 {
    let mut chamber: Vec<Vec<u8>> = vec![];
    let mut seen = HashMap::<(usize, usize, [u32; 7]), (u64, usize)>::new();

    let mut highest = 0;

    let jet_count = jets.len();
    let mut jet = 0;

    let rock_count = rocks.len();
    let mut r = 0u64;

    let mut added_highest = 0u64;
    while r < max_rocks {
        let rock_index = (r % rock_count as u64) as usize;
        let rock = &mut rocks[rock_index];

        add_rock(rock, &mut chamber, highest);

        loop {
            let can_move = move_rock(rock, jets[jet], &mut chamber, &mut highest);

            if !can_move {
                let top_view = get_topography(&chamber);
                if seen.contains_key(&(rock_index, jet, top_view)) {
                    let (old_r, old_highest) = seen.get(&(rock_index, jet, top_view)).unwrap();
                    let highest_delta = highest - old_highest;
                    let r_delta = r - old_r;
                    let skip_amount = (max_rocks - r) / r_delta;
                    added_highest += skip_amount * highest_delta as u64;
                    r += skip_amount * r_delta;
                }
                seen.insert((rock_index, jet, top_view), (r, highest));
            }
            jet = (jet + 1) % jet_count;
            if !can_move {
                break;
            }
        }
        r += 1;
    }

    added_highest + highest as u64
}

fn add_rock(rock: &mut Rock, chamber: &mut Vec<Vec<u8>>, highest: usize) {
    if chamber.len() == 0 {
        let d = rock.rows + 3;

        for _ in 0..d {
            chamber.push(vec![0; 7]);
        }
    } else if chamber.len() > 0 && chamber.len() - highest < rock.rows + 3 {
        let d = rock.rows + 3 - (chamber.len() - highest);

        for _ in 0..d {
            chamber.push(vec![0; 7]);
        }
    }

    rock.x = 2;
    // deal with zero index "highest" for first rock
    if highest == 0 {
        rock.y = (rock.rows + 3) - 1;
    } else {
        rock.y = (highest - 1) + (rock.rows + 3);
    }

    for i in 0..rock.rows {
        for j in 0..rock.cols {
            chamber[rock.y - i][rock.x + j] = rock.data[i][j];
        }
    }
}

fn move_rock(
    rock: &mut Rock,
    hot_gas: char,
    chamber: &mut Vec<Vec<u8>>,
    highest: &mut usize,
) -> bool {
    let mut new_x = rock.x;
    let mut new_y = rock.y;
    // gas tries to move rock < or >
    if hot_gas == '<' && rock.x > 0 {
        // move rock left
        new_x = rock.x - 1;
    } else if hot_gas == '>' && rock.x < chamber[0].len() - rock.cols {
        // move rock right
        new_x = rock.x + 1;
    }

    if new_x != rock.x && can_move(rock, chamber, new_x, new_y) {
        clear_rock(rock, chamber);
        do_move(rock, chamber, new_x, new_y);
    }

    // move rock down
    if rock.y - (rock.rows - 1) > 0 {
        new_y = rock.y - 1;
    }

    if new_y != rock.y && can_move(rock, chamber, rock.x, new_y) {
        clear_rock(rock, chamber);
        do_move(rock, chamber, rock.x, new_y);
    } else {
        // rock has stopped
        for i in 0..rock.rows {
            for j in 0..rock.cols {
                if rock.data[i][j] == FALLING {
                    chamber[rock.y - i][rock.x + j] = STOPPED;
                }
            }
        }
        *highest = std::cmp::max(*highest, rock.y + 1);
        return false;
    }
    true
}

fn can_move(rock: &Rock, chamber: &Vec<Vec<u8>>, to_x: usize, to_y: usize) -> bool {
    for y in 0..rock.rows {
        for x in 0..rock.cols {
            if rock.data[y][x] == FALLING && chamber[to_y - y][to_x + x] == STOPPED {
                return false;
            }
        }
    }
    true
}

// Find the unique signature of the topography of the chamber to find repeating patterns
fn get_topography(chamber: &Vec<Vec<u8>>) -> [u32; 7] {
    let mut topography = [0u32; 7];
    for x in 0..chamber[0].len() {
        for y in (0..chamber.len()).rev() {
            if chamber[y][x] == STOPPED {
                topography[x] = (chamber.len() - y) as u32;
                break;
            }
        }
    }
    topography
}

fn do_move(rock: &mut Rock, chamber: &mut Vec<Vec<u8>>, to_x: usize, to_y: usize) {
    for y in 0..rock.rows {
        for x in 0..rock.cols {
            if rock.data[y][x] == FALLING {
                chamber[to_y - y][to_x + x] = FALLING;
            }
        }
    }
    rock.x = to_x;
    rock.y = to_y;
}

fn clear_rock(rock: &Rock, chamber: &mut Vec<Vec<u8>>) {
    for y in 0..rock.rows {
        for x in 0..rock.cols {
            if chamber[rock.y - y][rock.x + x] == FALLING {
                chamber[rock.y - y][rock.x + x] = EMPTY;
            }
        }
    }
}

#[allow(dead_code)]
const TEST_INPUT: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;
