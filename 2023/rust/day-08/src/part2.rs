use std::{collections::BTreeMap, str::FromStr, string::ParseError};

use crate::custom_error::AocError;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

#[derive(Debug)]
struct Map {
    directions: Vec<char>,
    nodes: BTreeMap<String, Node>,
}

impl FromStr for Map {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (directions_part, nodes_part) = s.split_once("\n\n").expect("Invalid Map data");

        let mut nodes = BTreeMap::new();

        nodes_part.lines().for_each(|line| {
            let (index, node_data) = line.split_once('=').unwrap();
            let index = index.trim().to_owned();
            let node_data = node_data.replace(['(', ')'], "");
            let (left, right) = node_data.split_once(',').unwrap();
            nodes.insert(
                index,
                Node {
                    left: left.trim().to_owned(),
                    right: right.trim().to_owned(),
                },
            );
        });

        Ok(Self {
            directions: directions_part.chars().collect(),
            nodes,
        })
    }
}

impl Map {
    fn gather_steps(&self) -> BTreeMap<String, u128> {
        let items: Vec<&str> = self
            .nodes
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|k| k.as_str())
            .collect();

        let mut steps_map: BTreeMap<String, u128> = BTreeMap::new();

        for item in items.into_iter() {
            let total_directions = self.directions.len();
            let mut current_position = item;

            let mut i = 0;
            let mut steps = 0;

            while i < total_directions {
                let node = self.nodes.get(current_position).unwrap();
                current_position = if self.directions[i] == 'R' {
                    &node.right
                } else {
                    &node.left
                }
                .as_str();

                i += 1;
                steps += 1;

                if current_position.ends_with('Z') {
                    steps_map.insert(item.to_owned(), steps);
                    break;
                }

                if i == total_directions {
                    i = 0;
                }
            }
        }
        steps_map
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let map_data = Map::from_str(input).unwrap();
    let steps_map = &map_data.gather_steps();

    let numbers: Vec<_> = steps_map.iter().map(|(_, num)| num.to_owned()).collect();

    use num_integer::lcm;
    let result = numbers.iter().fold(1, |acc, &x| lcm(acc, x));
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() -> miette::Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!("6", process(input)?);

        Ok(())
    }
}
