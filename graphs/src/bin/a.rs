extern crate graphs;

use graphs::{
	InputReader
};

use std::collections::VecDeque;
use std::ops::Range;


fn swap_range<T>(q: &mut VecDeque<T>, range: Range<usize>) {
	let mut i = range.start;
	let mut j = range.end - 1;
	while i < j {
		q.swap(i, j);
		i += 1;
		j -= 1;
	}
}

fn main() {
	let mut input = InputReader::new();
	// let mut output = OutputWriter::new();

	let n = input.next();

	// Input symmetric triangle adjacency matrix
	let mut adj = vec![vec![false; n]; n];
	for i in 1..n {
		let row: String = input.next();
		debug_assert_eq!(row.len(), i);
		for (j, c) in row.chars().enumerate() {
			adj[i][j] = c == '1';
			adj[j][i] = adj[i][j];
		}
	}

	// Find hamiltonian cycle given that each vertex has degree at least N/2
	let mut queue = (0..n).collect::<VecDeque<_>>();
	for k in 0..n * (n - 1) {
		if !adj[queue[0]][queue[1]] {
			let mut i = 2;
			while !adj[queue[0]][queue[i]] || ! adj[queue[1]][queue[i + 1]] {
				i += 1;
			}
			swap_range(&mut queue, 1..i + 1);
		}
		let front = queue.pop_front().unwrap();
		queue.push_back(front);
	}

	// Output via join
	println!("{}", queue.into_iter().map(|x| (x + 1).to_string()).collect::<Vec<_>>().join(" "));
}

/*
4

1
11
101
 */
