use super::vec2::Vec2;
use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::ops::Sub;

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

    assert_eq!(21, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: this recursion is killing performance. We'll do it step by step instead and just
        // store the number of paths leading to each splitter, so we can deduplicate from there.
        fn cast(manifold: &Manifold, origin: Vec2) {
            let new_coords = origin + Vec2::new(0, 1);
            if new_coords.y > manifold.height as i32 {
                return;
            }
            if manifold.tiles.contains_key(&new_coords) {
                if manifold.tiles[&new_coords] == Tile::Splitter {
                    cast(manifold, new_coords - Vec2::new(1, 0));
                    cast(manifold, new_coords + Vec2::new(1, 0));
                    return;
                }
            }
            cast(manifold, new_coords);
        }

        let mut answer = 0;
        let mut manifold = Manifold::parse(reader);
        for coords in manifold.tiles.keys() {
            if manifold.tiles[coords] == Tile::Beam {
                println!("manifold height: {}", manifold.height);
                cast(&manifold, *coords);
            }
        }
        Ok(answer)
    }

    assert_eq!(40, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Eq, Hash, PartialEq)]
enum Tile {
    Unknown,
    Splitter,
    Beam,
}
struct Manifold {
    height: usize,
    tiles: HashMap<Vec2, Tile>,
}

impl Manifold {
    fn parse<R: BufRead>(reader: R) -> Self {
        let mut manifold = Manifold {
            height: 0,
            tiles: HashMap::new(),
        };
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
