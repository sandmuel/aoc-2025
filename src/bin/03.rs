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
            fn max_joltage<const USED_BATTERIES: usize>(&self) -> u8 {
                let battery_count = self.batteries.len();
                let mut first_digit = 0;
                let mut first_digit_idx = 0;
                // We need at least one battery after this since we can't reorder.
                for i in 0..(battery_count - 1) {
                    if self.batteries[i] > first_digit {
                        first_digit = self.batteries[i];
                        first_digit_idx = i;
                    }
                }

                let mut second_digit = 0;
                for i in (first_digit_idx + 1)..battery_count {
                    if self.batteries[i] > second_digit {
                        second_digit = self.batteries[i];
                    }
                }

                let joltage = (first_digit.to_string() + &second_digit.to_string());
                joltage.parse::<u8>().unwrap()
            }
        }

        let mut total_power = 0;
        let banks = Banks::parse(reader);
        for bank in banks.banks {
            total_power += bank.max_joltage::<2>() as usize;
        }

        Ok(total_power)
    }

    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
