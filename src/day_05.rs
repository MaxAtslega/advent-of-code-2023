use rayon::prelude::*;
use std::sync::Arc;

#[derive(Default)]
pub struct AdventOfCode;

impl AdventOfCode {
    pub fn part_a(input: &str) -> usize {
        let (seeds, _, mappings) = parse_case(input);
        let mut lowest_location = usize::MAX;

        for seed in seeds {
            let mut number = seed;

            for map in &mappings {
                number = map_number(number, map);
            }
            lowest_location = lowest_location.min(number);
        }

        lowest_location
    }


    pub fn part_b(input: &str) -> usize {
        let (_, seed_ranges, mappings) = parse_case(input);
        let mappings_arc = Arc::new(mappings);

        let lowest_location = seed_ranges.par_iter()
            .map(|&(start, length)| {
                let mut local_lowest = usize::MAX;
                for seed in start..start + length {
                    let mut number = seed;
                    for map in mappings_arc.as_ref() {
                        number = map_number(number, map);
                    }
                    local_lowest = local_lowest.min(number);
                }
                local_lowest
            })
            .min()
            .unwrap_or(usize::MAX);


        lowest_location
    }

    pub fn part_b2(input: &str) -> usize {
        let (_, seed_ranges, mappings) = parse_case(input);

        let mut lowest_location = usize::MAX;

        for (start, length) in seed_ranges {
            for seed in start..start + length {
                let mut number = seed;
                for map in &mappings {
                    number = map_number(number, map);
                }
                lowest_location = lowest_location.min(number);
            }
        }

        lowest_location
    }

}

fn parse_case(case: &str) -> (Vec<usize>, Vec<(usize, usize)>, Vec<Vec<(usize, usize, usize)>>) {
    let mut lines = case.lines();
    let seeds_line = lines.next().unwrap();
    let seed_parts = seeds_line[7..]
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();

    let seed_ranges = seed_parts.chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect::<Vec<(usize, usize)>>();

    let mut mappings = Vec::new();
    let mut current_map = Vec::new();

    for line in lines {
        if line.ends_with("map:") {
            if !current_map.is_empty() {
                mappings.push(current_map);
                current_map = Vec::new();
            }
        } else {
            let nums = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<usize>>();
            if nums.len() == 3 {
                current_map.push((nums[0], nums[1], nums[2]));
            }
        }
    }
    if !current_map.is_empty() {
        mappings.push(current_map);
    }

    (seed_parts, seed_ranges, mappings)
}

fn map_number(number: usize, map: &[(usize, usize, usize)]) -> usize {
    map.iter()
        .find(|&&(_, src_start, length)| number >= src_start && number < src_start + length)
        .map_or(number, |&(dest_start, src_start, _)| dest_start + (number - src_start))
}


#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::AdventOfCode;

    const CASE: &str = indoc! {"
        seeds: 79 14 55 13

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
    "};

    #[test]
    fn test_a() {
        assert_eq!(AdventOfCode::part_a(CASE), 35);
    }

    #[test]
    fn test_b() {
        assert_eq!(AdventOfCode::part_b2(CASE), 46);
    }
}