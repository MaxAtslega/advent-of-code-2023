#[derive(Default)]
pub struct AdventOfCode;

impl AdventOfCode {
    pub fn part_a(input: &str) -> usize {
        let mut count = 0;

        for line in input.lines() {
            let parts = line.split(": ").collect::<Vec<&str>>();
            let numbers = parts[1].split(" | ").collect::<Vec<&str>>();

            let winning_numbers: Vec<i32> = numbers[0]
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect();

            let my_numbers: Vec<i32> = numbers[1]
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect();

            let match_count = my_numbers
                .iter()
                .filter(|&my_number| winning_numbers.contains(my_number))
                .count();

            let points = if match_count > 0 {
                1 << (match_count - 1)
            } else {
                0
            };

            count += points;
        }

        count
    }


    pub fn part_b(input: &str) -> u32 {
        let mut cards: Vec<(Vec<i32>, Vec<i32>)> = input
            .lines()
            .map(|line| {
                let parts = line.split(" | ").collect::<Vec<&str>>();
                let original_numbers = parts[0]
                    .split_whitespace()
                    .filter_map(|num| num.parse::<i32>().ok())
                    .collect::<Vec<i32>>();
                let my_numbers = parts[1]
                    .split_whitespace()
                    .filter_map(|num| num.parse::<i32>().ok())
                    .collect::<Vec<i32>>();
                (original_numbers, my_numbers)
            })
            .collect();

        let mut card_counts = vec![1; cards.len()];

        for i in 0..cards.len() {
            let (ref original_numbers, ref my_numbers) = cards[i];

            let matches = my_numbers.iter().filter(|&&num| original_numbers.contains(&num)).count();

            for j in i + 1..std::cmp::min(i + 1 + matches, cards.len()) {
                card_counts[j] += card_counts[i];
            }
        }

        card_counts.iter().sum()
    }

}


#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::AdventOfCode;

    const CASE: &str = indoc! {"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"};

    #[test]
    fn test_a() {
        assert_eq!(AdventOfCode::part_a(CASE), 13);
    }

    #[test]
    fn test_b() {
        assert_eq!(AdventOfCode::part_b(CASE), 30);
    }
}