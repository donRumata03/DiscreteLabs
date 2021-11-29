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

fn lzw_decode(encoded: &Vec<usize>) -> Vec<usize> {
	let mut dict = (0..26).into_iter().map(|c| vec![c]).collect::<Vec<_>>();
	let mut res = Vec::new();

	for (i, &this_code) in encoded.iter().enumerate() {
		if i != encoded.len() - 1 {
			let this_seq = &dict[this_code];

			let next_code = encoded[i + 1];
			let next_seq = &dict[next_code];

			// Buffer has been freed =>
			// next seq's last symbol placed after this seq doesn't exist in the dictionary and is added:

		}
	}

	if !buffer.is_empty() { res.push(dict.iter().position(|v| *v == buffer).unwrap()) };


	res
}


//////////////////////////////////////////////////////////////////////////////////////////////////


fn main() {
	let mut input = InputReader::new();
	// let mut output = OutputWriter::new();

	let n = input.next();
	let mut input_vector = Vec::new();
	for _ in 0..n {
		input_vector.push(input.next());
	}

	let lzw_decoded = lzw_decode(&input_vector);

	let ans_strings = lzw_decoded.iter().map(|&alpha_code| (alpha_code as u8 + 'a' as u8) as char).collect::<String>();
	println!("{}", ans_strings);
}
