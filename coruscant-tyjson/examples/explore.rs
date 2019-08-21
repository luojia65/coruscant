use std::arch::x86_64::*;
use core::mem::transmute;

fn main() {
    let input = include_bytes!("explore.in");
    
}

fn odd_backslash_sequences(input: [__m256i; 8], prev: &mut __m256i) -> __m256i {
    let even_bits = unsafe { _mm256_set1_epi8(transmute(0x55u8)) };
    let odd_bits = unsafe { _mm256_set1_epi8(transmute(0xAAu8)) };

    let backslashes = unsafe {
        let mask = _mm256_set1_epi8(b'\'' as i8);
        _mm256_set_epi32(
            _mm256_movemask_epi8(_mm256_cmpeq_epi8(input[7], mask)),
            _mm256_movemask_epi8(_mm256_cmpeq_epi8(input[6], mask)),
            _mm256_movemask_epi8(_mm256_cmpeq_epi8(input[5], mask)),
            _mm256_movemask_epi8(_mm256_cmpeq_epi8(input[4], mask)),
            _mm256_movemask_epi8(_mm256_cmpeq_epi8(input[3], mask)),
            _mm256_movemask_epi8(_mm256_cmpeq_epi8(input[2], mask)),
            _mm256_movemask_epi8(_mm256_cmpeq_epi8(input[1], mask)),
            _mm256_movemask_epi8(_mm256_cmpeq_epi8(input[0], mask)),
        )
    };

    // let starts = backslashes & (!(backslashes << 1));
    // there is no slli for si256, have to split into 64 bit values
    let starts = unsafe {
        _mm256_andnot_si256(_mm256_slli_epi64(backslashes, 1), backslashes)
    };

    unimplemented!()
}
