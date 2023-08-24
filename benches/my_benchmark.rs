use aes::key_expansion::rotate_word;
use aes::rijndael::{encrypt_aes_128, decrypt_aes_128};
use aes::round_utils::add_round_keys::{add_round_key_u8, add_round_key_u32, add_round_key_u128};
use aes::round_utils::shift_rows::{shift_rows_u128, shift_rows_u32, shift_rows_u8};
use aes::round_utils::sub_bytes::{sub_bytes_u8, sub_bytes_u32, sub_bytes_u128};
use aes::round_utils::mix_columns::{mix_columns_u8, mix_columns_u32, mix_columns_u128};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let state_u8: [[u8; 4]; 4] = [
        [0x32, 0x88, 0x31, 0xe0],
        [0x43, 0x5a, 0x31, 0x37],
        [0xf6, 0x30, 0x98, 0x07],
        [0xa8, 0x8d, 0xa2, 0x34],
    ];
    let key_u8: [[u8; 4]; 4] = [
        [0x2b, 0x28, 0xab, 0x09],
        [0x7e, 0xae, 0xf7, 0xcf],
        [0x15, 0xd2, 0x15, 0x4f],
        [0x16, 0xa6, 0x88, 0x3c],
    ];
    c.bench_function("u8_add_key", |b| {
        b.iter(|| add_round_key_u8(black_box(state_u8), black_box(key_u8)))
    });

    let state_u32: [u32; 4] = [
        0x328831e0, 0x435a3137, 0xf6309807, 0xa88da234
    ];
    let key_u32: [u32; 4] = [
        0x2b28ab09, 0x7eaef7cf, 0x15d2154f, 0x16a6883c
    ];
    c.bench_function("u32_add_key", |b| {
        b.iter(|| add_round_key_u32(black_box(state_u32), black_box(key_u32)))
    });

    let state_u128: u128 = 0x328831e0435a3137f6309807a88da234;
    let key_u128: u128 = 0x2b28ab097eaef7cf15d2154f16a6883c;
    c.bench_function("u128_add_key", |b| {
        b.iter(|| add_round_key_u128(black_box(state_u128), black_box(key_u128)))
    });

    c.bench_function("u8_shift_rows", |b| {
        b.iter(|| shift_rows_u8(black_box(state_u8)))
    });
    
    c.bench_function("u32_shift_rows", |b| {
        b.iter(|| shift_rows_u32(black_box(state_u32)))
    });
    
    c.bench_function("u128_shift_rows", |b| {
        b.iter(|| shift_rows_u128(black_box(state_u128)))
    });

    c.bench_function("u8_sub_bytes", |b| {
        b.iter(|| sub_bytes_u8(black_box(state_u8)))
    });
    
    c.bench_function("u32_sub_bytes", |b| {
        b.iter(|| sub_bytes_u32(black_box(state_u32)))
    });
    
    c.bench_function("u128_sub_bytes", |b| {
        b.iter(|| sub_bytes_u128(black_box(state_u128)))
    });

    c.bench_function("u8_mix_columns", |b| {
        b.iter(|| mix_columns_u8(black_box(state_u8)))
    });
    
    c.bench_function("u32_mix_columns", |b| {
        b.iter(|| mix_columns_u32(black_box(state_u32)))
    });
    
    c.bench_function("u128_mix_columns", |b| {
        b.iter(|| mix_columns_u128(black_box(state_u128)))
    });

    c.bench_function("rotate_word", |b| {
        b.iter(|| rotate_word(black_box(state_u32[0])))
    });

    c.bench_function("encrypt_block", |b| {
        b.iter(|| encrypt_aes_128(black_box(state_u128), black_box(state_u128)))
    });
    c.bench_function("decrypt_block", |b| {
        b.iter(|| decrypt_aes_128(black_box(state_u128), black_box(state_u128)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
