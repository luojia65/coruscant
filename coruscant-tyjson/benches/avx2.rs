#![feature(test)]
extern crate test;
use test::*;

const INPUT: &[u8] = include_bytes!("16KB.in");

use std::arch::x86_64::*;

#[bench]
fn fill_input_avx2_m256x8_m256(b: &mut Bencher) {
    use coruscant_tyjson::mux::avx2_m256x8_m256::*;
    b.iter(|| {
        let mut input_vec: [__m256i; 8] = unsafe { core::mem::zeroed() };
        let mut ptr = INPUT.as_ptr();
        for _ in 0..(INPUT.len() / (32 * 8)) {
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
    use coruscant_tyjson::mux::avx2_m256x8_m256::*;
    b.iter(|| {
        let mut input_vec: [__m256i; 2] = unsafe { core::mem::zeroed() };
        let mut prev_ov = 0;
        let mut ptr = INPUT.as_ptr();
        for _ in 0..(INPUT.len() / 64) {
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
    b.iter(|| {
        let mut input_vec: [__m256i; 8] = unsafe { core::mem::zeroed() };
        let mut prev_ov = false;
        let mut ptr = INPUT.as_ptr();
        for _ in 0..(INPUT.len() / (32 * 8)) {
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
    b.iter(|| {
        let mut input_vec: [__m256i; 2] = unsafe { core::mem::zeroed() };
        let mut prev_ov = 0;
        let mut ptr = INPUT.as_ptr();
        for _ in 0..(INPUT.len() / 64) {
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
    b.iter(|| {
        let mut input_vec: [__m256i; 8] = unsafe { core::mem::zeroed() };
        let mut whitespace: __m256i = unsafe { core::mem::zeroed() };
        let mut structurals: __m256i = unsafe { core::mem::zeroed() };
        let mut ptr = INPUT.as_ptr();
        for _ in 0..(INPUT.len() / (32 * 8)) {
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
    b.iter(|| {
        let mut input_vec: [__m256i; 2] = unsafe { core::mem::zeroed() };
        let mut whitespace = 0;
        let mut structurals = 0;
        let mut ptr = INPUT.as_ptr();
        for _ in 0..(INPUT.len() / 64) {
            input_vec[0] = unsafe { _mm256_loadu_si256(ptr as *const _) };
            input_vec[1] = unsafe { _mm256_loadu_si256(ptr.add(32) as *const _) };
            unsafe { ptr = ptr.add(64) };
            find_whitespace_and_structurals(input_vec, &mut whitespace, &mut structurals);
        }
    })
}
