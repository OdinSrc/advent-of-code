use crate::custom_error::AocError;

#[derive(Debug)]
struct NumberMeta {
    start_index: usize,
    value: usize,
    length: usize,
}

type SpecialsInLine = Vec<usize>;
type DigitsInLine = Vec<NumberMeta>;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let total = solve_aoc(input);
    Ok(total.to_string())
}

fn is_part_number(
    v_specials: &[SpecialsInLine],
    line_num: &usize,
    number_meta: &NumberMeta,
    total_lines: usize,
) -> bool {
    let line_num = *line_num;
    let mut starting = 0;
    if number_meta.start_index != 0 {
        starting = number_meta.start_index - 1;
    }

    let ending = number_meta.start_index + number_meta.length;
    let find_range = starting..=ending;

    let special_line = &v_specials[line_num];

    let mut is_part_number = special_line.iter().any(|i| find_range.contains(i));

    if !is_part_number && line_num != 0 {
        let top_special_line = &v_specials[line_num - 1];

        is_part_number = top_special_line.iter().any(|i| find_range.contains(i));
    }

    if !is_part_number && line_num < total_lines - 1 {
        let bottom_special_line = &v_specials[line_num + 1];

        is_part_number = bottom_special_line.iter().any(|i| find_range.contains(i));
    }

    is_part_number
}

pub fn solve_aoc(data: &str) -> usize {
    let lines: Vec<&str> = data.split('\n').filter(|s| !s.is_empty()).collect();

    let total_lines = lines.len();
    let (v_specials, v_digits) = parse_lines(lines);

    v_digits
        .iter()
        .enumerate()
        .map(|(line_num, line_digits)| {
            line_digits
                .iter()
                .filter_map(|nd| {
                    if is_part_number(&v_specials, &line_num, nd, total_lines) {
                        Some(nd.value)
                    } else {
                        None
                    }
                })
                .sum::<usize>()
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
                if c != '.' {
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
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
