use bit_vec::BitVec;
use fasthash::murmur3;
use extprim::u128::u128;
use std::num::Wrapping;

pub struct BloomFilter {
    num_bits: usize,
    num_funcs: usize,
    bv: BitVec
}

impl BloomFilter {
    pub fn new(num_bits: usize, num_funcs: usize) -> BloomFilter {
        BloomFilter {
            num_bits,
            num_funcs,
            bv: BitVec::from_elem(num_bits, false)
        }
    }

    fn hash(&self, h: u128, i: usize) -> usize {
        return (Wrapping(h.high64()) + Wrapping(i as u64) * Wrapping(h.low64())).0 as usize % self.num_bits;
    }

    pub fn add(&mut self, value: &[u8]) {
        let h = u128::from_built_in(murmur3::hash128(value));
        for i in 0..self.num_funcs {
            let index = self.hash(h, i);
            self.bv.set(index, true);
        }
    }

    pub fn test(&self, value: &[u8]) -> bool {
        let h = u128::from_built_in(murmur3::hash128(value));
        let mut exists = true;
        for i in 0..self.num_funcs {
            let index = self.hash(h, i);
            exists &= self.bv[index];
        }
        return exists;
    }
}
