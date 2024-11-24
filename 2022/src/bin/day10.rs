use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./inputs/day10.txt").unwrap();

    solution(&input);
    Ok(())
}

fn handle_tick(tick: usize, register: i32, crt: &mut [[char; 40]; 6], signal_sum: &mut i32) {
    if (register - (tick % 40) as i32).abs() <= 1 {
        crt[tick / 40][tick % 40] = '#';
    } else {
        crt[tick / 40][tick % 40] = ' ';
    }
    // ticks start at 0 so the 20th tick is actually tick 19
    if [20, 60, 100, 140, 180, 220].contains(&(tick + 1)) {
        *signal_sum += register * (tick as i32 + 1);
    }
}

fn solution(input: &str) {
    let mut tick: usize = 0;
    let mut register: i32 = 1;
    let mut signal_sum = 0;
    let mut crt: [[char; 40]; 6] = [[' '; 40]; 6];

    input.lines().for_each(|line| {
        if line == "noop" {
            handle_tick(tick, register, &mut crt, &mut signal_sum);
            tick += 1;
        } else if line.starts_with("addx") {
            let x = &line[5..].parse::<i32>().unwrap();
            handle_tick(tick, register, &mut crt, &mut signal_sum);
            tick += 1;
            handle_tick(tick, register, &mut crt, &mut signal_sum);
            register += x;
            tick += 1;
        }
    });

    println!("Part 1: {}", signal_sum);
    println!("Part 2:");
    for row in crt.iter() {
        for col in row.iter() {
            print!("{}", col);
        }
        println!();
    }
}

#[allow(dead_code)]
const TEST_INPUT: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;
