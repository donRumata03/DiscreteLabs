#![allow(dead_code)]
#![allow(unused_imports)]

use {
	std::{
		io::{
			self,
			Read,
			Write,
			Stdin,
			Stdout,
			BufReader
		},
		fmt::{Display},
		str,
		cmp::min,
		iter::once,
		fs::{File, OpenOptions},
	}
};
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use std::ops::{Mul, Div, Sub, Add};
use std::fmt;


/// Writer
pub struct OutputWriter<W: Write> {
	writer: W,
	buf: Vec<u8>,
}

impl OutputWriter<Stdout> {
	pub fn new() -> Self { Self::from_writer(io::stdout()) }
}

impl OutputWriter<File> {
	pub fn from_file(path: &str) -> Self {
		let file = OpenOptions::new()
			.write(true)
			.create(true)
			.open(path);
		Self::from_writer(file.unwrap())
	}
}

impl<W: Write> OutputWriter<W> {
	pub fn from_writer(writer: W) -> Self {
		let buf = Vec::with_capacity(1 << 16);
		Self { writer, buf }
	}

	pub fn print<T: Display>(&mut self, t: T) {
		write!(self, "{}", t).unwrap();
	}

	pub fn prints<T: Display>(&mut self, t: T) {
		write!(self, "{} ", t).unwrap();
	}

	pub fn println<T: Display>(&mut self, t: T) {
		writeln!(self, "{}", t).unwrap();
	}
}

impl<W: Write> Write for OutputWriter<W> {
	fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
		self.buf.extend(bytes);
		Ok(bytes.len())
	}

	fn flush(&mut self) -> std::io::Result<()> {
		self.writer.write_all(&self.buf)?;
		self.writer.flush()?;
		self.buf.clear();
		Ok(())
	}
}

impl<W: Write> Drop for OutputWriter<W> {
	fn drop(&mut self) { self.flush().unwrap(); }
}


const EOF: &'static str = "InputReader: Reached end of input!";

pub struct InputReader<R: Read> {
	reader: R,
	buf: Vec<u8>,
	bytes_read: usize,
	current_index: usize,
}

impl InputReader<Stdin> {
	pub fn new() -> Self {
		Self::from_reader(io::stdin())
	}
}

impl InputReader<File> {
	pub fn from_file(path: &str) -> Self {
		Self::from_reader(File::open(path).unwrap())
	}
}

impl<R: Read> InputReader<R> {
	pub fn from_reader(reader: R) -> Self {
		Self {
			reader,
			buf: vec![0; 1 << 16],
			bytes_read: 0,
			current_index: 0,
		}
	}

	pub fn next<T: InputReadable>(&mut self) -> T {
		T::from_input(self)
	}

	pub fn next_line(&mut self) -> String {
		assert!(self.has_more(), EOF);
		let mut line = String::new();
		while self.peek() != '\n' {
			line.push(self.peek());
			self.consume();
			if !self.has_more() { break; }
		}
		self.consume(); // consume '\n'
		line
	}

	pub fn has_more(&mut self) -> bool {
		if self.current_index >= self.bytes_read {
			self.bytes_read = self.reader.read(&mut self.buf[..]).unwrap();
			self.current_index = 0
		}
		self.bytes_read > 0
	}

	pub fn set_buf_size(&mut self, buf_size: usize) {
		self.buf.resize(buf_size, 0);
	}

	fn peek(&self) -> char { self.buf[self.current_index] as char }

	fn consume(&mut self) { self.current_index += 1; }

	fn pop_digit(&mut self) -> u64 {
		let c = self.peek();
		self.consume();
		c as u64 - '0' as u64
	}

	fn consume_until<F: Fn(char) -> bool>(&mut self, test: F) {
		loop {
			assert!(self.has_more(), EOF);
			if test(self.peek()) { return; }
			self.consume();
		}
	}

	fn consume_until_sign(&mut self) -> i64 {
		loop {
			self.consume_until(|c| c.is_ascii_digit() || c == '-');
			if self.peek() != '-' { return 1; }

			self.consume();
			assert!(self.has_more(), EOF);
			if self.peek().is_ascii_digit() { return -1; }
		}
	}
}

pub trait InputReadable {
	fn from_input<R: Read>(input: &mut InputReader<R>) -> Self;
}

impl InputReadable for u64 {
	fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
		input.consume_until(|c| c.is_ascii_digit());
		let mut num = 0;
		while input.peek().is_ascii_digit() {
			num = num * 10 + input.pop_digit();
			if !input.has_more() { break; }
		}
		num
	}
}

impl InputReadable for i64 {
	fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
		let sign = input.consume_until_sign();
		u64::from_input(input) as i64 * sign
	}
}

impl InputReadable for f64 {
	fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
		let sign = input.consume_until_sign() as f64;
		let mut num = 0.0;
		while input.peek().is_ascii_digit() {
			num = num * 10.0 + input.pop_digit() as f64;
			if !input.has_more() { break; }
		}

		let mut factor = 1.0;
		if input.peek() == '.' {
			input.consume();
			while input.has_more() && input.peek().is_ascii_digit() {
				num = num * 10.0 + input.pop_digit() as f64;
				factor *= 10.0;
			}
		}
		sign * num / factor
	}
}

impl InputReadable for String {
	fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
		input.consume_until(|c| c.is_ascii_graphic());
		let mut word = String::new();
		while input.peek().is_ascii_graphic() {
			word.push(input.peek());
			input.consume();
			if !input.has_more() { break; }
		}
		word
	}
}

impl InputReadable for char {
	fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
		input.consume_until(|c| c.is_ascii_graphic());
		let c = input.peek();
		input.consume();
		c
	}
}

macro_rules! impl_readable_from {
  ($A:ty, [$($T:ty),+]) => {
    $(impl InputReadable for $T {
      fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
        <$A>::from_input(input) as $T
      }
    })+
  };
}
impl_readable_from!{ u64, [u32, u16, u8, usize] }
impl_readable_from!{ i64, [i32, i16, i8, isize] }
impl_readable_from!{ f64, [f32] }
//////////////////////////////////////////////////////////////////////////////////////////////////

//! Module scope to avoid namespace polution
pub mod bigint {
	// Don't change the base
	const BASE: i32 = 1000;

	/// BigInteger (Integer)
	#[derive(Clone, Debug)]
	pub struct BigInteger {
		/// Sign are used to determine wether the number is positive or negative
		/// * true for positive
		/// * false for negative
		pub sign: bool,
		pub number: Vec<i32>,
	}

	impl BigInteger {
		pub fn from_str(number_string: &str) -> BigInteger {
			// Check if it's a negative value or not by it's sign
			let sign = number_string.chars().nth(0).unwrap();

			// Remove the sign
			let number_string_after_sign = match sign {
				'-' | '+' => &number_string[1..],
				_ => number_string,
			};

			BigInteger {
				sign: sign != '-',
				number: BigUInteger::from_str(number_string_after_sign).number,
			}
		}
	}

	//#[allow(clippy::suspicious_arithmetic_impl)]
	impl std::ops::Sub for BigInteger {
		type Output = BigInteger;

		fn sub(self, other: BigInteger) -> BigInteger {
			let mut other_ = BigInteger { ..other };
			let mut self_ = BigInteger { ..self };
			// To make the bigger number goes to left (self_) and the smaller goes to right (other_)
			let swapped = if BigUInteger::from_BigInteger(self_.clone())
				< BigUInteger::from_BigInteger(other_.clone())
			{
				std::mem::swap(&mut self_, &mut other_);
				true
			} else {
				false
			};
			// (1+) - (2+)
			if self_.sign && other_.sign {
				BigInteger {
					sign: !swapped,
					number: (BigUInteger::from_BigInteger(self_.clone())
						- BigUInteger::from_BigInteger(other_.clone()))
						.number,
				}
				// (1-) - (2-) == -((1+) - (2+))
			} else if !self_.sign && !other_.sign {
				BigInteger {
					sign: swapped,
					number: (BigUInteger::from_BigInteger(self_.clone())
						- BigUInteger::from_BigInteger(other_.clone()))
						.number,
				}
				// (1-) - (2+) == -((1+) + (2+))
			} else if !self_.sign && other_.sign {
				BigInteger {
					sign: swapped,
					number: (BigUInteger::from_BigInteger(self_.clone())
						+ BigUInteger::from_BigInteger(other_.clone()))
						.number,
				}
				// (1+) - (2-) == (1+) + (2+)
			} else {
				BigInteger {
					sign: !swapped,
					number: (BigUInteger::from_BigInteger(self_.clone())
						+ BigUInteger::from_BigInteger(other_.clone()))
						.number,
				}
			}
		}
	}

	//#[allow(clippy::suspicious_arithmetic_impl)]
	impl std::ops::Add for BigInteger {
		type Output = BigInteger;

		fn add(self, other: BigInteger) -> BigInteger {
			let mut other_ = BigInteger { ..other };
			let mut self_ = BigInteger { ..self };
			// (1+) + (2+)
			if self_.sign == other_.sign {
				BigInteger {
					sign: self_.sign,
					number: (BigUInteger::from_BigInteger(self_.clone())
						+ BigUInteger::from_BigInteger(other_.clone()))
						.number,
				}
			} else {
				// To make the bigger number goes to left (self_) and the smaller goes to right (other_)
				if BigUInteger::from_BigInteger(self_.clone())
					< BigUInteger::from_BigInteger(other_.clone())
				{
					std::mem::swap(&mut self_, &mut other_);
				}
				// (1+) +  (2-) == (1+) - (2+)
				if self_.sign && !other_.sign {
					BigInteger {
						sign: true,
						number: (BigUInteger::from_BigInteger(self_.clone())
							- BigUInteger::from_BigInteger(other_.clone()))
							.number,
					}
					// (1-) + (2+) == -((1+) - (2+))
				} else {
					BigInteger {
						sign: false,
						number: (BigUInteger::from_BigInteger(self_.clone())
							- BigUInteger::from_BigInteger(other_.clone()))
							.number,
					}
				}
			}
		}
	}

	impl std::cmp::Eq for BigInteger {}

	impl std::cmp::PartialEq for BigInteger {
		fn eq(&self, other: &BigInteger) -> bool {
			self.sign == other.sign
				&& BigUInteger::from_BigInteger(self.clone())
				== BigUInteger::from_BigInteger(other.clone())
		}
	}

	impl std::cmp::Ord for BigInteger {
		fn cmp(&self, other: &BigInteger) -> std::cmp::Ordering {
			if self.sign != other.sign {
				if !self.sign {
					std::cmp::Ordering::Less
				} else {
					std::cmp::Ordering::Greater
				}
			} else if self.sign {
				BigUInteger::from_BigInteger(self.clone())
					.cmp(&BigUInteger::from_BigInteger(other.clone()))
			} else {
				BigUInteger::from_BigInteger(other.clone())
					.cmp(&BigUInteger::from_BigInteger(self.clone()))
			}
		}
	}

	impl std::cmp::PartialOrd for BigInteger {
		fn partial_cmp(&self, other: &BigInteger) -> std::option::Option<std::cmp::Ordering> {
			Some(self.cmp(other))
		}
	}

	impl std::fmt::Display for BigInteger {
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			let sign = if self.sign {
				"" // Positive sign
			} else {
				"-" // Negative sign
			};

			write!(
				f,
				"{}{}",
				sign,
				BigUInteger::from_BigInteger(self.clone()).to_string()
			)
		}
	}

	/// BigUInteger (Whole number)
	#[derive(Clone, Debug)]
	pub struct BigUInteger {
		pub number: Vec<i32>,
	}

	impl BigUInteger {
		pub fn from_str(number_string: &str) -> BigUInteger {
			let mut number_vector: Vec<i32> = Vec::new();
			let base_dig = f64::from(BASE).log10() as usize;
			// Backward version compatibility, siome of online judger doesn't support step_by yet
			// - .step_by(base_dig)
			// + .filter(|x| (number_string.len() - x) % base_dig == 0)
			//#[allow(clippy::range_plus_one)]
			for i in (base_dig..number_string.len() + 1)
				.rev()
				.filter(|x| (number_string.len() - x) % base_dig == 0)
			{
				let temp = &number_string[i - base_dig..i].parse::<i32>().unwrap();
				number_vector.push(*temp);
			}
			if number_string.len() % base_dig != 0 {
				let temp = &number_string[..number_string.len() % base_dig]
					.parse::<i32>()
					.unwrap();
				number_vector.push(*temp);
			}
			BigUInteger {
				number: number_vector.iter().rev().cloned().collect::<Vec<i32>>(),
			}
		}

		/// Convertion from BigInteger to BigUInteger by dropping the sign
		#[allow(non_snake_case)]
		#[inline]
		fn from_BigInteger(big_integer: BigInteger) -> BigUInteger {
			BigUInteger {
				number: big_integer.number,
			}
		}

		fn to_string(&self) -> String {
			let mut output_string: String = self
				.number
				.iter()
				.map(|&x| format!("{:03}", x))
				.collect::<Vec<String>>()
				.join("");
			while &output_string[0..1] == "0" && output_string.len() > 1 {
				output_string.remove(0);
			}
			output_string
		}
	}

	//#[allow(clippy::suspicious_arithmetic_impl)]
	impl std::ops::Sub for BigUInteger {
		type Output = BigUInteger;

		fn sub(self, other: BigUInteger) -> BigUInteger {
			// Implemented from: https://www.geeksforgeeks.org/difference-of-two-large-numbers/
			let mut other_ = BigUInteger { ..other };
			let mut self_ = BigUInteger { ..self };
			if self_.number.len() < other_.number.len() {
				std::mem::swap(&mut self_, &mut other_);
			}
			let mut number_output: Vec<i32> = Vec::new();
			let digit_difference = self_.number.len() - other_.number.len();
			let mut carry: i32 = 0;
			for i in (0..other_.number.len()).rev() {
				let mut sub: i32 = self_.number[i + digit_difference] - other_.number[i] - carry;
				if sub < 0 {
					sub += BASE;
					carry = 1;
				} else {
					carry = 0;
				}
				number_output.push(sub);
			}
			for i in (0..self_.number.len() - other_.number.len()).rev() {
				if self_.number[i] == 0 && carry != 0 {
					number_output.push(BASE - 1);
					continue;
				}
				let sub = self_.number[i] - carry;
				if i > 0 || sub > 0 {
					number_output.push(sub);
				}
				carry = 0;
			}
			BigUInteger {
				number: number_output.iter().rev().cloned().collect::<Vec<i32>>(),
			}
		}
	}

	//#[allow(clippy::suspicious_arithmetic_impl)]
	impl std::ops::Add for BigUInteger {
		type Output = BigUInteger;

		fn add(self, other: BigUInteger) -> BigUInteger {
			// Implemented from https://www.geeksforgeeks.org/sum-two-large-numbers/
			let mut other_ = BigUInteger { ..other };
			let mut self_ = BigUInteger { ..self };
			if self_.number.len() > other_.number.len() {
				std::mem::swap(&mut self_, &mut other_);
			}
			let mut number_output: Vec<i32> = Vec::new();
			let digit_difference = other_.number.len() - self_.number.len();
			let mut carry: i32 = 0;
			for i in (0..self_.number.len()).rev() {
				let sum = self_.number[i] + other_.number[i + digit_difference] + carry;
				number_output.push(sum % BASE);
				carry = sum / BASE;
			}
			for i in (0..other_.number.len() - self_.number.len()).rev() {
				let sum = other_.number[i] + carry;
				number_output.push(sum % BASE);
				carry = sum / BASE;
			}
			if carry != 0 {
				number_output.push(carry);
			}
			BigUInteger {
				number: number_output.iter().rev().cloned().collect::<Vec<i32>>(),
			}
		}
	}

	impl std::cmp::Eq for BigUInteger {}

	impl std::cmp::PartialEq for BigUInteger {
		fn eq(&self, other: &BigUInteger) -> bool {
			self.number == other.number
		}
	}

	impl std::cmp::Ord for BigUInteger {
		fn cmp(&self, other: &BigUInteger) -> std::cmp::Ordering {
			if self.number.len() < other.number.len() {
				std::cmp::Ordering::Less
			} else if self.number.len() > other.number.len() {
				std::cmp::Ordering::Greater
			} else {
				for (self_i, other_i) in self.number.iter().zip(&other.number) {
					if self_i < other_i {
						return std::cmp::Ordering::Less;
					} else if self_i > other_i {
						return std::cmp::Ordering::Greater;
					}
				}
				std::cmp::Ordering::Equal
			}
		}
	}

	impl std::cmp::PartialOrd for BigUInteger {
		fn partial_cmp(&self, other: &BigUInteger) -> std::option::Option<std::cmp::Ordering> {
			Some(self.cmp(other))
		}
	}

	impl std::fmt::Display for BigUInteger {
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			write!(f, "{}", self.to_string())
		}
	}
}


//////////////////////////////////////////////////////////////////////////////////////////////////


/// Rational (m/n) numbers
#[derive(Clone, Copy, Debug)]
pub struct Rational {
	x: i32,
	y: i32,
}

impl Rational {
	fn gcd(x: i32, y: i32) -> i32 {
		if y == 0 {
			x
		} else {
			Rational::gcd(y, x%y)
		}
	}

	pub fn new(x: i32, y: i32) -> Rational {
		assert_ne!(y, 0);

		let d = Rational::gcd(i32::abs(x), i32::abs(y));

		Rational {
			x: x / d,
			y: y / d,
		}
	}
}


impl fmt::Display for Rational {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.y == 1 {
			write!(f, "{}", self.x)
		} else {
			write!(f, "{}/{}", self.x, self.y)
		}
	}
}

// to allow equality checks
impl PartialEq for Rational {
	fn eq(&self, other: &Rational) -> bool {
		(self.x == other.x) && (self.y == other.y)
	}
}

impl Eq for Rational {}


// All the add traits
impl Add for Rational {
	type Output = Rational;

	fn add(self, other: Rational) -> Rational {
		Rational::new(self.x * other.y + self.y * other.x, self.y * other.y)
	}
}

impl Add<i32> for Rational {
	type Output = Rational;

	fn add(self, other: i32) -> Rational {
		Rational::add(self, Rational::new(other, 1))
	}
}

impl Add<Rational> for i32 {
	type Output = Rational;

	fn add(self, other: Rational) -> Rational {
		Rational::add(Rational::new(self, 1), other)
	}
}

// All the sub traits
impl Sub for Rational {
	type Output = Rational;

	fn sub(self, other: Rational) -> Rational {
		Rational::new(self.x * other.y - self.y * other.x, self.y * other.y)
	}
}

impl Sub<i32> for Rational {
	type Output = Rational;

	fn sub(self, other: i32) -> Rational {
		Rational::sub(self, Rational::new(other, 1))
	}

}

impl Sub<Rational> for i32 {
	type Output = Rational;

	fn sub(self, other: Rational) -> Rational {
		Rational::sub(Rational::new(self, 1), other)
	}
}

// All the mul traits
impl Mul for Rational {
	type Output = Rational;

	fn mul(self, other: Rational) -> Rational {
		Rational::new(self.x * other.x, self.y * other.y)
	}
}

impl Mul<i32> for Rational {
	type Output = Rational;

	fn mul(self, other: i32) -> Rational {
		Rational::mul(self, Rational::new(other, 1))
	}
}

impl Mul<Rational> for i32 {
	type Output = Rational;

	fn mul(self, other: Rational) -> Rational {
		Rational::mul(Rational::new(self, 1), other)
	}
}


// All the div traits
impl Div for Rational {
	type Output = Rational;

	fn div(self, other: Rational) -> Rational {
		Rational::new(self.x * other.y, self.y * other.x)
	}
}

impl Div<i32> for Rational {
	type Output = Rational;

	fn div(self, other: i32) -> Rational {
		Rational::div(self, Rational::new(other, 1))
	}
}

impl Div<Rational> for i32 {
	type Output = Rational;

	fn div(self, other: Rational) -> Rational {
		Rational::div(Rational::new(self, 1), other)
	}
}

//////////////////////////////////////////////////////////////////////////////////////////////////

// IO and parsing bit strings:

fn read_line() -> String {
	let mut input_string = String::new();
	io::stdin().read_line(&mut input_string).expect("");
	while input_string.as_bytes().last().unwrap().is_ascii_whitespace() {
		input_string.pop();
	}

	input_string
}
trait ToBoolVector {
	fn ascii_as_bool_vector(&self) -> Vec<bool>;
}
impl ToBoolVector for String {
	fn ascii_as_bool_vector(&self) -> Vec<bool> {
		self.chars().map(|c| match c {
			'0' => false,
			'1' => true,
			_ => panic!()
		}).collect()
	}
}

trait BoolVecToString {
	fn to_string(&self) -> String;
}
impl BoolVecToString for Vec<bool> {
	fn to_string(&self) -> String {
		self.iter().map(
			|&v| if v { '1' } else {'0'}
		).collect::<String>()
	}
}

//////////////////////////////////////////////////////////////////////////////////////////////////

fn ariphmetic_encode(encoded: &Vec<bool>) -> Vec<bool> {

}

fn build_freqs(n: usize, string: &str) {
	assert!(n <= 26);
	let mut res = vec![];
}

fn main() {
	// let mut input = InputReader::new();
	let mut output = OutputWriter::new();

	let mode = read_line().parse::<usize>().unwrap();

	let string_for_encoding = read_line();
	let bit_vec = string_for_encoding.ascii_as_bool_vector();

	let encoded = ariphmetic_encode(&bit_vec);
	let encoded = encoded.to_string();

	output.println(encoded);
}
