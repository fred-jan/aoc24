use std::fs;

#[derive(Debug)]
struct Stones {
    elements: Vec<usize>,
}

impl Stones {
    fn from_string(string: &str) -> Self {
        Self {
            elements: string
                .split_whitespace()
                .map(|stone| stone.parse().unwrap())
                .collect(),
        }
    }

    fn blink(&self, times: u32) -> Self {
        let new = Self {
            elements: self
                .elements
                .iter()
                .flat_map(|&stone| {
                    if stone == 0 {
                        return vec![1];
                    }

                    let string = stone.to_string();

                    if string.len() % 2 == 0 {
                        let imid = string.len() / 2;
                        return vec![
                            string[0..imid].parse().unwrap(),
                            string[imid..].parse().unwrap(),
                        ];
                    }

                    vec![stone * 2024]
                })
                .collect(),
        };

        if times > 1 {
            return new.blink(times - 1);
        }

        new
    }
}

#[derive(Debug)]
struct Problem {
    stones: Stones,
}

impl Problem {
    fn from_string(string: &str) -> Self {
        Self {
            stones: Stones::from_string(string),
        }
    }

    fn part_1(&self) -> usize {
        self.stones.blink(25).elements.len()
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day11.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    println!("Part 1: {}", problem.part_1()); // Attempts: 216996
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"125 17"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(55312, Problem::from_string(SAMPLE).part_1());
    }
}
