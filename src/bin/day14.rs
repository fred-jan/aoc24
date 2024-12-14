use std::fs;
use std::ops::{Add, Div, Mul, Rem};

#[derive(Debug, Copy, Clone)]
struct Vec2i {
    x: i32,
    y: i32,
}

impl Vec2i {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Mul<i32> for Vec2i {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        Vec2i::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<i32> for Vec2i {
    type Output = Self;
    fn div(self, rhs: i32) -> Self {
        Vec2i::new(self.x / rhs, self.y / rhs)
    }
}

impl Div<Vec2i> for Vec2i {
    type Output = Self;
    fn div(self, rhs: Vec2i) -> Self {
        Vec2i::new(self.x / rhs.x, self.y / rhs.y)
    }
}

// impl Rem<i32> for Vec2i {
//     type Output = Self;
//     fn rem(self, rhs: i32) -> Self {
//         Vec2i::new(self.x % rhs, self.y % rhs)
//     }
// }

impl Rem<Vec2i> for Vec2i {
    type Output = Self;
    fn rem(self, rhs: Vec2i) -> Self {
        Vec2i::new(self.x % rhs.x, self.y % rhs.y)
    }
}

impl Add<Vec2i> for Vec2i {
    type Output = Self;
    fn add(self, rhs: Vec2i) -> Self {
        Vec2i::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug, Copy, Clone)]
struct Robot {
    position: Vec2i,
    velocity: Vec2i,
}

#[derive(Debug)]
struct Area {
    dimensions: Vec2i,
    robots: Vec<Robot>,
}

impl Area {
    fn from_string(width: u32, height: u32, string: &str) -> Self {
        Self {
            dimensions: Vec2i::new(width as i32, height as i32),
            robots: string
                .lines()
                .map(|line| line.split_once(" ").unwrap())
                .map(|(left, right)| {
                    let (px, py) = left[2..].split_once(",").unwrap();
                    let (vx, vy) = right[2..].split_once(",").unwrap();

                    Robot {
                        position: Vec2i::new(px.parse().unwrap(), py.parse().unwrap()),
                        velocity: Vec2i::new(vx.parse().unwrap(), vy.parse().unwrap()),
                    }
                })
                .collect(),
        }
    }

    fn elapse_time(&self, seconds: u32) -> Self {
        Self {
            dimensions: self.dimensions,
            robots: self
                .robots
                .iter()
                .map(|robot| {
                    let mut position =
                        (robot.position + robot.velocity * seconds as i32) % self.dimensions;

                    // Boundary warpping (teleports)
                    position = (position + self.dimensions) % self.dimensions;

                    Robot {
                        position: position,
                        velocity: robot.velocity,
                    }
                })
                .collect(),
        }
    }

    fn quadrants(&self) -> Vec<Self> {
        // dims: (11,7) -> 4x (5,3)
        // bounds: (0-4)

        let quadrant_dims = self.dimensions / 2;

        (0..2)
            .flat_map(|i| {
                (0..2).map(move |j| Self {
                    dimensions: quadrant_dims,
                    robots: self
                        .robots
                        .iter()
                        .filter(|robot| {
                            robot.position.x >= (i * quadrant_dims.x) + i
                                && robot.position.x < ((i + 1) * quadrant_dims.x) + i
                                && robot.position.y >= (j * quadrant_dims.y) + j
                                && robot.position.y < ((j + 1) * quadrant_dims.y) + j
                        })
                        .copied()
                        .collect(),
                })
            })
            .collect()
    }

    fn robot_count(&self) -> usize {
        self.robots.len()
    }
}

#[derive(Debug)]
struct Problem {
    area: Area,
}

impl Problem {
    fn from_string(width: u32, height: u32, string: &str) -> Self {
        Self {
            area: Area::from_string(width, height, string),
        }
    }

    fn part_1(&self) -> usize {
        self.area
            .elapse_time(100)
            .quadrants()
            .iter()
            .map(|quadrant| quadrant.robot_count())
            .reduce(|acc, count| acc * count)
            .unwrap()
    }
}

fn main() {
    let problem = Problem::from_string(
        101,
        103,
        fs::read_to_string("input/day14.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    println!("Part 1: {}", problem.part_1()); // Attempts:
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(12, Problem::from_string(11, 7, SAMPLE).part_1());
    }
}
