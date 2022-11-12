/// Computes the Modular multiplicative inverse of two integers using Euclid's algorithm
/// (https://en.wikipedia.org/wiki/Euclidean_algorithm).
use num::integer::gcd;

/// Calculates the Greatest Common Denominator (GCD) of two integers *a* and *b*.
/// ```rust
/// (gcd(21, 33), 1);
/// ```
/// The result is always positive.
/// /// ```
/// use num::integer::gcd;
///
/// let a = 21;
/// let b = 33;
/// GCD(21,33) = 3.
/// ```
#[inline]
fn main() {
    let tup = (21, 33);
    println!("GCD of [21 and 33] is {:?}", gcd(tup.0, tup.1));

}

