#[derive(Default)]
pub struct AdventOfCode;

struct Position {
    index: usize,
    digit: char,
}

const NUMBER_WORDS: [(&str, char); 9] = [
("one", '1'), ("two", '2'), ("three", '3'), ("four", '4'),
("five", '5'), ("six", '6'), ("seven", '7'), ("eight", '8'), ("nine", '9'),
];

fn extract_and_convert(input: &str) -> (u32, u32) {
    let mut first_pos = Position { index: usize::MAX, digit: '0' };
    let mut last_pos = Position { index: 0, digit: '0' };

    for (i, c) in input.chars().enumerate() {
        if c.is_digit(10) {
            if i < first_pos.index {
                first_pos = Position { index: i, digit: c };
            }
            last_pos = Position { index: i, digit: c };
        }
    }

    for (word, digit) in &NUMBER_WORDS {
        if let Some(index) = input.find(word) {
            if index < first_pos.index {
                first_pos = Position { index, digit: *digit };
            }
        }
        if let Some(index) = input.rfind(word) {
            if index >= last_pos.index {
                last_pos = Position { index, digit: *digit };
            }
        }
    }

    let first_digit = if first_pos.index != usize::MAX {
        first_pos.digit.to_digit(10).unwrap()
    } else {
        0
    };

    let last_digit = if last_pos.index != 0 {
        last_pos.digit.to_digit(10).unwrap()
    } else {
        first_digit
    };

    (first_digit, last_digit)
}



impl AdventOfCode {
    pub fn part_a(input: &str) -> u32 {
        let mut sum = 0;

        for line in input.lines() {
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

            let first_number = *digits.first().unwrap_or(&0);
            let last_number = *digits.last().unwrap_or(&first_number);

            sum += first_number * 10 + last_number;
        }

        sum
    }

    pub fn part_b(input: &str) -> u32 {
        let mut sum = 0;

        for line in input.lines() {
            let (first_number, last_number)  = extract_and_convert(line);
            sum += first_number * 10 + last_number;
        }

        sum
    }
}


#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::AdventOfCode;

    const CASE_A: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    const CASE_B: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};


    #[test]
    fn test_a() {
        assert_eq!(AdventOfCode::part_a(CASE_A), 142);
    }

    #[test]
    fn test_b() {
        assert_eq!(AdventOfCode::part_b(CASE_B), 281);
    }
}