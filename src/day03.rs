use crate::prelude::*;

#[aoc_generator(day3)]
fn gen(input: &str) -> Vec<Vec<bool>> {
    input.lines()
        .map(|line| line.chars().map(|c| c == '1').collect())
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<bool>]) -> Result<u32, ParseIntError> {
    let gamma = (0..input[0].len())
        .map(|idx| input.iter().fold(0, |sum, line| if line[idx] { sum + 1 } else { sum - 1 }) >= 0)
        .collect_vec();
    let epsilon = gamma.iter().map(|bit| !bit).collect_vec();
    Ok(
        u32::from_str_radix(&gamma.into_iter().map(|bit| if bit { '1' } else { '0' }).collect::<String>(), 2)?
        * u32::from_str_radix(&epsilon.into_iter().map(|bit| if bit { '1' } else { '0' }).collect::<String>(), 2)?
    )
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<bool>]) -> Result<u32, ParseIntError> {
    let mut oxygen_candidates = input.to_owned();
    let mut co2_candidates = input.to_owned();
    for idx in 0..input[0].len() {
        if oxygen_candidates.len() > 1 {
            let most_common = oxygen_candidates.iter().fold(0, |sum, candidate| if candidate[idx] { sum + 1 } else { sum - 1 }) >= 0;
            oxygen_candidates.retain(|candidate| candidate[idx] == most_common);
        }
        if co2_candidates.len() > 1 {
            let least_common = co2_candidates.iter().fold(0, |sum, candidate| if candidate[idx] { sum + 1 } else { sum - 1 }) < 0;
            co2_candidates.retain(|candidate| candidate[idx] == least_common);
        }
    }
    Ok(
        u32::from_str_radix(&oxygen_candidates.remove(0).into_iter().map(|bit| if bit { '1' } else { '0' }).collect::<String>(), 2)?
        * u32::from_str_radix(&co2_candidates.remove(0).into_iter().map(|bit| if bit { '1' } else { '0' }).collect::<String>(), 2)?
    )
}
