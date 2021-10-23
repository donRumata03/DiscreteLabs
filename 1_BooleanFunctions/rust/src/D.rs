#![allow(unused_imports)]

use std::{ops, io};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fmt;
use std::iter::FromIterator;
use std::io::{stdin, BufRead, BufReader};
use std::str;

macro_rules! matches {
    ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )?) => {
        match $expression {
            $( $pattern )|+ $( if $guard )? => true,
            _ => false
        }
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

/// ////////////////////////////////////////////////////

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

		self.mask = (self.mask & !(1 << position)) | ((value as usize) << position);
	}

	fn set_true(&mut self, position: usize) {
		self.mask |= (1_usize) << position;
	}
	fn set_false(&mut self, position: usize) {
		self.mask &= !((1_usize) << position);
	}
}



fn reverse_bitmask(bitmask: usize, dims: usize) -> usize {
	let mut res = BitSet::from(0);

	for i in 0..dims {
		res.set_to(dims - i - 1, bitmask & (1 << i) != 0);
	}

	res.mask
}
fn to_bin_vec(bitmask: usize, dims: usize) -> Vec<bool> {
	let mut res = Vec::with_capacity(dims);

	for i in 0..dims {
		res.push(bitmask & (1 << i) != 0);
	}

	res
}

/// ////////////////////////////////////////////////////

enum Element {
	Not(usize),
	And(usize, usize),
	Or(usize, usize)
}

impl Display for Element {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Element::Not(arg) => write!(f, "1 {}", arg),
			Element::And(l, r) => write!(f, "2 {} {}", l, r),
			Element::Or(l, r) => write!(f, "3 {} {}", l, r),
		}
	}
}

fn main() {
	let mut scanner = Scanner::new(BufReader::new(io::stdin()));

	let n: usize = scanner.token();
	let mut tt = vec![false; 2usize.pow(n as u32)];
	for i in 0usize..2usize.pow(n as u32) {
		let msk = scanner.token::<usize>();
		// assert_eq!(i, msk);
		// tt.push(scanner.token::<usize>() != 0);
		tt[reverse_bitmask(i, n as usize)] = scanner.token::<usize>() != 0;
	}


	let mut res = Vec::new();
	// Add «not vars»:
	for i in 0..n {
		res.push(Element::Not((i + 1) as usize));
	}
	let last_el_index = |rs: &Vec<Element>| rs.len() + n;

	let get_var_ind = |var| var + 1;
	let get_neg_ind = |var| var + n + 1;
	let var_with_neg = |var| (get_var_ind(var), get_neg_ind(var));

	// Preform conjunctions
	let mut conj_indexes = Vec::new();
	for msk in 0usize..2usize.pow(n as u32) {
		if !tt[msk] { continue; }

		let bs = BitSet::from(msk);
		let right_index = |vi| if bs.get(vi as usize) { get_var_ind(vi) } else {get_neg_ind(vi)};

		let (l, r) = (right_index(0), right_index(1));
		res.push(Element::And(l, r));
		for vi in 2..n {
			res.push(Element::And(last_el_index(&res), right_index(vi)));
		}
		conj_indexes.push(last_el_index(&res));
	}

	if conj_indexes.is_empty() {
		res.push(Element::And(get_var_ind(0), get_neg_ind(0)));
	} else {
		// Disjunct results:
		if conj_indexes.len() >= 2 {
			res.push(Element::Or(conj_indexes[0], conj_indexes[1]));

			for &di in conj_indexes[2..].iter() {
				res.push(Element::Or(last_el_index(&res), di))
			}
		}
	}


	// Print answer:
	println!("{}", last_el_index(&res));
	for el in res.iter() {
		println!("{}", el);
	}
}