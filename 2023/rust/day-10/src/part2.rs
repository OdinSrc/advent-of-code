use std::{collections::HashSet, fmt::Display};

//Ref: https://nickymeuleman.netlify.app/garden/aoc2023-day10

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
    Start,
    // . is ground; there is no pipe in this tile.
    Empty,
    // | is a vertical pipe connecting north and south.
    NorthSouth,
    // - is a horizontal pipe connecting west and and.
    EastWest,
    // L is a 90-degree bend connecting north and east.
    NorthEast,
    // J is a 90-degree bend connecting north and west.
    NorthWest,
    // 7 is a 90-degree bend connecting south and west.
    SouthWest,
    // F is a 90-degree bend connecting south and east.
    SouthEast,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Empty,
            b'S' => Self::Start,
            b'|' => Self::NorthSouth,
            b'-' => Self::EastWest,
            b'L' => Self::NorthEast,
            b'J' => Self::NorthWest,
            b'7' => Self::SouthWest,
            b'F' => Self::SouthEast,
            _ => unreachable!("Blackhole!"),
        }
    }
}

use Tile::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    row_idx: usize,
    col_idx: usize,
}

impl Coord {
    fn new(row_idx: usize, col_idx: usize) -> Self {
        Self { row_idx, col_idx }
    }

    fn valid_neighbours(&self, map: &[Vec<Tile>]) -> Vec<Coord> {
        let mut neighbours = vec![];
        let max_height = map.len() - 1;
        let max_width = map[0].len() - 1;

        match map[self.row_idx][self.col_idx] {
            Empty => (),
            Start => {
                // north
                if self.row_idx > 0 {
                    let top_row = self.row_idx - 1;
                    let tile = &map[top_row][self.col_idx];
                    if matches!(tile, NorthSouth | SouthEast | SouthWest) {
                        neighbours.push(Coord::new(top_row, self.col_idx));
                    }
                }

                //south
                if self.row_idx < max_height {
                    let bottom_row = self.row_idx + 1;
                    let tile = &map[bottom_row][self.col_idx];
                    if matches!(tile, NorthSouth | NorthEast | NorthWest) {
                        neighbours.push(Coord::new(bottom_row, self.col_idx));
                    }
                }

                //West
                if self.col_idx > 0 {
                    let left_col = self.col_idx - 1;
                    let tile = &map[self.row_idx][left_col];
                    if matches!(tile, EastWest | NorthEast | SouthEast) {
                        neighbours.push(Coord::new(self.row_idx, left_col));
                    }
                }

                //East
                if self.col_idx < max_width {
                    let right_col = self.col_idx + 1;
                    let tile = &map[self.row_idx][right_col];
                    if matches!(tile, EastWest | NorthWest | SouthWest) {
                        neighbours.push(Coord::new(self.row_idx, right_col));
                    }
                }
            }
            NorthSouth => {
                //north
                if self.row_idx > 0 {
                    let top_row = self.row_idx - 1;
                    let tile = &map[top_row][self.col_idx];
                    if matches!(tile, NorthSouth | SouthWest | SouthEast | Start) {
                        neighbours.push(Coord::new(top_row, self.col_idx));
                    }
                }

                //south
                if self.row_idx < max_height {
                    let bottom_row = self.row_idx + 1;
                    let tile = &map[bottom_row][self.col_idx];
                    if matches!(tile, NorthSouth | NorthEast | NorthWest) {
                        neighbours.push(Coord::new(bottom_row, self.col_idx));
                    }
                }
            }
            EastWest => {
                //West
                if self.col_idx > 0 {
                    let left_col = self.col_idx - 1;
                    let tile = &map[self.row_idx][left_col];
                    if matches!(tile, EastWest | SouthEast | NorthEast | Start) {
                        neighbours.push(Coord::new(self.row_idx, left_col));
                    }
                }

                //East
                if self.col_idx < max_height {
                    let right_col = self.col_idx + 1;
                    let tile = &map[self.row_idx][right_col];
                    if matches!(tile, EastWest | SouthWest | NorthWest | Start) {
                        neighbours.push(Coord::new(self.row_idx, right_col));
                    }
                }
            }
            NorthEast => {
                //north
                if self.row_idx > 0 {
                    let top_row = self.row_idx - 1;
                    let tile = &map[top_row][self.col_idx];
                    if matches!(tile, NorthSouth | SouthWest | SouthEast | Start) {
                        neighbours.push(Coord::new(top_row, self.col_idx));
                    }
                }

                //East
                if self.col_idx < max_height {
                    let right_col = self.col_idx + 1;
                    let tile = &map[self.row_idx][right_col];
                    if matches!(tile, EastWest | SouthWest | NorthWest | Start) {
                        neighbours.push(Coord::new(self.row_idx, right_col));
                    }
                }
            }
            NorthWest => {
                //north
                if self.row_idx > 0 {
                    let top_row = self.row_idx - 1;
                    let tile = &map[top_row][self.col_idx];
                    if matches!(tile, NorthSouth | SouthWest | SouthEast | Start) {
                        neighbours.push(Coord::new(top_row, self.col_idx));
                    }
                }

                //West
                if self.col_idx > 0 {
                    let left_col = self.col_idx - 1;
                    let tile = &map[self.row_idx][left_col];
                    if matches!(tile, EastWest | SouthEast | NorthEast | Start) {
                        neighbours.push(Coord::new(self.row_idx, left_col));
                    }
                }
            }
            SouthWest => {
                //south
                if self.row_idx < max_height {
                    let bottom_row = self.row_idx + 1;
                    let tile = &map[bottom_row][self.col_idx];
                    if matches!(tile, NorthSouth | NorthEast | NorthWest) {
                        neighbours.push(Coord::new(bottom_row, self.col_idx));
                    }
                }

                //West
                if self.col_idx > 0 {
                    let left_col = self.col_idx - 1;
                    let tile = &map[self.row_idx][left_col];
                    if matches!(tile, EastWest | SouthEast | NorthEast | Start) {
                        neighbours.push(Coord::new(self.row_idx, left_col));
                    }
                }
            }
            SouthEast => {
                //south
                if self.row_idx < max_height {
                    let bottom_row = self.row_idx + 1;
                    let tile = &map[bottom_row][self.col_idx];
                    if matches!(tile, NorthSouth | NorthEast | NorthWest) {
                        neighbours.push(Coord::new(bottom_row, self.col_idx));
                    }
                }

                //East
                if self.col_idx < max_height {
                    let right_col = self.col_idx + 1;
                    let tile = &map[self.row_idx][right_col];
                    if matches!(tile, EastWest | SouthWest | NorthWest | Start) {
                        neighbours.push(Coord::new(self.row_idx, right_col));
                    }
                }
            }
        }

        neighbours
    }
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, Coord) {
    let mut start = Coord::new(0, 0);

    let map = input
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            line.bytes()
                .enumerate()
                .map(|(col_idx, b)| {
                    let tile = Tile::from(b);
                    if tile == Tile::Start {
                        start = Coord::new(row_idx, col_idx);
                    }
                    tile
                })
                .collect()
        })
        .collect();

    (map, start)
}

fn build_loop(start: Coord, map: &[Vec<Tile>]) -> HashSet<Coord> {
    let mut loop_coords = HashSet::new();

    loop_coords.insert(start);
    let mut to_visit = start.valid_neighbours(map);

    while let Some(curr_pos) = to_visit.pop() {
        for neighbour in curr_pos.valid_neighbours(map) {
            if !loop_coords.contains(&neighbour) {
                to_visit.push(neighbour);
                loop_coords.insert(neighbour);
            }
        }
    }

    loop_coords
}

fn get_start_pipe(map: &[Vec<Tile>], start: Coord) -> Tile {
    let neighbours = start.valid_neighbours(map);
    let mut north = false;
    let mut south = false;
    let mut west = false;
    let mut east = false;
    neighbours.iter().for_each(|coord| {
        if coord.row_idx < start.row_idx {
            north = true;
        }
        if coord.row_idx > start.row_idx {
            south = true;
        }
        if coord.col_idx < start.col_idx {
            west = true;
        }
        if coord.col_idx > start.col_idx {
            east = true;
        }
    });

    match (north, west, south, east) {
        (true, true, _, _) => NorthWest,
        (true, _, true, _) => NorthSouth,
        (true, _, _, true) => NorthEast,
        (_, true, true, _) => SouthWest,
        (_, _, true, true) => SouthEast,
        (_, true, _, true) => EastWest,
        _ => unreachable!("No valid tile to replace Start with was found"),
    }
}

/// replace start with a valid pipe segment, and only keep pipe segments that are part of the loop
fn clean_map(start: Coord, loop_coords: &HashSet<Coord>, map: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let start_pipe = get_start_pipe(&map, start);

    map.into_iter()
        .enumerate()
        .map(|(row_idx, line)| {
            line.into_iter()
                .enumerate()
                .map(|(col_idx, tile)| match tile {
                    Start => start_pipe,
                    pipe if loop_coords.contains(&Coord::new(row_idx, col_idx)) => pipe,
                    _ => Empty,
                })
                .collect()
        })
        .collect()
}

pub fn run(input: &str) -> impl Display {
    let (map, start) = parse(input);

    let loop_coords = build_loop(start, &map);
    let map = clean_map(start, &loop_coords, map);

    // scan from top to bottom and left to right, counting how many tiles are inside the loop.
    // keep track of a boolean that tells me if I'm inside the loop
    // every time I cross a vertical pipe that does not horizontally block the top (the place where I am in the loop), flip that state
    let mut inside = false;

    map.into_iter()
        .flatten()
        .filter(|tile| match tile {
            Empty => inside,
            NorthSouth | NorthWest | NorthEast => {
                inside = !inside;
                false
            }
            _ => false,
        })
        .count()
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
        let input = "";
        // assert_eq!("", process(input)?);
        Ok(())
    }
}
