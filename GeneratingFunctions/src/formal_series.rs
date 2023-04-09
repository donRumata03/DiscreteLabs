use crate::*;
use std::cell::RefCell;
use std::ops::Index;
use std::rc::Rc;

/// Lazy formal power series over a commutative ring `R[[x]]`
pub trait FormalSeries<R: ERing>
where
    R::E: Copy + Eq,
{
    fn at(&mut self, n: usize, ring: &R) -> R::E;
}

trait FormalSeriesForCaching<R: ERing>
where
    R::E: Copy + Eq,
{
    fn get_computed_prefix(&mut self) -> &mut Polynomial<R>;
    fn compute_next_at(&mut self, n: usize, ring: &R) -> R::E;
    fn compute_up_to(&mut self, n: usize, ring: &R) {
        for i in self.get_computed_prefix().coefficients.len()..=n {
            let next = self.compute_next_at(i, ring);
            self.computed(i, next, ring);
        }
    }
}

impl<R: ERing, C: FormalSeriesForCaching<R>> FormalSeries<R> for C
where
    R::E: Copy + Eq,
{
    fn at(&mut self, n: usize, ring: &R) -> R::E {
        if self.get_computed_prefix().coefficients.len() <= n {
            self.compute_up_to(n, &ring);
        }
        self.get_computed_prefix().coefficients[n]
    }
}

trait Wtf<R: ERing>
where
    R::E: Copy + Eq,
{
    fn computed(&mut self, n: usize, value: R::E, ring: &R);
}
impl<R: ERing, C: FormalSeriesForCaching<R> + ?Sized> Wtf<R> for C
where
    R::E: Copy + Eq,
{
    fn computed(&mut self, n: usize, value: R::E, ring: &R) {
        let mut coefficients = &mut self.get_computed_prefix().coefficients;
        while coefficients.len() <= n {
            coefficients.push(ring.zero());
        }
        self.get_computed_prefix().coefficients[n] = value;
    }
}

/// Series addition

pub struct FormalSeriesAdd<R: ERing>
where
    R::E: Copy + Eq,
{
    a: Rc<RefCell<dyn FormalSeries<R>>>,
    b: Rc<RefCell<dyn FormalSeries<R>>>,
    computed_prefix: Polynomial<R>,
}

impl<R: ERing> FormalSeriesAdd<R>
where
    R::E: Copy + Eq,
{
    pub fn new(
        a: Rc<RefCell<dyn FormalSeries<R>>>,
        b: Rc<RefCell<dyn FormalSeries<R>>>,
        ring: &R,
    ) -> Self {
        FormalSeriesAdd {
            a,
            b,
            computed_prefix: Polynomial::new(vec![]),
        }
    }
}

impl<R: ERing> FormalSeriesForCaching<R> for FormalSeriesAdd<R>
where
    R::E: Copy + Eq,
{
    fn get_computed_prefix(&mut self) -> &mut Polynomial<R> {
        &mut self.computed_prefix
    }

    fn compute_next_at(&mut self, n: usize, ring: &R) -> R::E {
        ring.add(
            self.a.borrow_mut().at(n, ring),
            self.b.borrow_mut().at(n, ring),
        )
    }
}

/// Series negation

pub struct FormalSeriesNegation<R: ERing>
where
    R::E: Copy + Eq,
{
    a: Rc<RefCell<dyn FormalSeries<R>>>,
    computed_prefix: Polynomial<R>,
}

impl<R: ERing> FormalSeriesNegation<R>
where
    R::E: Copy + Eq,
{
    pub fn new(a: Rc<RefCell<dyn FormalSeries<R>>>, ring: &R) -> Self {
        FormalSeriesNegation {
            a,
            computed_prefix: Polynomial::new(vec![]),
        }
    }
}

impl<R: ERing> FormalSeriesForCaching<R> for FormalSeriesNegation<R>
where
    R::E: Copy + Eq,
{
    fn get_computed_prefix(&mut self) -> &mut Polynomial<R> {
        &mut self.computed_prefix
    }

    fn compute_next_at(&mut self, n: usize, ring: &R) -> R::E {
        ring.negate(self.a.borrow_mut().at(n, ring))
    }
}

/// Series multiplication

pub struct FormalSeriesMul<R: ERing>
where
    R::E: Copy + Eq,
{
    a: Rc<RefCell<dyn FormalSeries<R>>>,
    b: Rc<RefCell<dyn FormalSeries<R>>>,
    computed_prefix: Polynomial<R>,
}

impl<R: ERing> FormalSeriesMul<R>
where
    R::E: Copy + Eq,
{
    pub fn new(
        a: Rc<RefCell<dyn FormalSeries<R>>>,
        b: Rc<RefCell<dyn FormalSeries<R>>>,
        ring: &R,
    ) -> Self {
        FormalSeriesMul {
            a,
            b,
            computed_prefix: Polynomial::new(vec![]),
        }
    }
}

impl<R: ERing> FormalSeriesForCaching<R> for FormalSeriesMul<R>
where
    R::E: Copy + Eq,
{
    fn get_computed_prefix(&mut self) -> &mut Polynomial<R> {
        &mut self.computed_prefix
    }

    fn compute_next_at(&mut self, n: usize, ring: &R) -> R::E {
        let mut sum = ring.zero();
        for i in 0..=n {
            sum = ring.add(
                sum,
                ring.multiply(
                    self.a.borrow_mut().at(i, ring),
                    self.b.borrow_mut().at(n - i, ring),
                ),
            );
        }
        sum
    }
}

/// Series division

pub struct FormalSeriesDiv<F: DField>
where
    F::E: Copy + Eq,
{
    a: Rc<RefCell<dyn FormalSeries<F>>>,
    b: Rc<RefCell<dyn FormalSeries<F>>>,
    computed_prefix: Polynomial<F>,
}

impl<F: DField> FormalSeriesDiv<F>
where
    F::E: Copy + Eq,
{
    pub fn new(
        a: Rc<RefCell<dyn FormalSeries<F>>>,
        b: Rc<RefCell<dyn FormalSeries<F>>>,
        field: &F,
    ) -> Self {
        FormalSeriesDiv {
            a,
            b,
            computed_prefix: Polynomial::new(vec![]),
        }
    }
}

impl<F: EField> FormalSeriesForCaching<F> for FormalSeriesDiv<F> {
    fn get_computed_prefix(&mut self) -> &mut Polynomial<F> {
        &mut self.computed_prefix
    }

    fn compute_next_at(&mut self, n: usize, field: &F) -> F::E {
        let mut sum = field.zero();
        for i in 0..n {
            sum = field.add(
                sum,
                field.multiply(self.at(i, field), self.b.borrow_mut().at(n - i, field)),
            );
        }
        field.divide(
            field.subtract(self.a.borrow_mut().at(n, &field), sum),
            self.b.borrow_mut().at(0, &field),
        )
    }
}

/// Constant series

pub struct FormalSeriesAlways<R: ERing> {
    value: R::E,
    computed_prefix: Polynomial<R>,
}

impl<R: ERing> FormalSeriesAlways<R> {
    pub fn new(value: R::E) -> Self {
        Self {
            value,
            computed_prefix: Polynomial::new(vec![]),
        }
    }
}

impl<R: ERing> FormalSeriesForCaching<R> for FormalSeriesAlways<R> {
    fn get_computed_prefix(&mut self) -> &mut Polynomial<R> {
        &mut self.computed_prefix
    }

    fn compute_next_at(&mut self, n: usize, ring: &R) -> R::E {
        self.value
    }
}

/// Polynomial series

pub struct FormalSeriesPolynomial<R: ERing> {
    poly: Polynomial<R>,
    computed_prefix: Polynomial<R>,
}

impl<R: ERing> FormalSeriesPolynomial<R> {
    pub fn new(poly: Polynomial<R>) -> Self {
        Self {
            poly,
            computed_prefix: Polynomial::new(vec![]),
        }
    }
}

impl<R: ERing> FormalSeriesForCaching<R> for FormalSeriesPolynomial<R> {
    fn get_computed_prefix(&mut self) -> &mut Polynomial<R> {
        &mut self.computed_prefix
    }

    fn compute_next_at(&mut self, n: usize, ring: &R) -> R::E {
        if n > self.poly.degree() {
            ring.zero()
        } else {
            self.poly.at(n, ring)
        }
    }
}

/// Ring of formal power series

struct FormalSeriesRing<R: ERing> {
    ring: R,
}

impl<R: ERing> FormalSeriesRing<R> {
    pub fn new(ring: R) -> Self {
        FormalSeriesRing { ring }
    }
}

impl<R: ERing + 'static> CRing for FormalSeriesRing<R> {
    type E = Rc<RefCell<dyn FormalSeries<R>>>;

    fn zero(&self) -> Self::E {
        Rc::new(RefCell::new(FormalSeriesAlways::new(self.ring.zero())))
    }

    fn one(&self) -> Self::E {
        Rc::new(RefCell::new(FormalSeriesAlways::new(self.ring.one())))
    }

    fn add(&self, a: Self::E, b: Self::E) -> Self::E {
        Rc::new(RefCell::new(FormalSeriesAdd::new(a, b, &self.ring)))
    }

    fn multiply(&self, a: Self::E, b: Self::E) -> Self::E {
        Rc::new(RefCell::new(FormalSeriesMul::new(a, b, &self.ring)))
    }

    fn negate(&self, a: Self::E) -> Self::E {
        Rc::new(RefCell::new(FormalSeriesNegation::new(a, &self.ring)))
    }
}

/// Field of formal power series
impl<F: EField + 'static> Field for FormalSeriesRing<F> {
    fn inverse(&self, a: Self::E) -> Self::E {
        Rc::new(RefCell::new(FormalSeriesDiv::new(
            self.one(),
            a,
            &self.ring,
        )))
    }
}

// Series composition

// Series differentiation

// Series integration

// Series exponentiation

// Series power

// TODO: egf
