const INPUT: &str = include_str!("data/day3");

fn part1(input: &[&str]) -> u32 {
	let mut res = vec![0i32; input[0].len()];
	for line in input.iter().copied().map(|line| line.as_bytes()) {
		res.iter_mut()
			.zip(line)
			.for_each(|(a, &b)| *a += if b == b'1' { 1 } else { -1 })
	}
	let a = res
		.into_iter()
		.fold(0u32, |acc, curr| (acc << 1) + if curr >= 0 { 1 } else { 0 });
	let b = !a & (!0 >> a.leading_zeros());
	a * b
}

fn gt(a: u64, b: u64) -> bool {
	a > b
}
fn eq(a: u64, b: u64) -> bool {
	a == b
}

fn part2(input: &[&str]) -> u64 {
	let mut a: Vec<u64> = input
		.into_iter()
		.map(|line| u64::from_str_radix(line, 2).unwrap())
		.collect();
	let mut b: Vec<u64> = a.clone();
	for i in 0..12 {
		if a.len() > 1 {
			let bit_a = a
				.iter()
				.copied()
				.map(|num| (num >> (11 - i)) & 1)
				.fold(0i32, |acc, curr| acc + if curr == 1 { 1 } else { -1 });
			let test_a = if bit_a >= 0 { gt } else { eq };
			a = a
				.iter()
				.copied()
				.filter(|num| test_a(num & (1 << (11 - i)), 0))
				.collect();
		}
		if b.len() > 1 {
			let bit_b = b
				.iter()
				.copied()
				.map(|num| (num >> (11 - i)) & 1)
				.fold(0i32, |acc, curr| acc + if curr == 1 { 1 } else { -1 });
			let test_b = if bit_b >= 0 { eq } else { gt };
			b = b
				.iter()
				.copied()
				.filter(|num| test_b(num & (1 << (11 - i)), 0))
				.collect();
		}
	}
	assert_eq!(a.len(), 1);
	assert_eq!(b.len(), 1);
	a[0] * b[0]
}

fn main() {
	let input: Vec<&str> = INPUT.trim().lines().collect();
	println!("PART 1: {}", part1(&input));
	println!("PART 2: {}", part2(&input));
}
