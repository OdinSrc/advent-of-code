use crate::custom_error::AocError;

#[derive(Debug)]
struct Race {
    duration: u32,
    distance: u32,
}

fn parse(input: &str) -> Vec<Race> {
    let v: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.split(' ')
                .filter_map(|c| c.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .collect();
    if v.len() != 2 {
        panic!("Invalid Data");
    }

    v[0].iter()
        .zip(v[1].iter())
        .map(|(duration, distance)| Race {
            duration: duration.to_owned(),
            distance: distance.to_owned(),
        })
        .collect()
}

fn get_total_ways(previous_race: Race) -> u32 {
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
    let previous_races = parse(input);

    let prod: u32 = previous_races.into_iter().map(get_total_ways).product();
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
        assert_eq!("288", process(input)?);
        Ok(())
    }
}
