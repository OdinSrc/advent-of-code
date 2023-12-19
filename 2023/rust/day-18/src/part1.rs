use std::{fmt::Display, usize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coord {
    row_idx: isize,
    col_idx: isize,
}

impl Coord {
    fn new(row_idx: isize, col_idx: isize) -> Self {
        Self { row_idx, col_idx }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => unreachable!("Invalid direction"),
        }
    }
}

#[derive(Debug)]
struct DigInstr {
    direction: Direction,
    depth: isize,
    color_code: String,
}

#[derive(Debug)]
struct DigPlan {
    instrs: Vec<DigInstr>,
}

impl From<&str> for DigPlan {
    fn from(value: &str) -> Self {
        Self {
            instrs: value
                .lines()
                .map(|line| {
                    let parts: Vec<&str> = line.split(" ").collect();
                    let dir = parts.iter().nth(0).unwrap();
                    let dep = parts.iter().nth(1).unwrap();
                    let color_code = parts.iter().nth(2).expect(&format!("Invalid Input {line}"));
                    let color_code = color_code.strip_prefix('(').unwrap();
                    let color_code = color_code.strip_suffix(')').unwrap();
                    DigInstr {
                        direction: Direction::from(dir.chars().nth(0).unwrap()),
                        depth: dep.parse().unwrap(),
                        color_code: color_code.to_string(),
                    }
                })
                .collect(),
        }
    }
}

fn dig(plan: DigPlan) -> usize {
    let mut digged_edges: Vec<Coord> = Vec::new();
    // let start = Coord::new(0, 0);

    // digged_edges.push(start);
    let mut current = Coord::new(0, 0);
    let mut max_col = 0;
    let mut min_col = 0;
    let mut max_row = 0;
    let mut min_row = 0;

    plan.instrs.iter().for_each(|instr| {
        match instr.direction {
            Direction::Right => {
                let prev_col = current.col_idx;
                current.col_idx += instr.depth;
                // println!("{} + {} = {}", prev_col, instr.depth, current.col_idx);
                if max_col < current.col_idx {
                    max_col = current.col_idx;
                }
                (prev_col + 1..=current.col_idx).for_each(|idx| {
                    digged_edges.push(Coord::new(current.row_idx, idx));
                });
            }
            Direction::Left => {
                let prev_col = current.col_idx;

                current.col_idx -= instr.depth;
                // println!("{} - {} = {}", prev_col, instr.depth, current.col_idx);
                // let depth = if instr.depth > current.col_idx {
                //     current.col_idx
                // } else {
                //     instr.depth
                // };

                // if depth > 0 {
                if min_col > current.col_idx {
                    min_col = current.col_idx;
                }
                for i in 1..=instr.depth {
                    digged_edges.push(Coord::new(current.row_idx, prev_col - i));
                }
                // }
            }
            Direction::Up => {
                let prev_row = current.row_idx;

                // let depth = if instr.depth > current.row_idx {
                //     current.row_idx
                // } else {
                //     instr.depth
                // };

                // if depth > 0 {
                current.row_idx -= instr.depth;
                if min_row > current.row_idx {
                    min_row = current.row_idx;
                }
                for i in 1..=instr.depth {
                    digged_edges.push(Coord::new(prev_row - i, current.col_idx));
                }
                // }
            }
            Direction::Down => {
                let prev_row = current.row_idx;

                current.row_idx += instr.depth;
                if max_row < current.row_idx {
                    max_row = current.row_idx;
                }
                (prev_row + 1..=current.row_idx).for_each(|idx| {
                    digged_edges.push(Coord::new(idx, current.col_idx));
                });
            }
        };
    });

    // let found_rows = digged_edges.len();

    // dbg!(0 - min_col, 0 - min_row, max_col, max_row);
    let normalize_col = 0 - min_col;
    let normalize_row = 0 - min_row;

    let max_row = (max_row + normalize_row) as usize;
    let max_col = (max_col + normalize_col) as usize;

    digged_edges.iter_mut().for_each(|c| {
        c.col_idx += normalize_col;
        c.row_idx += normalize_row;
    });

    // dbg!(digged_edges);

    const COLS_TO_FILL: Vec<usize> = Vec::new();
    let mut v: Vec<Vec<usize>> = vec![COLS_TO_FILL; max_row + 1];
    digged_edges.iter().for_each(|c| {
        let columns = v.get_mut(c.row_idx as usize).unwrap();
        columns.push(c.col_idx as usize);
    });

    // // v.iter_mut().for_each(|columns| {
    // //     let col_length = columns.len();
    // //     if col_length < max_col {
    // //         columns.sort();
    // //         // dbg!(columns);
    // //     }
    // // });

    // for i in 0..max_row + 1 {
    //     for j in 0..max_col + 1 {
    //         let columns = &v[i];
    //         if columns.contains(&j) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     print!("\n");
    // }
    // dbg!(v);
    // const COLS_TO_FILL: Vec<usize> = Vec::new();
    // let mut v: Vec<Vec<usize>> = vec![cols_to_fill; max_row + 1];
    // digged_edges.iter().for_each(|c| {
    //     let columns = v.get_mut(c.row_idx).unwrap();
    //     if let Some(idx) = columns.iter().position(|&j| j == c.col_idx) {
    //         columns.remove(idx);
    //     }
    // });

    // let remaining = v.iter().map(|line| line.iter().count()).sum::<usize>();

    0
    // remaining + digged_edges.len()
}

pub fn run(input: &str) -> impl Display {
    let plan = DigPlan::from(input);
    dig(plan)
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
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!("62", process(input)?);
        Ok(())
    }
}
