pub trait Solver {
    fn part1(&self) -> Result<i32, std::io::Error>;

    fn part2(&self) -> Result<i32, std::io::Error>;
}
