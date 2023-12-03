use std::str::Chars;

#[derive(Default)]
pub struct AdventOfCode;

fn find_numbers_and_positions(input: &str) -> Vec<(u32, isize, isize)> {
    let mut numbers_with_positions = Vec::new();
    let mut current_number = String::new();
    let mut start_index = 0;

    for (index, ch) in input.char_indices() {
        if ch.is_digit(10) {
            if current_number.is_empty() {
                start_index = index;
            }
            current_number.push(ch);
        } else if !current_number.is_empty() {
            if let Ok(number) = current_number.parse::<u32>() {
                numbers_with_positions.push((number, start_index as isize, index as isize));
            }
            current_number.clear();
        }
    }

    // Check for a number at the end of the string
    if !current_number.is_empty() {
        if let Ok(number) = current_number.parse::<u32>() {
            numbers_with_positions.push((number, start_index as isize, input.len() as isize));
        }
    }

    numbers_with_positions
}

fn find_stars_positions(input: &str) -> Vec<usize> {
    input.char_indices()
        .filter_map(|(index, ch)| if ch == '*' { Some(index) } else { None })
        .collect()
}


impl AdventOfCode {
    pub fn part_a(input: &str) -> u32 {
        let lines: Vec<&str> = input.lines().collect();
        let mut dot_string = String::new();
        let mut numbers_adjacent_to_a_symbol:Vec<u32> = Vec::new();

        for (index, line)  in input.lines().enumerate() {
            if dot_string.is_empty() { dot_string = ".".repeat(line.len()); }

            let line_before = if index > 0 { lines[index - 1] } else { &dot_string };
            let line_after = if index < lines.len() - 1 { lines[index + 1] } else { &dot_string };

            let numbers_with_positions = find_numbers_and_positions(line);

            for (number, start, end) in numbers_with_positions {

                let line_chars: Chars = line.chars();
                let line_before_chars: Chars = line_before.chars();
                let line_after_chars: Chars = line_after.chars();

                let mut range_start = start -1;
                let range_end = end + 1;

                if range_start < 0 || range_start as usize >= line.len()-1 {
                    range_start = 0;
                }


                let mut found = false;

                for ch in line_chars.skip(range_start as usize).take(range_end as usize - range_start as usize) {
                    if ch != '.' && !ch.is_numeric()  {
                        found = true;
                    }
                }

                for ch in line_before_chars.skip(range_start as usize).take(range_end as usize - range_start as usize) {
                    if ch != '.' && !ch.is_numeric()  {
                        found = true;
                    }
                }

                for ch in line_after_chars.skip(range_start as usize).take(range_end as usize - range_start as usize) {
                    if ch != '.' && !ch.is_numeric() {
                        found = true;
                    }
                }

                if found {
                    numbers_adjacent_to_a_symbol.push(number);
                }

            }
        }


        numbers_adjacent_to_a_symbol.iter().sum()
    }

    pub fn part_b(input: &str) -> u32 {
        let lines: Vec<&str> = input.lines().collect();
        let mut dot_string = String::new();
        let mut numbers_adjacent_to_a_symbol:Vec<u32> = Vec::new();

        for (index, line)  in input.lines().enumerate() {
            if dot_string.is_empty() { dot_string = ".".repeat(line.len()); }

            let line_before = if index > 0 { lines[index - 1] } else { &dot_string };
            let line_after = if index < lines.len() - 1 { lines[index + 1] } else { &dot_string };

            let stars_with_positions = find_stars_positions(line);


            println!("{} {} {}", line_before, line, line_after);
            for star_position in stars_with_positions {

                let line_numbers = find_numbers_and_positions(line);
                let line_before_numbers = find_numbers_and_positions(line_before);
                let line_after_numbers = find_numbers_and_positions(line_after);

                let mut number_top: Vec<u32>= Vec::new();
                let mut number_bottom: Vec<u32>= Vec::new();
                let mut number_beside: Vec<u32>= Vec::new();


                for number in line_before_numbers.iter() {
                    if star_position as isize >= (number.1 - 1)  && star_position as isize <= (number.2) {
                        number_top.push(number.0);
                    }
                }

                for number in line_after_numbers.iter() {
                    if star_position as isize >= (number.1 - 1)  && star_position as isize <= (number.2) {
                        number_bottom.push(number.0);
                    }
                }

                for number in line_numbers.iter() {
                    if star_position as isize >= (number.1 - 1)  && star_position as isize <= (number.2) {
                        number_beside.push(number.0);
                    }
                }

                if number_beside.len() == 2 {
                    numbers_adjacent_to_a_symbol.push(number_beside.iter().product());
                    println!("{} * {} = {}", number_beside[0], number_beside[1], number_beside.iter().product::<u32>())
                } else{
                    if number_beside.len() == 1 {
                        if number_top.len() == 1 {
                            numbers_adjacent_to_a_symbol.push(number_top[0] * number_beside[0]);
                            println!("{} * {} = {}", number_top[0], number_beside[0], number_top[0] * number_beside[0])
                        }
                        if number_bottom.len() == 1 {
                            numbers_adjacent_to_a_symbol.push(number_bottom[0] * number_beside[0]);
                            println!("{} * {} = {}", number_bottom[0], number_beside[0], number_bottom[0] * number_beside[0])
                        }
                    } else {
                        if number_top.len() == 2 {
                            numbers_adjacent_to_a_symbol.push(number_top.iter().product());
                            println!("{} * {} = {}", number_top[0], number_top[1], number_top.iter().product::<u32>())
                        } else {
                            if number_bottom.len() == 2 {
                                numbers_adjacent_to_a_symbol.push(number_bottom.iter().product());
                                println!("{} * {} = {}", number_bottom[0], number_bottom[1], number_bottom.iter().product::<u32>())
                            } else {
                                if number_top.len() == 1 && number_bottom.len() == 1 {
                                    numbers_adjacent_to_a_symbol.push(number_top[0] * number_bottom[0]);
                                    println!("{} * {} = {}", number_top[0], number_bottom[0], number_top[0] * number_bottom[0])
                                }
                            }
                        }
                    }
                }
            }
        }


        numbers_adjacent_to_a_symbol.iter().sum()
    }

}


#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::AdventOfCode;

    const CASE: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."};

    #[test]
    fn test_a() {
        assert_eq!(AdventOfCode::part_a(CASE), 4361);
    }

    #[test]
    fn test_b() {
        assert_eq!(AdventOfCode::part_b(CASE), 467835);
    }
}