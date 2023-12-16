use std::fmt::Display;

#[derive(Debug)]
enum Tile {
    Empty,
    Mirror135,         // \ 135deg
    Mirror45,          // / 45 deg
    VerticalSplitter,  // |
    HorizontalSpliter, // -
}

use Tile::*;

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Empty,
            b'\\' => Mirror135,
            b'/' => Mirror45,
            b'|' => VerticalSplitter,
            b'-' => HorizontalSpliter,
            _ => unreachable!("Invalid Tile"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Coord {
    row_idx: usize,
    col_idx: usize,
}

impl Coord {
    fn new(row_idx: usize, col_idx: usize) -> Self {
        Self { row_idx, col_idx }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug)]
struct Grid {
    row_boundary: usize,
    col_boundary: usize,
    data: Vec<Vec<Tile>>,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let data: Vec<Vec<Tile>> = input
            .lines()
            .map(|line| line.bytes().map(Tile::from).collect())
            .collect();
        Self {
            row_boundary: data.len() - 1,
            col_boundary: data[0].len() - 1,
            data,
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
struct VisitedDirection {
    up: bool,
    down: bool,
    right: bool,
    left: bool,
}

const VD_DEFAULT: VisitedDirection = VisitedDirection {
    up: false,
    down: false,
    right: false,
    left: false,
};

fn move_beam(grid: &Grid) -> usize {
    let start = Coord::new(0, 0);
    let mut visited = vec![vec![false; grid.col_boundary + 1]; grid.row_boundary + 1];

    let mut visited_directions: Vec<Vec<VisitedDirection>> =
        vec![vec![VD_DEFAULT; grid.col_boundary + 1]; grid.row_boundary + 1];

    visit_coords(
        &mut visited,
        &mut visited_directions,
        start,
        Direction::Right,
        grid,
    );

    visited
        .into_iter()
        .flat_map(|v| v.into_iter().filter(|&b| b).collect::<Vec<bool>>())
        .count()
}

fn visit_coords(
    visited: &mut Vec<Vec<bool>>,
    visited_directions: &mut Vec<Vec<VisitedDirection>>,
    current_coord: Coord,
    current_direction: Direction,
    grid: &Grid,
) {
    let vd = visited_directions[current_coord.row_idx][current_coord.col_idx];
    match current_direction {
        Direction::Up => {
            if vd.up {
                return;
            } else {
                visited_directions[current_coord.row_idx][current_coord.col_idx].up = true;
            }
        }
        Direction::Down => {
            if vd.down {
                return;
            } else {
                visited_directions[current_coord.row_idx][current_coord.col_idx].down = true;
            }
        }
        Direction::Right => {
            if vd.right {
                return;
            } else {
                visited_directions[current_coord.row_idx][current_coord.col_idx].right = true;
            }
        }
        Direction::Left => {
            if vd.left {
                return;
            } else {
                visited_directions[current_coord.row_idx][current_coord.col_idx].left = true;
            }
        }
    };

    visited[current_coord.row_idx][current_coord.col_idx] = true;

    let next_directions = get_next_directions(current_direction, current_coord, grid);
    if next_directions.is_empty() {
        return;
    }

    for new_dir in next_directions.iter() {
        match new_dir {
            Direction::Up => {
                if current_coord.row_idx > 0 {
                    visit_coords(
                        visited,
                        visited_directions,
                        Coord::new(current_coord.row_idx - 1, current_coord.col_idx),
                        Direction::Up,
                        grid,
                    );
                }
            }
            Direction::Down => {
                if current_coord.row_idx < grid.row_boundary {
                    visit_coords(
                        visited,
                        visited_directions,
                        Coord::new(current_coord.row_idx + 1, current_coord.col_idx),
                        Direction::Down,
                        grid,
                    );
                }
            }
            Direction::Right => {
                if current_coord.col_idx < grid.col_boundary {
                    visit_coords(
                        visited,
                        visited_directions,
                        Coord::new(current_coord.row_idx, current_coord.col_idx + 1),
                        Direction::Right,
                        grid,
                    );
                }
            }
            Direction::Left => {
                if current_coord.col_idx > 0 {
                    visit_coords(
                        visited,
                        visited_directions,
                        Coord::new(current_coord.row_idx, current_coord.col_idx - 1),
                        Direction::Left,
                        // &next_directions,
                        grid,
                    );
                }
            }
        }
    }
}

fn get_next_directions(dir: Direction, coord: Coord, grid: &Grid) -> Vec<Direction> {
    let mut up = false;
    let mut down = false;
    let mut right = false;
    let mut left = false;
    match &grid.data[coord.row_idx][coord.col_idx] {
        Tile::Empty => match dir {
            Direction::Right => {
                right = true;
            }
            Direction::Left => {
                left = true;
            }
            Direction::Up => {
                up = true;
            }
            Direction::Down => {
                down = true;
            }
        },
        Tile::Mirror135 => match dir {
            // i.e \
            Direction::Right => {
                down = true;
            }
            Direction::Left => {
                up = true;
            }
            Direction::Up => {
                left = true;
            }
            Direction::Down => {
                right = true;
            }
        },
        Tile::Mirror45 => match dir {
            // i.e /
            Direction::Right => {
                up = true;
            }
            Direction::Left => {
                down = true;
            }
            Direction::Up => {
                right = true;
            }
            Direction::Down => {
                left = true;
            }
        },
        Tile::HorizontalSpliter => match dir {
            Direction::Right => right = true,
            Direction::Left => left = true,
            Direction::Up | Direction::Down => {
                left = true;
                right = true;
            }
        },
        Tile::VerticalSplitter => match dir {
            Direction::Right | Direction::Left => {
                down = true;
                up = true;
            }
            Direction::Down => down = true,
            Direction::Up => up = true,
        },
    };

    let mut v = Vec::new();

    if up && coord.row_idx > 0 {
        v.push(Direction::Up);
    }
    if down && coord.row_idx < grid.row_boundary {
        v.push(Direction::Down);
    }
    if right && coord.col_idx < grid.col_boundary {
        v.push(Direction::Right);
    }
    if left && coord.col_idx > 0 {
        v.push(Direction::Left);
    }

    v
}

pub fn run(input: &str) -> impl Display {
    let grid = Grid::from(input);

    move_beam(&grid)
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
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!("46", process(input)?);
        Ok(())
    }
}
