use crate::*;
use std::ops::Index;

/// Lazy formal power series over a commutative ring `R[[x]]`
pub trait FormalSeries<R: CRing> {
    fn at(&mut self, n: usize, ring: &R) -> R::E;
}

pub trait FormalSeriesForCaching<R: CRing> {
    fn get_computed_prefix(&mut self) -> &mut Polynomial<R>;
    fn compute_next_at(&mut self, n: usize, ring: &R) -> R::E;
    fn compute_up_to(&mut self, n: usize, ring: &R) {
        for i in self.get_computed_prefix().coefficients.len()..=n {
            self.computed(i, self.compute_next_at(i, ring), ring);
        }
    }
}

impl<R: CRing, C: FormalSeriesForCaching<R>> FormalSeries<R> for C {
    fn at(&mut self, n: usize, ring: &R) -> R::E {
        if self.get_computed_prefix().coefficients.len() <= n {
            self.compute_up_to(n, &ring);
        }
        self.get_computed_prefix().coefficients[n]
    }
}

trait Wtf<R: CRing> {
    fn computed(&mut self, n: usize, value: R::E, ring: &R);
}
impl<R: CRing, C: FormalSeriesForCaching<R>> Wtf<R> for C {
    fn computed(&mut self, n: usize, value: R::E, ring: &R) {
        let mut coefficients = &mut self.get_computed_prefix().coefficients;
        while coefficients.len() <= n {
            coefficients.push(ring.zero());
        }
        self.get_computed_prefix().coefficients[n] = value;
    }
}

/// Series addition

pub struct FormalSeriesAdd<R: CRing> {
    a: Box<dyn FormalSeries<R>>,
    b: Box<dyn FormalSeries<R>>,
    computed_prefix: Polynomial<R>,
}

impl<R: CRing> FormalSeriesAdd<R> {
    pub fn new(a: Box<dyn FormalSeries<R>>, b: Box<dyn FormalSeries<R>>, ring: &R) -> Self {
        FormalSeriesAdd {
            a,
            b,
            computed_prefix: Polynomial::new(vec![]),
        }
    }
}

impl<R: CRing> FormalSeriesForCaching<R> for FormalSeriesAdd<R> {
    fn get_computed_prefix(&mut self) -> &mut Polynomial<R> {
        &mut self.computed_prefix
    }

    fn compute_next_at(&mut self, n: usize, ring: &R) -> R::E {
        ring.add(self.a.at(n, ring), self.b.at(n, ring))
    }
}

/// Series negation

pub struct FormalSeriesNegation<R: CRing> {
    a: Box<dyn FormalSeries<R>>,
    computed_prefix: Polynomial<R>,
}

impl<R: CRing> FormalSeriesNegation<R> {
    pub fn new(a: Box<dyn FormalSeries<R>>, ring: &R) -> Self {
        FormalSeriesNegation {
            a,
            computed_prefix: Polynomial::new(vec![]),
        }
    }
}

impl<R: CRing> FormalSeriesForCaching<R> for FormalSeriesNegation<R> {
    fn get_computed_prefix(&mut self) -> &mut Polynomial<R> {
        &mut self.computed_prefix
    }

    fn compute_next_at(&mut self, n: usize, ring: &R) -> R::E {
        ring.negate(self.a.at(n, ring))
    }
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

    fn compute_next_at(&mut self, n: usize, ring: &R) -> R::E {
        let mut sum = ring.zero();
        for i in 0..=n {
            sum = ring.add(
                sum,
                ring.multiply(self.a.at(i, &ring), self.b.at(n - i, &ring)),
            );
        }
    }
}

// Series division

#[derive(Debug, Clone)]
pub struct FormalSeriesDiv<R: CRing> {
    a: Box<dyn FormalSeries<R>>,
    b: Box<dyn FormalSeries<R>>,
    computed_prefix: Polynomial<R>,
}

impl<R: CRing> FormalSeriesDiv<R> {
    pub fn new(a: Box<dyn FormalSeries<R>>, b: Box<dyn FormalSeries<R>>) -> Self {
        FormalSeriesDiv {
            a,
            b,
            computed_prefix: Polynomial::new(vec![]),
        }
    }
}

impl<F: Field> FormalSeriesForCaching<F> for FormalSeriesDiv<F> {
    fn get_computed_prefix(&mut self) -> &mut Polynomial<F> {
        &mut self.computed_prefix
    }

    fn compute_next_at(&mut self, n: usize, field: &F) -> F::E {
        let mut sum = field.zero();
        for i in 0..n {
            sum = field.add(sum, field.multiply(self.at(i), self.b.at(n - i, &field)));
        }
        field.divide(
            field.subtract(self.a.at(n, &field), sum),
            self.b.at(0, &field),
        )
    }
}

/// Constant series

pub struct FormalSeriesAlways<R: CRing> {
    value: R::E,
}

impl<R: CRing> FormalSeriesAlways<R> {
    pub fn new(value: R::E) -> Self {
        FormalSeriesAlways { value }
    }
}

impl<R: CRing> FormalSeries<R> for FormalSeriesAlways<R> {
    fn at(&self, _n: usize, _ring: &R) -> R::E {
        self.value
    }
}

/// Polynomial series

pub struct FormalSeriesPolynomial<R: CRing> {
    poly: Polynomial<R>,
}

impl<R: CRing> FormalSeriesPolynomial<R> {
    pub fn new(poly: Polynomial<R>) -> Self {
        FormalSeriesPolynomial { poly }
    }
}

impl<R: CRing> FormalSeries<R> for FormalSeriesPolynomial<R> {
    fn at(&mut self, n: usize, ring: &R) -> R::E {
        self.poly.at(n, ring)
    }
}

/// Ring of formal power series

struct FormalSeriesRing<R: CRing> {
    ring: R,
}

impl<R: CRing> FormalSeriesRing<R> {
    pub fn new(ring: R) -> Self {
        FormalSeriesRing { ring }
    }
}

impl<R: CRing> Ring for FormalSeriesRing<R> {
    type E = Box<dyn FormalSeries<R>>;

    fn zero(&self) -> Self::E {
        Box::new(FormalSeriesAlways::new(self.ring.zero()))
    }

    fn one(&self) -> Self::E {
        Box::new(FormalSeriesAlways::new(self.ring.one()))
    }

    fn add(&self, a: &Self::E, b: &Self::E) -> Self::E {
        Box::new(FormalSeriesAdd::new(a.clone(), b.clone(), &self.ring))
    }

    fn negate(&self, a: &Self::E) -> Self::E {
        Box::new(FormalSeriesNegation::new(a.clone(), &self.ring))
    }

    fn multiply(&self, a: &Self::E, b: &Self::E) -> Self::E {
        Box::new(FormalSeriesMul::new(a.clone(), b.clone(), &self.ring))
    }

    fn divide(&self, a: &Self::E, b: &Self::E) -> Self::E {
        Box::new(FormalSeriesDiv::new(a.clone(), b.clone()))
    }
}

// Series composition

// Series differentiation

// Series integration

// Series exponentiation

// Series power

// TODO: egf
