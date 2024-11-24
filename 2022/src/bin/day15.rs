use std::collections::HashSet;

use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./inputs/day15.txt")?;

    println!("Part 1: {}", part1(&input, 2_000_000));
    println!("Part 2: {}", part2(&input, 4_000_000));

    Ok(())
}

fn part1(input: &String, target_y: i32) -> usize {
    let sensors = input
        .lines()
        .flat_map(|line| {
            line.split_once(":").map(|(sensor, beacon)| {
                let sensor_cords = sensor
                    .split_once("x=")
                    .unwrap()
                    .1
                    .split_once(", y=")
                    .unwrap();
                let sensor_x = sensor_cords.0.parse::<i32>().unwrap();
                let sensor_y = sensor_cords.1.parse::<i32>().unwrap();

                let beacon_cords = beacon
                    .split_once("x=")
                    .unwrap()
                    .1
                    .split_once(", y=")
                    .unwrap();
                let beacon_x = beacon_cords.0.parse::<i32>().unwrap();
                let beacon_y = beacon_cords.1.parse::<i32>().unwrap();
                ((sensor_x, sensor_y), (beacon_x, beacon_y))
            })
        })
        .collect::<Vec<_>>();

    let mut no_beacons = HashSet::<(i32, i32)>::new();
    let mut yes_beacons = HashSet::<(i32, i32)>::new();
    for (sensor, beacon) in sensors.iter() {
        let d = distance(sensor, beacon);
        let y_dist = (sensor.1 - target_y).abs();
        let x_dist = d - y_dist;
        for x in sensor.0 - x_dist..=sensor.0 + x_dist {
            no_beacons.insert((x, target_y));
        }
        if beacon.1 == target_y {
            yes_beacons.insert(*beacon);
        }
    }

    for yes_beacon in yes_beacons.iter() {
        no_beacons.remove(yes_beacon);
    }

    no_beacons.len()
}

fn part2(input: &String, search_space: i32) -> usize {
    let sensors = input
        .lines()
        .flat_map(|line| {
            line.split_once(":").map(|(sensor, beacon)| {
                let sensor_cords = sensor
                    .split_once("x=")
                    .unwrap()
                    .1
                    .split_once(", y=")
                    .unwrap();
                let sensor_x = sensor_cords.0.parse::<i32>().unwrap();
                let sensor_y = sensor_cords.1.parse::<i32>().unwrap();

                let beacon_cords = beacon
                    .split_once("x=")
                    .unwrap()
                    .1
                    .split_once(", y=")
                    .unwrap();
                let beacon_x = beacon_cords.0.parse::<i32>().unwrap();
                let beacon_y = beacon_cords.1.parse::<i32>().unwrap();
                let d = distance(&(sensor_x, sensor_y), &(beacon_x, beacon_y));
                ((sensor_x, sensor_y), d)
            })
        })
        .collect::<Vec<_>>();

    let answer = find_tuning_freq(&sensors, search_space);

    answer as usize
}

//  Taxicab Distance: | x 1 − x 2 | + | y 1 − y 2 |
fn distance(sensor: &(i32, i32), beacon: &(i32, i32)) -> i32 {
    (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs()
}

fn find_tuning_freq(sensors: &Vec<((i32, i32), i32)>, search_space: i32) -> u64 {
    for (sensor, d) in sensors.iter() {
        for dx in 0..=d + 1 {
            let dy = d + 1 - dx;
            for (x, y) in [
                (sensor.0 + dx, sensor.1 + dy),
                (sensor.0 - dx, sensor.1 - dy),
                (sensor.0 - dx, sensor.1 + dy),
                (sensor.0 + dx, sensor.1 - dy),
            ] {
                if x < 0 || y < 0 || x > search_space || y > search_space {
                    continue;
                }
                if check_point(x, y, sensors) {
                    return 4_000_000 * x as u64 + y as u64;
                }
            }
        }
    }
    0
}

fn check_point(x: i32, y: i32, sensors: &Vec<((i32, i32), i32)>) -> bool {
    for (sensor, d) in sensors.iter() {
        if distance(&(x, y), sensor) <= *d {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
const TEST_INPUT: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;
