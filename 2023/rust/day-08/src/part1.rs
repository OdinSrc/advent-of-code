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
    fn move_until_end(&self) -> u32 {
        let mut steps = 0;
        let total_directions = self.directions.len();
        let mut current_position = self.nodes.keys().nth(0).unwrap().as_str();

        let mut i = 0;
        while i < total_directions {
            let node = self.nodes.get(current_position).unwrap();
            current_position = if self.directions[i] == 'R' {
                &node.right
            } else {
                &node.left
            }
            .as_str();

            steps += 1;
            if current_position.contains("ZZZ") {
                break;
            }

            i += 1;
            if i == total_directions {
                i = 0;
            }
        }

        steps
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let map_data = Map::from_str(input).unwrap();
    let steps = map_data.move_until_end();
    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() -> miette::Result<()> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
        assert_eq!("2", process(input)?);

        let input2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)        
";
        assert_eq!("6", process(input2)?);
        Ok(())
    }
}
