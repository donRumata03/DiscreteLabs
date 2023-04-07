use std::cmp::Eq;

/// Commutative ring
pub trait CRing {
    type E: Copy + Eq; // Element type

    fn add(a: Self::E, b: Self::E) -> Self::E; // Addition is both associative and commutative
    fn negate(a: Self::E) -> Self::E; // negate is inverse element by addition

    fn zero() -> Self::E;

    fn multiply(a: Self::E, b: Self::E) -> Self::E; // Both associative and commutative
    fn one() -> Self::E;

    fn power(a: Self::E, n: usize) -> Self::E {
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

    fn factorial(n: usize) -> Self::E {
        let mut result = Self::one();
        let mut multiplier = Self::one();

        for _ in 0..n {
            multiplier = Self::add(multiplier, Self::one());
            result = Self::multiply(result, multiplier);
        }

        result
    }
}
