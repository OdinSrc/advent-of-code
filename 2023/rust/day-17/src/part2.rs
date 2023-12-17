use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CrucibleState {
    x: usize,
    y: usize,
    x_dir: isize,
    y_dir: isize,
    distance: usize,
}

struct HeatMap {
    max_rows: usize,
    max_cols: usize,
    data: Vec<Vec<usize>>,
}

impl From<&str> for HeatMap {
    fn from(value: &str) -> Self {
        let data: Vec<Vec<usize>> = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

        Self {
            max_rows: data.len() - 1,
            max_cols: data[0].len() - 1,
            data,
        }
    }
}

fn get_min_heat_loss(heat_map: HeatMap, braking_duration: usize, max_distance: usize) -> usize {
    let mut movements = BinaryHeap::new();
    let mut movements_history = std::collections::HashSet::new();

    let grid_height = heat_map.max_rows + 1;
    let grid_width = heat_map.max_cols + 1;
    let data = heat_map.data;
    let destination = (heat_map.max_cols, heat_map.max_rows);

    movements.push(Reverse((0, 0, 0, 0, 0, 0)));

    while let Some(Reverse((heat_loss, x, y, x_dir, y_dir, distance))) = movements.pop() {
        let state = CrucibleState {
            x,
            y,
            x_dir,
            y_dir,
            distance,
        };
        if movements_history.contains(&state) {
            continue;
        }
        movements_history.insert(state);

        if (x, y) == destination && distance >= braking_duration {
            return heat_loss;
        }

        if distance >= braking_duration || distance == 0 {
            if x_dir == 0 {
                if x < heat_map.max_cols {
                    movements.push(Reverse((heat_loss + data[y][x + 1], x + 1, y, 1, 0, 1)));
                }

                if x > 0 {
                    movements.push(Reverse((heat_loss + data[y][x - 1], x - 1, y, -1, 0, 1)));
                }
            }

            if y_dir == 0 {
                if y < heat_map.max_rows {
                    movements.push(Reverse((heat_loss + data[y + 1][x], x, y + 1, 0, 1, 1)));
                }

                if y > 0 {
                    movements.push(Reverse((heat_loss + data[y - 1][x], x, y - 1, 0, -1, 1)));
                }
            }
        }

        if distance < max_distance {
            let new_x = x.wrapping_add(x_dir as usize);
            let new_y = y.wrapping_add(y_dir as usize);
            if new_x < grid_width && new_y < grid_height {
                movements.push(Reverse((
                    heat_loss + data[new_y][new_x],
                    new_x,
                    new_y,
                    x_dir,
                    y_dir,
                    distance + 1,
                )));
            }
        }
    }

    0
}

fn run(input: &str) -> impl Display {
    get_min_heat_loss(HeatMap::from(input), 4, 10)
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
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!("94", process(input)?);

        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";
        assert_eq!("71", process(input)?);
        Ok(())
    }
}
