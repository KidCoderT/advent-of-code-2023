use day_1::read_input;

pub fn main() -> i32 {
    let buffer = read_input();
    let mut last_numbers: Vec<i32> = Vec::new();

    for line in buffer.lines() {
        let numbers: Vec<u32> = Vec::from_iter(
            line.chars()
                .filter(|char| char.is_numeric())
                .map(|char| char.to_digit(10).unwrap()),
        );

        last_numbers.push(
            (format!("{}{}", numbers[0], numbers[numbers.len() - 1]))
                .parse()
                .unwrap(),
        );
    }

    let answer: i32 = last_numbers.iter().sum();
    answer
}
