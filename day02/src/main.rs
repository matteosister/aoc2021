use std::iter::Sum;
use std::str::FromStr;

fn main() {
    let values: Vec<Step> = utils::from_file("day02/input");
    println!("step 1: {}", step_1(&values).multiply());
    //println!("step 2: {}", step_2(&values));
}

fn step_1(values: &Vec<Step>) -> Position {
    values
        .iter()
        .fold(Position { x: 0, y: 0 }, |current_position, step| {
            current_position.step_to(step)
        })
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn step_to(self, step: &Step) -> Self {
        match step.direction {
            Direction::Up => Self {
                y: self.y - step.amount,
                ..self
            },
            Direction::Down => Self {
                y: self.y + step.amount,
                ..self
            },
            Direction::Forward => Self {
                x: self.x + step.amount,
                ..self
            },
            Direction::Backward => Self {
                x: self.x - step.amount,
                ..self
            },
        }
    }

    fn multiply(&self) -> i32 {
        self.x * self.y
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Forward,
    Backward,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            "forward" => Ok(Self::Forward),
            "backward" => Ok(Self::Backward),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Step {
    amount: i32,
    direction: Direction,
}

impl FromStr for Step {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let idx = s.find(" ").unwrap();
        let (direction_str, amount_str) = s.split_at(idx);
        Ok(Step {
            amount: amount_str.trim().parse::<i32>().unwrap(),
            direction: direction_str.trim().parse().unwrap(),
        })
    }
}
