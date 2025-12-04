use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut total_power = 0;
        let banks = Banks::parse(reader);
        for bank in banks.banks {
            total_power += bank.max_joltage::<2>();
        }

        Ok(total_power)
    }

    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut total_power = 0;
        let banks = Banks::parse(reader);
        for bank in banks.banks {
            total_power += bank.max_joltage::<12>();
        }

        Ok(total_power)
    }

    assert_eq!(3121910778619, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

struct Banks {
    banks: Vec<Bank>,
}

impl Banks {
    fn parse<R: BufRead>(reader: R) -> Self {
        let mut banks = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let line = line.trim();
            let batteries = line
                .chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect();
            banks.push(Bank { batteries });
        }
        Self { banks }
    }
}

struct Bank {
    batteries: Vec<u8>,
}

impl Bank {
    fn max_joltage<const USED_BATTERIES: usize>(&self) -> usize {
        let battery_count = self.batteries.len();
        let mut digits = Vec::new();
        let mut start_idx = 0;

        for digit_idx in 0..USED_BATTERIES {
            // We need to leave enough batteries for the remaining digits.
            let max_search = (battery_count - (USED_BATTERIES - digit_idx - 1));

            // Start at the leftmost available battery.
            let mut selection_idx = start_idx;

            for i in selection_idx..max_search {
                // Pick the best battery in our selection.
                if self.batteries[i] > self.batteries[selection_idx] {
                    selection_idx = i;
                }
            }

            digits.push(self.batteries[selection_idx]);
            start_idx = selection_idx + 1;
        }

        let mut combined_batteries = String::new();
        for digit in digits {
            combined_batteries += &digit.to_string()
        }
        combined_batteries.parse::<usize>().unwrap()
    }
}
