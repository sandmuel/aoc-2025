use std::fmt::Formatter;
use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        type Light = bool;
        type Lights = Vec<Light>;
        struct Button(Vec<usize>);

        impl Debug for Button {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
                write!(f, "(")?;
                for (i, val) in self.0.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, ")")
            }
        }

        impl Button {
            pub fn new(connections: Vec<usize>) -> Self {
                Self(connections)
            }

            pub fn connections(&self) -> &Vec<usize> {
                &self.0
            }

            fn toggle(&self, lights: &mut Lights) {
                for i in &self.0 {
                    lights[*i] = !lights[*i];
                }
            }
        }

        #[derive(Debug)]
        struct Machine {
            lights: Lights,
            target_lights: Lights,
            buttons: Vec<Button>,
        }

        impl Machine {
            fn parse(line: &str) -> Self {
                let mut target_lights = Lights::new();
                let mut buttons = Vec::<Button>::new();

                let lights_start = line.find('[').unwrap() + 1;
                let lights_end = line.find(']').unwrap() - 1;
                let button_starts = line.match_indices('(').map(|x| x.0 + 1);
                let button_ends: Vec<usize> = line.match_indices(')').map(|x| x.0 - 1).collect();
                for light in line[lights_start..=lights_end].chars() {
                    let light = match light {
                        '.' => false,
                        '#' => true,
                        _ => panic!("invalid char for light"),
                    };
                    target_lights.push(light);
                }
                for (i, start) in button_starts.enumerate() {
                    let end = button_ends[i];
                    let connections = &line[start..=end];
                    let connections = connections.split(',');
                    let connections = connections.map(|x| x.parse::<usize>().unwrap());
                    buttons.push(Button::new(connections.collect::<Vec<usize>>()));
                }
                Self {
                    lights: vec![false; lights_end - lights_start + 1],
                    target_lights,
                    buttons,
                }
            }
        }

        fn parse_machines<R: BufRead>(reader: R) -> Vec<Machine> {
            let mut machines: Vec<Machine> = Vec::new();
            for line in reader.lines() {
                let line = line.unwrap();
                let line = line.trim();
                machines.push(Machine::parse(line));
            }
            machines
        }

        let machines = parse_machines(reader);
        let mut total_presses = 0;
        for mut machine in machines {
            println!("{:?}", machine);
            let mut lights_to_change: Vec<usize> = Vec::new();
            for (i, light) in machine.target_lights.iter().enumerate() {
                if *light == true {
                    lights_to_change.push(i)
                }
            }
            while machine.lights != machine.target_lights {
                for button in &machine.buttons {
                    let mut usefulness = 0;
                    // Check if this button is useful on its own.
                    for connection in button.connections() {
                        if lights_to_change.contains(connection) {
                            usefulness += 1;
                        }
                    }
                }
            }
        }
        Ok(total_presses)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(7, part1(BufReader::new(TEST.as_bytes()))?);

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
