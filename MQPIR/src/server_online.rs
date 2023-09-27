use rand_core::{RngCore, OsRng};
use crate::globals;
use packed_simd::u8x64;



fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = vec![0; globals::BLOCK_SIZE];
    for i in (0..globals::BLOCK_SIZE).step_by(u8x64::lanes()) {
        let a_simd: u8x64 = u8x64::from_slice_unaligned(&a[i..]);
        let b_simd: u8x64 = u8x64::from_slice_unaligned(&b[i..]);
        let result_simd: u8x64 = a_simd ^ b_simd;
        result_simd.write_to_slice_unaligned(&mut result[i..]);
    }
    result
}

pub fn handle_client() {
    let mut key: [u8; globals::KEY_SIZE] = [0; globals::KEY_SIZE];
    let mut rng = OsRng;
    rng.fill_bytes(&mut key);
    for j in 0..(2 * globals::SQRT_N) {
        let point = j.to_be_bytes();
        let _mac = crate::globals::key_mac(&key, &point);
    }


        let mut block1: [u8; globals::BLOCK_SIZE] = [0; globals::BLOCK_SIZE];
        let mut block2: [u8; globals::BLOCK_SIZE] = [0; globals::BLOCK_SIZE];

        let mut rng = OsRng;
        rng.fill_bytes(&mut block1);
        rng.fill_bytes(&mut block2);
        // create SIMD vectors from the blocks
        let mut results = [0; globals::SQRT_N * globals::BLOCK_SIZE];
        let mut index = 0;
        for _j in 0 .. globals::SQRT_N {
            let result = xor_bytes(&block1, &block2);
            for k in 0..globals::BLOCK_SIZE {
                results[index] = result[k];
                index += 1;
            }
        }      
}