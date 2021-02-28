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

#![no_std]

#[cfg(test)]
extern crate std;

use core::{
    cmp::PartialEq,
    convert::TryFrom,
    ops::{Add, AddAssign, Div},
};

/// Returns the number at `index` of the Halton sequence for `base`. The number
/// returned will be > 0 and < 1, assuming `index` > 1.
///
/// While [`Sequence`] will be faster for most cases, this function may be
/// useful for calulating a single number from a Halton sequence, or creating
/// a 'leaped' sequence.
///
/// # Index
///
/// Beware that indexing [`Sequence`] is effectively 0-based, whereas the
/// `index` argument for [`number`] is 1-based.
///
/// ```
/// use halton::{number, Sequence};
///
/// assert_eq!(Sequence::new(2).nth(0).unwrap(), number(2, 1));
/// ```
///
/// # Examples
///
/// 'leaped' Halton sequence:
///
/// ```
/// let step = 409;
/// let mut i = 1;
/// while i < 10 * step {
///     println!("{}", halton::number(17, i));
///     i += step;
/// }
/// ```
#[inline]
pub fn number(base: u16, mut index: usize) -> f64 {
    let mut factor = 1.0;
    let mut result = 0.0;
    while index > 0 {
        factor /= f64::from(base);
        result += factor * (index % usize::from(base)) as f64;
        index /= usize::from(base);
    }
    result
}

/// An iterator implementing the fast generation of Halton sequences.
/// The method of generation is adapted from _Fast, portable, and reliable
/// algorithm for the calculation of Halton numbers_ by Miroslav Kolář and
/// Seamus F. O'Shea.
///
/// The numbers returned from the iterator will be in the range > 0 and < 1.
///
/// # Examples
///
/// These examples use [`Sequence`], this is an alias for [`GenericSequence`]
/// with default type parameters.
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
/// let mut seq = Sequence::new(17).skip(20);
///
/// assert_eq!(Some(0.23875432525951557), seq.next());
/// ```
#[derive(Clone)]
pub struct GenericSequence<I, F, const D: usize> {
    b: I,
    d: [I; D],
    r: [F; D],
}

impl<I, F, const D: usize> GenericSequence<I, F, D>
where
    I: AddAssign + Copy + From<u8> + PartialEq + TryFrom<usize>,
    f64: From<I>,
    usize: From<I>,
    F: Add<Output = F> + Copy + Div<Output = F> + From<I> + From<f32>,
{
    /// Constructs a new [`GenericSequence`] for `base`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use halton::GenericSequence;
    /// let mut seq = GenericSequence::<u16, f64, 20>::new(2);
    ///
    /// assert_eq!(Some(0.5), seq.next());
    /// ```
    #[inline]
    pub fn new(base: I) -> Self {
        GenericSequence {
            b: base,
            d: [I::from(0u8); D],
            r: [F::from(0.0f32); D],
        }
    }

    fn pos(&self) -> Option<usize> {
        self.d
            .iter()
            .zip(1..)
            .map(|(v, i)| usize::from(*v).checked_mul(i))
            .try_fold(0usize, |acc, v| acc.checked_add(v?))
    }

    fn max(&self) -> Option<usize> {
        u32::try_from(self.d.len())
            .ok()
            .and_then(|len| usize::from(self.b).checked_pow(len).map(|v| v - 1))
    }

    fn remaining(&self) -> Option<usize> {
        Some(self.max()? - self.pos()?)
    }
}

impl<I, F, const D: usize> Iterator for GenericSequence<I, F, D>
where
    I: AddAssign + Copy + From<u8> + PartialEq + TryFrom<usize>,
    f64: From<I>,
    usize: From<I>,
    F: Add<Output = F> + Copy + Div<Output = F> + From<I> + From<f32>,
{
    type Item = F;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut l = 0;

        self.d[l] += I::from(1u8);
        if self.d[l] == self.b {
            while self.d[l] == self.b {
                self.d[l] = I::from(0u8);
                l += 1;
                if l == self.d.len() {
                    return None;
                }
                self.d[l] += I::from(1u8);
            }
            self.r[l - 1] = (F::from(self.d[l]) + self.r[l]) / F::from(self.b);
            for i in (1..l).rev() {
                self.r[i - 1] = self.r[i] / F::from(self.b);
            }
            Some(self.r[0] / F::from(self.b))
        } else {
            Some((F::from(self.d[0]) + self.r[0]) / F::from(self.b))
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if let Some(remaining) = self.remaining() {
            (remaining, Some(remaining))
        } else {
            (0, None)
        }
    }

    #[inline]
    fn count(self) -> usize {
        if let Some(remaining) = self.remaining() {
            remaining
        } else {
            panic!("attempt to add with overflow")
        }
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        if let Some(remaining) = self.remaining() {
            self.nth(remaining - 1)
        } else {
            self.fold(None, |_, v| Some(v))
        }
    }

    #[inline]
    fn nth(&mut self, mut n: usize) -> Option<Self::Item> {
        if n > 50 {
            if let Some(mut n) = self.pos().and_then(|p| n.checked_add(p)) {
                self.d.iter_mut().for_each(|v| *v = I::from(0u8));
                self.r.iter_mut().for_each(|v| *v = F::from(0.0f32));
                let mut last = 0;
                while n >= usize::from(self.b) {
                    self.d[last] = I::try_from(n % usize::from(self.b)).ok().unwrap();
                    last += 1;
                    n /= usize::from(self.b);
                }
                self.d[last] = I::try_from(n).ok().unwrap();
                for i in (1..self.r.len()).rev() {
                    self.r[i - 1] = (F::from(self.d[i]) + self.r[i]) / F::from(self.b);
                }
                return self.next();
            }
        }
        for x in self {
            if n == 0 {
                return Some(x);
            }
            n -= 1;
        }
        None
    }
}

/// A type alias for [`GenericSequence`] with defaults.
pub type Sequence = GenericSequence<u16, f64, 20>;

#[cfg(test)]
mod tests {
    use super::{number, Sequence};
    use approx::assert_relative_eq;
    use std::vec;

    #[test]
    fn number_base_2() {
        assert_relative_eq!(0.0, number(2, 0));
        assert_relative_eq!(0.5, number(2, 1));
        assert_relative_eq!(0.25, number(2, 2));
        assert_relative_eq!(0.75, number(2, 3));
        assert_relative_eq!(0.125, number(2, 4));
        assert_relative_eq!(0.625, number(2, 5));
        assert_relative_eq!(0.375, number(2, 6));
        assert_relative_eq!(0.875, number(2, 7));
        assert_relative_eq!(0.0625, number(2, 8));
        assert_relative_eq!(0.5625, number(2, 9));
    }

    #[test]
    fn number_base_3() {
        assert_relative_eq!(0.0, number(3, 0));
        assert_relative_eq!(0.3333333333333333, number(3, 1));
        assert_relative_eq!(0.6666666666666666, number(3, 2));
        assert_relative_eq!(0.1111111111111111, number(3, 3));
        assert_relative_eq!(0.4444444444444444, number(3, 4));
        assert_relative_eq!(0.7777777777777777, number(3, 5));
        assert_relative_eq!(0.2222222222222222, number(3, 6));
        assert_relative_eq!(0.5555555555555555, number(3, 7));
        assert_relative_eq!(0.8888888888888888, number(3, 8));
        assert_relative_eq!(0.0370370370370370, number(3, 9));
    }

    #[test]
    fn sequence_base_2() {
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
    fn sequence_base_3() {
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
    fn sequence_skip_base_2() {
        let mut seq = Sequence::new(2).skip(8);
        assert_relative_eq!(0.5625, seq.next().unwrap());
    }

    #[test]
    fn sequence_skip_base_3() {
        let mut seq = Sequence::new(3).skip(8);
        assert_relative_eq!(0.0370370370370370, seq.next().unwrap());
    }

    #[test]
    fn sequence_iteratate_to_last() {
        let seq = Sequence::new(2);
        assert_relative_eq!(0.9999990463256836, seq.fold(None, |_, x| Some(x)).unwrap());
    }

    #[test]
    fn sequence_last() {
        let seq = Sequence::new(2);
        assert_relative_eq!(0.9999990463256836, seq.last().unwrap());
    }

    #[test]
    fn sequence_nth_last() {
        let mut seq = Sequence::new(2);
        assert_relative_eq!(0.9999990463256836, seq.nth(1048574).unwrap());
        assert_eq!(None, seq.next());
    }

    #[test]
    fn sequence_skip_last() {
        let mut seq = Sequence::new(2).skip(1048574);
        assert_relative_eq!(0.9999990463256836, seq.next().unwrap());
        assert_eq!(None, seq.next());
    }

    #[test]
    fn sequence_iter() {
        use std::vec::Vec;

        let seq = Sequence::new(2);
        assert_eq!(vec![0.5, 0.25, 0.75], seq.take(3).collect::<Vec<f64>>());
    }

    #[test]
    fn sequence_count() {
        let seq = Sequence::new(2);
        assert_eq!(1048575, seq.count());
    }

    #[test]
    fn sequence_size_hint() {
        let seq = Sequence::new(2);
        assert_eq!((1048575, Some(1048575)), seq.size_hint());
    }
}
