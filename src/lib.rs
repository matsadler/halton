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
