use std::arch::x86_64::*;
use core::mem::transmute;

fn main() {
    let input = include_bytes!("explore.in");
    let mut input_vec: [__m256i; 2] = unsafe { core::mem::zeroed() };
    let mut prev_ov = 0;
    let mut whitespace = 0;
    let mut structurals = 0;
    let mut ptr = input.as_ptr();
    for _ in 0..64 {
        input_vec[0] = unsafe { _mm256_loadu_si256(ptr as *const _) };
        input_vec[1] = unsafe { _mm256_loadu_si256(ptr.add(32) as *const _) };
        unsafe { ptr = ptr.add(64) };
        let od = odd_backslash_sequences(input_vec, &mut prev_ov);
        find_whitespace_and_structurals(input_vec, &mut whitespace, &mut structurals);
        print!("{} ", prev_ov);
        print!("{:016X} ", od);
        println!("{:016X}", whitespace);
    }
}

#[inline(always)]
fn odd_backslash_sequences(input: [__m256i; 2], prev_ov: &mut u64) -> u64 {
    const EVEN_BITS: u64 = 0x5555_5555_5555_5555;
    const ODD_BITS: u64 = 0xAAAA_AAAA_AAAA_AAAA;
    let mask = unsafe { _mm256_set1_epi8(b'\\' as i8) };
    let backslashes = unsafe { 
        let hi32: u32 = transmute(_mm256_movemask_epi8(_mm256_cmpeq_epi8(input[1], mask)));
        let lo32: u32 = transmute(_mm256_movemask_epi8(_mm256_cmpeq_epi8(input[0], mask)));
        lo32 as u64 | ((hi32 as u64) << 32)
    };
    let starts = backslashes & (!(backslashes << 1));
    let even_start_mask = EVEN_BITS ^ *prev_ov;
    let even_starts = starts & even_start_mask;
    let odd_starts = starts & (!even_start_mask);
    let even_carries = u64::wrapping_add(backslashes, even_starts);
    let (odd_carries, ov) = u64::overflowing_add(backslashes, odd_starts);
    let odd_carries = odd_carries | *prev_ov;
    *prev_ov = if ov { 1 } else { 0 }; 
    let even_carry_ends = even_carries & (!backslashes);
    let odd_carry_ends = odd_carries & (!backslashes);
    let even_start_odd_end = even_carry_ends & ODD_BITS;
    let odd_start_even_end = odd_carry_ends & EVEN_BITS;
    even_start_odd_end | odd_start_even_end
}

#[inline(always)]
fn find_whitespace_and_structurals(input: [__m256i; 2], whitespace: &mut u64, structurals: &mut u64) {
    let low_nibble_mask = unsafe { _mm256_setr_epi8(
        16, 0, 0, 0, 0, 0, 0, 0, 0, 8, 10, 4, 1, 12, 0, 0, 
        16, 0, 0, 0, 0, 0, 0, 0, 0, 8, 10, 4, 1, 12, 0, 0,
    ) };
    let high_nibble_mask = unsafe { _mm256_setr_epi8(
        8, 0, 17, 2, 0, 4, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 
        8, 0, 17, 2, 0, 4, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
    ) };
    let structural_mask = unsafe { _mm256_set1_epi8(0x07) };
    let whitespace_mask = unsafe { _mm256_set1_epi8(0x18) };
    let zero = unsafe { _mm256_set1_epi8(0) };
    let category_hi32 = unsafe {
        let lo_nibble = _mm256_shuffle_epi8(low_nibble_mask, input[0]);
        let hi_nibble = _mm256_shuffle_epi8(high_nibble_mask, 
            _mm256_and_si256(_mm256_srli_epi64(input[0], 4), _mm256_set1_epi8(0x7f))
        );
        _mm256_and_si256(lo_nibble, hi_nibble)
    };
    let category_lo32 = unsafe {
        let lo_nibble = _mm256_shuffle_epi8(low_nibble_mask, input[1]);
        let hi_nibble = _mm256_shuffle_epi8(high_nibble_mask, 
            _mm256_and_si256(_mm256_srli_epi64(input[1], 4), _mm256_set1_epi8(0x7f))
        );
        _mm256_and_si256(lo_nibble, hi_nibble)
    };
    let structural_hi32: u32 = unsafe { 
        let category_structural = _mm256_and_si256(category_hi32, structural_mask);
        let ans = _mm256_cmpgt_epi8(category_structural, zero);
        transmute(_mm256_movemask_epi8(ans))
    };
    let structural_lo32: u32 = unsafe { 
        let category_structural = _mm256_and_si256(category_lo32, structural_mask);
        let ans = _mm256_cmpgt_epi8(category_structural, zero);
        transmute(_mm256_movemask_epi8(ans))
    };
    *structurals = structural_lo32 as u64 | ((structural_hi32 as u64) << 32);

    let whitespace_hi32: u32 = unsafe { 
        let category_whitespace = _mm256_and_si256(category_hi32, whitespace_mask);
        let ans = _mm256_cmpgt_epi8(category_whitespace, zero);
        transmute(_mm256_movemask_epi8(ans))
    };
    let whitespace_lo32: u32 = unsafe { 
        let category_whitespace = _mm256_and_si256(category_lo32, whitespace_mask);
        let ans = _mm256_cmpgt_epi8(category_whitespace, zero);
        transmute(_mm256_movemask_epi8(ans))
    };
    *whitespace = whitespace_lo32 as u64 | ((whitespace_hi32 as u64) << 32);
}

fn print_m256(input: __m256i) {
    let arr = [0u64; 4];
    unsafe { _mm256_storeu_si256(&arr as *const _ as *mut __m256i, input) }
    print!("{:016X} {:016X} {:016X} {:016X}", arr[3], arr[2], arr[1], arr[0]);
    println!()
}
