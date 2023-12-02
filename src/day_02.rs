#[derive(Default)]
pub struct AdventOfCode;

struct Game {
    id: i32,
    red: i32,
    blue: i32,
    green: i32,
}

impl Game {
    fn new(id: i32) -> Self {
        Game { id, red: 0, blue: 0, green: 0 }
    }

    fn update(&mut self, color: &str, count: i32) {
        match color {
            "red" => self.red += count,
            "blue" => self.blue += count,
            "green" => self.green += count,
            _ => panic!("Unknown color: {}", color),
        }
    }

    fn max(&mut self, other: &Game) {
        self.red = self.red.max(other.red);
        self.blue = self.blue.max(other.blue);
        self.green = self.green.max(other.green);
    }
}

fn parse_game(line: &str) -> Option<Game> {
    let parts = line.split(": ").collect::<Vec<&str>>();
    let id = parts[0].replace("Game ", "").parse::<i32>().ok()?;
    let rounds = parts[1].split("; ").collect::<Vec<&str>>();
    let mut parse_round = Game::new(id);

    for round in rounds {
        let colors = round.split(", ").collect::<Vec<&str>>();
        let mut game_color = Game::new(id);

        for color in colors {
            let color_parts = color.split(" ").collect::<Vec<&str>>();
            let count = color_parts[0].parse::<i32>().ok()?;
            game_color.update(color_parts[1], count);
        }

        parse_round.max(&game_color);
    }

    Some(parse_round)
}

impl AdventOfCode {
    pub fn part_a(input: &str, target_red: i32, target_green: i32, target_blue: i32) -> i32 {
        input.lines()
            .filter_map(|line| parse_game(line))
            .filter(|game| game.red <= target_red && game.blue <= target_blue && game.green <= target_green)
            .map(|game| game.id)
            .sum()
    }

    pub fn part_b(input: &str) -> i32 {
        input.lines()
            .filter_map(|line| parse_game(line))
            .map(|game| game.red * game.blue * game.green)
            .sum()
    }

}


#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::AdventOfCode;

    const CASE_A: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    const CASE_B: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"};

    const CASE_A_RED: i32 = 12;
    const CASE_A_GREEN: i32 = 13;
    const CASE_A_BLUE: i32 = 14;


    #[test]
    fn test_a() {
        assert_eq!(AdventOfCode::part_a(CASE_A, CASE_A_RED, CASE_A_GREEN, CASE_A_BLUE), 8);
    }

    #[test]
    fn test_b() {
        assert_eq!(AdventOfCode::part_b(CASE_B), 2286);
    }

}