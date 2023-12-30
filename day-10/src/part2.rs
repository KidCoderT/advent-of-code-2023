use day_10::{read_input, verify, Direction, Pos};
use std::collections::HashSet;

pub fn main() -> u32 {
    let input: String = read_input();
    let width: usize = input.split_once("\n").unwrap().0.chars().count();
    let height: usize = input.split("\n").count();

    let four_directions = [
        Direction(-1, 0),
        Direction(1, 0),
        Direction(0, -1),
        Direction(0, 1),
    ];

    let mut map = vec![vec![String::from("."); width]; height];

    let mut start = Pos(0, 0);
    for (y, row) in input.split("\n").enumerate() {
        for (x, char) in row.chars().enumerate() {
            if char == 'S' {
                start = Pos(x, y)
            }
            map[y][x] = char.to_string();
        }
    }

    let mut directions: Vec<Vec<Pos>> = four_directions
        .iter()
        .filter(|d| {
            (start + **d).is_some()
                && verify(
                    (start + **d).unwrap(),
                    start,
                    map[(start + **d).unwrap().1][(start + **d).unwrap().0].to_string(),
                )
                .is_ok()
        })
        .map(|pos| vec![start, (start + *pos).unwrap()])
        .collect();

    // println!("{:?}", directions);
    assert_eq!(directions.iter().count(), 2);
    let mut i = 0;

    loop {
        i += 1;

        for location in directions.iter_mut() {
            let mut loc_rev = location.clone();
            loc_rev.reverse();
            // println!(
            //     "{:?}, {:?}",
            //     loc_rev,
            //     map[loc_rev[0].1][loc_rev[0].0].to_string()
            // );

            location.push(
                verify(
                    loc_rev[0],
                    loc_rev[1],
                    map[loc_rev[0].1][loc_rev[0].0].to_string(),
                )
                .unwrap(),
            )
        }

        if *directions[0].last().unwrap() == *directions[1].last().unwrap() {
            break;
        }
    }

    let walls: Vec<Pos> = {
        let mut seen = HashSet::new();
        directions
            .concat()
            .into_iter()
            .filter(|value| seen.insert(*value))
            .collect()
    };

    0
}
