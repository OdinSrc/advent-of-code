use std::ops::Range;

use crate::custom_error::AocError;

#[derive(Debug)]
struct DataMap {
    input_range: RangeMap,
    output_range: RangeMap,
}

type RangeMap = Range<u64>;

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

    let soil = translate_map(initial_seeds, &seed_to_soil);
    let fert = translate_map(soil, &soil_to_fertilizer);
    let water = translate_map(fert, &fertilizer_to_water);
    let light = translate_map(water, &water_to_light);
    let temp = translate_map(light, &light_to_temp);
    let hum = translate_map(temp, &temp_to_hum);
    let loc = translate_map(hum, &hum_to_loc);

    let mut min = loc[0].start;
    for r in loc.iter() {
        if r.start < min {
            min = r.start;
        }
    }

    Ok(min.to_string())
}

fn translate_map(input: Vec<RangeMap>, data_map: &[DataMap]) -> Vec<RangeMap> {
    let mut output: Vec<RangeMap> = Vec::new();

    input.iter().for_each(|input_range| {
        let mut not_covered_ranges: Vec<RangeMap> = Vec::new();
        let mut covered_ranges: Vec<RangeMap> = Vec::new();
        let mut output_ranges: Vec<RangeMap> = Vec::new();
        for dm in data_map.iter() {
            if dm.input_range.start < input_range.end && dm.input_range.end > input_range.start {
                let start_range = dm.input_range.start.max(input_range.start);
                let end_range = dm.input_range.end.min(input_range.end);
                let total_items = (start_range..end_range).count() as u64;

                let mut gap = 0;
                if start_range > dm.input_range.start {
                    gap = start_range - dm.input_range.start;
                }

                covered_ranges.push(start_range..end_range);
                output_ranges.push(RangeMap {
                    start: dm.output_range.start + gap,
                    end: dm.output_range.start + gap + total_items,
                });
            }
        }

        if !covered_ranges.is_empty() {
            covered_ranges.sort_by_key(|range| range.start);
            let mut current_range = &covered_ranges[0];
            if input_range.start != current_range.start {
                not_covered_ranges.push(RangeMap {
                    start: input_range.start,
                    end: current_range.end - 1,
                })
            }

            for next_range in covered_ranges.iter().skip(1) {
                if current_range.end < next_range.start {
                    output_ranges.push(RangeMap {
                        start: current_range.end,
                        end: next_range.start,
                    });
                }
                current_range = next_range;
            }

            if current_range.end < input_range.end {
                output_ranges.push(RangeMap {
                    start: current_range.end,
                    end: input_range.end,
                });
            }
        } else {
            output.push(input_range.to_owned());
        }

        output.extend(output_ranges);
        output.extend(not_covered_ranges);
    });

    output
}

fn get_seeds(input: &str) -> Vec<RangeMap> {
    let (_, seeds_part) = input.split_once(':').expect("Unable to split seeds data");
    let parsed: Vec<u64> = seeds_part
        .split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    parsed
        .chunks(2)
        .map(|chunk| RangeMap {
            start: chunk[0],
            end: chunk[0] + chunk[1],
        })
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

        let left_val = v[0];
        let right_val = v[1];
        let length = v[2];

        vd.push(DataMap {
            input_range: RangeMap {
                start: right_val,
                end: right_val + length,
            },
            output_range: RangeMap {
                start: left_val,
                end: left_val + length,
            },
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
        assert_eq!("46", process(input)?);
        Ok(())
    }
}
