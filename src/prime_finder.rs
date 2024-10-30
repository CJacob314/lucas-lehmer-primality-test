use std::iter::Iterator;
use std::sync::LazyLock;

use num_bigint::BigUint;
use num_traits::{cast::FromPrimitive, One};
use crate::PrimeFinderError;

pub struct MersenePrimeFinder {
    max_exp: Option<u32>,
    exp_cursor: u32
}

enum NumType {
    Prime(BigUint),
    Composite
}

impl MersenePrimeFinder {
    pub fn new(starting_exponent: u32) -> Self {
        Self {
            max_exp: None,
            exp_cursor: starting_exponent
        }
    }

    pub fn set_max_exponent(&mut self, max_exp: u32) {
        self.max_exp = Some(max_exp);
    }

    fn lucas_lehmer(&self) -> NumType {
        use NumType::*;

        let mut s = BigUint::from(4u8);
        let m = TWO.pow(self.exp_cursor) - 1u8;
        for _ in 0..self.exp_cursor - 2 {
            s = ((&s * &s) - 2u8) % &m;
        }

        if s == *ZERO {
            Prime(m)
        } else {
            Composite
        }
    }
}

impl Iterator for MersenePrimeFinder {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        use NumType::*;

        loop {
            let prime = match self.lucas_lehmer() {
                Composite => {
                    self.exp_cursor += 1;
                    if let Some(max_exp) = self.max_exp {
                        if self.exp_cursor > max_exp {
                            return None;
                        }
                    }
                    continue;
                }
                Prime(p) => p
            };
            self.exp_cursor += 1;
            return Some(prime);
        }
    }
}

const ZERO: LazyLock<BigUint> = LazyLock::new(|| {
    BigUint::from(0u8)
});
const TWO: LazyLock<BigUint> = LazyLock::new(|| {
    BigUint::from(2u8)
});
