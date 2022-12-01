
#[aoc_generator(day1)]
pub fn gen(input: &str) -> Vec<usize> {
        input.split('\n').map(|x| x.parse().unwrap_or(0)).collect()
		
}

#[aoc(day1, part1)]
pub fn part1(input: &[usize]) -> usize {
	input.split(|x| *x == 0).map(|elf| elf.iter().sum()).max().unwrap()
}

#[aoc(day1, part1, forloop)]
pub fn part1_forloop(input: &[usize]) -> usize {
	let mut elf = 0;
	let mut max = 0;
	for food in input {
		if *food == 0 {
			if elf > max {
				max = elf;
			}
			elf = 0;
		} else {
			elf += food;
		}
	}
	max
}

#[aoc(day1, part2)]
pub fn part2(input: &[usize]) -> usize {
	let mut elves: Vec<usize> = input.split(|x| *x == 0).map(|elf| elf.iter().sum()).collect();
	elves.sort_by(|a, b| b.cmp(a));
	elves[0] + elves[1] + elves[2]
}

#[aoc(day1, part2, faster)]
pub fn part2_faster(input: &[usize]) -> usize {
	let elves: Vec<usize> = input.split(|x| *x == 0)
									  .map(|elf| elf.iter().sum())
									  .collect();
	let (mut a, mut b, mut c) = (0,0,0);
	for elf in elves {
		if elf > c {
			if elf > b {
				if elf > a {
					c = b;
					b = a;
					a = elf;
				} else {
					c = b;
					b = elf;
				}
			} else {
				c = elf;
			}
		}
	}
	a+b+c
}

#[cfg(test)]
mod tests {
	const EXAMPLE: &'static str = indoc!{"
		1000
		2000
		3000

		4000

		5000
		6000

		7000
		8000
		9000

		10000"};

	aoc_tests!(day 1 sample1, EXAMPLE=24000; gen:part1, gen:part1_forloop);
	aoc_tests!(day 1 part1, puzzle=70374; gen:part1, gen:part1_forloop);

	aoc_tests!(day 1 sample2, EXAMPLE=45000; gen:part2, gen:part2_faster);
	aoc_tests!(day 1 part2, puzzle=204610; gen:part2, gen:part2_faster);
}
