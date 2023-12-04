use crate::custom_error::AocError;

use std::collections::HashMap;

#[derive(Debug)]
struct NumberMeta {
    start_index: usize,
    value: usize,
    length: usize,
}

type SpecialsInLine = Vec<usize>;
type DigitsInLine = Vec<NumberMeta>;

type SpecialIndex = HashMap<usize, Vec<usize>>;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let total = solve_aoc(input);
    Ok(total.to_string())
}

fn process_part_number(
    v_specials: &[SpecialsInLine],
    line_num: &usize,
    number_meta: &NumberMeta,
    total_lines: usize,
) -> Option<(usize, usize)> {
    let line_num = *line_num;
    let mut starting = 0;
    if number_meta.start_index != 0 {
        starting = number_meta.start_index - 1;
    }

    let ending = number_meta.start_index + number_meta.length;
    let find_range = starting..=ending;

    let special_line = &v_specials[line_num];

    let mut special_position = special_line.iter().find(|i| find_range.contains(i));

    if let Some(special_position) = special_position {
        return Some((line_num, *special_position));
    }

    if special_position.is_none() && line_num != 0 {
        let top_special_line = &v_specials[line_num - 1];

        special_position = top_special_line.iter().find(|i| find_range.contains(i));
        if let Some(special_position) = special_position {
            return Some((line_num - 1, *special_position));
        }
    }

    if special_position.is_none() && line_num < total_lines - 1 {
        let bottom_special_line = &v_specials[line_num + 1];

        special_position = bottom_special_line.iter().find(|i| find_range.contains(i));
        if let Some(special_position) = special_position {
            return Some((line_num + 1, *special_position));
        }
    }

    None
}

pub fn solve_aoc(data: &str) -> usize {
    let lines: Vec<&str> = data.split('\n').filter(|s| !s.is_empty()).collect();

    let total_lines = lines.len();
    let (v_specials, v_digits) = parse_lines(lines);

    let mut final_index: HashMap<usize, SpecialIndex> = HashMap::new();

    v_digits
        .iter()
        .enumerate()
        .for_each(|(line_num, line_digits)| {
            line_digits.iter().for_each(|nd| {
                let meta_data = process_part_number(&v_specials, &line_num, nd, total_lines);
                if let Some(meta_data) = meta_data {
                    let (matched_line, matched_index) = meta_data;
                    let mut special_map = final_index
                        .get(&matched_line)
                        .unwrap_or(&HashMap::new())
                        .to_owned();
                    let mut special_entry = special_map
                        .get(&matched_index)
                        .unwrap_or(&Vec::new())
                        .to_owned();

                    special_entry.push(nd.value);
                    special_map.insert(matched_index, special_entry);
                    final_index.insert(matched_line, special_map);
                }
            });
        });

    final_index
        .values()
        .map(|h| {
            let line_total: usize = h
                .iter()
                .map(|(_, v)| {
                    if v.len() > 1 {
                        return v.iter().product();
                    }
                    0
                })
                .sum();
            line_total
        })
        .sum()
}

fn parse_lines(lines: Vec<&str>) -> (Vec<SpecialsInLine>, Vec<DigitsInLine>) {
    let mut v_digits = Vec::new();
    let mut v_specials = Vec::new();

    lines.iter().for_each(|line| {
        let mut line_specials: SpecialsInLine = Vec::new();
        let mut line_digits: DigitsInLine = Vec::new();

        let mut starting_index = 0;
        let mut digit_str = String::new();

        line.chars().enumerate().for_each(|(i, c)| {
            if c.is_ascii_digit() {
                if digit_str.is_empty() {
                    starting_index = i;
                }
                digit_str.push(c);
            } else {
                if c == '*' {
                    line_specials.push(i);
                }
                if !digit_str.is_empty() {
                    line_digits.push(NumberMeta {
                        start_index: starting_index,
                        length: digit_str.len(),
                        value: digit_str.parse().unwrap(),
                    });

                    digit_str.clear();
                    starting_index = i;
                }
            }
        });

        if !digit_str.is_empty() {
            line_digits.push(NumberMeta {
                start_index: starting_index,
                length: digit_str.len(),
                value: digit_str.parse().unwrap(),
            });
        }

        v_digits.push(line_digits);
        v_specials.push(line_specials);
    });

    (v_specials, v_digits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}
