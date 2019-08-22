use std::arch::x86_64::*;
use core::mem::transmute;

fn main() {
    let input = include_bytes!("explore.in");
    let mut input_vec: [__m256i; 8] = unsafe {
        core::mem::zeroed() 
    };
    let mut ptr = input.as_ptr();
    for i in 0..8 {
        input_vec[i] = unsafe { _mm256_loadu_si256(ptr as *const _) };
        unsafe { ptr = ptr.add(32) };
    }
    // for input in &input_vec {
    //     print_m256(*input)
    // }
    let mut prev_ends_odd_backslash = 0;
    odd_backslash_sequences(input_vec, &mut prev_ends_odd_backslash);
}

fn odd_backslash_sequences(input: [__m256i; 8], prev_ends_odd_backslash: &mut i32) -> __m256i {
    let even_bits = unsafe { _mm256_set1_epi8(transmute(0x55u8)) };
    let odd_bits = unsafe { _mm256_set1_epi8(transmute(0xAAu8)) };

    let backslashes = unsafe {
        let mask = _mm256_set1_epi8(b'\\' as i8);
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
    print_m256(backslashes);

    // let starts = backslashes & (!(backslashes << 1));
    let starts = unsafe {
        // calculate ans_sll = backslash << 1
        let sll64 = _mm256_slli_epi64(backslashes, 1);
        let srl64 = _mm256_srli_epi64(backslashes, 63);
        let hi64 = _mm256_permute4x64_epi64(srl64, 0b10_01_00_11);
        let hi64 = _mm256_insert_epi64(hi64, 0, 0);
        let ans_sll = _mm256_add_epi64(sll64, hi64);
        // calculate backslashes & (!ans_sll)
        _mm256_andnot_si256(ans_sll, backslashes)
    };
    print_m256(starts);

    let even_starts = unsafe { _mm256_andnot_si256(odd_bits, starts) };
    print_m256(even_starts);

    let even_carries = unsafe { _mm256_add_epi64(even_starts, backslashes) };
    print_m256(even_carries);

    unimplemented!()
}

fn print_m256(input: __m256i) {
    let arr = [0u64; 4];
    unsafe { _mm256_storeu_si256(&arr as *const _ as *mut __m256i, input) }
    print!("{:016X} {:016X} {:016X} {:016X}", arr[3], arr[2], arr[1], arr[0]);
    println!()
}
