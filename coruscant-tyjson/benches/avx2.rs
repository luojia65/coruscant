#![feature(test)]
extern crate test;
use test::*;

use std::arch::x86_64::*;
use core::mem::transmute;

fn read_input() -> Vec<u8> {
    use std::fs::File;
    use std::io::Read;
    let mut ans = Vec::new();
    File::open("benches/canada.json")
        .unwrap()
        .read_to_end(&mut ans)
        .unwrap();
    ans
}

#[bench]
fn fill_input_avx2_m256x8_m256(b: &mut Bencher) {
    let input = read_input();
    b.iter(|| {
        let mut input_vec: [__m256i; 8] = unsafe { core::mem::zeroed() };
        let mut ptr = input.as_ptr();
        for _ in 0..(input.len() / (32 * 8)) {
            for i in 0..8 {
                input_vec[i] = unsafe { _mm256_loadu_si256(ptr as *const _) };
                unsafe { ptr = ptr.add(32) };
            }
            let _ = black_box(input_vec);
        }
    })
}

#[bench]
fn fill_input_avx2_m256x2_u64(b: &mut Bencher) {
    let input = read_input();
    b.iter(|| {
        let mut input_vec: [__m256i; 2] = unsafe { core::mem::zeroed() };
        let mut ptr = input.as_ptr();
        for _ in 0..(input.len() / 64) {
            input_vec[0] = unsafe { _mm256_loadu_si256(ptr as *const _) };
            input_vec[1] = unsafe { _mm256_loadu_si256(ptr.add(32) as *const _) };
            unsafe { ptr = ptr.add(64) };
            let _ = black_box(input_vec);
        }
    })
}

#[bench]
fn odd_backslash_sequence_avx2_m256x8_m256(b: &mut Bencher) {
    use coruscant_tyjson::mux::avx2_m256x8_m256::*;
    let input = read_input();
    b.iter(|| {
        let mut input_vec: [__m256i; 8] = unsafe { core::mem::zeroed() };
        let mut prev_ov = false;
        let mut ptr = input.as_ptr();
        for _ in 0..(input.len() / (32 * 8)) {
            for i in 0..8 {
                input_vec[i] = unsafe { _mm256_loadu_si256(ptr as *const _) };
                unsafe { ptr = ptr.add(32) };
            }
            let od = odd_backslash_sequences(input_vec, &mut prev_ov);
            let _ = black_box(od);
        }
    })
}

#[bench]
fn odd_backslash_sequence_avx2_m256x2_u64(b: &mut Bencher) {
    use coruscant_tyjson::mux::avx2_m256x2_u64::*;
    let input = read_input();
    b.iter(|| {
        let mut input_vec: [__m256i; 2] = unsafe { core::mem::zeroed() };
        let mut prev_ov = 0;
        let mut ptr = input.as_ptr();
        for _ in 0..(input.len() / 64) {
            input_vec[0] = unsafe { _mm256_loadu_si256(ptr as *const _) };
            input_vec[1] = unsafe { _mm256_loadu_si256(ptr.add(32) as *const _) };
            unsafe { ptr = ptr.add(64) };
            let od = odd_backslash_sequences(input_vec, &mut prev_ov);
            let _ = black_box(od);
        }
    })
}

#[bench]
fn whitespace_structurals_avx2_m256x8_m256(b: &mut Bencher) {
    use coruscant_tyjson::mux::avx2_m256x8_m256::*;
    let input = read_input();
    b.iter(|| {
        let mut input_vec: [__m256i; 8] = unsafe { core::mem::zeroed() };
        let mut whitespace: __m256i = unsafe { core::mem::zeroed() };
        let mut structurals: __m256i = unsafe { core::mem::zeroed() };
        let mut ptr = input.as_ptr();
        for _ in 0..(input.len() / (32 * 8)) {
            for i in 0..8 {
                input_vec[i] = unsafe { _mm256_loadu_si256(ptr as *const _) };
                unsafe { ptr = ptr.add(32) };
            }
            find_whitespace_and_structurals(input_vec, &mut whitespace, &mut structurals);
        }
    })
}

#[bench]
fn whitespace_structurals_avx2_m256x2_u64(b: &mut Bencher) {
    use coruscant_tyjson::mux::avx2_m256x2_u64::*;
    let input = read_input();
    b.iter(|| {
        let mut input_vec: [__m256i; 2] = unsafe { core::mem::zeroed() };
        let mut whitespace = 0;
        let mut structurals = 0;
        let mut ptr = input.as_ptr();
        for _ in 0..(input.len() / 64) {
            input_vec[0] = unsafe { _mm256_loadu_si256(ptr as *const _) };
            input_vec[1] = unsafe { _mm256_loadu_si256(ptr.add(32) as *const _) };
            unsafe { ptr = ptr.add(64) };
            find_whitespace_and_structurals(input_vec, &mut whitespace, &mut structurals);
        }
    })
}

#[bench]
fn analyze_avx2_m256x8_m256(b: &mut Bencher) {
    use coruscant_tyjson::mux::avx2_m256x8_m256::*;
    let input = read_input();
    b.iter(|| {
        let mut input_vec: [__m256i; 8] = unsafe { core::mem::zeroed() };
        let mut whitespace: __m256i = unsafe { core::mem::zeroed() };
        let mut structurals: __m256i = unsafe { core::mem::zeroed() };
        let mut prev_ov = false;
        let mut ptr = input.as_ptr();
        for _ in 0..(input.len() / (32 * 8)) {
            for i in 0..8 {
                input_vec[i] = unsafe { _mm256_loadu_si256(ptr as *const _) };
                unsafe { ptr = ptr.add(32) };
            }
            let od = odd_backslash_sequences(input_vec, &mut prev_ov);
            find_whitespace_and_structurals(input_vec, &mut whitespace, &mut structurals);
            let _ = black_box(od);
        }
    })
}

#[bench]
fn analyze_avx2_m256x2_u64(b: &mut Bencher) {
    use coruscant_tyjson::mux::avx2_m256x2_u64::*;
    let input = read_input();
    b.iter(|| {
        let mut input_vec: [__m256i; 2] = unsafe { core::mem::zeroed() };
        let mut whitespace = 0;
        let mut structurals = 0;
        let mut prev_ov = 0;
        let mut ptr = input.as_ptr();
        for _ in 0..(input.len() / 64) {
            input_vec[0] = unsafe { _mm256_loadu_si256(ptr as *const _) };
            input_vec[1] = unsafe { _mm256_loadu_si256(ptr.add(32) as *const _) };
            unsafe { ptr = ptr.add(64) };
            let od = odd_backslash_sequences(input_vec, &mut prev_ov);
            find_whitespace_and_structurals(input_vec, &mut whitespace, &mut structurals);
            let _ = black_box(od);
        }
    })
}

#[bench]
fn analyze_avx2_mixed(b: &mut Bencher) {
    use coruscant_tyjson::mux::*;
    let input = read_input();
    b.iter(|| {
        let mut whitespace: __m256i = unsafe { core::mem::zeroed() };
        let mut structurals: __m256i = unsafe { core::mem::zeroed() };
        let mut prev_ov = 0;
        let mut ptr = input.as_ptr();
        for _ in 0..(input.len() / (32 * 8)) {
            let input_0 = unsafe { _mm256_loadu_si256(ptr as *const _) };
            unsafe { ptr = ptr.add(32) };
            let input_1 = unsafe { _mm256_loadu_si256(ptr as *const _) };
            unsafe { ptr = ptr.add(32) };
            let input_2 = unsafe { _mm256_loadu_si256(ptr as *const _) };
            unsafe { ptr = ptr.add(32) };
            let input_3 = unsafe { _mm256_loadu_si256(ptr as *const _) };
            unsafe { ptr = ptr.add(32) };
            let input_4 = unsafe { _mm256_loadu_si256(ptr as *const _) };
            unsafe { ptr = ptr.add(32) };
            let input_5 = unsafe { _mm256_loadu_si256(ptr as *const _) };
            unsafe { ptr = ptr.add(32) };
            let input_6 = unsafe { _mm256_loadu_si256(ptr as *const _) };
            unsafe { ptr = ptr.add(32) };
            let input_7 = unsafe { _mm256_loadu_si256(ptr as *const _) };
            unsafe { ptr = ptr.add(32) };
            let od10 = avx2_m256x2_u64::odd_backslash_sequences([input_1, input_0], &mut prev_ov);
            let od32 = avx2_m256x2_u64::odd_backslash_sequences([input_3, input_2], &mut prev_ov);
            let od54 = avx2_m256x2_u64::odd_backslash_sequences([input_5, input_4], &mut prev_ov);
            let od76 = avx2_m256x2_u64::odd_backslash_sequences([input_7, input_6], &mut prev_ov);
            let od = unsafe { _mm256_set_epi64x(transmute(od76), transmute(od54), transmute(od32), transmute(od10))};
            avx2_m256x8_m256::find_whitespace_and_structurals(
                [input_0, input_1, input_2, input_3, input_4, input_5, input_6, input_7], 
                &mut whitespace, &mut structurals
            );
            let _ = black_box(od);
        }
    })
}
