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

        fn valid_id(product_id: &ProductId) -> usize {
            let first = product_id.first.value.to_string();
            let last = product_id.last.value.to_string();

            let invalidness = 0;
            if product.first.zero_prefix || repeating_segments(&first) {

            }
            if product.last.zero_prefix || repeating_segments(&last) {

            }
        }

        fn repeating_segments(string: &String) -> bool {
            const SEGMENT_LEN: usize = 2;
            // Go through it like so:
            // [1__2][3__4][5__6]
            let mut quantity = string.len() / SEGMENT_LEN; // Any odd number is ignored in int division.
            let mut segments = Vec::new();
            for i in 0..quantity {
                let seg_start = i * 2;
                let seg_end = seg_start + 1;
                segments.push(&string[seg_start..seg_end]);
            }
            let mut first_overlaps = false;
            for segment in segments {
                if string.find(segment).unwrap() > 1 {
                    first_overlaps = true;
                }
            }

            // And now like so:
            // _1_[2__3][4__5]_6_
            if string.len() % SEGMENT_LEN == 0 {
                quantity -= 1; // Any even number can fit one fewer segment from orginal alignment.
            }
            let mut segments = Vec::new();
            for i in 0..quantity {
                let seg_start = 1 + i * 2;
                let seg_end = seg_start + 1;
                segments.push(&string[seg_start..seg_end]);
            }
            let mut last_overlaps = false;
            for segment in segments {
                if string.find(segment).unwrap() > 1 {
                    last_overlaps = true;
                }
            }

            first_overlaps || last_overlaps
        }

        let sequence = Sequence::parse(reader);
        let mut answer = 0;
        for product_id in sequence {
            answer += !valid_id(&product_id) as usize;
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
