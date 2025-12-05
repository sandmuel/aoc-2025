use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut fresh_ingredients = 0;
        let ingredient_db = IngredientDb::parse(reader);
        for ingredient in ingredient_db.available {
            let mut is_fresh = false;
            for range in &ingredient_db.fresh {
                if ingredient >= range.0 && ingredient <= range.1 {
                    is_fresh = true;
                }
            }
            if is_fresh {
                fresh_ingredients += 1;
            }
        }
        Ok(fresh_ingredients)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut fresh_ingredient_ids = 0;
        let ingredient_db = IngredientDb::parse(reader);

        Ok(fresh_ingredient_ids)
    }

    assert_eq!(14, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

struct IngredientDb {
    fresh: Vec<(u64, u64)>,
    available: Vec<u64>,
}

impl IngredientDb {
    fn parse<R: BufRead>(reader: R) -> Self {
        let mut fresh = Vec::new();
        let mut available = Vec::new();

        enum Section {
            Fresh,
            Available,
        }
        let mut section = Section::Fresh;
        for line in reader.lines() {
            let line = line.unwrap();
            // Switch sections once we reach the empty seperator line.
            if line == "" {
                section = Section::Available;
                continue;
            }
            match section {
                Section::Fresh => {
                    let (first, last) = line.split_once('-').unwrap();
                    let (first, last) = (first.parse::<u64>().unwrap(), last.parse::<u64>().unwrap());

                    // Fancy check for overlap.

                    fresh.push((first, last));
                },
                Section::Available => {
                    available.push(line.parse::<u64>().unwrap());
                }
            }
        }

        Self { fresh, available }
    }
}
