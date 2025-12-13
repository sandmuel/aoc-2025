use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod vec2;
use vec2::Vec2;

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut red_tiles: Vec<Vec2> = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let line = line.trim();
            let (x, y) = line.split_once(',').unwrap();
            red_tiles.push(Vec2::new(x.parse().unwrap(), y.parse().unwrap()));
        }
        println!("red tiles: {:?}", red_tiles);
        let mut largest_area = 0;
        for tile in &red_tiles {
            for other_tile in &red_tiles {
                let width = (tile.x - other_tile.x + 1).abs() as usize;
                let height = (tile.y - other_tile.y + 1).abs() as usize;
                let area = width * height;
                if area as usize > largest_area {
                    largest_area = area as usize;
                }
            }
        }
        Ok(largest_area)
    }

    assert_eq!(50, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut red_tiles: Vec<Vec2> = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let line = line.trim();
            let (x, y) = line.split_once(',').unwrap();
            red_tiles.push(Vec2::new(x.parse().unwrap(), y.parse().unwrap()));
        }
        println!("red tiles: {:?}", red_tiles);
        let mut largest_area = 0;
        for tile in &red_tiles {
            for other_tile in &red_tiles {
                let width = (tile.x - other_tile.x + 1).abs() as usize;
                let height = (tile.y - other_tile.y + 1).abs() as usize;
                let area = width * height;
                if area as usize > largest_area {
                    // Ensure this is within a green area.
                    let mut hit_x = false;
                    let mut hit_y = false;
                    for another_tile in &red_tiles {
                        if another_tile.y == tile.y {
                            let same_x_dir =
                                (another_tile.x - tile.x < 0) == (other_tile.x - tile.x < 0);
                            if another_tile.x.abs() >= other_tile.x.abs() && same_x_dir {
                                hit_x = true;
                            }
                        }
                        if another_tile.x == tile.x {
                            let same_y_dir =
                                (another_tile.y - tile.y < 0) == (other_tile.y - tile.y < 0);
                            if another_tile.y.abs() >= other_tile.y.abs() && same_y_dir {
                                hit_y = true;
                            }
                        }
                    }
                    if hit_x && hit_y {
                        largest_area = area as usize;
                    }
                }
            }
        }
        Ok(largest_area)
    }

    assert_eq!(24, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
