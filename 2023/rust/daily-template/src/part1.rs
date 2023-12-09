use std::fmt::Display;

pub fn run(_input: &str) -> impl Display {
    let result = "";

    result
}

use crate::custom_error::AocError;
#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    Ok(run(input).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() -> miette::Result<()> {
        let input = "";
        // assert_eq!("", process(input)?);
        Ok(())
    }
}
