extern crate graphs;

use std::cmp::Reverse;
use graphs::{Decomposition, DFSSpace, Graph, InputReader};

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

	fn strip(&mut self) {
		while self.coefficients.last() == Some(&0) {
			self.coefficients.pop();
		}
	}

	fn monomial(degree: usize, coefficient: i64) -> Self {
		let mut coefficients = vec![0; degree + 1];
		coefficients[degree] = coefficient;
		Self::new(coefficients)
	}

	fn degree(&self) -> usize {
		self.coefficients.len() - 1
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
	let mut component_ptr = vec![0; graph.vertexes()];
	for (i, component) in decomposition.component_list.iter().enumerate() {
		for &v in component {
			index_in_component[v] = component_ptr[i];
			component_ptr[i] += 1;
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
			if u > v {
				continue;
			}
			let u_component = decomposition.component_of(u);
			let v_component = decomposition.component_of(v);
			if u_component == v_component {
				component_graphs[u_component].add_undirected_edge(index_in_component[u], index_in_component[v]);
			}
		}
	}

	(decomposition, index_in_component, component_graphs)
}


/// Chromatic polynomial of a connected graph
fn chromatic_polynomial_dummy(graph: &Graph) -> Polynomial {
	// If the graph has no edges,
	// then the chromatic polynomial is t^n
	if graph.edges.iter().all(|e| e.is_empty()) {
		return Polynomial::monomial(graph.vertexes(), 1);
	}

	// Otherwise, take an edge and apply the formula
	// Find the vertex with the smallest degree
	let smallest_degree_vertex = graph.edges.iter()
		.enumerate()
		.min_by_key(|(_, edges)| edges.len())
		.unwrap().0;
	let edge_index = graph.edges[smallest_degree_vertex][0].edge_index; // It's guaranteed to be non-empty because graph is connected => doesn't contain isolated vertexes
	let mut without_edge = graph.remove_edges_renumbered(&[edge_index]);
	assert_eq!(without_edge.edges(), graph.edges() - 1);

	let pulled = graph.pull_edge(edge_index);
	assert!(pulled.vertexes() == graph.vertexes() - 1);

	return chromatic_polynomial(&without_edge) - chromatic_polynomial(&pulled);
}

/// Compute chromatic polynomial of a graph
fn chromatic_polynomial(graph: &Graph) -> Polynomial {
	let (decomposition, re_numeration, component_graphs) = decompose_connected_components(graph);

	let mut result = Polynomial::new(vec![1]);
	for component in component_graphs {
		result *= chromatic_polynomial_dummy(&component);
	}

	result
}

fn main() {
	let mut input = InputReader::new();
	// let mut output = OutputWriter::new();

	let graph = Graph::from_stdin(&mut input, false);
	let result = chromatic_polynomial(&graph);

	println!("{}", result.degree());
	// Print the result via join
	println!("{}", result.coefficients.iter()
		.rev()
		.map(|c| c.to_string())
		.collect::<Vec<_>>()
		.join(" ")
	);
}
