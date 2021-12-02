use crate::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Direction {
    Forward,
    Down,
    Up,
}

#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<(Direction, u32)> {
    input.lines()
        .map(|line| {
            let (direction, units) = line.split_once(' ').unwrap();
            (serde_plain::from_str(direction).unwrap(), units.parse().unwrap())
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(Direction, u32)]) -> u32 {
    let (horiz, depth) = input.iter()
        .fold((0, 0), |(horiz, depth), (dir, units)| match dir {
            Direction::Forward => (horiz + units, depth),
            Direction::Down => (horiz, depth + units),
            Direction::Up => (horiz, depth - units),
        });
    horiz * depth
}

#[aoc(day2, part2)]
fn part2(input: &[(Direction, u32)]) -> u32 {
    let (horiz, depth, _) = input.iter()
        .fold((0, 0, 0), |(horiz, depth, aim), (dir, units)| match dir {
            Direction::Forward => (horiz + units, depth + aim * units, aim),
            Direction::Down => (horiz, depth, aim + units),
            Direction::Up => (horiz, depth, aim - units),
        });
    horiz * depth
}
