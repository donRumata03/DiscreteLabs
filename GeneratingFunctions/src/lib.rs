use std::ops::{Add, Mul, Neg};

/// Commutative ring
trait CRing: Copy + Add<Output = Self> + Mul<Output = Self> + Neg<Output = Self> {
    // Addition is both associative and commutative
    // negate is inverse element by addition
    fn zero() -> Self;

    fn multiply(a: Self, b: Self) -> Self; // Both associative and commutative
    fn one() -> Self;

    fn power(a: Self, n: usize) -> Self {
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
}

/// Formal polynomial over a commutative ring `R[x]`
struct Polynomial<T: CRing> {
    coefficients: Vec<T>,
}

impl<T: CRing> Polynomial<T> {
    fn new(coefficients: Vec<T>) -> Polynomial<T> {
        Polynomial { coefficients }
    }

    fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }
}

// Polynomial addition

impl<T: CRing> Add for Polynomial<T> {
    type Output = Polynomial<T>;

    fn add(self, other: Polynomial<T>) -> Polynomial<T> {
        let mut result = Vec::new();
        let mut self_iter = self.coefficients.into_iter();
        let mut other_iter = other.coefficients.into_iter();
        loop {
            match (self_iter.next(), other_iter.next()) {
                (Some(self_coefficient), Some(other_coefficient)) => {
                    result.push(T::add(self_coefficient, other_coefficient));
                }
                (Some(self_coefficient), None) => {
                    result.push(self_coefficient);
                }
                (None, Some(other_coefficient)) => {
                    result.push(other_coefficient);
                }
                (None, None) => {
                    break;
                }
            }
        }
        Polynomial::new(result)
    }
}

// Polynomial multiplication

impl<T: CRing> Polynomial<T> {
    fn multiply(&self, other: &Polynomial<T>) -> Polynomial<T> {
        let mut result = Vec::new();
        for i in 0..self.degree() + other.degree() + 1 {
            result.push(T::zero());
        }
        for (i, self_coefficient) in self.coefficients.iter().enumerate() {
            for (j, other_coefficient) in other.coefficients.iter().enumerate() {
                result[i + j] = T::add(
                    result[i + j],
                    T::multiply(*self_coefficient, *other_coefficient),
                );
            }
        }
        Polynomial::new(result)
    }
}

// negation

impl<T: CRing> Neg for Polynomial<T> {
    type Output = Polynomial<T>;

    fn neg(self) -> Polynomial<T> {
        Polynomial::new(self.coefficients.into_iter().map(|c| -c).collect())
    }
}

/// Lazy formal power series over a commutative ring `R[[x]]`
struct FormalSeries<T: CRing> {
    computed_prefix: Polynomial<T>,
}
