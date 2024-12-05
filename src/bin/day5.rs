use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Problem {
    rules: HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>,
}

impl Problem {
    fn from_string(string: &str) -> Self {
        let (top, bottom) = string.split_once("\n\n").expect("Invalid input");

        Self {
            rules: top
                .lines()
                .map(|line| line.split_once("|").expect("Invalid rule line"))
                .fold(HashMap::new(), |mut rules, (page_left, page_right)| {
                    rules
                        .entry(page_left.parse().unwrap())
                        .or_insert(vec![])
                        .push(page_right.parse().unwrap());

                    rules
                }),
            updates: bottom
                .lines()
                .map(|line| line.split(",").map(|page| page.parse().unwrap()).collect())
                .collect(),
        }
    }

    fn part_1(&self) -> u32 {
        self.updates
            .iter()
            .filter(|update| {
                update
                    .iter()
                    .enumerate()
                    // For each page in the update search for incorrect preceding pages
                    .find(|(update_page_idx, update_page)| {
                        match self.rules.get(update_page) {
                            // No ordering rule found, so this page is correct
                            None => false,
                            // Ordering rules found for given page
                            Some(invalid_pages) => invalid_pages
                                .iter()
                                .find(|invalid_page| {
                                    // Check preceding pages for invalid pages
                                    update[0..*update_page_idx].contains(invalid_page)
                                })
                                .is_some(),
                        }
                    })
                    // None indicates that no invalidly ordered pages were found
                    .is_none()
            })
            .map(|update| update[(update.len() - 1) / 2]) // Take the middle page
            .sum()
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day5.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    println!("Part 1: {}", problem.part_1()); // Attempts:
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "47|53\n\
                          97|13\n\
                          97|61\n\
                          97|47\n\
                          75|29\n\
                          61|13\n\
                          75|53\n\
                          29|13\n\
                          97|29\n\
                          53|29\n\
                          61|53\n\
                          97|53\n\
                          61|29\n\
                          47|13\n\
                          75|47\n\
                          97|75\n\
                          47|61\n\
                          75|61\n\
                          47|29\n\
                          75|13\n\
                          53|13\n\
                          \n\
                          75,47,61,53,29\n\
                          97,61,53,29,13\n\
                          75,29,13\n\
                          75,97,47,61,53\n\
                          61,13,29\n\
                          97,13,75,29,47";

    #[test]
    fn test_sample_part_1() {
        assert_eq!(143, Problem::from_string(SAMPLE).part_1());
    }
}
