const INPUT: &str = include_str!("data/day9");

const SIZE: usize = 100;

fn part1(board: &[u8]) -> usize {
	board
		.iter()
		.copied()
		.enumerate()
		.filter(is_dip(board))
		.fold(0, |acc, (_, curr)| acc + (curr as usize) + 1)
}

fn part2(mut board: Vec<u8>) -> usize {
	#[inline]
	fn recurse(board: &mut [u8], idx: usize) -> usize {
		let c = board[idx];

		if c == 9 {
			return 0;
		}

		board[idx] = 9;

		let mut res = 1;
		let (x, y) = idx_to_coords(idx);
		if x > 0 {
			res += recurse(board, idx - 1);
		}
		if x < SIZE - 1 {
			res += recurse(board, idx + 1);
		}
		if y > 0 {
			res += recurse(board, idx - SIZE);
		}
		if y < SIZE - 1 {
			res += recurse(board, idx + SIZE);
		}
		res
	}

	let mut basins = Vec::new();

	while let Some(idx) = board.iter().position(|&val| val != 9) {
		basins.push(recurse(&mut board, idx));
	}

	basins.sort_unstable_by(|a, b| b.cmp(a));

	basins[0] * basins[1] * basins[2]
}

#[inline]
fn is_dip(board: &[u8]) -> impl FnMut(&(usize, u8)) -> bool + '_ {
	|&(idx, val)| {
		let mut res = true;
		let (x, y) = idx_to_coords(idx);
		if x > 0 {
			res &= board[idx - 1] > val;
		}
		if x < SIZE - 1 {
			res &= board[idx + 1] > val;
		}
		if y > 0 {
			res &= board[idx - SIZE] > val;
		}
		if y < SIZE - 1 {
			res &= board[idx + SIZE] > val;
		}
		res
	}
}

#[inline]
fn idx_to_coords(idx: usize) -> (usize, usize) {
	(idx % SIZE, idx / SIZE)
}

fn main() {
	let board: Vec<u8> = INPUT
		.trim()
		.lines()
		.flat_map(|line| line.as_bytes())
		.map(|b| b - b'0')
		.collect();
	println!("PART 1: {}", part1(&board));
	println!("PART 2: {}", part2(board));
}
