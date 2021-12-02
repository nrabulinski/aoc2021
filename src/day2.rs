const INPUT: &str = include_str!("data/day2");

#[derive(Debug, Clone, Copy)]
enum Direction {
	Forward,
	Up,
	Down,
}

use Direction::*;

impl From<&str> for Direction {
	fn from(s: &str) -> Self {
		match s {
			"up" => Up,
			"down" => Down,
			"forward" => Forward,
			_ => unreachable!(),
		}
	}
}

fn part1(input: &[(Direction, i32)]) -> i32 {
	let (x, y) = input
		.iter()
		.copied()
		.fold((0, 0), |(x, y), (dir, n)| match dir {
			Forward => (x + n, y),
			Up => (x, y - n),
			Down => (x, y + n),
		});
	x * y
}

fn part2(input: &[(Direction, i32)]) -> i32 {
	let (_, x, y) = input
		.iter()
		.copied()
		.fold((0, 0, 0), |(aim, x, y), (dir, n)| match dir {
			Forward => (aim, x + n, y + aim * n),
			Up => (aim - n, x, y),
			Down => (aim + n, x, y),
		});
	x * y
}

fn main() {
	let input: Vec<(Direction, i32)> = INPUT
		.trim()
		.lines()
		.map(|line| {
			let mut line = line.split_ascii_whitespace();
			(
				line.next().unwrap().into(),
				line.next().unwrap().parse().unwrap(),
			)
		})
		.collect();
	println!("PART 1: {}", part1(&input));
	println!("PART 2: {}", part2(&input));
}
