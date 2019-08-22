use std::arch::x86_64::*;
use core::mem::transmute;

fn main() {
    let input = include_bytes!("explore.in");
    let mut input_vec: [__m256i; 8] = unsafe { core::mem::zeroed() };
    let mut prev_ov = false;
    let mut ptr = input.as_ptr();
    for _ in 0..16 {
        for i in 0..8 {
            input_vec[i] = unsafe { _mm256_loadu_si256(ptr as *const _) };
            unsafe { ptr = ptr.add(32) };
        }
        let od = odd_backslash_sequences(input_vec, &mut prev_ov);
        print!("{} ", if prev_ov { 1 } else { 0 });
        print_m256(od);
    }
}

#[inline(always)]
fn odd_backslash_sequences(input: [__m256i; 8], prev_ov: &mut bool) -> __m256i {
    let even_bits = unsafe { _mm256_set1_epi8(transmute(0x55u8)) };
    let odd_bits = unsafe { _mm256_set1_epi8(transmute(0xAAu8)) };
    let one = unsafe { _mm256_set_epi64x(0, 0, 0, 1) };

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

    let even_start_flipped_mask = if *prev_ov { 
        unsafe { _mm256_xor_si256(odd_bits, one) }
    } else { 
        odd_bits  
    };
    let even_starts = unsafe { _mm256_andnot_si256(even_start_flipped_mask, starts) };
    let odd_starts = unsafe { _mm256_and_si256(even_start_flipped_mask, starts) };
    // this 64-bit add would not overflow; all the odd bits are already zero
    let even_carries = unsafe { _mm256_add_epi64(even_starts, backslashes) };
    let (odd_carries, ov) = unsafe { 
        let add64 = _mm256_add_epi64(odd_starts, backslashes);
        let ov64_hi = _mm256_and_si256(odd_starts, backslashes);
        let ov64_inplace = _mm256_srli_epi64(ov64_hi, 63);
        let ov64_rol = _mm256_permute4x64_epi64(ov64_inplace, 0b10_01_00_11);
        let ov256 = _mm256_cvtsi256_si32(ov64_rol);
        let ov64_rol = _mm256_insert_epi64(ov64_rol, 0, 0);
        let add256 = _mm256_add_epi64(add64, ov64_rol);
        (add256, ov256)
    };
    let odd_carries = if *prev_ov { 
        unsafe { _mm256_or_si256(odd_carries, one) }
    } else {
        odd_carries
    };
    *prev_ov = ov != 0;
    let even_carry_ends = unsafe { _mm256_andnot_si256(backslashes, even_carries) };
    let odd_carry_ends = unsafe { _mm256_andnot_si256(backslashes, odd_carries) };
    let even_start_odd_end = unsafe { _mm256_and_si256(even_carry_ends, odd_bits) };
    let odd_start_even_end = unsafe { _mm256_and_si256(odd_carry_ends, even_bits) };
    unsafe { _mm256_or_si256(even_start_odd_end, odd_start_even_end) }
}

fn print_m256(input: __m256i) {
    let arr = [0u64; 4];
    unsafe { _mm256_storeu_si256(&arr as *const _ as *mut __m256i, input) }
    println!("{:016X} {:016X} {:016X} {:016X}", arr[3], arr[2], arr[1], arr[0]);
}

// fn print_m256_bits(input: __m256i) {
//     let arr = [0u64; 4];
//     unsafe { _mm256_storeu_si256(&arr as *const _ as *mut __m256i, input) }
//     for i in 0..4 {
//         for j in 0..64 {
//             if (arr[i] & (0b1 << j)) != 0 { 
//                 print!("1")
//             } else {
//                 print!("-")
//             }
//         };
//     }
//     println!()
// }
