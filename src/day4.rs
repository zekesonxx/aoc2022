use std::ops::RangeInclusive;

type Elf = RangeInclusive<usize>;

#[aoc_generator(day4)]
pub fn gen(input: &str) -> Vec<(Elf, Elf)> {
        input
		.split('\n')
		.map(|x| {
			let mut iter = x.split(',').map(|y| {
				let mut iter = y.split('-').map(|z| z.parse().unwrap_or(0));
				RangeInclusive::new(iter.next().unwrap_or(0), iter.next().unwrap_or(0))
			});
			(iter.next().unwrap_or(0..=0), iter.next().unwrap_or(0..=0))
		}).collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[(Elf, Elf)]) -> usize {
	input.iter().map(|(a, b)| {
		if a.contains(b.start()) && a.contains(b.end()) {
			1
		} else if b.contains(a.start()) && b.contains(a.end()) {
			1
		} else {
			0
		}
	}).sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &[(Elf, Elf)]) -> usize {
	input.iter().map(|(a, b)| {
		if a.contains(b.start()) || a.contains(b.end()) {
			1
		} else if b.contains(a.start()) || b.contains(a.end()) {
			1
		} else {
			0
		}
	}).sum()
}

#[cfg(test)]
mod tests {
	const EXAMPLE: &'static str = indoc!{"2-4,6-8
	2-3,4-5
	5-7,7-9
	2-8,3-7
	6-6,4-6
	2-6,4-8"};

	aoc_tests!(day 4 sample1, EXAMPLE=2; gen:part1);
	//aoc_tests!(day 4 part1, puzzle=0; gen:part1);

	aoc_tests!(day 4 sample2, EXAMPLE=4; gen:part2);
	//aoc_tests!(day 4 part2, puzzle=0; gen:part2);
}
