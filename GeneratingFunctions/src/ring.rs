use std::cmp::Eq;

/// Commutative ring
pub trait CRing {
    type E: Copy + Eq; // Element type

    fn add(&self, a: Self::E, b: Self::E) -> Self::E; // Addition is both associative and commutative
    fn negate(&self, a: Self::E) -> Self::E; // negate is inverse element by addition

    fn zero(&self) -> Self::E;

    fn multiply(&self, a: Self::E, b: Self::E) -> Self::E; // Both associative and commutative
    fn one(&self) -> Self::E;

    fn power(&self, a: Self::E, n: usize) -> Self::E {
        // Use binary exponentiation
        let mut result = Self::one();
        let mut a = a;
        let mut n = n;
        while n > 0 {
            if n % 2 == 1 {
                result = Self::multiply(result, a);
            }
            a = Self::multiply(a, a);
            n /= 2;
        }
        result
    }

    fn factorial(&self, n: usize) -> Self::E {
        let mut result = Self::one();
        let mut multiplier = Self::one();

        for _ in 0..n {
            multiplier = Self::add(multiplier, Self::one());
            result = Self::multiply(result, multiplier);
        }

        result
    }
}

/// Ring instance for Residues mod `m`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Residue {
    modulo: u64,
}

impl Residue {
    pub fn new(modulo: u64) -> Self {
        Residue { modulo }
    }

    pub fn modulo(&self) -> u64 {
        self.modulo
    }
}

impl CRing for Residue {
    type E = u64;

    fn add(&self, a: Self::E, b: Self::E) -> Self::E {
        (a + b) % self.modulo
    }

    fn negate(&self, a: Self::E) -> Self::E {
        self.modulo - a
    }

    fn zero(&self) -> Self::E {
        0
    }

    fn multiply(&self, a: Self::E, b: Self::E) -> Self::E {
        (a * b) % self.modulo
    }

    fn one(&self) -> Self::E {
        1
    }
}
