const INPUT: &str = include_str!("data/day4");

const BOARD_SIZE: usize = 5;

#[derive(Debug, Clone, Copy)]
enum BoardField {
	Covered,
	Uncovered(i32),
}

type Board = Vec<BoardField>;

fn idx_to_coords(idx: usize) -> (usize, usize) {
	(idx % BOARD_SIZE, idx / BOARD_SIZE)
}

fn coords_to_idx(x: usize, y: usize) -> usize {
	y * BOARD_SIZE + x
}

fn is_winner(board: &[BoardField], x: usize, y: usize) -> bool {
	(0..BOARD_SIZE)
		.map(|x| board[coords_to_idx(x, y)])
		.all(|field| matches!(field, BoardField::Covered))
		|| (0..BOARD_SIZE)
			.map(|y| board[coords_to_idx(x, y)])
			.all(|field| matches!(field, BoardField::Covered))
}

fn board_score(board: &[BoardField], num: i32) -> i32 {
	board
		.iter()
		.filter_map(|&field| match field {
			BoardField::Covered => None,
			BoardField::Uncovered(n) => Some(n),
		})
		.sum::<i32>()
		* num
}

fn part1(nums: &[i32], boards: &[Board]) -> i32 {
	let mut boards = boards.to_vec();
	for &num in nums {
		for board in boards.iter_mut() {
			if let Some(idx) = board
				.iter()
				.position(|&field| matches!(field, BoardField::Uncovered(n) if n == num))
			{
				board[idx] = BoardField::Covered;
				let (x, y) = idx_to_coords(idx);
				let winner = is_winner(board, x, y);

				if winner {
					return board_score(board, num);
				}
			}
		}
	}
	unreachable!()
}

fn part2(nums: &[i32], boards: &[Board]) -> i32 {
	let mut boards = boards.to_vec();
	let mut last_winner = None;
	for &num in nums {
		for i in (0..boards.len()).rev() {
			if let Some(idx) = boards[i]
				.iter()
				.position(|&field| matches!(field, BoardField::Uncovered(n) if n == num))
			{
				boards[i][idx] = BoardField::Covered;
				let (x, y) = idx_to_coords(idx);
				let winner = is_winner(&boards[i], x, y);

				if winner {
					last_winner = Some((boards.remove(i), num));
				}
			}
		}
	}
	match last_winner {
		None => unreachable!(),
		Some((board, num)) => board_score(&board, num),
	}
}

fn parse_board(chunk: &[&str]) -> Board {
	chunk
		.into_iter()
		.flat_map(|line| line.split_ascii_whitespace())
		.map(|num| num.parse().unwrap())
		.map(BoardField::Uncovered)
		.collect()
}

fn get_input() -> (Vec<i32>, Vec<Board>) {
	let mut input: Vec<_> = INPUT
		.trim()
		.lines()
		.filter(|line| !line.is_empty())
		.collect();
	let nums: Vec<i32> = input[0]
		.split(',')
		.map(|num| num.parse().unwrap())
		.collect();
	let boards: Vec<Board> = input[1..].chunks(BOARD_SIZE).map(parse_board).collect();
	(nums, boards)
}

fn main() {
	let (nums, boards) = get_input();
	println!("PART 1: {}", part1(&nums, &boards));
	println!("PART 2: {}", part2(&nums, &boards));
}
