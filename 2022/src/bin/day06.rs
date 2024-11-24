use anyhow::Result;
use std::collections::VecDeque;

fn find_marker(input: &str, marker_size: usize) -> Option<usize> {
    let mut queue = VecDeque::<u8>::new();
    for (i, c) in input.as_bytes().iter().enumerate() {
        if queue.contains(&c) {
            let mut removed = queue.pop_front();

            while removed != Some(*c) || removed == None {
                removed = queue.pop_front();
            }
        }

        queue.push_back(*c);
        if queue.len() == marker_size {
            return Some(i + 1);
        }
    }
    None
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./inputs/day06.txt").unwrap();

    let part1 = find_marker(&input, 4);
    println!("Part 1: {}", part1.unwrap());

    let part2 = find_marker(&input, 14);
    println!("Part 2: {}", part2.unwrap());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_part1() {
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), Some(5));
    }
    #[test]
    fn test_two_part1() {
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), Some(6));
    }
    #[test]
    fn test_three_part1() {
        //                01234567890
        assert_eq!(
            find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            Some(10)
        );
    }
    #[test]
    fn test_four_part1() {
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), Some(11));
    }
}
