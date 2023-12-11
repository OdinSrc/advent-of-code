use std::fmt::Display;

use std::collections::{HashSet, VecDeque};
use std::ops::Add;
use std::ops::Index;

// Credits: https://github.com/believer/advent-of-code/tree/master/rust/2023

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub const fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Self) -> Self {
        Position::new(self.x + other.x, self.y + other.y)
    }
}

const DIRECTION_UP: Position = Position::new(0, -1);
const DIRECTION_DOWN: Position = Position::new(0, 1);
const DIRECTION_LEFT: Position = Position::new(-1, 0);
const DIRECTION_RIGHT: Position = Position::new(1, 0);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Start,
    NoPipe,
}

impl Pipe {
    fn is_valid_start(&self, next_pipe: &Pipe, direction: &Position) -> bool {
        if !matches!(self, Pipe::Start) {
            return true;
        };

        matches!(
            (next_pipe, direction),
            (Pipe::Horizontal, &DIRECTION_RIGHT)
                | (Pipe::Horizontal, &DIRECTION_LEFT)
                | (Pipe::Vertical, &DIRECTION_UP)
                | (Pipe::Vertical, &DIRECTION_DOWN)
                | (Pipe::NorthEast, &DIRECTION_LEFT)
                | (Pipe::NorthEast, &DIRECTION_DOWN)
                | (Pipe::NorthWest, &DIRECTION_DOWN)
                | (Pipe::NorthWest, &DIRECTION_RIGHT)
                | (Pipe::SouthEast, &DIRECTION_LEFT)
                | (Pipe::SouthEast, &DIRECTION_UP)
                | (Pipe::SouthWest, &DIRECTION_RIGHT)
                | (Pipe::SouthWest, &DIRECTION_UP)
        )
    }
}

impl From<u8> for Pipe {
    fn from(value: u8) -> Self {
        match value {
            b'|' => Pipe::Vertical,
            b'-' => Pipe::Horizontal,
            b'L' => Pipe::NorthEast,
            b'J' => Pipe::NorthWest,
            b'F' => Pipe::SouthEast,
            b'7' => Pipe::SouthWest,
            b'S' => Pipe::Start,
            b'.' => Pipe::NoPipe,
            _ => panic!("Invalid pipe: {}", value),
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    pub width: i32,
    pub height: i32,
    pub data: Vec<Pipe>,
}

fn directions(pipe: &Pipe) -> Vec<Position> {
    match pipe {
        Pipe::Vertical => vec![DIRECTION_UP, DIRECTION_DOWN],
        Pipe::Horizontal => vec![DIRECTION_LEFT, DIRECTION_RIGHT],
        Pipe::NorthEast => vec![DIRECTION_UP, DIRECTION_RIGHT],
        Pipe::NorthWest => vec![DIRECTION_UP, DIRECTION_LEFT],
        Pipe::SouthEast => vec![DIRECTION_DOWN, DIRECTION_RIGHT],
        Pipe::SouthWest => vec![DIRECTION_DOWN, DIRECTION_LEFT],
        Pipe::Start => vec![
            DIRECTION_UP,
            DIRECTION_RIGHT,
            DIRECTION_DOWN,
            DIRECTION_LEFT,
        ],
        Pipe::NoPipe => vec![],
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let data = value
            .lines()
            .flat_map(|line| line.bytes().filter_map(|b| Pipe::from(b).into()))
            .collect::<Vec<_>>();
        let width = value.lines().next().unwrap_or_default().len() as i32;
        let height = value.lines().count() as i32;

        Grid {
            width,
            height,
            data: data,
        }
    }
}

impl Grid {
    fn find(&self, value: Pipe) -> Option<Position> {
        self.data
            .iter()
            .position(|&x| x == value)
            .map(|i| Position::new((i as i32) % self.width, (i as i32) / self.width))
    }
}

impl Index<Position> for Grid {
    type Output = Pipe;

    #[inline]
    fn index(&self, point: Position) -> &Self::Output {
        &self.data[(self.width * point.y + point.x) as usize]
    }
}

pub fn run(input: &str) -> impl Display {
    let grid: Grid = Grid::from(input);
    let start = grid.find(Pipe::Start).unwrap();

    let mut visited: HashSet<Position> = HashSet::new();
    let mut queue = VecDeque::new();

    // Furthest point and steps
    let mut furthest = 0;

    // Add the starting point to the queue
    queue.push_back((start, 0));

    // While there are still points to visit keep going through the pipe
    while let Some((current_point, steps)) = queue.pop_front() {
        let current_tile = grid[current_point];

        visited.insert(current_point);

        for direction in directions(&current_tile) {
            let new_point = current_point + direction;

            // Check if the next tile is a valid start direction
            if !current_tile.is_valid_start(&grid[new_point], &direction) {
                continue;
            }

            // If we haven't seen the point before, create the next point and steps.
            // If it is the furthest point, update the it.
            // Add the next point to the queue
            if !visited.contains(&new_point) {
                let next_steps = steps + 1;

                if furthest == 0 || next_steps > furthest {
                    furthest = next_steps
                }

                queue.push_back((new_point, next_steps));
            }
        }
    }

    furthest
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
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!("4", process(input)?);

        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
