use crate::*;

/// Lazy formal power series over a commutative ring `R[[x]]`
pub trait FormalSeries<T: CRing> {
    fn at(&self, n: usize) -> T::E;
}

pub struct FormalSeriesHelper<T: CRing> {
    computed_prefix: Polynomial<T>,
}

// Series multiplication

pub struct FormalSeriesMul<T: CRing> {
    a: Box<dyn FormalSeries<T>>,
    b: Box<dyn FormalSeries<T>>,
    computed_prefix: Polynomial<T>,
}

impl<T: CRing> FormalSeriesMul<T> {
    pub fn new(a: Box<dyn FormalSeries<T>>, b: Box<dyn FormalSeries<T>>) -> Self {
        FormalSeriesMul {
            a,
            b,
            computed_prefix: Polynomial::new(),
        }
    }
}

impl<T: CRing> FormalSeries<T> for FormalSeriesMul<T> {
    fn at(&self, n: usize) -> T::E {
        if n < self.computed_prefix.len() {
            self.computed_prefix.at(n)
        } else {
            let mut result = T::zero();
            for i in 0..n {
                result = T::add(result, T::multiply(self.a.at(i), self.b.at(n - i)));
            }
            self.computed_prefix.push(result);
            result
        }
    }
}

// Series division

// Series composition

// Series differentiation

// Series integration

// Series exponentiation

// Series power

// TODO: egf
