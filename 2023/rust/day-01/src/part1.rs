use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let total = input
        .lines()
        .map(|l| {
            let mut digit_str = String::new();

            let mut first_digit = '0';
            let mut last_digit: Option<char> = None;
            l.chars().for_each(|c| {
                if let Some(_) = c.to_digit(10) {
                    if digit_str.is_empty() {
                        first_digit = c;
                        digit_str.push(c);
                    } else {
                        last_digit = Some(c);
                    }
                }
            });

            if let Some(last_digit) = last_digit {
                digit_str.push(last_digit);
            } else {
                digit_str.push(first_digit);
            }

            digit_str.parse::<u32>().unwrap()
        })
        .sum::<u32>();

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() -> miette::Result<()> {
        let test_input = r#"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"#;

        assert_eq!(process(test_input)?, "142");

        Ok(())
    }
}
