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

#[derive(Clone)]
enum VarState {
	Nothing,
	Direct,
	Inverted
}

fn has_resolutions(horn_form: Vec<Vec<VarState>>) -> bool {
	let clauses_n = horn_form.len();
	let var_n = horn_form.first().unwrap().len();

	let x: Option<Vec<i32>> = Some(Vec::new());

	// let mut temp_expr = horn_form.iter().clone().map(|c| Some(*c.clone())).collect::<Vec<Option<Vec<VarState>>>>();

	let mut var_values = vec![None; var_n];

	loop {
		let lonely_var_clauses: Vec<&Vec<VarState>> = horn_form.iter().filter(|&disjunction|
			disjunction.iter().map(|element| if let VarState::Nothing = element {0} else {1}).sum::<usize>() == 1
		).collect();

		if lonely_var_clauses.is_empty() {
			return true;
		} else {
			for l_var_clause in lonely_var_clauses {
				let var_index = l_var_clause.iter().take_while(|&var_state|
                   if let VarState::Nothing = var_state {true} else {false}
				).count();

				let var_state = l_var_clause.iter().filter(|vs|
					!matches!(vs, VarState::Nothing)
				).next().unwrap();

				let required_value = matches!(var_state, VarState::Direct);

				match var_values[var_index] {
					Some(v) => if v != required_value { return false; },
					None => { var_values[var_index] = Some(required_value); }
				}
			}

		}

	}

	println!("dfdf");

	return false;
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
				0 => VarState::Inverted,
				-1 => VarState::Nothing,
				_ => panic!()
			});
		}
	}

	println!("{}", if has_resolutions(horn_form) { "NO" } else { "YES" });
}
