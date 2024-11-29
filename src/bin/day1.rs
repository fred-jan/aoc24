use std::fs;

fn part1() -> usize {
    let contents = fs::read_to_string("assets/day1/input.txt").expect("Failed to read input");

    10
}

fn main() {
    println!("Part 1: {}", part1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(true, true);
    }
}