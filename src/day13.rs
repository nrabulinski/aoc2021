use std::collections::HashSet;

const INPUT: &str = include_str!("data/day13");

fn part1(points: &[Point], fold: Fold) -> usize {
	points
		.iter()
		.map(|&point| translate(point, fold))
		.collect::<HashSet<_>>()
		.len()
}

fn part2(mut points: Vec<Point>, folds: &[Fold]) {
	for &fold in folds {
		for point in points.iter_mut() {
			*point = translate(*point, fold);
		}
	}

	let mut set = HashSet::new();
	let mut y = 0;
	let mut x = 0;

	for point in points.into_iter() {
		set.insert(point);
		if point.0 > x {
			x = point.0;
		}
		if point.1 > y {
			y = point.1;
		}
	}

	for y in 0..=y {
		for x in 0..=x {
			print!("{}", if set.contains(&(x, y)) { '#' } else { '.' });
		}
		println!();
	}
}

fn translate((x, y): Point, fold: Fold) -> Point {
	match fold {
		Fold::X(a) if x > a => (2 * a - x, y),
		Fold::Y(a) if y > a => (x, 2 * a - y),
		_ => (x, y),
	}
}

#[derive(Debug, Clone, Copy)]
enum Fold {
	X(usize),
	Y(usize),
}

type Point = (usize, usize);

fn get_input() -> (Vec<Point>, Vec<Fold>) {
	let mut lines = INPUT.trim().lines();

	let points: Vec<_> = std::iter::from_fn(|| lines.next())
		.take_while(|line| !line.is_empty())
		.map(|line| {
			let (x, y) = line.split_once(',').unwrap();
			(x.parse().unwrap(), y.parse().unwrap())
		})
		.collect();

	let folds: Vec<_> = lines
		.map(|line| {
			let (a, num) = line.split_once('=').unwrap();

			let num = num.parse().unwrap();

			match a.as_bytes()[a.len() - 1] {
				b'x' => Fold::X(num),
				b'y' => Fold::Y(num),
				_ => unreachable!(),
			}
		})
		.collect();

	(points, folds)
}

fn main() {
	let (points, folds) = get_input();
	println!("PART 1: {}", part1(&points, folds[0]));
	part2(points, &folds);
}
