use crate::prelude::*;

#[aoc_generator(day6)]
fn gen(input: &str) -> Result<Vec<u8>, ParseIntError> {
    input.split(',').map(str::parse).try_collect()
}

fn simulate(fish: &mut HashMap<u8, usize>, days: usize) {
    for _ in 0..days {
        let mut new_fish = fish.iter().filter_map(|(&timer, &count)| (timer != 0).then(|| (timer - 1, count))).collect::<HashMap<_, _>>();
        if let Some(&spawn) = fish.get(&0) {
            new_fish.insert(8, spawn);
            *new_fish.entry(6).or_default() += spawn;
        }
        *fish = new_fish;
    }
}

#[aoc(day6, part1)]
fn part1(input: &[u8]) -> usize {
    let mut fish = input.iter().copied().counts();
    simulate(&mut fish, 80);
    fish.values().sum()
}

#[aoc(day6, part2)]
fn part2(input: &[u8]) -> usize {
    let mut fish = input.iter().copied().counts();
    simulate(&mut fish, 256);
    fish.values().sum()
}

#[test]
fn sample() {
    let mut fish = [3, 4, 3, 1, 2].into_iter().counts();
    for (i, expected) in [
        &[3,4,3,1,2][..],
        &[2,3,2,0,1][..],
        &[1,2,1,6,0,8][..],
        &[0,1,0,5,6,7,8][..],
        &[6,0,6,4,5,6,7,8,8][..],
        &[5,6,5,3,4,5,6,7,7,8][..],
        &[4,5,4,2,3,4,5,6,6,7][..],
        &[3,4,3,1,2,3,4,5,5,6][..],
        &[2,3,2,0,1,2,3,4,4,5][..],
        &[1,2,1,6,0,1,2,3,3,4,8][..],
        &[0,1,0,5,6,0,1,2,2,3,7,8][..],
        &[6,0,6,4,5,6,0,1,1,2,6,7,8,8,8][..],
        &[5,6,5,3,4,5,6,0,0,1,5,6,7,7,7,8,8][..],
        &[4,5,4,2,3,4,5,6,6,0,4,5,6,6,6,7,7,8,8][..],
        &[3,4,3,1,2,3,4,5,5,6,3,4,5,5,5,6,6,7,7,8][..],
        &[2,3,2,0,1,2,3,4,4,5,2,3,4,4,4,5,5,6,6,7][..],
        &[1,2,1,6,0,1,2,3,3,4,1,2,3,3,3,4,4,5,5,6,8][..],
        &[0,1,0,5,6,0,1,2,2,3,0,1,2,2,2,3,3,4,4,5,7,8][..],
    ].into_iter().enumerate() {
        assert_eq!(fish, expected.iter().copied().counts(), "iteration {}", i);
        simulate(&mut fish, 1);
    }
    assert_eq!(fish, [6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8].into_iter().counts());
    simulate(&mut fish, 80 - 18);
    assert_eq!(fish.values().sum::<usize>(), 5934);
}
