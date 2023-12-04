use std::collections::HashMap;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let total = input
        .lines()
        .map(|l| {
            let mut digit_words = find_digit_words(l);

            l.chars().enumerate().for_each(|(i, c)| {
                if c.is_ascii_digit() {
                    digit_words.insert(i, c);
                }
            });

            let mut digit_str = String::new();

            let dw_min_index = digit_words.keys().min().unwrap();
            let first_letter = digit_words.get(dw_min_index).unwrap().to_owned();
            digit_str.push(first_letter);

            let dw_max_index = digit_words.keys().max();
            if digit_words.len() == 1 {
                digit_str.push(first_letter);
            } else if let Some(dw_max_index) = dw_max_index {
                let c = digit_words.get(dw_max_index).unwrap().to_owned();
                digit_str.push(c);
            }

            digit_str.parse::<u32>().unwrap()
        })
        .sum::<u32>();

    Ok(total.to_string())
}

fn convert_to_digit(word: &str) -> Option<u8> {
    let d = match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "zero" => 0,
        _ => return None,
    };

    Some(d)
}

fn find_digit_words(line: &str) -> HashMap<usize, char> {
    let mut v: HashMap<usize, char> = HashMap::new();

    let words_to_find = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for w in words_to_find {
        if line.contains(w) {
            let oc = find_all_occurrences(line, w);
            oc.into_iter().for_each(|i| {
                let digit = convert_to_digit(w).unwrap();
                v.insert(i, (digit + b'0') as char);
            });
        }
    }
    v
}

fn find_all_occurrences(data: &str, word: &str) -> Vec<usize> {
    let mut indexes = Vec::new();

    let mut start = 0;

    while let Some(index) = data[start..].find(word) {
        let abs_index = start + index;
        indexes.push(abs_index);
        start = abs_index + word.len();
    }

    indexes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() -> miette::Result<()> {
        let test_input = r#"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"#;

        assert_eq!(process(test_input)?, "281");

        Ok(())
    }
}
