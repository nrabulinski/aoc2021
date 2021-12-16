use std::{
	cell::RefCell,
	collections::{HashMap, HashSet},
	hash::Hash,
	rc::Rc,
};

const INPUT: &str = include_str!("data/day12");

fn part1(start: Node) -> usize {
	fn recurse(node: Node, mut visited: HashSet<Kind>) -> usize {
		if matches!(node.kind, Kind::End) {
			return 1;
		}

		if matches!(node.kind, Kind::Small(_)) {
			visited.insert(node.kind);
		}

		node.children
			.borrow()
			.iter()
			.filter(|child| !visited.contains(&child.kind))
			.map(|child| recurse(child.clone(), visited.clone()))
			.sum()
	}

	recurse(start, HashSet::new())
}

fn part2(start: Node) -> usize {
	fn recurse(node: Node, mut visited: HashMap<Kind, usize>, mut can_twice: bool) -> usize {
		if matches!(node.kind, Kind::End) {
			return 1;
		}

		if matches!(node.kind, Kind::Small(_)) {
			let counter = visited.entry(node.kind).or_insert(0);
			*counter += 1;

			if *counter > 1 {
				can_twice = false;
			}
		}

		let check = if can_twice { lt2 } else { lt1 };

		node.children
			.borrow()
			.iter()
			.filter(|child| visited.get(&child.kind).copied().map(check).unwrap_or(true))
			.map(|child| recurse(child.clone(), visited.clone(), can_twice))
			.sum()
	}

	recurse(start, HashMap::new(), true)
}

fn lt1(n: usize) -> bool {
	n < 1
}

fn lt2(n: usize) -> bool {
	n < 2
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
	kind: Kind,
	children: Rc<RefCell<Vec<Node>>>,
}

impl Node {
	fn new(kind: impl Into<Kind>) -> Node {
		Node {
			kind: kind.into(),
			children: Default::default(),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Kind {
	Start,
	Small(&'static str),
	Big(&'static str),
	End,
}

impl From<&'static str> for Kind {
	fn from(s: &'static str) -> Self {
		match s {
			"start" => Kind::Start,
			"end" => Kind::End,
			a if a == a.to_ascii_uppercase() => Kind::Big(a),
			a => Kind::Small(a),
		}
	}
}

fn get_input() -> Node {
	let connections: Vec<_> = INPUT
		.trim()
		.lines()
		.map(|line| {
			let mut iter = line.split('-');
			let a = iter.next().unwrap();
			let b = iter.next().unwrap();
			(Kind::from(a), Kind::from(b))
		})
		.collect();

	let mut nodes: HashMap<_, _> = connections
		.iter()
		.flat_map(|&(a, b)| [(a, Node::new(a)), (b, Node::new(b))])
		.collect();

	for (a, b) in connections {
		let a = nodes.get(&a).unwrap();
		let b = nodes.get(&b).unwrap();

		if !matches!(a.kind, Kind::End) && !matches!(b.kind, Kind::Start) {
			a.children.borrow_mut().push(Node {
				kind: b.kind,
				children: b.children.clone(),
			});
		}

		if !matches!(b.kind, Kind::End) && !matches!(a.kind, Kind::Start) {
			b.children.borrow_mut().push(Node {
				kind: a.kind,
				children: a.children.clone(),
			});
		}
	}

	nodes.remove(&Kind::Start).unwrap()
}

fn main() {
	let start = get_input();
	println!("PART 1: {}", part1(start.clone()));
	println!("PART 2: {}", part2(start));
}
