use std::fmt::Display;

fn transpose_vec<T: Copy>(v: Vec<Vec<T>>, default_value: T) -> Vec<Vec<T>> {
    let rows = v.len();
    let cols = v[0].len();
    let mut tv = vec![vec![default_value; rows]; cols];
    for i in 0..rows {
        for j in 0..cols {
            tv[j][i] = v[i][j];
        }
    }
    tv
}

fn find_pattern_index(pattern_group: &Vec<Vec<u8>>) -> i64 {
    let last_boundary = pattern_group.len() - 1;

    for index in 0..last_boundary {
        let second_index = index + 1;
        let pattern = &pattern_group[index];
        let second_pattern = &pattern_group[second_index];

        if pattern == second_pattern {
            if index == 0 || second_index == last_boundary {
                return index as i64;
            }

            let mut all_matches = true;
            let mut p_index = index - 1;
            let mut n_index = second_index + 1;
            if n_index <= last_boundary {
                loop {
                    let pp = &pattern_group[p_index];
                    let np = &pattern_group[n_index];
                    if pp != np {
                        all_matches = false;
                        break;
                    }

                    if p_index == 0 || n_index == last_boundary {
                        break;
                    }

                    p_index -= 1;
                    n_index += 1;
                }
            }

            if all_matches {
                return index as i64;
            }
        }
    }

    -1
}

pub fn pattern_total(pattern_group: Vec<Vec<u8>>) -> i64 {
    let pattern_val = find_pattern_index(&pattern_group);

    if pattern_val != -1 {
        return (pattern_val + 1) * 100;
    }
    let transposed = transpose_vec(pattern_group, b'$');
    let pattern_val = find_pattern_index(&transposed);

    if pattern_val != -1 {
        return pattern_val + 1;
    }
    0
}

pub fn run(input: &str) -> impl Display {
    input
        .split("\n\n")
        .map(|pattern_group| {
            pattern_total(pattern_group.lines().map(|l| l.bytes().collect()).collect())
        })
        .sum::<i64>()
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
        assert_eq!("405", process(input)?);
        Ok(())
    }
}
