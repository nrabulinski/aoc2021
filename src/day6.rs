const INPUT: &str = include_str!("data/day6");

fn calc_pop(input: &[usize], days: usize) -> usize {
	let mut pop = [0usize; 9];
	for &idx in input {
		pop[idx] += 1;
	}

	for _ in 0..days {
		pop.rotate_left(1);
		pop[6] += pop[8];
	}

	pop.into_iter().sum()
}

fn part1(input: &[usize]) -> usize {
	calc_pop(input, 80)
}

fn part2(input: &[usize]) -> usize {
	calc_pop(input, 256)
}

fn main() {
	let input: Vec<usize> = INPUT
		.trim()
		.split(',')
		.map(|num| num.parse().unwrap())
		.collect();
	println!("PART 1: {}", part1(&input));
	println!("PART 2: {}", part2(&input));
}
