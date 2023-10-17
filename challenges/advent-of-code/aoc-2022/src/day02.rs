use aoc_common::Solver;
use std::str::FromStr;

aoc_macros::aoc_problem!((2022, 2, "Rock, Paper, Scissors", "input/day02.txt"));

const LOSS_SCORE: i32 = 0;
const TIE_SCORE: i32 = 3;
const WIN_SCORE: i32 = 6;

enum Hand {
    Rock(i32),
    Paper(i32),
    Scissors(i32),
}

struct Game {
    rounds: Vec<(Hand, Hand)>,
}

impl Game {
    fn score_rounds(&self) -> Result<i32, GameError> {
        let res: i32 = self
            .rounds
            .iter()
            .map(|f| Self::score_round(&f.0, &f.1).unwrap())
            .sum();

        Ok(res)
    }

    fn score_round(p1_hand: &Hand, p2_hand: &Hand) -> Result<i32, GameError> {
        match (p1_hand, p2_hand) {
            (Hand::Rock(_), Hand::Rock(hand_score)) => Ok(hand_score + TIE_SCORE),
            (Hand::Rock(_), Hand::Paper(hand_score)) => Ok(hand_score + WIN_SCORE),
            (Hand::Rock(_), Hand::Scissors(hand_score)) => Ok(hand_score + LOSS_SCORE),
            (Hand::Paper(_), Hand::Rock(hand_score)) => Ok(hand_score + LOSS_SCORE),
            (Hand::Paper(_), Hand::Paper(hand_score)) => Ok(hand_score + TIE_SCORE),
            (Hand::Paper(_), Hand::Scissors(hand_score)) => Ok(hand_score + WIN_SCORE),
            (Hand::Scissors(_), Hand::Rock(hand_score)) => Ok(hand_score + WIN_SCORE),
            (Hand::Scissors(_), Hand::Paper(hand_score)) => Ok(hand_score + LOSS_SCORE),
            (Hand::Scissors(_), Hand::Scissors(hand_score)) => Ok(hand_score + TIE_SCORE),
        }
    }
}

#[derive(Debug)]
struct GameError;

impl FromStr for Game {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s
            .trim()
            .lines()
            .map(|line| {
                let mut parts = line.split(' ').take(2);
                let (p1hand, p2hand) = (parts.next().unwrap(), parts.next().unwrap());
                let p1hand = match p1hand {
                    "A" => Hand::Rock(1),
                    "B" => Hand::Paper(2),
                    "C" => Hand::Scissors(3),
                    _ => panic!("Invalid hand"),
                };
                let p2hand = match p2hand {
                    "X" => Hand::Rock(1),
                    "Y" => Hand::Paper(2),
                    "Z" => Hand::Scissors(3),
                    _ => panic!("Invalid hand"),
                };

                (p1hand, p2hand)
            })
            .collect::<Vec<(Hand, Hand)>>();

        Ok(Game { rounds: res })
    }
}

impl Solver for AoCProblem20222 {
    fn part1(&self) -> Result<i32, std::io::Error> {
        let game = Game::from_str(self.input.as_str())?;

        Ok(game.score_rounds().unwrap())
    }

    fn part2(&self) -> Result<i32, std::io::Error> {
        Ok(0)
    }
}
