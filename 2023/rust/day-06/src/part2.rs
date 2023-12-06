use crate::custom_error::AocError;

#[derive(Debug)]
struct Race {
    duration: u64,
    distance: u64,
}

fn parse(input: &str) -> Race {
    let v: Vec<u64> = input
        .lines()
        .filter_map(|line| {
            let (_, digit_str) = line.split_once(':').unwrap();
            let digit_str = digit_str
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>();
            // line.chars().for_each(|c| {
            //     if c.is_ascii_digit() {
            //         digit_str.push(c);
            //     }
            // });
            digit_str.parse::<u64>().ok()
        })
        .collect();
    if v.len() != 2 {
        panic!("Invalid Data");
    }

    Race {
        duration: v[0],
        distance: v[1],
    }
}

fn get_total_ways(previous_race: Race) -> u64 {
    let mut total_ways = 0;
    for i in 1..previous_race.duration {
        let remaining_ms = previous_race.duration - i;
        let distance_covered = remaining_ms * i;
        if distance_covered > previous_race.distance {
            total_ways += 1;
        }
    }
    total_ways
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let previous_race = parse(input);

    let prod: u64 = get_total_ways(previous_race);
    Ok(prod.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200
";
        assert_eq!("71503", process(input)?);
        Ok(())
    }
}
