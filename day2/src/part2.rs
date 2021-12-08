use std::ops;

use crate::pair::Pair;

const FORWARD: &str = "forward";
const DOWN: &str = "down";
const UP: &str = "up";

#[derive(Debug)]
struct Loc {
    x: i64, // x : horizontal
    y: i64, // y : vertical
    aim: i64,
}
impl Loc {
    pub fn new() -> Self {
        Self { x: 0, y: 0, aim: 0 }
    }

    pub fn new_with_init(x: i64, y: i64, aim: i64) -> Self {
        Self { x, y, aim }
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
            FORWARD => {
                self.x += rhs.shift;
                self.y += self.aim * rhs.shift;
            }
            DOWN => self.aim += rhs.shift,
            UP => self.aim -= rhs.shift,
            _ => {}
        }
    }
}

impl ops::Add<&Pair> for Loc {
    type Output = Loc;

    fn add(self, rhs: &Pair) -> Self::Output {
        match rhs.operator.as_str() {
            FORWARD => {
                Loc::new_with_init(self.x + rhs.shift, self.y + self.aim * rhs.shift, self.aim)
            }
            DOWN => Loc::new_with_init(self.x, self.y, self.aim + rhs.shift),
            UP => Loc::new_with_init(self.x, self.y, self.aim - rhs.shift),
            _ => Loc::new(),
        }
    }
}

pub fn part2(vec: &Vec<Pair>) {
    let mut result = Loc::new();

    // FP style
    // result = vec.iter().fold(result, |acc, p| acc + p);

    for p in vec {
        result += p;
    }

    println!("{}", result);
}
