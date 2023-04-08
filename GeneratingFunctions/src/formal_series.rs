use crate::*;
use std::ops::Index;

/// Lazy formal power series over a commutative ring `R[[x]]`
pub trait FormalSeries<R: CRing> {
    fn at(&mut self, n: usize, ring: &R) -> R::E;
}

pub trait FormalSeriesForCaching<R: CRing> {
    fn get_computed_prefix(&mut self) -> &mut Polynomial<R>;
    fn compute_up_to(&mut self, n: usize, ring: &R);
}

impl<R: CRing, C: FormalSeriesForCaching<R>> FormalSeries<R> for C {
    fn at(&mut self, n: usize, ring: &R) -> R::E {
        if self.get_computed_prefix().coefficients.len() <= n {
            self.compute_up_to(n, &ring);
        }
        self.get_computed_prefix().coefficients[n]
    }
}

trait Nothing<R: CRing> {
    fn computed(&mut self, n: usize, value: R::E, ring: &R);
}
impl<R: CRing, C: FormalSeriesForCaching<R>> Nothing<R> for C {
    fn computed(&mut self, n: usize, value: R::E, ring: &R) {
        let mut coefficients = &mut self.get_computed_prefix().coefficients;
        while coefficients.len() <= n {
            coefficients.push(ring.zero());
        }
        self.get_computed_prefix().coefficients[n] = value;
    }
}

pub struct FormalSeriesCacher<R: CRing> {
    computed_prefix: Polynomial<R>,
}

// Series multiplication

pub struct FormalSeriesMul<R: CRing> {
    a: Box<dyn FormalSeries<R>>,
    b: Box<dyn FormalSeries<R>>,
    computed_prefix: Polynomial<R>,
}

impl<R: CRing> FormalSeriesMul<R> {
    pub fn new(a: Box<dyn FormalSeries<R>>, b: Box<dyn FormalSeries<R>>, ring: &R) -> Self {
        FormalSeriesMul {
            a,
            b,
            computed_prefix: Polynomial::new_truncated(vec![], ring),
        }
    }
}

impl<R: CRing> FormalSeriesForCaching<R> for FormalSeriesMul<R> {
    fn get_computed_prefix(&mut self) -> &mut Polynomial<R> {
        &mut self.computed_prefix
    }

    fn compute_up_to(&mut self, n: usize, ring: &R) {
        for i in self.get_computed_prefix().coefficients.len()..=n {
            let mut sum = ring.zero();
            for j in 0..=i {
                sum = ring.add(
                    sum,
                    ring.multiply(self.a.at(j, &ring), self.b.at(i - j, &ring)),
                );
            }
            self.computed(i, sum, &ring);
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
