use std::{
	cmp::{max, min},
	collections::HashMap,
};

const INPUT: &str = include_str!("data/day5");

type Point = (i32, i32);
type Line = (Point, Point);

fn part1(lines: &[Line]) -> usize {
	let mut map = HashMap::new();

	lines
		.into_iter()
		.filter(|&line| is_straight(line))
		.for_each(|&((x1, y1), (x2, y2))| {
			if y1 == y2 {
				for x in range(x1, x2) {
					let count = map.entry((x, y1)).or_insert(0);
					*count += 1;
				}
			} else {
				for y in range(y1, y2) {
					let count = map.entry((x1, y)).or_insert(0);
					*count += 1;
				}
			}
		});

	map.values().filter(|&&v| v > 1).count()
}

fn range(start: i32, end: i32) -> impl Iterator<Item = i32> {
	let step = if start < end { 1 } else { -1 };
	std::iter::successors(
		Some(start),
		move |&n| if n == end { None } else { Some(n + step) },
	)
}

fn part2(lines: &[Line]) -> usize {
	let mut map = HashMap::new();

	lines.into_iter().for_each(|&((x1, y1), (x2, y2))| {
		if y1 == y2 {
			let a = min(x1, x2);
			let b = max(x1, x2);
			for x in a..=b {
				let count = map.entry((x, y1)).or_insert(0);
				*count += 1;
			}
		} else if x1 == x2 {
			let a = min(y1, y2);
			let b = max(y1, y2);
			for y in a..=b {
				let count = map.entry((x1, y)).or_insert(0);
				*count += 1;
			}
		} else {
			for (x, y) in range(x1, x2).zip(range(y1, y2)) {
				let count = map.entry((x, y)).or_insert(0);
				*count += 1;
			}
		}
	});

	map.values().filter(|&&v| v > 1).count()
}

fn is_straight(line: &Line) -> bool {
	line.0 .0 == line.1 .0 || line.0 .1 == line.1 .1
}

fn parse_point(s: &str) -> Point {
	let mut x = s.split(',').map(|s| s.parse().unwrap());
	(x.next().unwrap(), x.next().unwrap())
}

fn parse_line(s: &str) -> Line {
	let mut x = s.split(" -> ").map(parse_point);
	(x.next().unwrap(), x.next().unwrap())
}

fn main() {
	let lines: Vec<_> = INPUT.trim().lines().map(parse_line).collect();
	println!("PART 1: {}", part1(&lines));
	println!("PART 2: {}", part2(&lines));
}
