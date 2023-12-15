use std::{cmp::Ordering, fmt::Display};

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    data: Vec<Vec<u8>>,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let data: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();
        let height = data.len();
        let width = data[0].len();
        Self {
            data,
            height,
            width,
        }
    }
}

// fn transpose_vector(input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
//     let height = input.len();
//     let width = input[0].len();
//     let mut new_data = vec![vec![b'.'; height]; width];

//     for i in 0..height {
//         for j in 0..width {
//             new_data[j][i] = input[i][j];
//         }
//     }

//     new_data
// }

impl Grid {
    fn transpose(&mut self) {
        let mut new_data = vec![vec![b'.'; self.height]; self.width];

        for i in 0..self.height {
            for j in 0..self.width {
                new_data[j][i] = self.data[i][j];
            }
        }

        self.data = new_data
    }

    fn do_tilt(&mut self) {
        for i in 0..self.data.len() {
            let v = &self.data[i];

            let s = String::from_utf8(v.to_vec()).unwrap();
            let parts: Vec<&str> = s.split_inclusive("#").collect();

            let mut sorted: Vec<u8> = parts
                .iter()
                .rev()
                .flat_map(|p| {
                    let mut v = p.bytes().collect::<Vec<u8>>();

                    v.sort_by(|a, b| {
                        if *b == b'#' {
                            Ordering::Equal
                        } else if *a == b'O' {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }
                    });
                    v
                })
                .collect();

            sorted.reverse();

            self.data[i] = sorted;
        }
    }
}
pub fn run(input: &str) -> impl Display {
    let mut grid = Grid::from(input);
    grid.transpose();
    grid.do_tilt();
    //     let transposed = transpose_vector(moved);

    //     let total = transposed.len();

    //     transposed
    //         .into_iter()
    //         .enumerate()
    //         .map(|(i, v)| {
    //             let total_o = v.into_iter().filter(|c| *c == b'O').count();
    //             let m = total - i;
    //             total_o * m
    //         })
    //         .sum::<usize>()

    let total = grid.data.len();
    grid.data
        .into_iter()
        .enumerate()
        .map(|(_, v)| {
            v.into_iter()
                .enumerate()
                .map(|(j, c)| if c == b'O' { total - j } else { 0 })
                .sum::<usize>()
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
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!("136", process(input)?);
        Ok(())
    }
}
