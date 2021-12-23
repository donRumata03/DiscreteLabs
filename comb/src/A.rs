


trait CombinatorialObject<T> {
    fn is_self_contained(&self) -> bool;
    fn can_add(&self, element: &T) -> bool;
    fn count_successors(&self) -> u64;
    fn push(&mut self, element: T);
    fn pop(&mut self) -> T;
}

struct Sequence {
    len : usize,
    states : u64,
    data: Vec<u64>
}

impl Sequence {
    pub fn new(len: usize, states: u64) -> Self {
        Sequence { len, states, data: Vec::with_capacity(len) }
    }
}

impl CombinatorialObject<u64> for Sequence {
    fn is_self_contained(&self) -> bool {
        self.data.len() == self.len
    }

    fn can_add(&self, _: &u64) -> bool {
        self.len < self.data.len()
    }

    fn count_successors(&self) -> u64 {
        return ((self.len - self.data.len()) as u64)
            .pow(self.states as u32);
    }

    fn push(&mut self, element: u64) {
        self.data.push(element);
    }

    fn pop(&mut self) -> u64 {
        self.data.pop().unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}