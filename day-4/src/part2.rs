use day_4::read_input;

pub fn main() -> i32 {
    let buffer = read_input();
    let lines: Vec<&str> = buffer.lines().collect();
    let mut card_instances: Vec<i32> = vec![1; lines.len()];

    for (line_number, line) in lines.iter().enumerate() {
        let cards: Vec<&str> = line.split(": ").last().unwrap().split(" | ").collect();
        let winning_cards: Vec<i32> = cards[0]
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect();
        let player_cards: Vec<i32> = cards[1]
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse().unwrap())
            .filter(|x| winning_cards.contains(x))
            .collect();

        for i in 0..player_cards.len() {
            card_instances[line_number + i + 1] += card_instances[line_number]
        }
    }

    card_instances.iter().sum()
}
