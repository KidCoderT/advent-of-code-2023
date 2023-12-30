use std::fs;

pub fn read_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug, Clone, Copy)]
pub struct Direction(pub isize, pub isize);

impl Direction {
    pub fn get_possible_pipe(&self) -> Vec<&str> {
        match (self.0, self.1) {
            (-1, 0) => vec!["L", "F", "-"],
            (1, 0) => vec!["J", "7", "-"],
            (0, -1) => vec!["7", "F", "|"],
            (0, 1) => vec!["L", "J", "|"],
            _ => panic!("DIRECTION CANNOT BE > 1"),
        }
    }

    pub fn get_from_pipe(pipe: &str) -> (Self, Self) {
        match pipe {
            "L" => (Direction(1, 0), Direction(0, -1)),
            "J" => (Direction(0, -1), Direction(-1, 0)),
            "7" => (Direction(0, 1), Direction(-1, 0)),
            "F" => (Direction(0, 1), Direction(1, 0)),
            "|" => (Direction(0, 1), Direction(0, -1)),
            "-" => (Direction(1, 0), Direction(-1, 0)),
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(pub usize, pub usize);

impl std::ops::Sub for Pos {
    type Output = Direction;

    fn sub(self, other: Self) -> Direction {
        Direction(
            (self.0 as isize) - (other.0 as isize),
            (self.1 as isize) - (other.1 as isize),
        )
    }
}

impl std::ops::Add<Direction> for Pos {
    type Output = Option<Self>;

    fn add(self, other: Direction) -> Option<Self> {
        let x = self.0.checked_add_signed(other.0);
        let y = self.1.checked_add_signed(other.1);

        if x.is_none() || y.is_none() {
            return None;
        }

        Some(Pos(x.unwrap(), y.unwrap()))
    }
}

impl std::ops::Sub<Direction> for Pos {
    type Output = Option<Self>;

    fn sub(self, other: Direction) -> Option<Self> {
        let x = self.0.checked_sub(other.0 as usize);
        let y = self.1.checked_sub(other.1 as usize);

        match (x, y) {
            (Some(new_x), Some(new_y)) => Some(Pos(new_x, new_y)),
            _ => None,
        }
    }
}

pub fn verify(current: Pos, last: Pos, value: String) -> Result<Pos, &'static str> {
    let direction_of_last_movement = current - last;

    if !direction_of_last_movement
        .get_possible_pipe()
        .contains(&value.as_str())
    {
        return Err("FAILED1");
    }

    let (d1, d2) = Direction::get_from_pipe(value.as_str());
    let pos1 = current + d1;
    let pos2 = current + d2;

    if pos1.is_none() {
        if pos2.is_none() {
            return Err("FAILED2");
        }
        return Ok(pos2.unwrap());
    } else if pos2.is_some() && pos2.unwrap() != last {
        return Ok(pos2.unwrap());
    } else {
        return Ok(pos1.unwrap());
    }
}
