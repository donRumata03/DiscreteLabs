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
        fmt::{Display, Debug},
        str,
        cmp::min,
        iter::once,
        fs::{File, OpenOptions},
        ops::{Add, Mul}
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

fn factorial<T: Mul<Output=T> + Add<Output=T> + Copy + From<u64> + Eq> (value: &T) -> T {
    let mut res = T::from(1);
    let mut i = T::from(1);

    while i != *value {
        res = res * i;

        i = i + T::from(1_u64);
    }

    res * i
}

//////////////////////////////////////////////////////////////////////////////////////////////////


trait CombinatorialObject<T: Clone> {
    /// Self is guaranteed to be valid prefix be caller for these functions:

    fn is_self_contained(&self) -> bool;
    fn can_add(&self, element: &T) -> bool;
    fn count_successors(&self) -> u64;
    fn push(&mut self, element: T);
    fn pop(&mut self) -> T;

    // TODO: separate this to concept of static data for all objects of this subclass
    // TODO: and state-tracker object which can go forward and backward
    // fn view_sorted_alphabet(&self) -> &Vec<T>;
    fn possible_direct_successors(&self, sorted_alphabet: &Vec<T>) -> Vec<T> { // Not impl Iter<Output=T> because it changes
        sorted_alphabet.iter().filter(|&e| self.can_add(e)).cloned().collect() // FixMe…
    }

}

/// `prefix` initially has some state that will be considered
fn generate_all<T, C>(prefix: &mut C, sorted_alphabet: &Vec<T>, answer_container: &mut Vec<C>)
    where C: CombinatorialObject<T> + Clone + Debug,
            T: Clone
{
    // println!("{:?}", prefix);

    if prefix.is_self_contained() {
        answer_container.push(prefix.clone())
    }

    for c in prefix.possible_direct_successors(sorted_alphabet) /*.filter(|&c| prefix.can_add(c))*/ {
        prefix.push(c.clone());
        generate_all(prefix, sorted_alphabet, answer_container);
        prefix.pop();
    }
}

fn minimal_with_prefix<T, C>(prefix: &C, sorted_alphabet: &Vec<T>) 
-> C
    where C: CombinatorialObject<T> + Clone + Debug,
            T: Clone
{
    let mut current = prefix.clone();

    while !current.is_self_contained() {
        for c in sorted_alphabet.iter() {
            if current.can_add(c) {
                current.push(c.clone());
                break;
            }
        } // TODO: errors for infinite loops…
    }

    current
}

/// `prefix` initially has some state that will be considered
fn combinatorial_object_by_number<T, C>(number: u64, prefix: &C, sorted_alphabet: &Vec<T>)
    -> C
    where C: CombinatorialObject<T> + Clone + Debug,
            T: Clone
{
    let mut parent = prefix.clone();
    let mut smallest_index_with_parent = 0_u64;
    while smallest_index_with_parent != number {
        for c in sorted_alphabet.iter()/*.filter(|&c| prefix.can_add(c))*/ {
            if !parent.can_add(c) {
                continue;
            }
            parent.push(c.clone());
            let objects_with_new_prefix = parent.count_successors();
            if smallest_index_with_parent + objects_with_new_prefix > number 
            { 
                break
                ;//)
            }

            smallest_index_with_parent += objects_with_new_prefix;
            parent.pop();
        }
    }

    minimal_with_prefix(&parent, sorted_alphabet)
}


#[derive(Clone, Debug)]
struct Permutation<'a> {
    descriptor: &'a PermutationDescriptor,
    data: Vec<u64>,
}

#[derive(Clone, Debug)]
struct PermutationDescriptor {
    len : usize,
    sorted_alphabet: Vec<u64>
}

impl<'a> Permutation<'a> {
    pub fn new(descriptor: &'a PermutationDescriptor) -> Self {
        Permutation { descriptor, data: Vec::with_capacity(descriptor.len) }
    }
}

impl CombinatorialObject<u64> for Permutation<'_> {
    fn is_self_contained(&self) -> bool {
        self.data.len() == self.descriptor.len
    }

    fn can_add(&self, element: &u64) -> bool {
        self.data.len() < self.descriptor.len 
            && self.data
                    .iter()
                    .find(|&&existing_element| *element == existing_element)
                    .is_none()
    }

    fn count_successors(&self) -> u64 {
        let positions = self
                    .possible_direct_successors(&self.descriptor.sorted_alphabet)
                    .into_iter()
                    .count()
                    as u64;

        factorial(
            &positions   
            )

    }

    fn push(&mut self, element: u64) {
        self.data.push(element);
    }

    fn pop(&mut self) -> u64 {
        self.data.pop().unwrap()
    }

    // fn possible_direct_successors(&self) -> Vec<T> {
    //     let mut res = Vec::new();

    //     // for i in 1..self.descriptor.len {

    //     // }


    // }

    // fn view_sorted_alphabet(&self) -> &Vec<u64> {
    //     &self.sorted_alphabet
    // }
}


fn main() {
    let mut input = InputReader::new();
    let mut output = OutputWriter::new();

    // println!("{}", factorial(&5_u64));


    let n = input.next::<usize>();
    let query_index = input.next::<u64>();


    let alphabet: Vec<u64> = (1_u64..=(n as u64)).collect();

    let descriptor = PermutationDescriptor { len: n, sorted_alphabet: alphabet.clone() };
    let mut pref = Permutation::new(&descriptor);


    let ans = combinatorial_object_by_number(query_index, &mut pref, &alphabet);

    // dbg!(ans);
    println!("{}", ans.data.iter().map(u64::to_string).collect::<Vec<_>>().join(" "));

    // println!("{}", ans.len());
}