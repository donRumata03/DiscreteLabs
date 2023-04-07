use crate::*;

/// Lazy formal power series over a commutative ring `R[[x]]`
pub struct FormalSeries<T: CRing> {
    computed_prefix: Polynomial<T>,
}
