use day_4::read_input;

pub fn main() -> i32 {
    let buffer = read_input();
    let mut total_score = 0;

    for line in buffer.lines() {
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

        if !player_cards.is_empty() {
            let mut card_score = 1;
            for _ in 0..(player_cards.len() - 1) {
                card_score *= 2;
            }
            total_score += card_score;
        }
    }

    total_score
}
