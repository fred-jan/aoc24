use std::fs;

struct Problem {
    sequences: Vec<Vec<i32>>,
}

impl Problem {
    fn from_string(string: String) -> Self {
        Self {
            sequences: string
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|value| value.parse::<i32>().unwrap())
                        .collect()
                })
                .collect(),
        }
    }

    fn part1(&self) -> u32 {
        self.sequences
            .iter()
            .filter(|sequence| {
                let diff = sequence[1] - sequence[0];
                
                if diff == 0 {
                    return false;
                }
                
                let sign = diff / diff.abs(); // -1 if descending, 1 if ascending
                sequence
                    .windows(2)
                    .map(|window| sign * (window[1] - window[0])) // normalize each sequence to be ascending
                    .filter(|&diff| diff >= 1 && diff <= 3)
                    .count() == sequence.len() - 1
            })
            .count() as u32
    }
}

fn main() {
    let problem =
        Problem::from_string(fs::read_to_string("input/day2.txt").expect("Failed to read input"));

    println!("Part 1: {}", problem.part1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            2,
            Problem::from_string(
                "7 6 4 2 1\n\
                1 2 7 8 9\n\
                9 7 6 2 1\n\
                1 3 2 4 5\n\
                8 6 4 4 1\n\
                1 3 6 7 9"
                    .to_string()
            )
            .part1()
        );
    }
}
