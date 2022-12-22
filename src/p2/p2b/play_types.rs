use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Play {
    Rock,
    Paper,
    Scisor,
}

fn value_play(play: &Play) -> i32 {
    match play {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scisor => 3,
    }
}

impl FromStr for Play {
    type Err = ();

    fn from_str(input: &str) -> Result<Play, Self::Err> {
        match input {
            "A" => Ok(Play::Rock),
            "B" => Ok(Play::Paper),
            "C" => Ok(Play::Scisor),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RoundResult {
    Lose,
    Win,
    Draw,
}

impl FromStr for RoundResult {
    type Err = ();

    fn from_str(input: &str) -> Result<RoundResult, Self::Err> {
        match input {
            "X" => Ok(RoundResult::Lose),
            "Y" => Ok(RoundResult::Draw),
            "Z" => Ok(RoundResult::Win),
            _ => Err(()),
        }
    }
}

pub struct Round {
    oponent: Play,
    mine: Play,
}

impl Round {
    pub fn from_result(oponent: Play, result: RoundResult) -> Round {
        match (oponent, result) {
            (Play::Rock, RoundResult::Lose) => Round {
                oponent: Play::Rock,
                mine: Play::Scisor,
            },
            (Play::Paper, RoundResult::Lose) => Round {
                oponent: Play::Paper,
                mine: Play::Rock,
            },
            (Play::Scisor, RoundResult::Lose) => Round {
                oponent: Play::Scisor,
                mine: Play::Paper,
            },
            (Play::Rock, RoundResult::Draw) => Round {
                oponent: Play::Rock,
                mine: Play::Rock,
            },
            (Play::Paper, RoundResult::Draw) => Round {
                oponent: Play::Paper,
                mine: Play::Paper,
            },
            (Play::Scisor, RoundResult::Draw) => Round {
                oponent: Play::Scisor,
                mine: Play::Scisor,
            },
            (Play::Rock, RoundResult::Win) => Round {
                oponent: Play::Rock,
                mine: Play::Paper,
            },
            (Play::Paper, RoundResult::Win) => Round {
                oponent: Play::Paper,
                mine: Play::Scisor,
            },
            (Play::Scisor, RoundResult::Win) => Round {
                oponent: Play::Scisor,
                mine: Play::Rock,
            },
        }
    }

    fn score(&self) -> i32 {
        match (&self.oponent, &self.mine) {
            (Play::Rock, Play::Scisor) => 0,
            (Play::Paper, Play::Rock) => 0,
            (Play::Scisor, Play::Paper) => 0,
            (Play::Rock, Play::Rock) => 3,
            (Play::Paper, Play::Paper) => 3,
            (Play::Scisor, Play::Scisor) => 3,
            (Play::Rock, Play::Paper) => 6,
            (Play::Paper, Play::Scisor) => 6,
            (Play::Scisor, Play::Rock) => 6,
        }
    }

    pub fn total_score(&self) -> i32 {
        value_play(&self.mine) + self.score()
    }
}

#[derive(Debug, Deserialize)]
pub struct Record {
    pub oponent: String,
    pub mine: String,
}
