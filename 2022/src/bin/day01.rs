use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./inputs/day01.txt").unwrap();
    let mut elves: Vec<usize> = input
        .split("\n\n")
        .map(|line| {
            line.lines()
                .flat_map(|num| num.parse::<usize>())
                .sum::<usize>()
        })
        .collect();

    elves.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {}", elves[0]);
    println!("Part 2: {}", elves.iter().take(3).sum::<usize>());

    Ok(())
}
