use std::fs;

#[derive(Debug, Eq, PartialEq)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn new(levels: Vec<i32>) -> Report {
        Self { levels }
    }

    /// Returns the index of the left operand of the first encountered unsafe difference
    fn unsafe_level_index(&self) -> Option<usize> {
        // Gradient defined as last minus first
        let gradient = self.levels.last().expect("no last level") - self.levels[0];

        match self
            .levels
            .windows(2)
            .map(|window| gradient.signum() * (window[1] - window[0])) // Normalize each sequence to be ascending
            .enumerate()
            .find(|(_, diff)| *diff < 1 || *diff > 3)
        {
            None => None,
            Some((index, _)) => Some(index),
        }
    }

    fn is_safe(&self) -> bool {
        self.unsafe_level_index().is_none()
    }

    fn is_safe_tolerated(&self) -> bool {
        match self.unsafe_level_index() {
            None => true,
            Some(index) => {
                // Test safety once more without the left level, followed by another test without the right level
                self.without_level(index).unsafe_level_index().is_none()
                    || self.without_level(index + 1).unsafe_level_index().is_none()
            }
        }
    }

    fn without_level(&self, idx: usize) -> Self {
        let mut levels = self.levels.clone();
        levels.remove(idx);

        Self { levels }
    }
}

struct Problem {
    reports: Vec<Report>,
}

impl Problem {
    fn from_string(string: &str) -> Self {
        Self {
            reports: string
                .lines()
                .map(|line| {
                    Report::new(
                        line.split_whitespace()
                            .map(|value| value.parse::<i32>().unwrap())
                            .collect(),
                    )
                })
                .collect(),
        }
    }

    fn part_1(&self) -> usize {
        self.reports
            .iter()
            .filter(|report| report.is_safe())
            .count()
    }

    fn part_2(&self) -> usize {
        self.reports
            .iter()
            .filter(|report| report.is_safe_tolerated())
            .count()
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day2.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    println!("Part 1: {}", problem.part_1()); // Attempts: 524
    println!("Part 2: {}", problem.part_2()); // Attempts: 549 (too low), 554 (too low), 568 (too low), 569
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "7 6 4 2 1
                          1 2 7 8 9
                          9 7 6 2 1
                          1 3 2 4 5
                          8 6 4 4 1
                          1 3 6 7 9";

    #[test]
    fn test_sample_part_1() {
        assert_eq!(2, Problem::from_string(SAMPLE).part_1());
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(4, Problem::from_string(SAMPLE).part_2());
    }

    #[test]
    fn test_unsafe_level_index() {
        // Sample reports
        assert_eq!(None, Report::new(vec![7, 6, 4, 2, 1]).unsafe_level_index());
        assert_eq!(
            Some(1),
            Report::new(vec![1, 2, 7, 8, 9]).unsafe_level_index()
        );
        assert_eq!(
            Some(2),
            Report::new(vec![9, 7, 6, 2, 1]).unsafe_level_index()
        );
        assert_eq!(
            Some(1),
            Report::new(vec![1, 3, 2, 4, 5]).unsafe_level_index()
        );
        assert_eq!(
            Some(2),
            Report::new(vec![8, 6, 4, 4, 1]).unsafe_level_index()
        );
        assert_eq!(None, Report::new(vec![1, 3, 6, 7, 9]).unsafe_level_index());

        // Other reports
        assert_eq!(None, Report::new(vec![1, 4, 7, 8, 10]).unsafe_level_index());
        assert_eq!(
            Some(0),
            Report::new(vec![1, 1, 7, 8, 10]).unsafe_level_index()
        );
        assert_eq!(
            Some(1),
            Report::new(vec![1, 4, 4, 8, 10]).unsafe_level_index()
        );
    }

    #[test]
    fn test_is_safe_tolerated() {
        // Sample reports
        assert_eq!(true, Report::new(vec![7, 6, 4, 2, 1]).is_safe_tolerated());
        assert_eq!(false, Report::new(vec![1, 2, 7, 8, 9]).is_safe_tolerated());
        assert_eq!(false, Report::new(vec![9, 7, 6, 2, 1]).is_safe_tolerated());
        assert_eq!(true, Report::new(vec![1, 3, 2, 4, 5]).is_safe_tolerated());
        assert_eq!(true, Report::new(vec![8, 6, 4, 4, 1]).is_safe_tolerated());
        assert_eq!(true, Report::new(vec![1, 3, 6, 7, 9]).is_safe_tolerated());

        // Other
        assert_eq!(true, Report::new(vec![1, 1, 2, 3, 4]).is_safe_tolerated());
        assert_eq!(false, Report::new(vec![1, 1, 1, 3, 4]).is_safe_tolerated());
        assert_eq!(true, Report::new(vec![4, 4, 3, 2, 1]).is_safe_tolerated());
        assert_eq!(false, Report::new(vec![4, 4, 4, 2, 1]).is_safe_tolerated());
        assert_eq!(true, Report::new(vec![2, 3, 3, 6, 7]).is_safe_tolerated());
        assert_eq!(false, Report::new(vec![3, 3, 3, 6, 7]).is_safe_tolerated());

        // This case was causing the faulty first attempts (gradient was based on first two levels)
        assert_eq!(true, Report::new(vec![3, 1, 2, 3]).is_safe_tolerated());

        // From actual input
        assert_eq!(
            true,
            Report::new(vec![47, 49, 50, 52, 53, 54, 57, 59]).is_safe_tolerated()
        );
    }

    #[test]
    fn test_without_level() {
        assert_eq!(
            Report::new(vec![2, 3]),
            Report::new(vec![1, 2, 3]).without_level(0),
        );
        assert_eq!(
            Report::new(vec![1, 3]),
            Report::new(vec![1, 2, 3]).without_level(1),
        );
        assert_eq!(
            Report::new(vec![1, 2]),
            Report::new(vec![1, 2, 3]).without_level(2),
        );
    }
}
