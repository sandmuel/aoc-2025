use std::ops::Sub;
use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::collections::HashMap;

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        #[derive(Eq, Hash, PartialEq)]
        enum Tile {
            Unknown,
            Splitter,
            Beam,
        }

        #[derive(Eq, Hash, PartialEq, Copy, Clone)]
        struct Vec2 {
            x: i32,
            y: i32,
        }

        impl Vec2 {
            fn new(x: i32, y: i32) -> Self {
                Self { x, y }
            }
        }

        impl Add for Vec2 {
            type Output = Self;

            fn add(self, b: Vec2) -> <Self as Add<Vec2>>::Output {
                Self::Output { x: self.x + b.x, y: self.y + b.y }
            }
        }

        impl Sub for Vec2 {
            type Output = Self;

            fn sub(self, b: Vec2) -> <Self as Add<Vec2>>::Output {
                Self::Output { x: self.x - b.x, y: self.y - b.y }
            }
        }

        struct Manifold {
            height: usize,
            tiles: HashMap<Vec2, Tile>,
        }

        impl Manifold {
            fn parse<R: BufRead>(reader: R) -> Self {
                let mut manifold = Manifold { height: 0, tiles: HashMap::new() };
                for (y, line) in reader.lines().enumerate() {
                    manifold.height = y;
                    for (x, char) in line.unwrap().chars().enumerate() {
                        let tile = match char {
                            'S' => Tile::Beam,
                            '^' => Tile::Splitter,
                            _ => Tile::Unknown,
                        };
                        if tile != Tile::Unknown {
                            manifold.tiles.insert(Vec2::new(x as i32, y as i32), tile);
                        }
                    }
                }
                manifold
            }
        }

        let mut answer = 0;
        let mut manifold = Manifold::parse(reader);
        for step in 0..manifold.height {
            let mut remove_queue: Vec<Vec2> = Vec::new();
            let mut insert_queue: Vec<(Vec2, Tile)> = Vec::new();
            for coords in manifold.tiles.keys() {
                if manifold.tiles[coords] == Tile::Beam {
                    remove_queue.push(*coords);
                    // Step down a tile.
                    let new_coords = *coords + Vec2::new(0, 1);
                    if manifold.tiles.contains_key(&new_coords) {
                        if manifold.tiles[&new_coords] == Tile::Splitter {
                            insert_queue.push((new_coords - Vec2::new(1, 0), Tile::Beam));
                            insert_queue.push((new_coords + Vec2::new(1, 0), Tile::Beam));
                            answer += 1;
                            continue;
                        }
                    }
                    insert_queue.push((new_coords, Tile::Beam));
                }
            }
            for coords in remove_queue {
                manifold.tiles.remove(&coords);
            }
            for (coords, tile) in insert_queue {
                manifold.tiles.insert(coords, tile);
            }
        }
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(21, part1(BufReader::new(TEST.as_bytes()))?);

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
