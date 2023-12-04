use day_2::read_input;

pub fn main() -> u32 {
    let buffer = read_input();
    let mut game_ids_sum = 0;

    for line in buffer.lines() {
        let game_sets: Vec<&str> = line.split(":").last().unwrap().split("; ").collect();

        let mut min_red: u32 = 0;
        let mut min_green: u32 = 0;
        let mut min_blue: u32 = 0;

        for game_set in game_sets {
            for trial in game_set.split(",") {
                let trials: Vec<&str> = trial.split_whitespace().collect();
                match (trials[0].parse::<u32>().unwrap(), trials[1]) {
                    (x, "blue") => {
                        if min_blue == 0 {
                            min_blue = x;
                        } else {
                            min_blue = x.max(min_blue);
                        }
                    }
                    (x, "red") => {
                        if min_red == 0 {
                            min_red = x;
                        } else {
                            min_red = x.max(min_red);
                        }
                    }
                    (x, "green") => {
                        if min_green == 0 {
                            min_green = x;
                        } else {
                            min_green = x.max(min_green);
                        }
                    }
                    _ => panic!("ERROR"),
                }
            }
        }

        game_ids_sum += min_red * min_green * min_blue;
    }

    game_ids_sum
}
