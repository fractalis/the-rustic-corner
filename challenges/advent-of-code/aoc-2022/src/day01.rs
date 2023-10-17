use aoc_common::Solver;
use std::str::FromStr;

aoc_macros::aoc_problem!((2022, 1, "Calorie Counting", "input/day01.txt"));

#[derive(Debug)]
struct Elf {
    calories: Vec<u32>,
}

impl FromStr for Elf {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let calories = s
            .lines()
            .map(|line| line.parse::<u32>())
            .collect::<Result<Vec<u32>, Self::Err>>()?;
        Ok(Elf { calories })
    }
}

impl Solver for AoCProblem20221 {
    fn part1(&self) -> Result<i32, std::io::Error> {
        let res = self
            .input
            .split("\n\n")
            .map(|s| s.parse::<Elf>())
            .collect::<Result<Vec<Elf>, _>>();

        match res {
            Ok(elves) => {
                return Ok(elves
                    .iter()
                    .map(|e| e.calories.iter().sum::<u32>())
                    .max()
                    .unwrap() as i32)
            }
            Err(_) => {
                panic!("Could not parse Elves!");
            }
        }
    }

    fn part2(&self) -> Result<i32, std::io::Error> {
        let mut res = self
            .input
            .split("\n\n")
            .map(|s| s.parse::<Elf>())
            .collect::<Result<Vec<Elf>, _>>()
            .unwrap()
            .iter()
            .map(|e| e.calories.iter().sum::<u32>())
            .collect::<Vec<u32>>();
        res.sort_unstable();
        Ok(res.into_iter().rev().take(3).sum::<u32>() as i32)
    }
}
