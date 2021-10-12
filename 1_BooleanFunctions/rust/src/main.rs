use std::{ops, io};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fmt;
use std::iter::FromIterator;
use std::io::{stdin, BufRead, BufReader};
use std::str;

// _______________________          Bin Vec Utils           ___________________________

fn to_bin_vec(bitmask: usize, dims: usize) -> Vec<bool> {
	let mut res = Vec::with_capacity(dims);

	for i in 0..dims {
		res.push(bitmask & (1 << i) != 0);
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

// _______________________          Zhegalkin Polynomial           ___________________________


struct ZhegalkinPolynomial {
	data: Vec<bool>,
	dims: usize
}

impl ZhegalkinPolynomial {
	pub fn new(coeffs: Vec<bool>) -> ZhegalkinPolynomial {
		let sz = coeffs.len();
		ZhegalkinPolynomial {
			data: coeffs,
			dims: (sz as f32).log2() as usize
		}
	}
}

impl ops::Index<usize> for ZhegalkinPolynomial {
	type Output = bool;

	fn index(&self, i: usize) -> &bool {
		&self.data[i]
	}
}

// mobius_transform

fn get_slight_dominated(bitmask: usize) -> HashSet<usize> {
	let mut res = HashSet::new();

	let mut shift = 0usize;
	while (1 << shift) <= bitmask {
		if (1 << shift) & bitmask != 0 {
			res.insert(!(1 << shift) & bitmask);
		}
		shift += 1;
	}

	res
}

// TODO: use the method of branches and bounds with dyn
fn get_dominated_or_eq(bitmask: usize) -> HashSet<usize> {
	let mut res = HashSet::new();

	for submsk in get_slight_dominated(bitmask).iter() {
		res.extend(get_dominated_or_eq(*submsk));
	}
	res.insert(bitmask);

	res
}

fn mobius_transform(bf: &BoolFunction) -> ZhegalkinPolynomial {
	let n = bf.dims;

/*
	let res_vec = vec![None; n];

	for i in 0usize..(2usize.pow(n as u32)) {
		if res_vec[i].is_some() { continue; }


	}
*/

	ZhegalkinPolynomial::new(
		(0..(2usize.pow(n as u32))).map(
			|bitmask| get_dominated_or_eq(bitmask).into_iter()
				.map(|index| bf[index])
				.fold(false, |a, b| a ^ b)
		).collect()
	)
}

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

fn main() {

	// let xor = BoolFunction::new(vec![false, true, true, false]);
	// println!("{}", xor[0b00]);
	// println!("{}", xor[0b01]);
	// println!("{}", xor[0b10]);
	// println!("{}", xor[0b11]);
	//
	// println!("{}", xor);

	// let mut dct = HashMap::new();
	// dct.insert((false, false), false);
	// dct.insert((false, true), true);
	// dct.insert((true, false), true);
	// dct.insert((true, true), false);
	// println!("{:?}", dct);

	let mut input = String::from("");
	let mut scanner = Scanner::new(BufReader::new(io::stdin()));


	let n: usize = scanner.token();

	let mut tt = Vec::new();
	for _ in 0..(2usize.pow(n as u32)) {
		let spl_l = scanner.token::<usize>();
		let spl_r: usize = scanner.token();
		tt.push(spl_r > 0);
	}

	let z = mobius_transform(&BoolFunction::new(tt));
	for i in 0..(2usize.pow(n as u32)) {
		println!("{:0>w$b} {:b}", i, z[i] as u32, w=n);
	}

}
