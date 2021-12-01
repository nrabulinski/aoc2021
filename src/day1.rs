const INPUT: &str = include_str!("data/day1");

fn part1(input: &[i32]) -> usize {
	input.windows(2).filter(|w| w[1] > w[0]).count()
}

fn part2(input: &[i32]) -> usize {
	let sums: Vec<i32> = input.windows(3).map(|w| w[0] + w[1] + w[2]).collect();
	part1(&sums)
}

fn main() {
	let input: Vec<i32> = INPUT
		.trim()
		.lines()
		.map(|line| line.parse().unwrap())
		.collect();
	println!("PART 1: {}", part1(&input));
	println!("PART 2: {}", part2(&input));
}
