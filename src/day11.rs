const INPUT: &str = include_str!("data/day11");

const SIZE: usize = 10;

fn part1(board: &[u8]) -> usize {
	let mut board = board.to_vec();
	let mut flashes = 0;
	for _ in 0..100 {
		let mut idxs: Vec<_> = (0..board.len()).collect();
		let mut i = 0;
		loop {
			if i >= idxs.len() {
				break;
			}
			let idx = idxs[i];
			board[idx] += 1;
			i += 1;

			if board[idx] == 10 {
				let (x, y) = idx_to_coords(idx);
				if x > 0 {
					idxs.push(idx - 1);
				}
				if x < SIZE - 1 {
					idxs.push(idx + 1);
				}
				if y > 0 {
					idxs.push(idx - SIZE);
				}
				if y < SIZE - 1 {
					idxs.push(idx + SIZE);
				}
				if x > 0 && y > 0 {
					idxs.push(idx - SIZE - 1);
				}
				if x < SIZE - 1 && y > 0 {
					idxs.push(idx - SIZE + 1);
				}
				if x > 0 && y < SIZE - 1 {
					idxs.push(idx + SIZE - 1);
				}
				if x < SIZE - 1 && y < SIZE - 1 {
					idxs.push(idx + SIZE + 1);
				}
			}
		}

		for idx in 0..board.len() {
			if board[idx] > 9 {
				flashes += 1;
				board[idx] = 0;
			}
		}
	}
	flashes
}

fn part2(board: &[u8]) -> usize {
	let mut board = board.to_vec();
	let mut step = 0;
	while !board.iter().all(|&num| num == 0) {
		let mut idxs: Vec<_> = (0..board.len()).collect();
		let mut i = 0;
		loop {
			if i >= idxs.len() {
				break;
			}
			let idx = idxs[i];
			board[idx] += 1;
			i += 1;

			if board[idx] == 10 {
				let (x, y) = idx_to_coords(idx);
				if x > 0 {
					idxs.push(idx - 1);
				}
				if x < SIZE - 1 {
					idxs.push(idx + 1);
				}
				if y > 0 {
					idxs.push(idx - SIZE);
				}
				if y < SIZE - 1 {
					idxs.push(idx + SIZE);
				}
				if x > 0 && y > 0 {
					idxs.push(idx - SIZE - 1);
				}
				if x < SIZE - 1 && y > 0 {
					idxs.push(idx - SIZE + 1);
				}
				if x > 0 && y < SIZE - 1 {
					idxs.push(idx + SIZE - 1);
				}
				if x < SIZE - 1 && y < SIZE - 1 {
					idxs.push(idx + SIZE + 1);
				}
			}
		}

		for idx in 0..board.len() {
			if board[idx] > 9 {
				board[idx] = 0;
			}
		}
		step += 1;
	}
	step
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
		.map(|num| num - b'0')
		.collect();
	println!("PART 1: {}", part1(&board));
	println!("PART 2: {}", part2(&board));
}
