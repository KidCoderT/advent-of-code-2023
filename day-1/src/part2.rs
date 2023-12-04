use day_1::read_input;
use std::collections::HashMap;

fn check_word(line: &str, start: usize, check: &str) -> bool {
    let end = start + check.bytes().len();
    if end > line.len() {
        return false;
    }
    check == &line[start..end]
}

pub fn main() -> i32 {
    let buffer = read_input();

    let numbers_mapping: HashMap<&str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect();

    let mut last_numbers: Vec<i32> = Vec::new();

    for line in buffer.lines() {
        let mut numbers: Vec<u32> = Vec::new();

        for (index, charector) in line.chars().enumerate() {
            let mut found_number: bool = false;

            for (word, value) in &numbers_mapping {
                if check_word(line, index, word) {
                    found_number = true;
                    numbers.push(*value);
                    break;
                }
            }

            if !found_number && charector.is_numeric() {
                numbers.push(charector.to_digit(10).unwrap())
            }
        }

        last_numbers.push(
            (format!("{}{}", numbers[0], numbers[numbers.len() - 1]))
                .parse()
                .unwrap(),
        );
    }

    let answer: i32 = last_numbers.iter().sum();
    answer
}
