use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
<TEST-INPUT>
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        struct Dial {
            position: u8,
        }

        impl Dial {
            fn rotate(&mut self, rotation: Rotation) -> u8 {
                let new_pos_unwrapped = self.position as i16 + rotation.value();
                // Loop around the 0-99 range.
                self.position = wrap_position::<0, 99>(new_pos_unwrapped) as u8;
                self.position
            }
        }

        impl Default for Dial {
            fn default() -> Self {
                Self { position: 50 }
            }
        }

        fn wrap_position<const MIN: i16, const MAX: i16>(position: i16) -> i16 {
            let complete_turns = position / (MAX + 1);
            position - complete_turns * MAX
        }

        enum Direction {
            L,
            R,
        }

        struct Rotation {
            direction: Direction,
            distance: i16,
        }

        impl Rotation {
            fn value(&self) -> i16 {
                match self.direction {
                    Direction::L => -self.distance,
                    Direction::R => self.distance,
                }
            }
        }

        let answer = reader.lines().flatten().count();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    //assert_eq!(0, part1(BufReader::new(TEST.as_bytes()))?);

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
