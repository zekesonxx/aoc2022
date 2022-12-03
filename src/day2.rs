

use RPSMove::*;
use Outcome::*;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Outcome {
	Win = 6,
	Lose = 0,
	Tie = 3
}
impl From<char> for Outcome {
    fn from(c: char) -> Self {
        match c {
			'X' => Lose,
			'Y' => Tie,
			'Z' => Win,
			_ => panic!("malformed input")
		}
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum RPSMove {
	Rock = 1,
	Paper = 2,
	Scissors = 3
}

impl From<char> for RPSMove {
    fn from(c: char) -> Self {
        match c {
			'A' | 'X' => RPSMove::Rock,
			'B' | 'Y' => RPSMove::Paper,
			'C' | 'Z' => RPSMove::Scissors,
			_ => panic!("malformed input")
		}
    }
}


impl RPSMove {
	pub fn outcome_against(&self, opponent: &RPSMove) -> Outcome {
		match (*self, *opponent) {
			(Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Tie,
			(Rock, Paper) | (Scissors, Rock) | (Paper, Scissors) => Lose,
			(Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Win
		}
	}
	pub fn counter_with_for_outcome(&self, outcome: &Outcome) -> RPSMove {
		match (*self, *outcome) {
			(Rock, Tie) | (Paper, Lose) | (Scissors, Win) => Rock,
			(Rock, Win) | (Scissors, Lose) | (Paper, Tie) => Paper,
			(Rock, Lose) | (Paper, Win) | (Scissors, Tie) => Scissors
		}
	}
	
}


#[aoc_generator(day2)]
pub fn gen(input: &str) -> Vec<(char, char)> {
        input.split('\n')
		.map(|x| {
			let mut chars = x.chars();
			(chars.next().unwrap(), chars.skip(1).next().unwrap())
		}).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(char, char)]) -> usize {
	let moves: Vec<(RPSMove, RPSMove)> = input.iter()
		.map(|(a, b)| ((*a).into(), (*b).into())).collect();
	let mut score = 0;
	for (opponent, me) in moves {
		score += me as usize;
		score += me.outcome_against(&opponent) as usize;
	}
	score
}

#[aoc(day2, part1, variant)]
pub fn part1_variant(input: &[(char, char)]) -> usize {
	input.par_iter()
		.map(|(a, b)| {
			let a: RPSMove = (*a).into();
			let b: RPSMove = (*b).into();
			(a, b)
		}).map(|(opponent, me)| {
			me as usize + me.outcome_against(&opponent) as usize
		})
		.sum()
	}

#[aoc(day2, part2)]
pub fn part2(input: &[(char, char)]) -> usize {
	let moves: Vec<(RPSMove, Outcome)> = input.iter()
		.map(|(a, b)| ((*a).into(), (*b).into())).collect();
	let mut score = 0;
	for (opponent, outcome) in moves {
		score += outcome as usize;
		score += opponent.counter_with_for_outcome(&outcome) as usize;
	}
	score
}

#[cfg(test)]
mod tests {
	const EXAMPLE: &'static str = indoc!{"A Y
	B X
	C Z"};

	aoc_tests!(day 2 sample1, EXAMPLE=15; gen:part1, gen:part1_variant);
	aoc_tests!(day 2 part1, puzzle=11767; gen:part1, gen:part1_variant);

	aoc_tests!(day 2 sample2, EXAMPLE=12; gen:part2);
	aoc_tests!(day 2 part2, puzzle=13886; gen:part2);
}
