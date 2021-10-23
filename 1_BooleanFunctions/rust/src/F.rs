// F - Horn-Sat

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

#[derive(Clone, Copy, Debug)]
enum VarState {
	Nothing,
	Direct,
	Inverted
}

fn has_resolutions(horn_form: Vec<Vec<VarState>>) -> bool {
	let clauses_n = horn_form.len();
	let var_n = horn_form.first().unwrap().len();

	let mut temp_expr: Vec<Option<Vec<VarState>>> =
		horn_form.into_iter().map(Some).collect();

	let mut var_values = vec![None; var_n];

	loop {
		// let lonely_var_clauses: Vec<&Vec<VarState>> = temp_expr.iter().filter(|&disjunction| -> bool {
		// 	if disjunction.is_none() { false } else {
		// 		disjunction.as_ref().unwrap().iter().map(|element| if let VarState::Nothing = element { 0 } else { 1 }).sum::<usize>() == 1
		// 	}
		// }).map(|c| c.as_ref().unwrap()).collect();

		let mut lonely_var_clauses: Vec<&Vec<VarState>> = Vec::new();
		for disjunction in temp_expr.iter() {
			if disjunction.is_none() {
				continue;
			}
			let ref_to_value = disjunction.as_ref().unwrap();
			// for vs in ref_to_value {
			// 	if !matches!(ref_to_value, VarState::Nothing) {
			//
			// 	}
			// }
			if ref_to_value.iter().map(|element| if let VarState::Nothing = element { 0 } else { 1 }).sum::<usize>() == 1 {
				lonely_var_clauses.push(ref_to_value);
			}
		}

		if lonely_var_clauses.is_empty() {
			return true;
		}
		for l_var_clause in lonely_var_clauses {
			let var_index = l_var_clause.iter().take_while(|&var_state|
				if let VarState::Nothing = var_state { true } else { false }
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

		// println!("{:?}", var_values);
		// println!("{:?}", temp_expr);

// 	for mut clause in temp_expr.iter_mut().filter(|it| it.is_some()) {
		for i in 0..temp_expr.len() {
			if temp_expr[i].is_none() { continue; }

			let clause = temp_expr.get_mut(i).unwrap();
			// let c: &mut Vec<VarState> = clause.as_ref().as_mut().unwrap();

			// for (var_index, var_state) in c.iter_mut()
			// 	.enumerate()
			// 	.filter(|it| !matches!(it.1, VarState::Nothing)).
			// 	filter(|it| var_values[it.0].is_some())
			for var_index in 0..var_n
			{
				let var_state = clause.as_ref().unwrap().get(var_index).unwrap();

				if let VarState::Nothing = var_state { continue; }
				if var_values[var_index].is_none() { continue; }

				if var_values[var_index].unwrap() && matches!(var_state, VarState::Direct) ||
					!var_values[var_index].unwrap() && matches!(var_state, VarState::Inverted) {
					*clause = None;
					break;
				} else {
					// let t = clause.as_mut().unwrap();
					*clause.as_mut().unwrap().get_mut(var_index).unwrap() = VarState::Nothing;
				}
			}

			if clause.is_some() && clause.as_ref().unwrap().iter().all(|vs| matches!(vs, VarState::Nothing)) { return false; }
		}
	}
	println!("dfdf");

	return false;
}


/// Main
fn main() {
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


/*

3 5
1 0 -1
0 1 0
-1 0 1
0 -1 -1
-1 1 -1

 */