use day_3::read_input;

#[derive(Debug, PartialEq, Eq)]
struct GearPos {
    x: usize,
    depth: usize,
}

#[derive(Debug)]
struct NumPos {
    depth: usize,
    start: usize,
    end: usize,
    num: i32,
}

fn multiply_iter(iter: Vec<i32>) -> i32 {
    let mut ans = 0;
    for val in iter {
        ans *= val;
    }
    ans
}

pub fn main() -> i32 {
    let buffer = read_input();
    let mut gear_positions: Vec<GearPos> = Vec::new();
    let mut num_positions: Vec<NumPos> = Vec::new();
    let mut connected_numbers_sum: i32 = 0;

    for (depth, line) in buffer.lines().enumerate() {
        let mut starting_index: Option<usize> = None;
        let mut num_str = String::from("");

        for (x, charector) in line.chars().enumerate() {
            match charector.to_string().parse::<u32>() {
                Ok(num) => {
                    if starting_index == None {
                        starting_index = Some(x);
                    }
                    num_str += &num.to_string();
                }
                Err(_) => {
                    if charector != '.' {
                        gear_positions.push(GearPos { x, depth })
                    }
                    if let Some(start) = starting_index {
                        num_positions.push(NumPos {
                            depth,
                            start,
                            end: x,
                            num: num_str.parse().unwrap(),
                        });
                        starting_index = None;
                        num_str = String::new();
                    }
                }
            }
        }

        if let Some(start) = starting_index {
            num_positions.push(NumPos {
                depth,
                start,
                end: line.len(),
                num: num_str.parse().unwrap(),
            });
        }
    }

    let mut connected_gears: Vec<Vec<usize>> = vec![Vec::new(); gear_positions.len()];

    for (index, num_pos) in num_positions.iter().enumerate() {
        let mut is_connected = false;

        for depth in num_pos.depth.checked_sub(1).unwrap_or(0)..=(num_pos.depth + 1) {
            for x in num_pos.start.checked_sub(1).unwrap_or(0)..=num_pos.end {
                if depth == num_pos.depth && num_pos.start <= x && x < num_pos.end {
                    continue;
                }

                let pos = GearPos { x, depth };

                if gear_positions.contains(&pos) {
                    is_connected = true;
                    connected_gears[gear_positions.iter().position(|r| r == &pos).unwrap()]
                        .push(index);
                    break;
                }
            }

            if is_connected {
                break;
            }
        }
    }

    connected_gears
        .iter()
        .filter(|list| list.len() > 1)
        .map(|list| {
            list.iter()
                .map(|index| num_positions[*index].num)
                .fold(1, |acc, x| acc * x)
        })
        .sum()
}
