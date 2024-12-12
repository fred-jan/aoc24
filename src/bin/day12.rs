use std::fs;

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: u32,
    y: u32,
}

impl Vec2 {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    fn is_adjacent(&self, other: Vec2) -> bool {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);

        dx == 0 && dy == 1 || dx == 1 && dy == 0
    }
}

#[derive(Debug, Clone)]
struct Region {
    plant: char,
    positions: Vec<Vec2>,
}

impl Region {
    fn perimeter(&self) -> u32 {
        let shared_borders: u32 = self
            .positions
            .iter()
            .enumerate()
            .map(|(index, pos)| {
                self.positions[index + 1..]
                    .iter()
                    .filter(|&&other| pos.is_adjacent(other))
                    .count() as u32
            })
            .sum();

        self.positions.len() as u32 * 4 - 2 * shared_borders
    }

    fn area(&self) -> u32 {
        self.positions.len() as u32
    }

    fn price(&self) -> u32 {
        self.perimeter() * self.area()
    }

    fn is_adjacent_pos(&self, pos: Vec2) -> bool {
        self.positions
            .iter()
            .find(|plant_pos| plant_pos.is_adjacent(pos))
            .is_some()
    }
}

#[derive(Debug)]
struct Plot {
    width: u32,
    plants: Vec<char>,
}

impl Plot {
    fn from_string(string: &str) -> Self {
        Self {
            width: string.find("\n").unwrap() as u32,
            plants: string.lines().flat_map(|line| line.chars()).collect(),
        }
    }

    fn regions(&self) -> Vec<Region> {
        let regions: Vec<Region> =
            self.plants
                .iter()
                .enumerate()
                .fold(Vec::new(), |mut regions, (index, &plant)| {
                    let pos = Vec2::new(index as u32 % self.width, index as u32 / self.width);

                    let adjacent_regions: Vec<(usize, Region)> = regions
                        .iter()
                        .enumerate()
                        .filter(|(_, region)| region.plant == plant && region.is_adjacent_pos(pos))
                        .map(|(index, region)| (index, region.clone()))
                        .collect();

                    let mut region = if adjacent_regions.len() == 0 {
                        Region {
                            plant,
                            positions: vec![],
                        }
                    } else {
                        // Multiple adjacent regions to join, so the plant is effectively connecting two
                        // or more region into one new big region. So let's remove the old regions first.
                        adjacent_regions.iter().rev().for_each(|(index, _)| {
                            regions.remove(*index);
                        });

                        // Next create one big new region containing all plants of previous regions
                        Region {
                            plant,
                            positions: adjacent_regions
                                .iter()
                                .flat_map(|(_, adjacent_region)| adjacent_region.positions.clone())
                                .collect(),
                        }
                    };

                    region.positions.push(pos);
                    regions.push(region);

                    regions
                });

        regions
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

    println!("Part 1: {}", problem.part_1()); // Attempts: 1449902
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
        assert_eq!(772, Problem::from_string(SAMPLE2).part_1());
        assert_eq!(1930, Problem::from_string(SAMPLE3).part_1());
    }
}
