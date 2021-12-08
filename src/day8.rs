const INPUT: &str = include_str!("data/day8");

fn part1(vals: &[Vec<u8>]) -> usize {
	vals.into_iter()
		.flat_map(|val| val.iter())
		.filter(|&val| matches!(val.count_ones(), 2 | 3 | 4 | 7))
		.count()
}

fn part2(nums: Vec<Vec<u8>>, vals: &[Vec<u8>]) -> usize {
	nums.into_iter()
		.map(analyze_numbers)
		.zip(vals)
		.map(|(nums, val)| {
			val.iter()
				.map(|&a| nums.iter().position(|&num| a == num).unwrap())
				.fold(0, |acc, curr| acc * 10 + curr)
		})
		.sum()
}

fn analyze_numbers(mut nums: Vec<u8>) -> [u8; 10] {
	let mut res = [0u8; 10];
	macro_rules! find_get {
		($e:expr) => {{
			let idx = nums.iter().position($e).unwrap();
			nums.swap_remove(idx)
		}};
	}
	res[1] = find_get!(|&num| num.count_ones() == 2);
	res[4] = find_get!(|&num| num.count_ones() == 4);
	res[7] = find_get!(|&num| num.count_ones() == 3);
	res[8] = find_get!(|&num| num.count_ones() == 7);
	res[0] = find_get!(|&num| {
		let t = res[8] ^ (res[4] ^ res[1]);
		num & t == t
	});
	res[6] = find_get!(|&num| {
		let t = res[8] ^ res[1];
		num & t == t
	});
	res[2] = find_get!(|&num| {
		let t = res[6] & res[1];
		num & t == 0
	});
	res[9] = find_get!(|&num| num & res[4] == res[4]);
	res[3] = find_get!(|&num| num & res[1] == res[1]);
	res[5] = find_get!(|_| true);
	res
}

fn parse_num(s: &str) -> u8 {
	s.as_bytes()
		.iter()
		.fold(0, |acc, &curr| acc | (1 << curr - b'a'))
}

fn parse_line(s: &str) -> (Vec<u8>, Vec<u8>) {
	let mut n = s.split('|');
	let nums = n.next().unwrap().trim();
	let val = n.next().unwrap().trim();
	(
		nums.split(' ').map(parse_num).collect(),
		val.split(' ').map(parse_num).collect(),
	)
}

fn main() {
	let (nums, vals): (Vec<Vec<u8>>, Vec<Vec<u8>>) = INPUT.trim().lines().map(parse_line).unzip();
	println!("PART 1: {}", part1(&vals));
	println!("PART 2: {}", part2(nums, &vals));
}
