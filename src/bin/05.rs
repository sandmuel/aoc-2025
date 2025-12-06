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
            for range in &ingredient_db.fresh {
                if ingredient >= range.0 && ingredient <= range.1 {
                    fresh_ingredients += 1;
                }
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
        for range in ingredient_db.fresh {
            fresh_ingredient_ids += 1 + (range.1 - range.0) as usize;
        }
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
        let mut fresh: Vec<(u64, u64)> = Vec::new();
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
                    let (mut first, mut last) =
                        (first.parse::<u64>().unwrap(), last.parse::<u64>().unwrap());
                    println!("trying to add: {:?}", (first, last));

                    let mut remove_queue: Vec<(u64, u64)> = Vec::new();
                    let mut useless = false;
                    // Fancy check for overlap with previous ranges.
                    for range in &fresh {
                        // Check if this range is swallowed.
                        if range.0 < first && range.1 > last {
                            useless = true;
                            break;
                        }
                        // Check if this range swallows another.
                        if range.0 >= first && range.1 <= last {
                            remove_queue.push(*range);
                            continue;
                        }
                        // If another range overlaps with the left of our range, move first to the right of it.
                        if range.1 >= first && range.1 <= last {
                            first = range.1 + 1;
                            println!("moved first to: {}", first);
                        }
                        // Do the same check on the other side.
                        if range.0 <= last && range.0 >= first {
                            last = range.0 - 1;
                            println!("moved last to: {}", last)
                        }
                        if first > last {
                            useless = true;
                            break;
                        }
                    }

                    if !useless {
                        for range in remove_queue {
                            let idx = fresh.iter().position(|x| *x == range).unwrap();
                            fresh.remove(idx);
                        }
                        fresh.push((first, last));
                        println!("added: {:?}", (first, last));
                    }
                }
                Section::Available => {
                    available.push(line.parse::<u64>().unwrap());
                }
            }
        }

        Self { fresh, available }
    }
}
