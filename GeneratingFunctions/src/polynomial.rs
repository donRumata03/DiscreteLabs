use crate::*;
use std::ops::{Add, Neg};

/// Formal polynomial over a commutative ring `R[x]`
pub struct Polynomial<R: CRing> {
    coefficients: Vec<R::E>,
}

impl<R: CRing> Polynomial<R> {
    fn new(coefficients: Vec<R::E>) -> Polynomial<R> {
        let mut res = Polynomial { coefficients };

        if let Some(i) = res.coefficients.iter().rposition(|&x| x != R::zero()) {
            res.coefficients.truncate(i + 1);
        }
        res
    }

    fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }
}

// Polynomial addition

impl<R: CRing> Add for Polynomial<R> {
    type Output = Polynomial<R>;

    fn add(self, other: Polynomial<R>) -> Polynomial<R> {
        let mut result = Vec::new();
        let mut self_iter = self.coefficients.into_iter();
        let mut other_iter = other.coefficients.into_iter();
        loop {
            match (self_iter.next(), other_iter.next()) {
                (Some(self_coefficient), Some(other_coefficient)) => {
                    result.push(R::add(self_coefficient, other_coefficient));
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

impl<R: CRing> Polynomial<R> {
    fn multiply(&self, other: &Polynomial<R>) -> Polynomial<R> {
        let mut result = Vec::new();
        for i in 0..self.degree() + other.degree() + 1 {
            result.push(R::zero());
        }
        for (i, self_coefficient) in self.coefficients.iter().enumerate() {
            for (j, other_coefficient) in other.coefficients.iter().enumerate() {
                result[i + j] = R::add(
                    result[i + j],
                    R::multiply(*self_coefficient, *other_coefficient),
                );
            }
        }
        Polynomial::new(result)
    }
}

// negation

impl<R: CRing> Neg for Polynomial<R> {
    type Output = Polynomial<R>;

    fn neg(self) -> Polynomial<R> {
        Polynomial::new(
            self.coefficients
                .into_iter()
                .map(|x| R::negate(x))
                .collect(),
        )
    }
}
