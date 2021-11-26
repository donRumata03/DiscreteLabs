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

#[derive(Ord, PartialOrd, PartialEq, Eq)]
enum HuffmanTreeNodeStructureData {
    Leaf(usize),
    Connector { left: Box<HuffmanTreeNode>, right: Box<HuffmanTreeNode> }
}

#[derive(PartialEq, Eq)]
struct HuffmanTreeNode {
    freq: usize,
    structure_data: HuffmanTreeNodeStructureData
}


impl PartialOrd<Self> for HuffmanTreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.freq.cmp(&other.freq).reverse())
    }
}

impl Ord for HuffmanTreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn count_symbol_freqs(text: &[usize]) -> HashMap<usize, usize> {
    let mut res = HashMap::new();

    for &c in text.iter() {
       let e = res.entry(c);
        *e.or_default() += 1;
    }

    res
}

/// Smallest, second smallest
fn indexes_of_two_smallest<T: Ord>(array: &[T]) -> (Option<usize>, Option<usize>) {
    let mut smallest: Option<usize> = None;
    let mut second_smallest: Option<usize> = None;

    for (i, v) in array.iter().enumerate() {
        if smallest.is_none() || v < &array[smallest.unwrap()] {
            second_smallest = smallest;
            smallest = Some(i);
        } else if second_smallest.is_none() || v < &array[second_smallest.unwrap()] {
            second_smallest = Some(i);
        }
    }

    (smallest, second_smallest)
}

fn build_huffman_tree(freqs: &HashMap<usize, usize>) -> HuffmanTreeNode {
    // let freqs = count_symbol_freqs(text);

    let mut node_pool = BinaryHeap::new();
    freqs.iter().for_each(|v| node_pool.push(HuffmanTreeNode {
        freq: *v.1,
        structure_data: HuffmanTreeNodeStructureData::Leaf(*v.0)
    }));

    while node_pool.len() > 1 {
        // // Find two smallest:
        // let (smallest, sec_smallest) = indexes_of_two_smallest(node_pool.as_slice());
        // let temp_vector = node_pool.iter().enumerate().filter_map(|v|)

        // Extract two smallest:
        let smallest = node_pool.pop().unwrap();
        let second_smallest = node_pool.pop().unwrap();

        node_pool.push(HuffmanTreeNode {
            freq: smallest.freq + second_smallest.freq,
            structure_data: HuffmanTreeNodeStructureData::Connector {
                left: Box::from(smallest),
                right: Box::from(second_smallest),
            }
        });
    }

    assert_eq!(node_pool.len(), 1);
    node_pool.into_iter().next().unwrap()
}

fn dump_tree_impl(tree: &HuffmanTreeNode, to: &mut HashMap<usize, Vec<bool>>, code_prefix: &mut Vec<bool>) {
    match &tree.structure_data {
        HuffmanTreeNodeStructureData::Leaf(value) => {
            to.insert(*value, code_prefix.clone());
        }
        HuffmanTreeNodeStructureData::Connector { left, right } => {
            code_prefix.push(false);
            dump_tree_impl(left, to, code_prefix);
            *(code_prefix.last_mut().unwrap()) = true;
            dump_tree_impl(right, to, code_prefix);
            code_prefix.pop();
        }
    };
}

fn dump_tree(tree: &HuffmanTreeNode) -> HashMap<usize, Vec<bool>> {
    let mut coding = HashMap::new();

    let mut code_prefix = Vec::new();
    dump_tree_impl(tree, &mut coding, &mut code_prefix);

    coding
}

fn main() {
    let mut input = InputReader::new();
    let mut output = OutputWriter::new();


    let n: usize = input.next();
    let mut data = Vec::new();

    for _ in 0..n {
        data.push(input.next());
    }

    // let freqs = count_symbol_freqs(&data);
    let freqs = data.iter().enumerate().map(|(i, &v)| (i, v)).collect();
    // println!("{:?}", freqs);

    let tree = build_huffman_tree(&freqs);
    let converter = dump_tree(&tree);

    let mut sum_len = 0_usize;
    for (k, v) in converter.iter() {
        let bin_string = v.iter().map(|&v| if v {'1'} else {'0'}).collect::<String>();
        // output.println(format!("{} {}", k, bin_string));
        sum_len += v.len() * freqs[k];
    }

    output.println(sum_len);

}

/*
10
1 2 3 4 5 6 7 8 9 10
*/