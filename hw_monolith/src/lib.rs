mod monolith_perm_bindings;
use monolith_perm_bindings::*;
use p3_mersenne_31::Mersenne31;
use p3_symmetric::{CryptographicPermutation, Permutation};
use p3_field::PrimeField32;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct HWMonolith {
    pub hw: MappedMonolith,
    pub mtx: Arc<Mutex<MappedMonolith>>
}

impl HWMonolith {
    pub fn new() -> HWMonolith {
        let mapping = unsafe { map_monolith() };
        HWMonolith { 
            hw: mapping,
            mtx: Arc::new(Mutex::new(mapping))
        }
    }

    // TODO: Is this safe, and idiomatic???
    pub fn hash(&self, input: u32) -> u32 {
        let mtx_binding = Arc::clone(&self.mtx);
        let hw = mtx_binding.lock().unwrap();
        unsafe { monolith_hash(hw.to_owned(), input) }
    }

    pub fn compress(&self, input1: u32, input2: u32) -> u32 {
        let mtx_binding = Arc::clone(&self.mtx);
        let hw = mtx_binding.lock().unwrap();
        unsafe { monolith_compress(hw.to_owned(), input1, input2) }
    }

    pub unsafe fn unmap(&mut self) {
        unmap_monolith(&raw mut self.hw);
        println!("----------------");
        println!("UNMAPPED HW ACC!");
        println!("----------------");
    }
}

impl Permutation<[Mersenne31; 1]> for HWMonolith {
    fn permute_mut(&self, input: &mut [Mersenne31; 1]) {
        let in_u32 = input[0].as_canonical_u32();
        input[0] = Mersenne31::new_checked(self.hash(in_u32)).unwrap();
    }
}
impl CryptographicPermutation<[Mersenne31; 1]> for HWMonolith {
}

impl Permutation<[Mersenne31; 2]> for HWMonolith {
    fn permute_mut(&self, input: &mut [Mersenne31; 2]) {
        let in1_u32 = input[0].as_canonical_u32();
        let in2_u32 = input[1].as_canonical_u32();
        let out = Mersenne31::new_checked(self.compress(in1_u32, in2_u32)).unwrap();
        input[0] = out; input[1] = out;
    }
}
impl CryptographicPermutation<[Mersenne31; 2]> for HWMonolith {
}

unsafe impl Sync for HWMonolith {}