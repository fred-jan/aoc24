use std::fs;

type Position = (i32, i32);

#[derive(Debug)]
struct Problem {
    bounds: (Position, Position),
    position: Position,
    obstructions: Vec<Position>,
}

impl Problem {
    fn from_string(string: &str) -> Self {
        let width = string.find("\n").unwrap();
        let position_index = string
            .split_whitespace()
            .collect::<String>()
            .find('^')
            .expect("No starting position found");

        Self {
            bounds: (
                (0, 0),
                ((width - 1) as i32, (string.lines().count() - 1) as i32),
            ),
            // row major position (index) to cartesian coordinates
            position: (
                (position_index % width) as i32,
                (position_index / width) as i32,
            ),
            obstructions: string
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|(_, char)| *char == '#')
                        .map(move |(x, _)| (x as i32, y as i32))
                })
                .collect(),
        }
    }

    fn part_1(&self) -> usize {
        let mut direction = (0, -1);
        let mut position = self.position;
        let mut visited: Vec<Position> = vec![position];

        loop {
            // Determine the next position, but do not move there yet
            let new_position = (position.0 + direction.0, position.1 + direction.1);

            // Check whether the new position lies on the map, if not stop
            if new_position.0 < self.bounds.0 .0
                || new_position.0 > self.bounds.1 .0
                || new_position.1 < self.bounds.0 .1
                || new_position.1 > self.bounds.1 .1
            {
                break;
            }

            // Obstruction at new position, so rotate direction vector 90 degrees clockwise
            if self.obstructions.contains(&new_position) {
                direction = (-direction.1, direction.0); // (x,y) = (-y, x)
                continue;
            }

            // Move to the new position
            position = new_position;

            if !visited.contains(&position) {
                visited.push(position);
            }
        }

        visited.len()
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day6.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    println!("Part 1: {}", problem.part_1()); // Attempts: 4696
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "....#.....\n\
                          .........#\n\
                          ..........\n\
                          ..#.......\n\
                          .......#..\n\
                          ..........\n\
                          .#..^.....\n\
                          ........#.\n\
                          #.........\n\
                          ......#...";

    #[test]
    fn test_sample_part_1() {
        let problem = Problem::from_string(SAMPLE);

        assert_eq!((4, 6), problem.position);
        assert_eq!(41, problem.part_1());
    }
}
