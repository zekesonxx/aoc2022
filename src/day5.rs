use itertools::Itertools;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
	pub amount: usize,
	pub from: usize,
	pub to: usize
}

#[aoc_generator(day5)]
pub fn gen(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
	let (yard, moves) = input.split_once("\n\n").unwrap();
	let mut yard = yard.split('\n').rev(); // go from bottom-to-top
	let numbers = yard.next().unwrap();
	let mut parsed_yard = vec![];
	// make the empty arrays
	for _ in numbers.split_whitespace() {
		parsed_yard.push(vec![]);
	}
	// parse out the boxes from bottom to top
	for line in yard {
		for (pile, value) in line.chars().skip(1).step_by(4).enumerate() {
			if !value.is_whitespace() {
				parsed_yard[pile].push(value);
			}
		}
	}

	let moves: Vec<Move> = moves.split('\n').map(|line| {
		let mut fields = line.split_whitespace()
		.skip(1)
		.step_by(2)
		.map(|z| z.parse().unwrap_or(0));
		Move {
			amount: fields.next().unwrap_or(0),
			from: fields.next().unwrap_or(0),
			to: fields.next().unwrap_or(0)
		}
	}).collect();
	(parsed_yard, moves)
}

#[aoc(day5, part1)]
pub fn part1(input: &(Vec<Vec<char>>, Vec<Move>)) -> String {
	let mut yard = input.0.clone();
	let moves = &input.1;
	for m in moves {
		for _ in 0..m.amount {
			let c = yard[m.from-1].pop().unwrap();
			yard[m.to-1].push(c);
		}
	}
	yard.iter_mut().map(|x| x.pop().unwrap()).join("")
}

#[aoc(day5, part2)]
pub fn part2(input: &(Vec<Vec<char>>, Vec<Move>)) -> String {
	let mut yard = input.0.clone();
	let moves = &input.1;
	for m in moves {
		let drain_position = yard[m.from-1].len()-m.amount;
		for _ in 0..m.amount {
			let c = yard[m.from-1].remove(drain_position);
			yard[m.to-1].push(c);
		}
	}
	yard.iter_mut().map(|x| x.pop().unwrap()).join("")
}

#[cfg(test)]
mod tests {
	const EXAMPLE: &'static str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

	aoc_tests!(day 5 sample1, EXAMPLE="CMZ"; gen:part1);
	aoc_tests!(day 5 part1, puzzle="JRVNHHCSJ"; gen:part1);

	aoc_tests!(day 5 sample2, EXAMPLE="MCD"; gen:part2);
	aoc_tests!(day 5 part2, puzzle="GNFBSBJLH"; gen:part2);
}
