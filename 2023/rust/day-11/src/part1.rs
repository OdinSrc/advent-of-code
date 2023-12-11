use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    const fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UniversePart {
    Empty,
    Galaxy,
}

#[derive(Debug)]
struct GalaxyPair {
    first_position: Position,
    second_position: Position,
}

type Grid = Vec<Vec<UniversePart>>;

fn get_galaxy_pairs(grid: &[Vec<UniversePart>]) -> Vec<GalaxyPair> {
    let v: Vec<Position> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(x, universe_part)| match universe_part {
                    UniversePart::Empty => None,
                    UniversePart::Galaxy => Some(Position::new(x, y)),
                })
        })
        .collect();

    let mut result_vec = Vec::with_capacity(v.len() * (v.len() - 1) / 2);

    for i in 0..v.len() {
        let p1 = &v[i];
        for p2 in v.iter().skip(i + 1) {
            result_vec.push(GalaxyPair {
                first_position: *p1,
                second_position: *p2,
            })
        }
    }

    result_vec
}

fn transpose_grid(input: &Grid) -> Vec<Vec<UniversePart>> {
    (0..input[0].len())
        .map(|i| input.iter().map(|c| c[i]).collect())
        .collect()
}

fn get_empty_cols(grid: &Grid) -> Vec<usize> {
    let v = transpose_grid(grid);

    v.into_iter()
        .enumerate()
        .filter_map(|(col_num, column)| {
            if column.iter().all(|&u| u == UniversePart::Empty) {
                return Some(col_num);
            }
            None
        })
        .collect()
}

fn get_empty_rows(grid: &Grid) -> Vec<usize> {
    grid.iter()
        .enumerate()
        .filter_map(|(row_num, row)| {
            if row.iter().all(|&u| u == UniversePart::Empty) {
                return Some(row_num);
            }
            None
        })
        .collect()
}

fn find_pair_paths(pair: &GalaxyPair, empty_rows: &[usize], empty_cols: &[usize]) -> usize {
    let first_position = pair.first_position;
    let second_position = pair.second_position;

    let (start_position, end_position) = if second_position.y > first_position.y
        || (first_position.y == second_position.y && second_position.x > first_position.x)
    {
        (first_position, second_position)
    } else {
        (second_position, first_position)
    };

    let empty_cols_count = empty_cols
        .iter()
        .filter(|&&x| {
            let (min_x, max_x) = if end_position.x > start_position.x {
                (start_position.x, end_position.x)
            } else {
                (end_position.x, start_position.x)
            };
            min_x <= x && x <= max_x
        })
        .count();

    let empty_rows_count = empty_rows
        .iter()
        .filter(|&&y| {
            let (min_y, max_y) = if end_position.y > start_position.y {
                (start_position.y, end_position.y)
            } else {
                (end_position.y, start_position.y)
            };
            min_y <= y && y <= max_y
        })
        .count();

    let y_diff = if end_position.y > start_position.y {
        end_position.y - start_position.y
    } else {
        start_position.y - end_position.y
    };

    let x_diff = if end_position.x > start_position.x {
        end_position.x - start_position.x
    } else {
        start_position.x - end_position.x
    };

    y_diff + x_diff + empty_cols_count + empty_rows_count
}

fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|c| match c {
                    b'.' => UniversePart::Empty,
                    b'#' => UniversePart::Galaxy,
                    _ => unreachable!("Blackhole found! :P"),
                })
                .collect()
        })
        .collect()
}

pub fn run(input: &str) -> impl Display {
    let grid = parse(input);

    let galaxy_pairs = get_galaxy_pairs(&grid);

    let empty_rows = get_empty_rows(&grid);
    let empty_cols = get_empty_cols(&grid);

    let result = galaxy_pairs
        .iter()
        .map(|pair| find_pair_paths(pair, &empty_rows, &empty_cols))
        .sum::<usize>();

    result
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
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!("374", process(input)?);
        Ok(())
    }
}
