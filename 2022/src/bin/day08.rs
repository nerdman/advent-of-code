use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./inputs/day08.txt").unwrap();
    let trees = input.lines().fold(vec![], |mut trees, line| {
        trees.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>(),
        );
        trees
    });

    let width = trees[0].len();
    let height = trees.len();

    let mut visible_trees = 0;
    let mut max_scenic_score = 0;

    for row in 0..height {
        for col in 0..width {
            if row == 0 || row == height - 1 || col == 0 || col == width - 1 {
                visible_trees += 1;
                continue;
            }

            let (north_visible, north_trees) = check_north(&trees, row, col);

            let (south_visible, south_trees) = check_south(&trees, row, col);

            let (east_visible, east_trees) = check_east(&trees, row, col);

            let (west_visible, west_trees) = check_west(&trees, row, col);

            let scenic_score = north_trees * south_trees * east_trees * west_trees;
            max_scenic_score = std::cmp::max(max_scenic_score, scenic_score);

            if north_visible || south_visible || east_visible || west_visible {
                visible_trees += 1;
            }
        }
    }
    println!("Part 1: {}", visible_trees);
    println!("Part 2: {}", max_scenic_score);

    Ok(())
}

fn check_north(trees: &Vec<Vec<u32>>, row: usize, col: usize) -> (bool, usize) {
    let mut north = row.checked_sub(1).unwrap();

    let mut visible_trees = 0;
    loop {
        visible_trees += 1;

        if trees[north][col] >= trees[row][col] {
            return (false, visible_trees);
        }
        north = match north.checked_sub(1) {
            Some(north) => north,
            None => break,
        };
    }
    (true, visible_trees)
}

fn check_south(trees: &Vec<Vec<u32>>, row: usize, col: usize) -> (bool, usize) {
    let mut visible_trees = 0;
    let mut south = row + 1;
    while south <= trees.len() - 1 {
        visible_trees += 1;

        if trees[south][col] >= trees[row][col] {
            return (false, visible_trees);
        }
        south += 1;
    }
    (true, visible_trees)
}

fn check_east(trees: &Vec<Vec<u32>>, row: usize, col: usize) -> (bool, usize) {
    let mut visible_trees = 0;
    let mut east = col + 1;
    while east <= trees[0].len() - 1 {
        visible_trees += 1;

        if trees[row][east] >= trees[row][col] {
            return (false, visible_trees);
        }
        east += 1;
    }
    (true, visible_trees)
}

fn check_west(trees: &Vec<Vec<u32>>, row: usize, col: usize) -> (bool, usize) {
    let mut visible_trees = 0;
    let mut west = col.checked_sub(1).unwrap();
    loop {
        visible_trees += 1;

        if trees[row][west] >= trees[row][col] {
            return (false, visible_trees);
        }
        west = match west.checked_sub(1) {
            Some(west) => west,
            None => break,
        };
    }
    (true, visible_trees)
}
