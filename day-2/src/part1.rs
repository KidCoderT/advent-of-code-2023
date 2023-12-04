use day_2::read_input;

pub fn main() -> u32 {
    let buffer = read_input();
    let mut game_ids_sum = 0;

    for line in buffer.lines() {
        let game_info: Vec<&str> = line.split(":").collect();
        let game_id: u32 = game_info[0].split(" ").last().unwrap().parse().unwrap();
        let game_sets: Vec<&str> = game_info[1].split("; ").collect();
        let mut not_possible = false;

        for game_set in game_sets {
            for trial in game_set.split(",") {
                let trials: Vec<&str> = trial.split_whitespace().collect();
                match (trials[0].parse::<u32>().unwrap(), trials[1]) {
                    (x, "blue") => {
                        if x > 14 {
                            not_possible = true;
                            break;
                        }
                    }
                    (x, "red") => {
                        if x > 12 {
                            not_possible = true;
                            break;
                        }
                    }
                    (x, "green") => {
                        if x > 13 {
                            not_possible = true;
                            break;
                        }
                    }
                    _ => {
                        panic!("ERROR")
                    }
                }
            }

            if not_possible {
                break;
            }
        }

        if !not_possible {
            game_ids_sum += game_id;
        }
    }

    game_ids_sum
}
