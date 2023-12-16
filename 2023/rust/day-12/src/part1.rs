use itertools::Itertools;
use std::fmt::Display;

struct Record {
    springs: Vec<Spring>,
    counts: Vec<usize>,
}

impl Record {
    fn is_valid(&self) -> bool {
        self.springs
            .iter()
            .group_by(|item| *item)
            .into_iter()
            .filter_map(|(key, group)| {
                if *key == Spring::Damaged {
                    Some(group.count())
                } else {
                    None
                }
            })
            .eq(self.counts.iter().copied())
    }

    fn valid_arrangements(&self) -> usize {
        if let Some(index) = self
            .springs
            .iter()
            .position(|spring| *spring == Spring::Unknown)
        {
            let mut as_damaged_spring = self.springs.clone();
            as_damaged_spring[index] = Spring::Damaged;
            let as_damaged = Record {
                springs: as_damaged_spring,
                counts: self.counts.to_vec(),
            };

            let mut as_operationl_spring = self.springs.clone();
            as_operationl_spring[index] = Spring::Operational;
            let as_operationl = Record {
                springs: as_operationl_spring,
                counts: self.counts.to_vec(),
            };

            as_damaged.valid_arrangements() + as_operationl.valid_arrangements()
        } else {
            if self.is_valid() {
                1
            } else {
                0
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Spring {
    Unknown,
    Operational,
    Damaged,
}

fn parse(input: &str) -> impl Iterator<Item = Record> + '_ {
    input.lines().map(|line| {
        let (springs, counts) = line.split_once(' ').unwrap();
        let springs = springs
            .chars()
            .map(|c| match c {
                '.' => Spring::Operational,
                '#' => Spring::Damaged,
                '?' => Spring::Unknown,
                _ => panic!("Invalid Spring"),
            })
            .collect();

        let counts = counts.split(',').map(|s| s.parse().unwrap()).collect();

        Record { springs, counts }
    })
}

pub fn run(input: &str) -> impl Display {
    parse(input)
        .map(|record| record.valid_arrangements())
        .sum::<usize>()
}

use crate::custom_error::AocError;
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(run(input).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() -> miette::Result<()> {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!("21", process(input)?);
        Ok(())
    }
}
