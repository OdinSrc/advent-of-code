use std::{collections::VecDeque, fmt::Display};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Ash,
    Rock,
}

fn parse(input: &str) -> Vec<VecDeque<Vec<Tile>>> {
    input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => Tile::Ash,
                            '#' => Tile::Rock,
                            _ => unreachable!("Invalid Tile"),
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn reflects_at(grid: &VecDeque<Vec<Tile>>) -> Option<usize> {
    (1..grid.len()).find(|&offset| {
        let half1 = grid.iter().take(offset).rev();
        let half2 = grid.iter().skip(offset);
        let combined = half1.zip(half2); // the shortest half determines how long this is!
        let differences: usize = combined
            .map(|(row1, row2)| row1.iter().zip(row2.iter()).filter(|(a, b)| a != b).count())
            .sum();

        differences == 1
    })
}

pub fn run(input: &str) -> impl Display {
    let grid = parse(input);
    grid.iter()
        .map(|grid| {
            // check horizontal
            if let Some(i) = reflects_at(grid) {
                return i * 100;
            }

            // check vertical
            let cols = (0..grid[0].len())
                .map(|i| grid.iter().map(|row| row[i]).collect())
                .collect();
            if let Some(i) = reflects_at(&cols) {
                return i;
            }

            // no reflection found
            0
        })
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
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!("400", process(input)?);
        Ok(())
    }
}
