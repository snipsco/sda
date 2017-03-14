use super::*;
use ::std::iter::repeat;

pub struct Combiner {
    modulus: i64,
}

impl Combiner {
    pub fn new(modulus: i64) -> Combiner {
        Combiner {
            modulus: modulus
        }
    }
}

impl ShareCombiner for Combiner {
    fn combine(&self, shares: &Vec<Vec<Share>>) -> Vec<Share> {
        let dimension: usize = shares.get(0).map_or(0, Vec::len);

        let mut result: Vec<Share> = repeat(0).take(dimension).collect();
        for share in shares {
            assert!(share.len() == dimension);
            for (ix, value) in share.iter().enumerate() {
                result[ix] += *value;
                result[ix] %= self.modulus;
            }
        }

        result
    }
}