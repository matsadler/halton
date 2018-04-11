//! A module for generating Halton sequences, a deterministic low discrepancy
//! sequence that appears to be random. The uniform distribution and
//! repeatability makes the sequence ideal for choosing sample points or
//! placing objects in 2D or 3D space.
//!
//! # Examples
//!
//! ```
//! use halton::Sequence;
//!
//! let mut grid = [["."; 10]; 10];
//! let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".split("").skip(1).take(26);
//! let seq = Sequence::new(2).zip(Sequence::new(3)).zip(alpha);
//! for ((x, y), c) in seq {
//!     grid[(y * 10.0) as usize][(x * 10.0) as usize] = c;
//! }
//! for row in grid.iter() {
//!     println!("{}", row.join(" "));
//! }
//! ```
//!
//! Outputs:
//!
//! ``` text
//! . . R . . I . . . .
//! . L . . . . U C . .
//! X . . F . . . . . O
//! . . . J . A . . . .
//! . D . . . . M S . .
//! P . . . V . . . G .
//! . . B . . Y . . . .
//! . T . . . . E . K .
//! H . . . N . . . . W
//! . . . Z . Q . . . .
//! ```

#[cfg(test)]
#[macro_use]
extern crate approx;

const D: usize = 20;

/// An iterator implementing the fast generation of Halton sequences.
/// The method of generation is adapted from _Fast, portable, and reliable
/// algorithm for the calculation of Halton numbers_ by Miroslav Kolar and
/// Seamus O'Shea.
///
/// # Examples
///
/// With a `for` loop:
///
/// ``` no_run
/// use halton::Sequence;
///
/// let seq = Sequence::new(2);
///
/// for x in seq {
///     println!("{}", x);
/// }
/// ```
///
/// Collecting into a `Vec`:
///
/// ```
/// use halton::Sequence;
///
/// let seq = Sequence::new(2);
///
/// assert_eq!(vec![0.5, 0.25, 0.75], seq.take(3).collect::<Vec<f64>>());
/// ```
///
/// Skipping entries on initialisation:
///
/// ```
/// use halton::Sequence;
///
/// // use base 17, skip the first 20 entries
/// let mut seq = Sequence::skip(17, 20);
///
/// assert_eq!(Some(0.23875432525951557), seq.next());
/// ```
pub struct Sequence {
    b: u8,
    d: [u8; D + 1],
    r: [f64; D + 1],
}

impl Sequence {
    /// Constructs a new `Sequence` for `base`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use halton::Sequence;
    /// let mut seq = Sequence::new(2);
    ///
    /// assert_eq!(Some(0.5), seq.next());
    /// ```
    pub fn new(base: u8) -> Self {
        Sequence {
            b: base,
            d: [0; D + 1],
            r: [0.0; D + 1],
        }
    }

    /// Constructs a new `Sequence` for `base`, skipping `n` elements.
    ///
    /// The method used to skip elements when constructing the `Sequence` is
    /// considerably faster than advancing the iterator to that point.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use halton::Sequence;
    /// let mut seq = Sequence::skip(2, 8);
    ///
    /// assert_eq!(Some(0.5625), seq.next());
    /// ```
    pub fn skip(base: u8, n: usize) -> Self {
        let b = base;
        let mut n0 = n;
        let mut d = [0; D + 1];
        let mut r = [0.0; D + 1];

        let mut last = 0;
        while n0 >= b as usize {
            d[last] = n0 as u8 % b;
            last += 1;
            n0 /= b as usize;
        }
        d[last] = n0 as u8;
        for i in (1..(D + 1)).rev() {
            r[i - 1] = (d[i] as f64 + r[i]) / b as f64;
        }
        Sequence { b, d, r }
    }
}

impl Iterator for Sequence {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.d[D] != 0 {
            return None;
        }

        let mut l = 0;

        self.d[0] += 1;
        if self.d[0] == self.b {
            loop {
                self.d[l] = 0;
                l += 1;
                self.d[l] += 1;
                if self.d[l] != self.b {
                    break;
                };
            }
            self.r[l - 1] = (self.d[l] as f64 + self.r[l]) / self.b as f64;
            for i in (1..l).rev() {
                self.r[i - 1] = self.r[i] / self.b as f64;
            }
            Some(self.r[0] / self.b as f64)
        } else {
            Some((self.d[0] as f64 + self.r[0]) / self.b as f64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Sequence;

    #[test]
    fn base_2() {
        let mut seq = Sequence::new(2);
        assert_relative_eq!(0.5, seq.next().unwrap());
        assert_relative_eq!(0.25, seq.next().unwrap());
        assert_relative_eq!(0.75, seq.next().unwrap());
        assert_relative_eq!(0.125, seq.next().unwrap());
        assert_relative_eq!(0.625, seq.next().unwrap());
        assert_relative_eq!(0.375, seq.next().unwrap());
        assert_relative_eq!(0.875, seq.next().unwrap());
        assert_relative_eq!(0.0625, seq.next().unwrap());
        assert_relative_eq!(0.5625, seq.next().unwrap());
    }

    #[test]
    fn base_3() {
        let mut seq = Sequence::new(3);
        assert_relative_eq!(0.3333333333333333, seq.next().unwrap());
        assert_relative_eq!(0.6666666666666666, seq.next().unwrap());
        assert_relative_eq!(0.1111111111111111, seq.next().unwrap());
        assert_relative_eq!(0.4444444444444444, seq.next().unwrap());
        assert_relative_eq!(0.7777777777777777, seq.next().unwrap());
        assert_relative_eq!(0.2222222222222222, seq.next().unwrap());
        assert_relative_eq!(0.5555555555555555, seq.next().unwrap());
        assert_relative_eq!(0.8888888888888888, seq.next().unwrap());
        assert_relative_eq!(0.0370370370370370, seq.next().unwrap());
    }

    #[test]
    fn skip_base_2() {
        let mut seq = Sequence::skip(2, 8);
        assert_relative_eq!(0.5625, seq.next().unwrap());
    }

    #[test]
    fn skip_base_3() {
        let mut seq = Sequence::skip(3, 8);
        assert_relative_eq!(0.0370370370370370, seq.next().unwrap());
    }

    #[test]
    fn last() {
        let mut seq = Sequence::new(2);
        assert_relative_eq!(4.76837158203125e-07, seq.nth(1048575).unwrap());
        assert_eq!(None, seq.next());
    }

    #[test]
    fn skip_last() {
        let mut seq = Sequence::skip(2, 1048575);
        assert_relative_eq!(4.76837158203125e-07, seq.next().unwrap());
        assert_eq!(None, seq.next());
    }

    #[test]
    fn iter() {
        let seq = Sequence::new(2);
        assert_eq!(vec![0.5, 0.25, 0.75], seq.take(3).collect::<Vec<f64>>());
    }
}
