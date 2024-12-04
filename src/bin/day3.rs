use std::fs;

struct Problem {
    memory: String,
}

impl Problem {
    fn from_string(string: &str) -> Self {
        Self {
            memory: string.to_string(),
        }
    }

    fn part_1(&self) -> usize {
        self.memory
            .match_indices("mul(")
            .filter_map(|(start, _)| {
                let offset = start + 4; // length of "mul("
                self.memory[offset..(offset + self.memory[offset..].find(")")?)].split_once(',')
            })
            .filter(|(left_operand, right_operand)| {
                left_operand.chars().all(|char| char.is_ascii_digit())
                    && right_operand.chars().all(|char| char.is_ascii_digit())
            })
            .map(|(left_operand, right_operand)| {
                left_operand.parse::<usize>().unwrap() * right_operand.parse::<usize>().unwrap()
            })
            .sum()
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day3.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    println!("Part 1: {}", problem.part_1()); // Attempts: 2545
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_sample_part_1() {
        assert_eq!(161, Problem::from_string(SAMPLE).part_1());
    }
}
