
#[aoc_generator(day1)]
pub fn gen(input: &str) -> Vec<usize> {
        input.split('\n').map(|x| x.parse().unwrap_or(0)).collect()
		
}

#[aoc(day1, part1)]
pub fn part1(input: &[usize]) -> usize {
	input.split(|x| *x == 0).map(|elf| elf.iter().sum()).max().unwrap()
	
}

#[aoc(day1, part2)]
pub fn part2(input: &[usize]) -> usize {
	let mut elves: Vec<usize> = input.split(|x| *x == 0).map(|elf| elf.iter().sum()).collect();
	elves.sort_by(|a, b| b.cmp(a));
	elves[0] + elves[1] + elves[2]
}

#[aoc(day1, part2, faster)]
pub fn part2_faster(input: &[usize]) -> usize {
	let mut elves: Vec<usize> = input.split(|x| *x == 0)
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
	use super::{gen, part1, part2, part2_faster};
	
	#[test]
	fn sample1() {
		assert_eq!(part1(&gen("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000")), 24000);
	}
	
	#[test]
	fn sample2() {
		assert_eq!(part2(&gen("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000")), 45000);
	}
	#[test]
	fn sample2_faster() {
		assert_eq!(part2_faster(&gen("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000")), 45000);
	}
}
