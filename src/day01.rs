use crate::prelude::*;

#[aoc_generator(day1)]
fn gen(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> usize {
    input.iter()
        .tuple_windows()
        .filter(|(prev, curr)| curr > prev)
        .count()
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> usize {
    input.windows(3)
        .tuple_windows()
        .filter(|(prev, curr)| curr.iter().sum::<u32>() > prev.iter().sum())
        .count()
}
