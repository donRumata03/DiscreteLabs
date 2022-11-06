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

/// Build Prufer code of the tree

fn main() {
	let mut input = InputReader::new();
	// let mut output = OutputWriter::new();

	let n = input.next();

	// Input tree
	let mut tree = Tree::new(n);
	for _ in 1..n {
		let u: usize = input.next();
		let v: usize = input.next();
		tree.add_edge(u - 1, v - 1);
	}

	let mut queue = BinaryHeap::new();
	for i in 0..n {
		if tree.degree(i) == 1 {
			queue.push(Reverse(i));
		}
	}

	let mut prufer = vec![0; n - 2];
	for i in 0..n - 2 {
		let Reverse(u) = queue.pop().unwrap();
		let v = *tree.adj_list[u].iter().next().unwrap();
		prufer[i] = v;
		tree.remove_edge(u, v);
		if tree.degree(v) == 1 {
			queue.push(Reverse(v));
		}
	}

	// Output via join
	println!("{}", prufer.into_iter().map(|x| (x + 1).to_string()).collect::<Vec<_>>().join(" "));
}
