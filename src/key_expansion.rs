use crate::round_utils::sub_bytes::sub_bytes_single_u32;
use crate::constants::{AES_128_KEY_SIZE, AES_128_NUM_ROUNDS};

pub const RCON: [u32; 11] = [
    0x00000000, 0x01000000, 0x02000000, 0x04000000, 0x08000000, 0x10000000, 0x20000000, 0x40000000,
    0x80000000, 0x1B000000, 0x36000000,
];

pub fn rotate_word(word: u32) -> u32 {
    return word.rotate_right(8);
}

pub fn get_round_keys_aes_128(key: u128) -> [u128; AES_128_NUM_ROUNDS as usize] {
    let broken_key: [u32; 4] = [
        (key >> 96) as u32,
        (key >> 64) as u32,
        (key >> 32) as u32,
        key as u32,
    ];
    let mut output: [u32; (4 * AES_128_NUM_ROUNDS - 1) as usize] =
        [0u32; (4 * AES_128_NUM_ROUNDS - 1) as usize];
    for i in 0..(4 * AES_128_NUM_ROUNDS - 1) {
        if i < AES_128_KEY_SIZE {
            output[i as usize] = broken_key[i as usize];
        } else if i % AES_128_KEY_SIZE == 0 {
            output[i as usize] = output[(i - AES_128_KEY_SIZE) as usize]
                ^ sub_bytes_single_u32(rotate_word(output[(i - 1) as usize]))
                ^ RCON[(i / AES_128_KEY_SIZE) as usize];
        } else {
            output[i as usize] = output[(i - AES_128_KEY_SIZE) as usize] ^ output[(i - 1) as usize];
        }
    }
    let mut output_u128: [u128; AES_128_NUM_ROUNDS as usize] = [0u128; AES_128_NUM_ROUNDS as usize];
    for i in 0..(AES_128_NUM_ROUNDS as usize) {
        output_u128[i] = (output[i] as u128) << 96
            | (output[i + 1] as u128) << 64
            | (output[i + 2] as u128) << 32
            | (output[i + 3]) as u128;
    }
    return output_u128;
}
