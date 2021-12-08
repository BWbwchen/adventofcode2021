use std::ops;

use crate::pair::Pair;

const FORWARD: &str = "forward";
const DOWN: &str = "down";
const UP: &str = "up";

#[derive(Debug)]
struct Loc {
    x: i64, // x : horizontal
    y: i64, // y : vertical
}
impl Loc {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
    pub fn represent(self) -> i64 {
        self.x * self.y
    }
}

impl std::fmt::Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.x * self.y)
    }
}

impl ops::AddAssign<&Pair> for Loc {
    fn add_assign(&mut self, rhs: &Pair) {
        match rhs.operator.as_str() {
            FORWARD => self.x += rhs.shift,
            DOWN => self.y += rhs.shift,
            UP => self.y -= rhs.shift,
            _ => {}
        }
    }
}

impl ops::Add<&Pair> for Loc {
    type Output = Loc;

    fn add(self, rhs: &Pair) -> Self::Output {
        match rhs.operator.as_str() {
            FORWARD => Loc {
                x: self.x + rhs.shift,
                y: self.y,
            },
            DOWN => Loc {
                x: self.x,
                y: self.y + rhs.shift,
            },
            UP => Loc {
                x: self.x,
                y: self.y - rhs.shift,
            },
            _ => Loc::new(),
        }
    }
}

pub fn part1(vec: &Vec<Pair>) {
    let mut result = Loc::new();

    // FP style
    // result = vec.iter().fold(result, |acc, p| acc + p);

    for p in vec {
        result += p;
    }

    println!("{:?}", result.represent());
}
