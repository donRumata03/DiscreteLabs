use std::fmt::Debug;

trait CombinatorialObject<T> {
    fn is_self_contained(&self) -> bool;
    fn can_add(&self, element: &T) -> bool;
    fn count_successors(&self) -> u64;
    fn push(&mut self, element: T);
    fn pop(&mut self) -> T;

    // TODO: separate this to concept of static data for all objects of this subclass
    // TODO: and state-tracker object which can go forward and backward
    // fn view_sorted_alphabet(&self) -> &Vec<T>;
    // fn possible_direct_successors(&self) -> Vec<T> {
    //     self.view_sorted_alphabet().iter().filter(|&e| self.can_add(e)).collect()
    // }
}

/// `prefix` initially has some state that will be considered
fn generate_all<T, C>(prefix: &mut C, sorted_alphabet: &Vec<T>, answer_container: &mut Vec<C>)
    where C: CombinatorialObject<T> + Clone + Debug,
            T: Clone
{
    println!("{:?}", prefix);

    if prefix.is_self_contained() {
        answer_container.push(prefix.clone())
    }

    for c in sorted_alphabet.iter()/*.filter(|&c| prefix.can_add(c))*/ {
        if !prefix.can_add(c) {
            continue;
        }
        prefix.push(c.clone());
        generate_all(prefix, sorted_alphabet, answer_container);
        prefix.pop();
    }
}


#[derive(Clone, Debug)]
struct Sequence {
    len : usize,
    states : u64,
    data: Vec<u64>
}

impl Sequence {
    pub fn new(len: usize, states: u64) -> Self {
        Sequence { len, states, data: Vec::with_capacity(len), /*sorted_alphabet: (1..=states).collect()*/ }
    }
}

impl CombinatorialObject<u64> for Sequence {
    fn is_self_contained(&self) -> bool {
        self.data.len() == self.len
    }

    fn can_add(&self, _: &u64) -> bool {
        self.data.len() < self.len
    }

    fn count_successors(&self) -> u64 {
        return (self.states as u64)
            .pow((self.len - self.data.len()) as u32);
    }

    fn push(&mut self, element: u64) {
        self.data.push(element);
    }

    fn pop(&mut self) -> u64 {
        self.data.pop().unwrap()
    }

    // fn view_sorted_alphabet(&self) -> &Vec<u64> {
    //     &self.sorted_alphabet
    // }
}

fn main() {
    let mut pref = Sequence::new(3, 2);
    let mut ans = Vec::with_capacity(pref.count_successors() as usize);
    let alphabet: Vec<u64> = (0..=1).collect();

    generate_all(&mut pref, &alphabet, &mut ans);

    println!("{:?}", ans);
}