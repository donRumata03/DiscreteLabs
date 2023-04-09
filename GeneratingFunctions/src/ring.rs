use std::cmp::Eq;

/// Commutative ring
pub trait CRing {
    type E: Clone; // Element type

    fn add(&self, a: Self::E, b: Self::E) -> Self::E; // Addition is both associative and commutative
    fn negate(&self, a: Self::E) -> Self::E; // negate is inverse element by addition

    fn zero(&self) -> Self::E;

    fn multiply(&self, a: Self::E, b: Self::E) -> Self::E; // Both associative and commutative
    fn one(&self) -> Self::E;

    fn subtract(&self, a: Self::E, b: Self::E) -> Self::E {
        self.add(a, self.negate(b))
    }

    fn power(&self, a: Self::E, n: usize) -> Self::E {
        // Use binary exponentiation
        let mut result = self.one();
        let mut a = a;
        let mut n = n;
        while n > 0 {
            if n % 2 == 1 {
                result = self.multiply(result, a.clone());
            }
            a = self.multiply(a.clone(), a.clone());
            n /= 2;
        }
        result
    }

    fn factorial(&self, n: usize) -> Self::E {
        let mut result = self.one();
        let mut multiplier = self.one();

        for _ in 0..n {
            multiplier = self.add(multiplier.clone(), self.one());
            result = self.multiply(result, multiplier.clone());
        }

        result
    }
}

/// Definitive ring
pub trait DRing: CRing
where
    <Self as CRing>::E: Copy + Eq,
{
}

pub trait Field: CRing {
    fn inverse(&self, a: Self::E) -> Self::E;
    fn divide(&self, a: Self::E, b: Self::E) -> Self::E {
        self.multiply(a, self.inverse(b))
    }

    // Also, distributivity of multiplication over addition is implied in field
}

/// Definitive field
pub trait DField: Field + DRing
where
    Self::E: Copy + Eq,
{
}

/// Ring instance for Residues mod `m`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrimeResidue {
    modulo: u64,
}

impl PrimeResidue {
    /// Modulo must be prime
    pub fn new(modulo: u64) -> Self {
        PrimeResidue { modulo }
    }

    pub fn modulo(&self) -> u64 {
        self.modulo
    }
}

impl CRing for PrimeResidue {
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
impl DRing for PrimeResidue {}

impl Field for PrimeResidue {
    fn inverse(&self, a: Self::E) -> Self::E {
        // Use binary exponentiation
        self.power(a, (self.modulo - 2) as usize)
    }
}
