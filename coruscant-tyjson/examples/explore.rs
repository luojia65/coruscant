use std::arch::x86_64::*;
use core::mem::transmute;

fn main() {
    let input = include_bytes!("explore.in");
    let mut input_vec: [__m256i; 8] = unsafe {
        core::mem::MaybeUninit::uninit().assume_init() 
    };
    let mut ptr = input.as_ptr();
    for i in 0..8 {
        input_vec[i] = unsafe { _mm256_load_si256(ptr as *const _) };
        unsafe { ptr = ptr.add(32) };
    }
    for input in &input_vec {
        print_m256(*input)
    }
    odd_backslash_sequences(input_vec);
}

fn odd_backslash_sequences(input: [__m256i; 8]) -> __m256i {
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
    // there is no slli for si256, have to split into 64 bit values
    let starts = unsafe {
        _mm256_andnot_si256(_mm256_slli_epi64(backslashes, 1), backslashes)
    };
    print_m256(starts);

    let even_starts = unsafe { _mm256_xor_si256(starts, even_bits) };
    print_m256(even_starts);

    unimplemented!()
}

fn print_m256(input: __m256i) {
    let arr = [0u64; 4];
    unsafe { _mm256_storeu_si256(&arr as *const _ as *mut __m256i, input) }
    print!("{:016X} {:016X} {:016X} {:016X}", arr[3], arr[2], arr[1], arr[0]);
    println!()
}
