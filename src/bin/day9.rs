use std::fs;

#[derive(Debug)]
struct Filesystem {
    sectors: Vec<Option<u32>>,
}

impl Filesystem {
    fn new() -> Self {
        Self { sectors: vec![] }
    }

    fn compact(&self) -> Self {
        let files = self.sectors.iter().filter(|sector| sector.is_some());
        let file_count = files.clone().count();

        let mut right = files.rev();

        Self {
            sectors: [
                self.sectors[0..file_count]
                    .iter()
                    .map(|&sector| {
                        match sector {
                            None => *right.next().unwrap(), // Free space -> fill from right
                            Some(_) => sector,
                        }
                    })
                    .collect(),
                vec![None; self.sectors.len() - file_count], // Pad with empty space
            ]
            .concat(),
        }
    }

    fn checksum(&self) -> usize {
        self.sectors
            .iter()
            .enumerate()
            .map(|(i, sector)| i * sector.unwrap_or(0) as usize)
            .sum()
    }
}

#[derive(Debug)]
struct Problem {
    filesystem: Filesystem,
}

impl Problem {
    fn from_string(string: &str) -> Self {
        Self {
            filesystem: string.lines().nth(0).unwrap().chars().enumerate().fold(
                Filesystem::new(),
                |mut acc, (i, char)| {
                    let size = char.to_digit(10).unwrap() as usize;

                    if i % 2 == 0 {
                        acc.sectors.append(&mut vec![Some(i as u32 / 2); size])
                    } else {
                        acc.sectors.append(&mut vec![None; size]);
                    }

                    acc
                },
            ),
        }
    }

    fn part_1(&self) -> usize {
        self.filesystem.compact().checksum()
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day9.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    println!("Part 1: {}", problem.part_1()); // Attempts:
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2333133121414131402";

    #[test]
    fn test_sample_part_1() {
        let problem = Problem::from_string(SAMPLE);

        assert_eq!(1928, problem.part_1());
    }
}
