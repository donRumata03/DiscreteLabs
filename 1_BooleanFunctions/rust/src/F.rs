// F - Horn-Sat

use std::{ops, io};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fmt;
use std::iter::FromIterator;
use std::io::{stdin, BufRead, BufReader};
use std::str;


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

enum VarState {
	Nothing,
	Direct,
	Inverted
}

fn has_resolutions(horn_form: Vec<Vec<VarState>>) -> bool {

}


/// Main
fn main() {
	// println!("{}", reverse_bitmask(0b001, 3));

	let mut scanner = Scanner::new(BufReader::new(io::stdin()));

	let n: usize = scanner.token();
	let k: usize = scanner.token();

	let mut horn_form = Vec::new();

	for line_index in 0..k {
		horn_form.push(Vec::new());
		let this_clause = horn_form.last_mut().unwrap();
		for var_index in 0..n {
			this_clause.push(match scanner.token::<isize>() {
				1 => VarState::Direct,
				-1 => VarState::Inverted,
				0 => VarState::Nothing,
				_ => panic!()
			});
		}
	}

	println!("{}", if has_resolutions(horn_form) { "NO" } else { "YES" });
}
