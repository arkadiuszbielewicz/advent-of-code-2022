use std::str::FromStr;

type Score = u32;

pub fn part_1(input: &str) -> Score {
    input.split("\n")
        .filter_map(|line| str::parse::<Round1>(line).ok())
        .map(|round| round.total_score())
        .sum()
}

pub fn part_2(input: &str) -> Score {
    input.split("\n")
        .filter_map(|line| str::parse::<Round2>(line).ok())
        .map(|round| round.total_score())
        .sum()
}

trait Scoreable {
    fn score(&self) -> Score;
}

struct Round1 {
    pub opponent: Figure,
    pub you: Figure,
}

impl Round1 {
    fn total_score(&self) -> Score {
        self.you.score() + self.score()
    }
}

impl Scoreable for Round1 {
    fn score(&self) -> Score {
        match (&self.you, &self.opponent) {
            (Figure::Rock, Figure::Rock) => RoundResult::Draw.score(),
            (Figure::Paper, Figure::Paper) => RoundResult::Draw.score(),
            (Figure::Scissors, Figure::Scissors) => RoundResult::Draw.score(),
            (Figure::Rock, Figure::Scissors) => RoundResult::Win.score(),
            (Figure::Paper, Figure::Rock) => RoundResult::Win.score(),
            (Figure::Scissors, Figure::Paper) => RoundResult::Win.score(),
            (Figure::Scissors, Figure::Rock) => RoundResult::Lose.score(),
            (Figure::Rock, Figure::Paper) => RoundResult::Lose.score(),
            (Figure::Paper, Figure::Scissors) => RoundResult::Lose.score(),
        }
    }
}

impl FromStr for Round1 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();
        let (opponent, you) = (split[0], split[1]);
        Ok(Self {
            opponent: str::parse::<Figure>(opponent)?,
            you: str::parse::<Figure>(you)?,
        })
    }
}

struct Round2 {
    pub opponent: Figure,
    pub you: RoundResult,
}

impl Round2 {
    fn total_score(&self) -> Score {
        self.you.score() + self.score()
    }
}


impl Scoreable for Round2 {
    fn score(&self) -> Score {
        match (&self.you, &self.opponent) {
            (RoundResult::Win, Figure::Rock) => Figure::Paper.score(),
            (RoundResult::Win, Figure::Paper) => Figure::Scissors.score(),
            (RoundResult::Win, Figure::Scissors) => Figure::Rock.score(),
            (RoundResult::Draw, Figure::Scissors) => Figure::Scissors.score(),
            (RoundResult::Draw, Figure::Rock) => Figure::Rock.score(),
            (RoundResult::Draw, Figure::Paper) => Figure::Paper.score(),
            (RoundResult::Lose, Figure::Rock) => Figure::Scissors.score(),
            (RoundResult::Lose, Figure::Paper) => Figure::Rock.score(),
            (RoundResult::Lose, Figure::Scissors) => Figure::Paper.score(),
        }
    }
}

impl FromStr for Round2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();
        let (opponent, you) = (split[0], split[1]);
        Ok(Self {
            opponent: str::parse::<Figure>(opponent)?,
            you: str::parse::<RoundResult>(you)?,
        })
    }
}

enum Figure {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Figure {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Figure::Rock),
            "B" => Ok(Figure::Paper),
            "C" => Ok(Figure::Scissors),
            "X" => Ok(Figure::Rock),
            "Y" => Ok(Figure::Paper),
            "Z" => Ok(Figure::Scissors),
            &_ => Err(()),
        }
    }
}

impl Scoreable for Figure {
    fn score(&self) -> Score {
        match self {
            Figure::Rock => 1,
            Figure::Paper => 2,
            Figure::Scissors => 3,
        }
    }
}

enum RoundResult {
    Lose,
    Draw,
    Win,
}

impl Scoreable for RoundResult {
    fn score(&self) -> Score {
        match self {
            RoundResult::Lose => 0,
            RoundResult::Draw => 3,
            RoundResult::Win => 6,
        }
    }
}

impl FromStr for RoundResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RoundResult::Lose),
            "Y" => Ok(RoundResult::Draw),
            "Z" => Ok(RoundResult::Win),
            &_ => Err(()),
        }
    }
}