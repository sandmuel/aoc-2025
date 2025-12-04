use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        struct ProductId {
            first: IdPart,
            last: IdPart,
        }

        struct IdPart {
            zero_prefix: bool,
            value: u64,
        }

        struct Sequence {
            product_ids: Vec<ProductId>,
        }

        impl Sequence {
            fn parse<R: BufRead>(reader: R) -> impl Iterator<Item = ProductId> {
                let entries = reader.split(b',');
                entries.map(|entry| {
                    let entry = String::try_from(entry.unwrap()).unwrap();
                    // Newlines stop us from parsing the u64s.
                    let entry = entry.trim();
                    let (first, last) = entry.split_once('-').unwrap();
                    let first_zero_prefix = if first.chars().nth(0).unwrap() == '0' { true } else { false };
                    let last_zero_prefix = if last.chars().nth(0).unwrap() == '0' { true } else { false };
                    let first = IdPart { zero_prefix: first_zero_prefix, value: first.parse::<u64>().unwrap() };
                    let last = IdPart { zero_prefix: last_zero_prefix, value: last.parse::<u64>().unwrap() };
                    ProductId { first, last }
                })
            }
        }

        fn invalid_ids(product_id: &ProductId) -> Vec<u64> {
            let mut invalid_ids = Vec::new();
            let (first, last) = (product_id.first.value, product_id.last.value);
            for id_num in first..last {
                let id = id_num.to_string();
                if id.len() % 2 != 0 {
                    continue;
                }
                let pattern_length = id.len() / 2;
                let (first_half, second_half) = id.split_at(pattern_length);
                if first_half == second_half {
                    invalid_ids.push(id_num);
                }
            }
            invalid_ids
        }

        let sequence = Sequence::parse(reader);
        let mut answer = 0;
        for product_id in sequence {
            let mut invalid_id_sum = 0;
            for invalid_id in invalid_ids(&product_id) {
                invalid_id_sum += invalid_id;
            }
            answer += invalid_id_sum as usize;
        }

        Ok(answer)
    }

    //assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);

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
