use std::collections::HashMap;

const INPUT: &str = include_str!("data/day14");

fn apply_steps(init: &str, p: &Patterns, steps: usize) -> usize {
	let mut pairs: HashMap<_, _> = p.keys().map(|&k| (k, 0)).collect();
	StrWindows::new(init, 2).for_each(|pair| *pairs.get_mut(pair).unwrap() += 1);

	for _ in 0..steps {
		let mut new = pairs.clone();

		for (pat, c) in pairs.iter() {
			*new.get_mut(pat).unwrap() -= c;
			let p = p.get(pat).unwrap();
			let a = format!("{}{}", pat.as_bytes()[0] as char, p);
			let b = format!("{}{}", p, pat.as_bytes()[1] as char);
			*new.get_mut(a.as_str()).unwrap() += c;
			*new.get_mut(b.as_str()).unwrap() += c;
		}

		pairs = new;
	}

	let mut pop = [0usize; 26];

	pairs.iter().for_each(|(pat, c)| {
		let a = pat.as_bytes()[0];
		let b = pat.as_bytes()[1];
		pop[(a - b'A') as usize] += c;
		pop[(b - b'A') as usize] += c;
	});

	let min = pop.iter().filter(|&&num| num > 0).min().unwrap();
	let max = pop.iter().max().unwrap();

	(max - min) / 2
}

fn part1(s: &str, p: &Patterns) -> usize {
	apply_steps(s, p, 10)
}

fn part2(s: &str, p: &Patterns) -> usize {
	apply_steps(s, p, 40)
}

struct StrWindows<'a> {
	s: &'a str,
	size: usize,
}

impl<'a> StrWindows<'a> {
	fn new(s: &'a str, size: usize) -> Self {
		StrWindows { s, size }
	}
}

impl<'a> Iterator for StrWindows<'a> {
	type Item = &'a str;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		if self.size > self.s.len() {
			None
		} else {
			let ret = Some(&self.s[..self.size]);
			self.s = &self.s[1..];
			ret
		}
	}
}

type Patterns = HashMap<&'static str, &'static str>;

fn get_input() -> (&'static str, Patterns) {
	let mut lines = INPUT.trim().lines();

	let template = lines.next().unwrap();

	lines.next().unwrap();

	let patterns = lines.map(|line| line.split_once(" -> ").unwrap()).collect();

	(template, patterns)
}

fn main() {
	let (template, patterns) = get_input();
	println!("PART 1: {}", part1(template, &patterns));
	println!("PART 2: {}", part2(template, &patterns));
}
