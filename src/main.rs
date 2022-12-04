macro_rules! day {
    ($path_part:ident) => {
        let day = stringify!($path_part);
        let input = read_file(&format!("input/{}.txt", day));
        println!(
            "===== Day {} =====",
            day.chars().filter(|c| c.is_digit(10)).collect::<String>()
        );
        println!(
            "Part 1: {}",
            advent_of_code_2022::$path_part::part_1(&input)
        );
        println!(
            "Part 2: {}",
            advent_of_code_2022::$path_part::part_2(&input)
        );
    };
}

fn main() {
    day!(day1);
    day!(day2);
    day!(day3);
    day!(day4);
}

fn read_file(file_name: &str) -> String {
    std::fs::read_to_string(file_name).unwrap()
}
