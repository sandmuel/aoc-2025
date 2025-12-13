use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::ops::Sub;
use std::sync::atomic::AtomicU32;

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        #[derive(Copy, Clone, Debug)]
        struct Vec3 {
            x: i32,
            y: i32,
            z: i32,
        }

        impl Vec3 {
            fn new(x: i32, y: i32, z: i32) -> Self {
                Self { x, y, z }
            }

            fn dist(&self, other: &Self) -> f32 {
                (self.x.pow(2) as f32 + self.y.pow(2) as f32 + self.z.pow(2) as f32).sqrt()
            }
        }

        impl Add for Vec3 {
            type Output = Self;
            fn add(self, other: Self) -> Self::Output {
                Self {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                }
            }
        }

        #[derive(Clone, Debug)]
        struct JunctionBox {
            position: Vec3,
            group: Option<AtomicU32>,
        }

        struct JunctionBoxes {
            boxes: Vec<JunctionBox>,
        }

        impl JunctionBoxes {
            fn parse<R: BufRead>(reader: R) -> Self {
                let mut boxes: Vec<JunctionBox> = Vec::new();
                for line in reader.lines() {
                    let line = line.unwrap();
                    let line = line.trim();
                    let vec3_data = line.split(',');
                    let vec3_data: Vec<i32> =
                        vec3_data.map(|x| x.parse::<i32>().unwrap()).collect();
                    let position = Vec3::new(vec3_data[0], vec3_data[1], vec3_data[2]);
                    boxes.push(JunctionBox {
                        position,
                        group: None,
                    });
                }
                Self { boxes }
            }
        }

        let mut junction_boxes = JunctionBoxes::parse(reader);
        let boxes_iter = junction_boxes();
        for node in &junction_boxes.boxes {
            let mut nearest_dist = f32::MAX;
            let mut nearest_other = None;
            for other in &junction_boxes.boxes {
                if node.position.dist(&other.position) < nearest_dist {
                    nearest_dist = node.position.dist(&other.position);
                    nearest_other = Some(other);
                }
            }
            if node.group.is_some() && nearest_other.unwrap().group.is_some() {
                if node.group.unwrap().get() == nearest_other.unwrap().group.unwrap().get() {
                    continue;
                }
            } else if node.group.is_some() {
                nearest_other.unwrap().group = node.group;
            } else if nearest_other.uwrap().group.is_some() {
                node.group = nearest_other.unwrap().group;
            } else {
                nearest_other.unwrap().group = node.group;
                node.group = nearest_other.unwrap().group;
            }
        }
        let mut groups = HashMap::new();
        for node in &junction_boxes.boxes {
            if !groups.contains(node.group.unwrap()) {
                groups.insert(node.group.unwrap(), 1);
            }
        }
        println!("{:?}", junction_boxes);
        Ok(0)
    }

    assert_eq!(40, part1(BufReader::new(TEST.as_bytes()))?);

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
