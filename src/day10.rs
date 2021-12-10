const INPUT: &str = include_str!("data/day10");

fn part1(lines: &[&str]) -> usize {
	fn recurse(mut line: &[u8], expected: u8) -> Result<&[u8], u8> {
		while !line.is_empty() {
			match line[0] {
				b'(' => line = recurse(&line[1..], b')')?,
				b'[' => line = recurse(&line[1..], b']')?,
				b'{' => line = recurse(&line[1..], b'}')?,
				b'<' => line = recurse(&line[1..], b'>')?,
				ch if ch == expected => return Ok(&line[1..]),
				ch => return Err(ch),
			}
		}
		Ok(&[])
	}

	lines
		.iter()
		.filter_map(|&line| match recurse(line.as_bytes(), 0) {
			Err(ch) => Some(ch),
			_ => None,
		})
		.map(|ch| match ch {
			b')' => 3,
			b']' => 57,
			b'}' => 1197,
			b'>' => 25137,
			_ => unreachable!(),
		})
		.sum()
}

fn part2(lines: &[&str]) -> usize {
	fn complete_line(line: &[u8]) -> Option<Vec<u8>> {
		let mut expected = Vec::new();
		for i in 0..line.len() {
			match (line[i], expected.last().copied()) {
				(b'(', _) => expected.push(b')'),
				(b'[', _) => expected.push(b']'),
				(b'{', _) => expected.push(b'}'),
				(b'<', _) => expected.push(b'>'),
				(ch, Some(ex)) if ch == ex => {
					expected.pop();
				}
				_ => return None,
			}
		}
		Some(expected)
	}

	let mut scores: Vec<usize> = lines
		.iter()
		.filter_map(|&line| complete_line(line.as_bytes()))
		.map(|chars| {
			chars.into_iter().rfold(0, |acc, curr| {
				acc * 5
					+ (match curr {
						b')' => 1,
						b']' => 2,
						b'}' => 3,
						b'>' => 4,
						_ => unreachable!(),
					})
			})
		})
		.collect();

	scores.sort_unstable();

	scores[scores.len() / 2]
}

fn main() {
	let lines: Vec<&str> = INPUT.trim().lines().collect();
	println!("PART 1: {}", part1(&lines));
	println!("PART 2: {}", part2(&lines));
}
