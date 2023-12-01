use indoc::indoc;

mod day_01;

fn main() {
    const CASE: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    print!("{}", day_01::AdventOfCode::part_b(CASE));
}



