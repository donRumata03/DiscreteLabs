use std::fmt::Debug;
use crate::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisitColor {
	White,
	Gray,
	Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WeightedEdge<W> {
	pub to: usize,
	pub edge_index: usize,
	pub weight: W
}

pub type Edge = WeightedEdge<()>;

impl Edge {
	pub fn new(to: usize, edge_index: usize) -> Self {
		Edge { to, edge_index, weight: () }
	}
}

// Graph as an adjacency list
#[derive(Debug, Clone)]
pub struct WeightedGraph<T> {
	pub edges: Vec<Vec<WeightedEdge<T>>>,
	pub total_edges: usize,
}
pub type Graph = WeightedGraph<()>;

impl<T: Clone + Copy + Debug> WeightedGraph<T> {
	pub fn new(n: usize) -> Self {
		Self {
			edges: vec![Vec::new(); n],
			total_edges: 0,
		}
	}

	pub fn validate_compact_enumeration(&self) {
		// Check that all edges are enumerated from 0 to n - 1 without gaps
		for (i, edges) in self.edges.iter().enumerate() {
			for edge in edges {
				debug_assert!(edge.edge_index < self.total_edges, "Edge index is out of bounds");
			}
		}

		// Check that all numbers from 0 to total_edges - 1 are present
		let mut present = vec![false; self.total_edges];
		for edges in self.edges.iter() {
			for edge in edges {
				present[edge.edge_index] = true;
			}
		}
		debug_assert!(present.iter().all(|&x| x), "Not all edge indexes are used");
	}

	pub fn add_weighted_indexed_directed_edge(&mut self, from: usize, to: usize, edge_index: usize, weight: T) {
		self.edges[from].push(WeightedEdge { to, edge_index, weight });
	}

	pub fn add_weighted_undirected_edge(&mut self, from: usize, to: usize, weight: T) {
		let edge_index = self.total_edges;
		self.add_weighted_indexed_directed_edge(from, to, edge_index, weight.clone());
		self.add_weighted_indexed_directed_edge(to, from, edge_index, weight.clone());
		self.total_edges += 1;
	}

	pub fn add_weighted_directed_edge(&mut self, from: usize, to: usize, weight: T) {
		let edge_index = self.total_edges;
		self.add_weighted_indexed_directed_edge(from, to, edge_index, weight);
		self.total_edges += 1;
	}

	pub fn vertexes(&self) -> usize {
		self.edges.len()
	}

	pub fn edges(&self) -> usize {
		self.total_edges
	}

	pub fn remove_edges(&mut self, edges: &[usize]) {
		let mut removed_edges = vec![false; self.total_edges];
		for edge in edges {
			removed_edges[*edge] = true;
		}
		for edges in &mut self.edges {
			// self.total_edges -= edges.len();
			edges.retain(|edge| !removed_edges[edge.edge_index]);
			// self.total_edges += edges.len();
		}
		self.total_edges -= edges.len();
	}

	/// Vertexes have the same indexes as in the original graph
	/// Edges have the same indexes as in the original graph
	/// But `from` and `to` of edges are swapped
	pub fn reversed(&self) -> Self {
		let mut reversed = Self::new(self.vertexes());
		for (from, edges) in self.edges.iter().enumerate() {
			for edge in edges {
				reversed.add_weighted_indexed_directed_edge(edge.to, from, edge.edge_index, edge.weight.clone());
				reversed.total_edges += 1;
			}
		}
		reversed
	}
}

impl<T: Copy + Default + PartialEq + Debug> WeightedGraph<T> {
	pub fn from_weight_matrix(matrix: &Vec<Vec<T>>) -> Self {
		let n = matrix.len();
		let mut graph = Self::new(n);
		for i in 0..n {
			for j in 0..n {
				if matrix[i][j] != T::default() {
					graph.add_weighted_directed_edge(i, j, matrix[i][j]);
				}
			}
		}
		graph
	}

}

impl WeightedGraph<()> {
	pub fn add_undirected_edge(&mut self, from: usize, to: usize) {
		self.add_weighted_undirected_edge(from, to, ());
	}

	pub fn add_directed_edge(&mut self, from: usize, to: usize) {
		self.add_weighted_directed_edge(from, to, ());
	}

	pub fn from_stdin(input_reader: &mut InputReader<Stdin>, directed: bool) -> Self {
		let n = input_reader.next::<usize>();
		let m = input_reader.next::<usize>();
		let mut graph = Self::new(n);
		for _ in 0..m {
			let from = input_reader.next::<usize>();
			let to = input_reader.next::<usize>();
			if directed {
				graph.add_directed_edge(from - 1, to - 1);
			} else {
				graph.add_undirected_edge(from - 1, to - 1);
			}
		}
		graph
	}

	pub fn from_adjacency_matrix(matrix: &Vec<Vec<bool>>) -> Self {
		let n = matrix.len();
		let mut graph = Self::new(n);
		for i in 0..n {
			for j in 0..n {
				if matrix[i][j] {
					graph.add_directed_edge(i, j);
				}
			}
		}
		graph
	}


	/// Deduplicate edges
	/// Edges are also renumerated (such that if `consider_inverse_edges_equal` is true,
	/// straight and inverse edges will have the same index in the new graph)
	pub fn deduplicated(&self, consider_inverse_edges_equal: bool) -> Self {
		// Group edges by sorted pair of `end` and `to`
		let double_ended_edges = self.edges.iter()
			.enumerate()
			.map(|(i, edges)| edges.iter().map(|edge| (i, edge.to, edge.edge_index))
				.collect::<Vec<(usize, usize, usize)>>())
			.flatten()
			.collect();

		let mut deduplicated = Self::new(self.vertexes());

		for (new_index, ((from, to), _group)) in
		group_by(&double_ended_edges,
		         |&(from, to, index)| if consider_inverse_edges_equal { minmax(from, to) } else { (from, to) })
			.into_iter()
			.enumerate()
		{
			// Straight edge
			deduplicated.add_weighted_indexed_directed_edge(from, to, new_index, ());
			deduplicated.total_edges += 1;
			// Inverse edge
			if consider_inverse_edges_equal {
				deduplicated.add_weighted_indexed_directed_edge(to, from, new_index, ());
			}
		}

		deduplicated.validate_compact_enumeration();
		deduplicated
	}


	// Merges vertexes at the ends of edge `edge_index` into one vertex
	pub fn pull_edge(&self, edge_index: usize) -> Self {
		// Find edge's `from` and `to`
		let (l, r) = sort_pair(self.edges.iter()
			.enumerate()
			.find_map(|(from, edges)| edges.iter()
				.find(|edge| edge.edge_index == edge_index)
				.map(|edge| (from, edge.to)))
			.unwrap());

		let get_new_vertex_index = |index| {
			if index < r {
				index
			} else if index == r {
				l
			} else {
				index - 1
			}
		};

		let mut pulled = Self::new(self.vertexes() - 1);

		for (from, edges) in self.edges.iter().enumerate() {
			for edge in edges {
				if edge.edge_index == edge_index {
					continue;
				}
				pulled.add_weighted_indexed_directed_edge(
					get_new_vertex_index(from),
					get_new_vertex_index(edge.to),
					edge.edge_index,
					edge.weight.clone()
				);
				pulled.total_edges += 1;
			}
		}

		pulled.deduplicated(true)
	}

	pub fn remove_edges_renumbered(&self, edges: &[usize]) -> Self {
		self.validate_compact_enumeration();
		let mut removed = self.clone();
		removed.remove_edges(edges);
		removed.deduplicated(true)
	}
}

impl<T: InputReadable + Clone + Copy + Debug> WeightedGraph<T> {
	pub fn weighted_from_stdin(input_reader: &mut InputReader<Stdin>, directed: bool) -> Self {
		let n = input_reader.next::<usize>();
		let m = input_reader.next::<usize>();
		let mut graph = Self::new(n);
		for _ in 0..m {
			let from = input_reader.next::<usize>();
			let to = input_reader.next::<usize>();
			let weight = input_reader.next::<T>();
			if directed {
				graph.add_weighted_directed_edge(from - 1, to - 1, weight);
			} else {
				graph.add_weighted_undirected_edge(from - 1, to - 1, weight);
			}
		}
		graph
	}
}
