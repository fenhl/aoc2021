use crate::prelude::*;

#[derive(PartialEq)]
enum BracketKind {
    Round,
    Square,
    Curly,
    Angle,
}

struct Bracket {
    closing: bool,
    kind: BracketKind,
}

impl TryFrom<char> for Bracket {
    type Error = ();

    fn try_from(c: char) -> Result<Self, ()> {
        Ok(match c {
            '(' => Self { closing: false, kind: BracketKind::Round },
            '[' => Self { closing: false, kind: BracketKind::Square },
            '{' => Self { closing: false, kind: BracketKind::Curly },
            '<' => Self { closing: false, kind: BracketKind::Angle },
            ')' => Self { closing: true, kind: BracketKind::Round },
            ']' => Self { closing: true, kind: BracketKind::Square },
            '}' => Self { closing: true, kind: BracketKind::Curly },
            '>' => Self { closing: true, kind: BracketKind::Angle },
            _ => return Err(()),
        })
    }
}

#[aoc_generator(day10)]
fn gen(input: &str) -> Vec<Vec<Bracket>> {
    input.lines()
        .map(|line| line.chars().map(Bracket::try_from).try_collect().unwrap())
        .collect()
}

#[aoc(day10, part1)]
fn part1(input: &[Vec<Bracket>]) -> u32 {
    input.iter()
        .filter_map(|line| {
            let mut stack = Vec::default();
            for Bracket { closing, kind } in line {
                if *closing {
                    if let Some(expected) = stack.pop() {
                        if expected != kind {
                            return Some(match kind {
                                BracketKind::Round => 3,
                                BracketKind::Square => 57,
                                BracketKind::Curly => 1197,
                                BracketKind::Angle => 25137,
                            })
                        }
                    }
                } else {
                    stack.push(kind);
                }
            }
            None
        })
        .sum()
}

#[aoc(day10, part2)]
fn part2(input: &[Vec<Bracket>]) -> u64 {
    let mut scores = input.iter()
        .filter_map(|line| {
            let mut stack = Vec::default();
            for Bracket { closing, kind } in line {
                if *closing {
                    if let Some(expected) = stack.pop() {
                        if expected != kind { return None }
                    }
                } else {
                    stack.push(kind);
                }
            }
            let mut score = 0;
            for kind in stack.into_iter().rev() {
                score = score * 5 + match kind {
                    BracketKind::Round => 1,
                    BracketKind::Square => 2,
                    BracketKind::Curly => 3,
                    BracketKind::Angle => 4,
                };
            }
            Some(score)
        })
        .collect_vec();
    scores.sort();
    scores[scores.len() / 2]
}

#[test]
fn sample() {
    const SAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]";

    assert_eq!(part2(&gen(SAMPLE)), 288957);
}
