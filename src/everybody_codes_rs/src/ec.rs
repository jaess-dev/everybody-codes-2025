#![allow(dead_code)]
use std::fs;

pub trait EcDay {
    fn get_day_number(&self) -> u8;
    fn get_file_path(&self) -> String;
    fn solve_part_1(&self, input: &str) -> Option<i64>;
    fn solve_part_2(&self, input: &str) -> Option<i64>;
    fn solve_part_3(&self, input: &str) -> Option<i64>;
}

#[derive(Default)]
pub struct EcDayData {
    day: u8,
    file: String,
    solve_1_fn: Option<fn(&str) -> i64>,
    solve_2_fn: Option<fn(&str) -> i64>,
    solve_3_fn: Option<fn(&str) -> i64>,
}

impl EcDayData {
    pub const fn new(
        day: u8,
        file: String,
        solve_1_fn: Option<fn(&str) -> i64>,
        solve_2_fn: Option<fn(&str) -> i64>,
        solve_3_fn: Option<fn(&str) -> i64>,
    ) -> Self {
        Self {
            day,
            file,
            solve_1_fn,
            solve_2_fn,
            solve_3_fn,
        }
    }

    pub fn solve(&self) {
        let day_number = self.get_day_number();
        let day_file = self.get_file_path();
        let content = fs::read_to_string(&day_file).expect("Could not find input file!");

        if let Some(res) = self.solve_part_1(&content) {
            println!("Result of Day {day_number} part 1 is {res}");
        }

        if let Some(res) = self.solve_part_2(&content) {
            println!("Result of Day {day_number} part 2 is {res}");
        }

        if let Some(res) = self.solve_part_2(&content) {
            println!("Result of Day {day_number} part 3 is {res}");
        }
    }
}

impl EcDay for EcDayData {
    fn get_day_number(&self) -> u8 {
        self.day
    }

    fn get_file_path(&self) -> String {
        self.file.to_string()
    }

    fn solve_part_1(&self, input: &str) -> Option<i64> {
        match self.solve_1_fn {
            None => {
                println!("No Solution for a was given!");
                None
            }
            Some(solve) => Some(solve(input)),
        }
    }

    fn solve_part_2(&self, input: &str) -> Option<i64> {
        match self.solve_2_fn {
            None => {
                println!("No Solution for b was given!");
                None
            }
            Some(solve) => Some(solve(input)),
        }
    }

    fn solve_part_3(&self, input: &str) -> Option<i64> {
        match self.solve_3_fn {
            None => {
                println!("No Solution for part 3 was given!");
                None
            }
            Some(solve) => Some(solve(input)),
        }
    }
}
