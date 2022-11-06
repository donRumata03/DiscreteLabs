extern crate graphs;

use std::cmp::Reverse;
use graphs::{Decomposition, DFSSpace, Graph, InputReader, WeghtedEdge};

use std::collections::{BinaryHeap, HashSet};
use std::ops::Range;


/// Struct polynomial
#[derive(Clone, Debug)]
struct Polynomial {
	coefficients: Vec<i64>,
}

impl Polynomial {
	fn new(coefficients: Vec<i64>) -> Self {
		let mut res = Self {
			coefficients,
		};
		res.strip();
		res
	}

	fn degree(&self) -> usize {
		self.coefficients.len() - 1
	}

	fn strip(&mut self) {
		while self.coefficients.len() > 1 && self.coefficients.last().unwrap() == &0 {
			self.coefficients.pop();
		}
	}
}

/// Operations with polynomials (overloaded operators)
impl std::ops::Add for Polynomial {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		let mut result = self.clone();
		let mut other = other.clone();
		if result.degree() < other.degree() {
			std::mem::swap(&mut result, &mut other);
		}
		for i in 0..other.coefficients.len() {
			result.coefficients[i] += other.coefficients[i];
		}
		result.strip();
		result
	}
}

impl std::ops::AddAssign for Polynomial {
	fn add_assign(&mut self, other: Self) {
		*self = self.clone() + other;
	}
}

impl std::ops::Sub for Polynomial {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		let mut result = self.clone();
		let mut other = other.clone();
		if result.degree() < other.degree() {
			std::mem::swap(&mut result, &mut other);
		}
		for i in 0..other.coefficients.len() {
			result.coefficients[i] -= other.coefficients[i];
		}
		result.strip();
		result
	}
}

impl std::ops::SubAssign for Polynomial {
	fn sub_assign(&mut self, other: Self) {
		*self = self.clone() - other;
	}
}

impl std::ops::Mul for Polynomial {
	type Output = Self;

	fn mul(self, other: Self) -> Self {
		let mut result = vec![0_i64; self.degree() + other.degree() + 1];
		for i in 0..self.degree() + 1 {
			for j in 0..other.degree() + 1 {
				result[i + j] += self.coefficients[i] * other.coefficients[j];
			}
		}
		Self::new(result)
	}
}

impl std::ops::MulAssign for Polynomial {
	fn mul_assign(&mut self, other: Self) {
		*self = self.clone() * other;
	}
}

fn decompose_connected_components(graph: &Graph) -> (Decomposition, Vec<usize>, Vec<Graph>) {
	let mut dfs_space = DFSSpace::new(graph);
	let decomposition = dfs_space.find_connected_components(&graph);

	// For each vertex its index in its component
	let mut index_in_component = vec![0; graph.vertexes()];
	for (i, component) in decomposition.component_list.iter().enumerate() {
		for &v in component {
			index_in_component[v] = i;
		}
	}
	// By component index and index of vertex in component we can get the vertex's index in th initial graph:
	// decomposition.component_list[component_index][index_in_component]

	let mut component_graphs = decomposition.component_list.iter()
		.map(|c| Graph::new(c.len()))
		.collect::<Vec<_>>();

	for (u, edges) in graph.edges.iter().enumerate() {
		for &e in edges {
			let v = e.to;
			let u_component = decomposition.component_of(u);
			let v_component = decomposition.component_of(v);
			if u_component != v_component {
				component_graphs[u_component].add_undirected_edge(index_in_component[u], index_in_component[v]);
			}
		}
	}

	(decomposition, index_in_component, component_graphs)
}


/// Chromatic polynomial of a connected graph
fn chromatic_polynom_connected(graph: &Graph) -> Polynomial {
	unimplemented!()
}

/// Compute chromatic polynomial of a graph
fn chromatic_polynomial(graph: &Graph) -> Polynomial {
	let n = graph.vertexes();
	let mut dfs_space = DFSSpace::new(&graph);
	let mut components = dfs_space.find_connected_components(&graph);
	components.sort_by_key(|c| c.len());
	components.reverse();
	let mut result = Polynomial::new(vec![1]);
	for component in components {
		let mut component_graph = Graph::new(n);
		for (u, v) in graph.edges() {
			if component.contains(&u) && component.contains(&v) {
				component_graph.add_edge(u, v);
			}
		}
		result *= chromatic_polynom_connected(&component_graph);
	}

	result
}

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
