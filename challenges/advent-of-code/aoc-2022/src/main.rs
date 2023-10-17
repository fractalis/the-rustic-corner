use aoc_2022::day01::AoCProblem20221;
use aoc_2022::day02::AoCProblem20222;
use aoc_common::Solver;

fn print_day(day: &dyn Solver) {
    print_line();
    let res = day.part1();
    println!("| Part 1: {:^25} |", format!("{:?}", res));

    let res = day.part2();
    println!("| Part 2: {:^25} |", format!("{:?}", res));
    print_line();
}

fn print_header() {
    print_line();
    println!("| Advent of Code 2022               |");
    print_line();
}

fn print_line() {
    println!("|{}|", "-".repeat(35));
}

fn print_newline() {
    println!("\n");
}

fn print_name<T>(problem: &T)
where
    T: std::fmt::Display,
{
    println!("| {:^25}  |", problem);
}

fn main() {
    print_header();

    //=================================
    // Day 1: Calorie Counter
    print_line();

    let day = AoCProblem20221::new();
    print_name(&day);
    print_day(&day);
    //=================================

    //=================================
    // Day 2: Rock, Paper, Scissors
    //=================================
    let day = AoCProblem20222::new();
    print_name(&day);
    print_day(&day);
    print_newline();
}
