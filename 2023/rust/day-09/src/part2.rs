use std::fmt::Display;

use crate::custom_error::AocError;

fn find_next_number(input: Vec<i64>) -> i64 {
    let mut output = Vec::new();
    get_diff_vec(input, &mut output);
    let mut previous_number = 0;
    output.reverse();

    for v in output.iter_mut() {
        let first_number: i64 = v.to_owned();
        previous_number = first_number - previous_number;
    }
    previous_number
}

fn get_diff_vec(input: Vec<i64>, output_vec: &mut Vec<i64>) {
    let next_input: Vec<i64> = input.windows(2).map(|w| w[1] - w[0]).collect();
    if next_input.iter().all(|&x| x == 0) {
        output_vec.push(input.first().unwrap().to_owned());
        return;
    }

    output_vec.push(input.first().unwrap().to_owned());
    get_diff_vec(next_input, output_vec)
}

fn run(input: &str) -> impl Display {
    let parsed: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|d| d.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    let result: i64 = parsed.into_iter().map(find_next_number).sum();

    result
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(run(input).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() -> miette::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
