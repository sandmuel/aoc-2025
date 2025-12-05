use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut map = Map::parse(reader);
        let rolls_collected = collect_rolls(&mut map);
        Ok(rolls_collected)
    }

    assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut map = Map::parse(reader);
        let mut rolls_collected = 0;
        loop {
            let new_rolls = collect_rolls(&mut map);
            rolls_collected += new_rolls;
            for y in 0..map.tiles.len() {
                for tile in &mut map.tiles[y] {
                    if *tile == TileType::ForkLift {
                        *tile = TileType::Empty;
                    }
                }
            }
            if new_rolls == 0 {
                break;
            }
        }
        Ok(rolls_collected)
    }

    assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn collect_rolls(map: &mut Map) -> usize {
    println!("{}", map);
    let mut rolls_collected = 0;
    for target_y in 0..map.tiles.len() {
        for target_x in 0..map.tiles[target_y].len() {
            // Only check for tiles with a papertowel.
            if map.get((target_x as i32, target_y as i32)) != TileType::PaperTowel {
                continue;
            }
            let mut adjacent_rolls = 0;
            for adj_y in -1isize..=1 {
                for adj_x in -1isize..=1 {
                    if adj_x == 0 && adj_y == 0 {
                        continue;
                    }
                    let sample_coords = (
                        target_x as i32 + adj_x as i32,
                        target_y as i32 + adj_y as i32,
                    );
                    let tile_type = map.get(sample_coords);
                    if tile_type != TileType::Empty {
                        adjacent_rolls += 1;
                    }
                }
            }
            if adjacent_rolls < 4 {
                map.set((target_x as i32, target_y as i32), TileType::ForkLift);
                rolls_collected += 1;
            }
        }
    }
    println!("{}", map);
    rolls_collected
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum TileType {
    Empty,
    PaperTowel,
    ForkLift,
}

struct Map {
    tiles: Vec<Vec<TileType>>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let map = String::new();
        for y in 0..self.tiles.len() {
            for tile in &self.tiles[y] {
                let symbol = match tile {
                    TileType::PaperTowel => '@',
                    TileType::ForkLift => 'x',
                    TileType::Empty => '.',
                };
                write!(f, "{}", symbol);
            }
            write!(f, "\n");
        }
        write!(f, "^ the map")
    }
}

impl Map {
    fn parse<R: BufRead>(reader: R) -> Self {
        let mut tiles = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let line = line.trim();
            let mut row = Vec::new();
            for char in line.chars() {
                row.push(match char {
                    '@' => TileType::PaperTowel,
                    'x' => TileType::ForkLift,
                    _ => TileType::Empty,
                })
            }
            tiles.push(row);
        }
        Self { tiles }
    }

    fn get(&self, coords: (i32, i32)) -> TileType {
        if coords.0 < 0 || coords.1 < 0 {
            return TileType::Empty;
        }
        if coords.1 >= self.tiles.len() as i32 {
            return TileType::Empty;
        }
        if coords.0 >= self.tiles[coords.1 as usize].len() as i32 {
            return TileType::Empty;
        }
        self.tiles[coords.1 as usize][coords.0 as usize]
    }

    fn set(&mut self, coords: (i32, i32), tile_type: TileType) {
        if coords.0 < 0 || coords.1 < 0 {
            return;
        }
        if coords.1 >= self.tiles.len() as i32 {
            return;
        }
        if coords.0 >= self.tiles[coords.1 as usize].len() as i32 {
            return;
        }
        self.tiles[coords.1 as usize][coords.0 as usize] = tile_type;
    }
}
