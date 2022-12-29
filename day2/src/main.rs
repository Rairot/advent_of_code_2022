use anyhow::{anyhow, Result};
use utils::read_lines;

#[derive(Debug, Copy, Clone)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

trait Resolve {
    fn resolve(&self) -> u8;
}

#[derive(Debug)]
enum GameResult {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

impl Hand {
    fn rock_paper_scissors(&self, vs: Hand) -> (GameResult, u32) {
        let hand_points = *self as u32;

        match self {
            Hand::Rock => match vs {
                Hand::Rock => (GameResult::Draw, hand_points + GameResult::Draw as u32),
                Hand::Paper => (GameResult::Lose, hand_points + GameResult::Lose as u32),
                Hand::Scissors => (GameResult::Win, hand_points + GameResult::Win as u32),
            },
            Hand::Paper => match vs {
                Hand::Rock => (GameResult::Win, hand_points + GameResult::Win as u32),
                Hand::Paper => (GameResult::Draw, hand_points + GameResult::Draw as u32),
                Hand::Scissors => (GameResult::Lose, hand_points + GameResult::Lose as u32),
            },
            Hand::Scissors => match vs {
                Hand::Rock => (GameResult::Lose, hand_points + GameResult::Lose as u32),
                Hand::Paper => (GameResult::Win, hand_points + GameResult::Win as u32),
                Hand::Scissors => (GameResult::Draw, hand_points + GameResult::Draw as u32),
            },
        }
    }
}

impl TryFrom<char> for Hand {
    type Error = anyhow::Error;

    fn try_from(value: char) -> core::result::Result<Self, anyhow::Error> {
        match value.to_ascii_uppercase() {
            'A' | 'X' => Ok(Hand::Rock),
            'B' | 'Y' => Ok(Hand::Paper),
            'C' | 'Z' => Ok(Hand::Scissors),
            _ => Err(anyhow!("Unknown Hand character")),
        }
    }
}

fn main() -> Result<()> {
    let lines = read_lines(".//src//input.txt").unwrap();

    let mut total_score = 0;

    for line_result in lines {
        if let Ok(line) = line_result {
            let opponent: Hand = line.chars().nth(0).expect("Missing hand one").try_into()?;
            let player: Hand = line.chars().nth(2).expect("Missing hand two").try_into()?;

            let (_, points) = player.rock_paper_scissors(opponent);
            total_score += points
        }
    }
    println!("Total score: {total_score}");

    Ok(())
}
