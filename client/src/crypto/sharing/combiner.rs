use super::*;

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
    fn combine(&self, shares: &Vec<Vec<Share>>) -> SdaClientResult<Vec<Share>> {
        let dimension: usize = shares.get(0).map_or(0, Vec::len);

        let mut result: Vec<Share> = vec![0; dimension];
        for share in shares {
            if share.len() != dimension { Err("Wrong dimension")? }
            for (ix, value) in share.iter().enumerate() {
                result[ix] += *value;
                result[ix] %= self.modulus;
            }
        }

        Ok(result)
    }
}