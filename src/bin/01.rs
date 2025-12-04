use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut dial: Dial<0, 99> = Dial::default();
        let sequence = Sequence::parse(reader);
        let mut answer = 0;
        for rotation in sequence {
            dial.rotate(rotation);
            if dial.position == 0 {
                answer += 1;
            }
        }

        Ok(answer)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut dial: Dial<0, 99> = Dial::default();
        let sequence = Sequence::parse(reader);
        let mut answer = 0;
        for rotation in sequence {
            let cycles = dial.rotate(rotation) as usize;
            answer += cycles;
        }

        Ok(answer)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

struct Dial<const MIN: i16, const MAX: i16> {
    position: u8,
}

impl<const MIN: i16, const MAX: i16> Dial<MIN, MAX> {
    fn rotate(&mut self, rotation: Rotation) -> i16 {
        let new_pos_unwrapped = self.position as i16 + rotation.value();
        let cycles = self.cycles_in(&rotation);
        self.position = self.wrap_position(new_pos_unwrapped) as u8;
        cycles
    }

    /// Returns the nummber of cycles the dial position goes through.
    fn cycles_in(&self, rotation: &Rotation) -> i16 {
        let cycle = MAX - MIN + 1;
        let position = self.position as i16;
        let new_pos_unwrapped = self.position as i16 + rotation.value();
        let mut cycles = (new_pos_unwrapped / cycle).unsigned_abs();
        if new_pos_unwrapped <= MIN && position > 0 {
            cycles += 1;
        }
        cycles as i16
    }

    /// Wraps the given value between the first and second const generic.
    fn wrap_position(&self, position: i16) -> i16 {
        let cycle = MAX - MIN + 1;
        let partial_cycle = position % cycle;
        if partial_cycle < MIN {
            return cycle + partial_cycle;
        }
        partial_cycle
    }
}

impl<const MIN: i16, const MAX: i16> Default for Dial<MIN, MAX> {
    fn default() -> Self {
        Self { position: 50 }
    }
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

struct Sequence {
    rotations: Vec<Rotation>,
}

impl Sequence {
    fn parse<R: BufRead>(reader: R) -> impl Iterator<Item = Rotation> {
        reader.lines().map(|line| {
            let line = line.unwrap();
            let (dir, dist) = line.split_at(1);

            let dir = dir.parse::<char>().unwrap();

            let direction = match dir {
                'L' => Direction::L,
                'R' => Direction::R,
                _ => panic!("invalid direction read"),
            };

            let distance = dist.parse::<i16>().unwrap();

            Rotation {
                direction,
                distance,
            }
        })
    }
}
