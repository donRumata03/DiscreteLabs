use crate::*;
use std::ops::{Add, Neg};

/// Formal polynomial over a commutative ring `R[x]`
pub struct Polynomial<R: CRing> {
    pub(crate) coefficients: Vec<R::E>,
}

impl<R: CRing> Polynomial<R> {
    pub fn new(coefficients: Vec<R::E>) -> Polynomial<R> {
        Polynomial { coefficients }
    }

    pub fn new_truncated(coefficients: Vec<R::E>, ring: &R) -> Polynomial<R> {
        let mut res = Polynomial { coefficients };
        res.truncate(ring);
        res
    }

    pub fn truncate(&mut self, ring: &R) {
        if let Some(i) = self.coefficients.iter().rposition(|&x| x != ring.zero()) {
            self.coefficients.truncate(i + 1);
        }
    }

    pub fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    pub fn at(&self, n: usize, ring: &R) -> R::E {
        self.coefficients[n]
    }
}

// Polynomial addition (take two polynomials and ring instance as arguments)
impl<R: CRing> Polynomial<R> {
    fn add(self, other: Polynomial<R>, ring: &R) -> Polynomial<R> {
        let mut res = Polynomial::new_truncated(
            vec![ring.zero(); self.degree().max(other.degree()) + 1],
            ring,
        );

        for i in 0..res.coefficients.len() {
            let subscript = |v: &Vec<R::E>| if i < v.len() { v[i] } else { ring.zero() };
            res.coefficients[i] = ring.add(
                subscript(&self.coefficients),
                subscript(&other.coefficients),
            );
        }

        res
    }
}

// Polynomial multiplication (take two polynomials and ring instance as arguments)
impl<R: CRing> Polynomial<R> {
    fn multiply(self, other: Polynomial<R>, ring: &R) -> Polynomial<R> {
        let mut res = Polynomial::new(vec![ring.zero(); self.degree() + other.degree() + 1]);

        for i in 0..self.coefficients.len() {
            for j in 0..other.coefficients.len() {
                res.coefficients[i + j] = ring.add(
                    res.coefficients[i + j],
                    ring.multiply(self.coefficients[i], other.coefficients[j]),
                );
            }
        }

        res
    }
}

// Polynomial negation (take a polynomial and ring instance as arguments)
impl<R: CRing> Polynomial<R> {
    fn negate(self, ring: &R) -> Polynomial<R> {
        let mut res = Polynomial::new(vec![ring.zero(); self.degree() + 1]);

        for i in 0..self.coefficients.len() {
            res.coefficients[i] = ring.negate(self.coefficients[i]);
        }

        res
    }
}
