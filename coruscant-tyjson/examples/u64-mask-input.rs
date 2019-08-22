use std::arch::x86_64::*;
use core::mem::transmute;

fn main() {
    let input = include_bytes!("explore.in");
    let mut input_vec: [__m256i; 2] = unsafe { core::mem::zeroed() };
    let mut prev_ov = 0;
    let mut ptr = input.as_ptr();
    for _ in 0..4 {
        input_vec[0] = unsafe { _mm256_loadu_si256(ptr as *const _) };
        input_vec[1] = unsafe { _mm256_loadu_si256(ptr.add(32) as *const _) };
        unsafe { ptr = ptr.add(64) };
        let od = odd_backslash_sequences(input_vec, &mut prev_ov);
        println!("{:064b} {}", od, prev_ov);
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



// fn print_m256(input: __m256i) {
//     let arr = [0u64; 4];
//     unsafe { _mm256_storeu_si256(&arr as *const _ as *mut __m256i, input) }
//     print!("{:016X} {:016X} {:016X} {:016X}", arr[3], arr[2], arr[1], arr[0]);
//     println!()
// }
