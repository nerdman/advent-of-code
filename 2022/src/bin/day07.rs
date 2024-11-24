use std::cmp;

use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./inputs/day07.txt").unwrap();

    let (total, smallest) = solution(input);
    println!("Part 1: {}", total);
    println!("Part 2: {}", smallest);

    Ok(())
}

fn solution(input: String) -> (usize, usize) {
    const TOTAL_DISK_SPACE: usize = 70_000_000;
    const REQUIRED_SPACE: usize = 30_000_000;
    let mut all_dirs: Vec<(&str, usize)> = vec![];

    let mut sub_100k_total: usize = 0;

    let mut path_stack = input.lines().fold(vec![("/", 0)], |mut path_stack, line| {
        if line == "$ cd /" || line == "$ ls" {
            return path_stack;
        }

        if line.starts_with("$ cd ") {
            let dir = &line[5..];
            if dir == ".." {
                let (name, size) = path_stack.pop().unwrap();

                if size <= 100_000 {
                    sub_100k_total += size;
                }
                all_dirs.push((name, size));
                path_stack.last_mut().unwrap().1 += size;
            } else {
                path_stack.push((dir, 0));
            }
            return path_stack;
        }

        let (amount, _) = line.split_once(" ").unwrap();

        if let Ok(amount) = amount.parse::<usize>() {
            path_stack.last_mut().unwrap().1 += amount;
        } else if amount == "dir" {
            // no-op
        }
        path_stack
    });

    while path_stack.len() > 0 {
        let (name, size) = path_stack.pop().unwrap();
        if size <= 100_000 {
            sub_100k_total += size;
        }

        if path_stack.len() > 0 {
            path_stack.last_mut().unwrap().1 += size;
        }

        all_dirs.push((name, size));
    }

    let unused_space = TOTAL_DISK_SPACE - all_dirs.last().unwrap().1;

    let mut smallest = usize::MAX;
    for (_name, size) in all_dirs {
        if size + unused_space >= REQUIRED_SPACE {
            smallest = cmp::min(smallest, size);
        }
    }

    (sub_100k_total, smallest)
}
