#[cfg(test)]
#[macro_use]
extern crate approx;

const D: usize = 20;

pub struct Sequence {
    b: u8,
    d: [u8; D + 1],
    r: [f64; D + 1],
}

impl Sequence {
    pub fn new(base: u8) -> Self {
        Sequence {
            b: base,
            d: [0; D + 1],
            r: [0.0; D + 1],
        }
    }

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
