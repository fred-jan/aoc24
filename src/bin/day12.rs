use std::fs;

#[derive(Debug, Copy, Clone)]
struct Vec2 {
    x: u32,
    y: u32,
}

impl Vec2 {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Region {
    plant: char,
    locations: Vec<Vec2>,
}

impl Region {
    fn perimeter(&self) -> u32 {
        0
    }

    fn area(&self) -> u32 {
        self.locations.len() as u32
    }

    fn price(&self) -> u32 {
        self.perimeter() * self.area()
    }
}

#[derive(Debug)]
struct Plot {
    width: u32,
    height: u32,
    tiles: Vec<char>,
}

impl Plot {
    fn from_string(string: &str) -> Self {
        Self {
            width: string.find("\n").unwrap() as u32,
            height: string.lines().count() as u32,
            tiles: string.lines().flat_map(|line| line.chars()).collect(),
        }
    }

    fn regions(&self) -> Vec<Region> {
        vec![]
    }
}

#[derive(Debug)]
struct Problem {
    plot: Plot,
}

impl Problem {
    fn from_string(string: &str) -> Self {
        Self {
            plot: Plot::from_string(string),
        }
    }

    fn part_1(&self) -> u32 {
        dbg!(&self.plot);

        self.plot
            .regions()
            .iter()
            .map(|region| region.price())
            .sum()
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day12.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    println!("Part 1: {}", problem.part_1()); // Attempts:
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = r#"AAAA
BBCD
BBCC
EEEC"#;

    const SAMPLE2: &str = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;

    const SAMPLE3: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(140, Problem::from_string(SAMPLE1).part_1());
        // assert_eq!(772, Problem::from_string(SAMPLE2).part_1());
        // assert_eq!(1930, Problem::from_string(SAMPLE3).part_1());
    }
}
