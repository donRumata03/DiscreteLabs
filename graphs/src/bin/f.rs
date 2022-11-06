extern crate graphs;

use std::cmp::Reverse;
use graphs::{Graph, InputReader};

use std::collections::{BinaryHeap, HashSet};
use std::ops::Range;

struct Tree {
	adj_list: Vec<HashSet<usize>>,
}

impl Tree {
	fn new(n: usize) -> Self {
		Self {
			adj_list: vec![HashSet::new(); n],
		}
	}

	fn add_edge(&mut self, from: usize, to: usize) {
		self.adj_list[from].insert(to);
		self.adj_list[to].insert(from);
	}

	fn remove_edge(&mut self, from: usize, to: usize) {
		self.adj_list[from].remove(&to);
		self.adj_list[to].remove(&from);
	}

	fn degree(&self, v: usize) -> usize {
		self.adj_list[v].len()
	}
}

/// Build tree by Prufer code

fn main() {
	let mut input = InputReader::new();
	// let mut output = OutputWriter::new();

	let n: usize = input.next();

	// Input prufer code
	let mut prufer = vec![0_usize; n - 2];
	for i in 0..n - 2 {
		prufer[i] = input.next();
	}

	// Build tree as a list of edges
	let mut tree = vec![];
	let mut degree = vec![1_usize; n + 1];
	for &u in &prufer {
		degree[u] += 1;
	}
	let mut leaf_queue = BinaryHeap::new();
	for i in 1..n + 1 {
		if degree[i] == 1 {
			leaf_queue.push(Reverse(i));
		}
	}
	for &u in &prufer {
		let Reverse(v) = leaf_queue.pop().unwrap();
		tree.push((u, v));
		degree[u] -= 1;
		if degree[u] == 1 {
			leaf_queue.push(Reverse(u));
		}
	}
	// add last edge
	let Reverse(v) = leaf_queue.pop().unwrap();
	let Reverse(u) = leaf_queue.pop().unwrap();
	tree.push((u, v));


	// Output tree
	for (u, v) in tree {
		println!("{} {}", u, v);
	}
}
