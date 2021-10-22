use std::{ops, io};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fmt;
use std::iter::FromIterator;
use std::io::{stdin, BufRead, BufReader};
use std::str;

// Basic utils:

struct BitSet {
	mask: usize
}
impl BitSet {
	fn from(msk: usize) -> BitSet {
		BitSet {
			mask: msk
		}
	}


	fn get(&self, index: usize) -> bool {
		(self.mask & (1 << index)) != 0
	}


	fn set_to(&mut self, position: usize, value: bool) {
		// if value {
		// 	self.set_true(position);
		// } else {
		// 	self.set_false(position);
		// }

		self.mask = (self.mask & !(1 << position)) | (value << position);
	}

	fn set_true(&mut self, position: usize) {
		self.mask |= (1_usize) << position;
	}
	fn set_false(&mut self, position: usize) {
		self.mask &= !((1_usize) << position);
	}
}


fn to_bin_vec(bitmask: usize, dims: usize) -> Vec<bool> {
	let mut res = Vec::with_capacity(dims);

	for i in 0..dims {
		res.push(bitmask & (1 << i) != 0);
	}

	res
}

fn reverse_bitmask(bitmask: usize, dims: usize) -> usize {
	let mut res = BitSet::from(0);

	for i in 0..dims {
		res.set_to(dims - i - 1, bitmask & (1 << i) != 0);
	}

	res.mask
}

fn to_bitmask(v: &Vec<bool>) -> usize {
	let mut res: usize = 0;

	for (i, value) in v.iter().enumerate() {
		res |= (*value as usize) << i;
	}

	res
}

// _______________________          Bool Function           ___________________________

struct BoolFunction {
	data: Vec<bool>,
	dims: usize
}

impl BoolFunction {
	pub fn new(truth_table: Vec<bool>) -> BoolFunction {
		let sz = truth_table.len();
		BoolFunction {
			data: truth_table,
			dims: (sz as f32).log2() as usize
		}
	}

	pub fn at(&self, index: usize) -> bool {
		self.data[index]
	}
}

impl ops::Index<usize> for BoolFunction {
	type Output = bool;

	fn index(&self, i: usize) -> &bool {
		&self.data[i]
	}
}

impl Display for BoolFunction {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let mut repr_map = HashMap::new();

		for (i, x) in self.data.iter().enumerate() {
			repr_map.insert(to_bin_vec(i, self.dims), x);
		}

		write!(f, "{:?}", repr_map)
	}
}


// _______________________          Scheme Node           ___________________________

struct FuncNode {
	inputs: Vec<usize>,
	func: BoolFunction
}
impl FuncNode {
	fn compute(&self, bitmask: usize) -> bool {
		self.func[bitmask]
	}
}

struct VarData {
	element_index: usize,
	var_index: usize
}

enum Node {
	Variable(VarData), // Var's index
	Function(FuncNode)
}

struct Scheme {
	nodes: Vec<Node>
}

impl Scheme {
	fn get_depth(&self, element: Option<usize>) -> usize {
		match &self.nodes[element.unwrap_or(self.nodes.len()  - 1)] {
			Node::Variable(_) => 0,

			Node::Function(f_data) => f_data.inputs.iter().map(
				|input_index| self.get_depth(Some(*input_index))
			).max().unwrap() + 1
		}
	}

	fn compute(&self, var_values: &BitSet) -> bool {
		// println!("{}: ", var_values.mask);


		let mut element_results = vec![None; self.nodes.len()];

		for (i, node) in self.nodes.iter().enumerate() {
			match node {
				Node::Variable(v) => element_results[i] = Some(var_values.get(v.var_index)),
				Node::Function(func_node) => {
					// Glue inputs:
					// for j in 0..i {
					// 	print!("{}", element_results[j].unwrap() as usize);
					// }

					let args: Vec<bool> =
						func_node.inputs.iter().map(|i| element_results[*i].unwrap()).collect();

					let func_argument_bitmask: usize = to_bitmask(&args);

					element_results[i] = Some(func_node.compute(func_argument_bitmask));
				}
			}
		}

		element_results.last().unwrap().unwrap()
	}
}

// _____________        Scanner         ______________

/// Reads white-space separated tokens one at a time.

pub struct Scanner<R> {
	reader: R,
	buffer: Vec<String>,
}

impl<R: io::BufRead> Scanner<R> {
	pub fn new(reader: R) -> Self {
		Self {
			reader,
			buffer: vec![],
		}
	}

	/// Use "turbofish" syntax token::<T>() to select data type of next token.
	///
	/// # Panics
	///
	/// Panics if there's an I/O error or if the token cannot be parsed as T.
	pub fn token<T: str::FromStr>(&mut self) -> T {
		loop {
			if let Some(token) = self.buffer.pop() {
				return token.parse().ok().expect("Failed parse");
			}
			let mut input = String::new();
			self.reader.read_line(&mut input).expect("Failed read");
			self.buffer = input.split_whitespace().rev().map(String::from).collect();
		}
	}
}


/// Main
fn main() {
	// println!("{}", reverse_bitmask(0b001, 3));

	let mut scanner = Scanner::new(BufReader::new(io::stdin()));

	// let boo: bool = scanner.token::<usize>() != 0;
	// println!("{}", boo);

	let n: usize = scanner.token();
	assert!(1 <= n && n <= 27);

	let mut nodes= Vec::new();

	let mut var_counter = 0;
	for i in 0..n {
		let m_i: usize = scanner.token();
		if m_i == 0 {
			nodes.push(Node::Variable(VarData { element_index: i, var_index: var_counter }));
			var_counter += 1;
		} else {
			// Read inputs:
			let mut inputs = Vec::new();
			for _ in 0..m_i {
				inputs.push(scanner.token::<usize>() - 1);
			}

			// Read truth table:
			let tt_size = 2_usize.pow(m_i as u32);
			let mut truth_table = vec![false; tt_size];
			for i in 0..tt_size {
				truth_table[reverse_bitmask(i, m_i)] = scanner.token::<usize>() != 0;
			}

			nodes.push(Node::Function(FuncNode {
				inputs,
				func: BoolFunction::new(truth_table)
			}));
		}
	}

	let scheme = Scheme {
		nodes
	};

	// println!("{}", scheme.compute(&BitSet::from(0b111)));

	// Depth:
	println!("{}", scheme.get_depth(None));

	// Resultant truth table:
	let tt_size = 2_usize.pow(var_counter as u32);
	let mut res_string = String::with_capacity(tt_size);
	for msk in 0..tt_size {
		res_string.push(if scheme.compute(&BitSet::from(reverse_bitmask(msk, var_counter))) {'1'} else { '0' });
	}
	println!("{}", res_string);
}

/*

5
0
0
2 1 2
1 1 0 1
0
2 3 4
1 0 0 1

 */
