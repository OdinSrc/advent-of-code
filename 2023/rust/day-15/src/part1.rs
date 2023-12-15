use std::fmt::Display;

macro_rules! str_to_single_number {
    ($input: expr) => {
        $input
            .bytes()
            .fold(0, |current_value, b| (current_value + b as u64) * 17)
            & 0xFF
    };
}
pub fn run(input: &str) -> impl Display {
    let input = input.trim();
    input
        .split(',')
        .map(|line| str_to_single_number!(line))
        .sum::<u64>()
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
        let input = "HASH";
        assert_eq!(52, str_to_single_number!(input));

        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("1320", process(input)?);
        Ok(())
    }
}
