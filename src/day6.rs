use std::collections::HashSet;


#[aoc_generator(day6)]
pub fn gen(input: &str) -> Vec<char> {
        input.chars().collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[char]) -> usize {
	input.windows(4).enumerate().filter(|(_, chars)| {
		chars[0] != chars[1] &&
		chars[0] != chars[2] &&
		chars[0] != chars[3] &&
		chars[1] != chars[2] &&
		chars[1] != chars[3] &&
		chars[2] != chars[3]
	}).next().unwrap().0+4
}

#[aoc(day6, part2)]
pub fn part2(input: &[char]) -> usize {
	input.windows(14).enumerate().filter(|(_, chars)| {
		let mut uniq = HashSet::new();
		chars.iter().all(move |x| uniq.insert(x))
	}).next().unwrap().0+14
}

#[cfg(test)]
mod tests {
	const EXAMPLE1: &'static str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
	const EXAMPLE2: &'static str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
	const EXAMPLE3: &'static str = "nppdvjthqldpwncqszvftbrmjlhg";
	const EXAMPLE4: &'static str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
	const EXAMPLE5: &'static str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

	aoc_tests!(day 6 sample1, EXAMPLE1=7; gen:part1);
	aoc_tests!(day 6 sample2, EXAMPLE2=5; gen:part1);
	aoc_tests!(day 6 sample3, EXAMPLE3=6; gen:part1);
	aoc_tests!(day 6 sample4, EXAMPLE4=10; gen:part1);
	aoc_tests!(day 6 sample5, EXAMPLE5=11; gen:part1);
	aoc_tests!(day 6 part1, puzzle=1093; gen:part1);

	aoc_tests!(day 6 sample21, EXAMPLE1=19; gen:part2);
	aoc_tests!(day 6 sample22, EXAMPLE2=23; gen:part2);
	aoc_tests!(day 6 sample23, EXAMPLE3=23; gen:part2);
	aoc_tests!(day 6 sample24, EXAMPLE4=29; gen:part2);
	aoc_tests!(day 6 sample25, EXAMPLE5=26; gen:part2);
	aoc_tests!(day 6 part2, puzzle=3534; gen:part2);
}
