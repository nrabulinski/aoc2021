const INPUT: &str = include_str!("data/day7");

fn part1(input: &[i32]) -> i32 {
	let avg = input.iter().copied().sum::<i32>() / (input.len() as i32);

	input.iter().map(|&num| (avg - num).abs()).sum()
}

fn part2(input: &[i32]) -> i32 {
	let avg = input.iter().copied().sum::<i32>() / (input.len() as i32);

	input
		.iter()
		.map(|&num| {
			let x = i32::abs(avg - num);
			x * (x + 1) / 2
		})
		.sum()
}

fn main() {
	let input: Vec<i32> = INPUT
		.trim()
		.split(',')
		.map(|num| num.parse().unwrap())
		.collect();
	println!("PART 1: {}", part1(&input));
	println!("PART 2: {}", part2(&input));
}
