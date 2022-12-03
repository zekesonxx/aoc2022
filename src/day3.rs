use itertools::Itertools;


pub fn priority(c: char) -> usize {
	let point = c as u8;
	if point >= 'A' as u8 && point <= 'Z' as u8 {
		26+(1+point - 'A' as u8) as usize
	} else if point >= 'a' as u8 && point <= 'z' as u8 {
		(1+point - 'a' as u8) as usize
	} else {
		0
	}
	
}

#[aoc_generator(day3)]
pub fn gen(input: &str) -> Vec<(Vec<char>, Vec<char>)> {
        input.split('\n')
		.map(|x| {
			let (a, b) = x.split_at(x.len()/2);
			assert_eq!(a.len(), b.len());
			(a.chars().collect(), b.chars().collect())
		})
		.collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[(Vec<char>, Vec<char>)]) -> usize {
	input.iter().cloned()
	.map(|(mut a, mut b)| {a.sort(); b.sort(); (a,b)})
	.map(|(a, b)| {
		*a.iter()
		.unique()
		.filter(|a| b.contains(a))
		.last().unwrap()
	})
	.map(|x| priority(x))
	.sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &[(Vec<char>, Vec<char>)]) -> usize {
	let input: Vec<Vec<char>> = input.iter().cloned()
	.map(|(a, b)| {
		let mut a = a.clone();
		a.append(&mut b.clone());
		a
	}).collect();
	input.chunks_exact(3)
	.map(|chunk| {
		assert_eq!(chunk.len(), 3);
		let a = &chunk[0];
		let b = &chunk[1];
		let c = &chunk[2];
		*a.iter()
		.unique()
		.filter(|a| b.contains(a) && c.contains(a))
		.last().unwrap()
	})
	.map(|x| priority(x))
	.sum()
}

#[cfg(test)]
mod tests {
	const EXAMPLE: &'static str = indoc!{
		"vJrwpWtwJgWrhcsFMMfFFhFp
		jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
		PmmdzqPrVvPwwTWBwg
		wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
		ttgJtRGJQctTZtZT
		CrZsJsPPZsGzwwsLwLmpwMDw"
	};

	aoc_tests!(day 3 sample1, EXAMPLE=157; gen:part1);
	aoc_tests!(day 3 part1, puzzle=7980; gen:part1);

	aoc_tests!(day 3 sample2, EXAMPLE=70; gen:part2);
	aoc_tests!(day 3 part2, puzzle=2881; gen:part2);
}
