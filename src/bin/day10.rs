use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Map {
    width: u32,
    height: u32,
    loc_heights: Vec<u8>,
}

impl Map {
    fn from_string(string: &str) -> Map {
        Self {
            width: string.find("\n").unwrap() as u32,
            height: string.lines().count() as u32,
            loc_heights: string
                .lines()
                .flat_map(|line| {
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap_or(10) as u8)
                })
                .collect(),
        }
    }

    fn height_locs(&self, height: u8) -> Vec<(u32, u32)> {
        self.loc_heights
            .iter()
            .enumerate()
            .filter(|(_, loc_height)| **loc_height == height)
            .map(|(i, _)| (i as u32 % self.width, i as u32 / self.width))
            .collect()
    }

    fn trailheads(&self) -> Vec<(u32, u32)> {
        self.height_locs(0)
    }

    fn peaks(&self) -> Vec<(u32, u32)> {
        self.height_locs(9)
    }

    fn neighbours_of(&self, x: u32, y: u32) -> Vec<(u32, u32)> {
        let mut neighbours = vec![];
        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if x < self.width - 1 {
            neighbours.push((x + 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        if y < self.height - 1 {
            neighbours.push((x, y + 1));
        }
        neighbours
    }

    fn directions_to(&self, dest_x: u32, dest_y: u32) -> HashMap<(u32, u32), (u32, u32)> {
        let mut graph = HashMap::new();
        let mut frontier = vec![(dest_x, dest_y)];
        while let Some((x, y)) = frontier.pop() {
            let height = self.loc_heights[(x + y * self.width) as usize];
            self.neighbours_of(x, y) // todo: filter x<0, x>width, y<0, y>height
                .iter()
                .filter(|(nx, ny)| self.loc_heights[(nx + ny * self.width) as usize] + 1 == height) // Only heights one lower than current
                .for_each(|(nx, ny)| {
                    frontier.push((*nx, *ny));
                    graph.insert((*nx, *ny), (x, y));
                });
        }

        graph
    }
}

#[derive(Debug)]
struct Problem {
    map: Map,
}

impl Problem {
    fn from_string(string: &str) -> Self {
        Self {
            map: Map::from_string(string),
        }
    }

    fn part_1(&self) -> usize {
        let trailheads = self.map.trailheads();

        self.map
            .peaks()
            .iter()
            .map(|(px, py)| {
                let peak_directions = self.map.directions_to(*px, *py);
                // For each trailhead check if directions to this peak are available (= reachable)
                trailheads
                    .iter()
                    .filter(|(tx, ty)| peak_directions.contains_key(&(*tx, *ty)))
                    .count()
            })
            .sum()
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day10.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    println!("Part 1: {}", problem.part_1()); // Attempts:
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_SIMPLIFIED_1: &str = r#"...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"#;

    const SAMPLE_SIMPLIFIED_2: &str = r#"..90..9
...1.98
...2..7
6543456
765.987
876....
987...."#;

    const SAMPLE_FULL: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(2, Problem::from_string(SAMPLE_SIMPLIFIED_1).part_1());
        assert_eq!(4, Problem::from_string(SAMPLE_SIMPLIFIED_2).part_1());
        assert_eq!(36, Problem::from_string(SAMPLE_FULL).part_1());
    }

    #[test]
    fn test_map_neighbours() {
        let map = Map::from_string(SAMPLE_FULL);
        assert_eq!(vec![(1, 0), (0, 1)], map.neighbours_of(0, 0));
        assert_eq!(
            vec![(0, 1), (2, 1), (1, 0), (1, 2)],
            map.neighbours_of(1, 1)
        );
        assert_eq!(vec![(6, 7), (7, 6)], map.neighbours_of(7, 7));
    }
}
