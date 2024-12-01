use std::fs;

struct Input {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl Input {
    fn from_string(string: String) -> Self {
        let (mut left, mut right): (Vec<u32>, Vec<u32>) = string
            .lines()
            .map(|line| {
                let mut ws = line.split_whitespace();
                (
                    ws.next().unwrap().parse::<u32>().unwrap(),
                    ws.next().unwrap().parse::<u32>().unwrap(),
                )
            })
            .unzip();

        left.sort();
        right.sort();

        Self { left, right }
    }

    fn part1(&self) -> u32 {
        self.left
            .iter()
            .zip(self.right.iter())
            .map(|(&a, &b)| a.abs_diff(b))
            .sum()
    }
}

fn part1() -> u32 {
    Input::from_string(fs::read_to_string("input/day1.txt").expect("Failed to read input"))
        .part1()
}

fn main() {
    println!("Part 1: {}", part1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            11,
            Input::from_string(
                "3   4\n\
                4   3\n\
                2   5\n\
                1   3\n\
                3   9\n\
                3   3"
                    .to_string(),
            )
            .part1()
        );
    }
}
