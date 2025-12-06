use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        struct Homework {
            problems: Vec<Problem>,
        }

        impl Homework {
            fn parse<R: BufRead>(reader: R) -> Self {
                let mut homework = Homework {
                    problems: Vec::new(),
                };

                for (line_num, line) in reader.lines().enumerate() {
                    let line = line.unwrap();
                    let items = line.split(' ');
                    // Remove any empty strings left over from the splitting.
                    let items = items.filter(|x| *x != "");
                    for (i, item) in items.enumerate() {
                        if homework.problems.len() <= i {
                            homework.problems.push(Problem::default());
                        }
                        let mut problem = &mut homework.problems[i];
                        let number = item.parse::<u32>().ok();
                        match number {
                            Some(x) => problem.numbers.push(x),
                            None => problem.op = item.parse::<char>().unwrap(),
                        }
                    }
                }

                homework
            }
        }

        let homework = Homework::parse(reader);
        let mut answer = 0;
        for problem in homework.problems {
            answer += problem.solve() as usize;
        }
        Ok(answer)
    }

    assert_eq!(4277556, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        struct Homework {
            problems: Vec<Problem>,
        }

        impl Homework {
            fn parse<R: BufRead>(reader: R) -> Self {
                let mut homework = Homework {
                    problems: Vec::new(),
                };

                let mut num_lines: Vec<String> = Vec::new();
                let mut equation_starts: Vec<usize> = Vec::new();
                let mut equation_ends: Vec<usize> = Vec::new();

                let mut max_line_len = 0;
                for line in reader.lines() {
                    let line = line.unwrap();
                    if line.contains('+') || line.contains('-') {
                        for (i, char) in line.chars().enumerate() {
                            if equation_starts.len() > 0 {
                                if char == '+' || char == '*' {
                                    equation_ends.push(i - 1);
                                }
                            }
                            if char == '+' || char == '*' {
                                equation_starts.push(i);
                                homework.problems.push(Problem { numbers: Vec::new(), op: char });
                            }
                        }
                    } else {
                        num_lines.push(line.to_string());
                    }
                    if max_line_len < line.len() {
                        max_line_len = line.len()
                    }
                }
                equation_ends.push(max_line_len);

                let mut num_columns: Vec<Vec<&str>> = Vec::new();
                for (i, start) in equation_starts.iter().enumerate() {
                    let end = equation_ends[i];
                    num_columns.push(Vec::new());
                    for line in &num_lines {
                        num_columns[i].push(&line[*start..end.min(line.len())]);
                    }
                }

                let mut char_columns: Vec<Vec<String>> = Vec::new();
                for (i, column) in num_columns.iter().enumerate() {
                    char_columns.push(Vec::new());
                    let column_width = equation_ends[i] - equation_starts[i];
                    for _ in 0..column_width {
                        char_columns[i].push(String::new())
                    }
                    for num in column {
                        for (place, digit) in num.chars().enumerate() {
                            char_columns[i][place] += &digit.to_string();
                        }
                    }
                }

                for (i, column) in char_columns.iter().enumerate() {
                    for num in column {
                        homework.problems[i].numbers.push(num.trim().parse::<u32>().unwrap());
                    }
                }

                homework
            }
        }

        let homework = Homework::parse(reader);
        let mut answer = 0;
        for problem in homework.problems {
            answer += problem.solve();
        }
        Ok(answer)
    }

    assert_eq!(3263827, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Default, Debug)]
struct Problem {
    numbers: Vec<u32>,
    op: char,
}

impl Problem {
    fn solve(&self) -> usize {
        match self.op {
            '+' => repeat_op(&self.numbers, |a, b| a + b),
            '*' => repeat_op(&self.numbers, |a, b| a * b),
            _ => panic!("not a valid operation"),
        }
    }
}

fn repeat_op<F: Fn(usize, usize) -> usize>(numbers: &Vec<u32>, op: F) -> usize {
    if numbers.len() == 0 {
        panic!("can't do anything with no numbers!");
    }
    let mut answer = numbers[0] as usize;
    for (i, number) in numbers.iter().enumerate() {
        if i > 0 {
            answer = op(answer, *number as usize);
        }
    }
    answer
}
