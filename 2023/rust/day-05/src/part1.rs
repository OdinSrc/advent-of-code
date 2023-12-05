use crate::custom_error::AocError;

struct DataMap {
    input_start: u64,
    input_end: u64,
    output_start: u64,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let initial_seeds = get_seeds(parts[0]);

    let seed_to_soil = parse_map(parts[1]);
    let soil_to_fertilizer = parse_map(parts[2]);
    let fertilizer_to_water = parse_map(parts[3]);
    let water_to_light = parse_map(parts[4]);
    let light_to_temp = parse_map(parts[5]);
    let temp_to_hum = parse_map(parts[6]);
    let hum_to_loc = parse_map(parts[7]);

    let min = initial_seeds
        .into_iter()
        .map(|value| {
            let soil = map_to_value(value, &seed_to_soil);
            let fert = map_to_value(soil, &soil_to_fertilizer);
            let water = map_to_value(fert, &fertilizer_to_water);
            let light = map_to_value(water, &water_to_light);
            let temp = map_to_value(light, &light_to_temp);
            let hum = map_to_value(temp, &temp_to_hum);
            map_to_value(hum, &hum_to_loc)
        })
        .min()
        .expect("Unable to get min location value");

    Ok(min.to_string())
}

fn map_to_value(input_value: u64, data_map: &[DataMap]) -> u64 {
    for dm in data_map.iter() {
        if input_value >= dm.input_start && input_value <= dm.input_end {
            let gap = input_value - dm.input_start;
            let target_value = dm.output_start + gap;
            return target_value;
        }
    }

    input_value
}

fn get_seeds(input: &str) -> Vec<u64> {
    let (_, seeds_part) = input.split_once(':').expect("Unable to split seeds data");
    seeds_part
        .split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
}

fn parse_map(input: &str) -> Vec<DataMap> {
    let mut vd = Vec::new();
    input.lines().skip(1).for_each(|line| {
        if line.is_empty() {
            return;
        }
        let v: Vec<u64> = line.split(' ').filter_map(|s| s.parse().ok()).collect();

        if v.len() != 3 {
            return;
        }

        let soil = v[0];
        let seed = v[1];
        let length = v[2];

        vd.push(DataMap {
            input_start: seed,
            input_end: seed + length,
            output_start: soil,
        })
    });

    vd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() -> miette::Result<()> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        assert_eq!("35", process(input)?);
        Ok(())
    }
}
