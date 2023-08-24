use crate::round_utils::{add_round_keys, mix_columns, shift_rows, sub_bytes};
use crate::key_expansion::get_round_keys_aes_128;
use crate::constants::AES_128_NUM_ROUNDS;

pub fn encrypt_aes_128(input_block: u128, key: u128) -> u128 {
    let mut state = input_block;

    // get round keys
    let round_keys = get_round_keys_aes_128(key);

    // first round
    state = add_round_keys::add_round_key_u128(state, round_keys[0]);
    // rounds 2 to n-1
    for i in 1..((AES_128_NUM_ROUNDS-1) as usize) {
        state = sub_bytes::sub_bytes_u128(state);
        state = shift_rows::shift_rows_u128(state);
        state = mix_columns::mix_columns_u128(state);
        state = add_round_keys::add_round_key_u128(state, round_keys[i]);
    }
    // last round
    state = sub_bytes::sub_bytes_u128(state);
    state = shift_rows::shift_rows_u128(state);
    state = add_round_keys::add_round_key_u128(state, round_keys[10]);
    return state;
}

pub fn decrypt_aes_128(input_block: u128, key: u128) -> u128 {
    let mut state = input_block;

    // get round keys
    let round_keys = get_round_keys_aes_128(key);

    // first round
    state = add_round_keys::add_round_key_u128(state, round_keys[10]);
    // rounds 2 to n-1
    for i in (1..((AES_128_NUM_ROUNDS-1) as usize)).rev() {
        state = shift_rows::inv_shift_rows_u128(state);
        state = sub_bytes::inv_sub_bytes_u128(state);
        state = add_round_keys::add_round_key_u128(state, round_keys[i]);
        state = mix_columns::inv_mix_columns_u128(state);
    }
    // last round
    state = shift_rows::inv_shift_rows_u128(state);
    state = sub_bytes::inv_sub_bytes_u128(state);
    state = add_round_keys::add_round_key_u128(state, round_keys[0]);
    return state;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let input: u128 = 0x3243f6a8885a308d313198a2e0370734;
        let key: u128 = 0x2b7e151628aed2a6abf7158809cf4f3c;

        let cipher = encrypt_aes_128(input, key);
        let decoded = decrypt_aes_128(cipher, key);

        assert_eq!(decoded, input);
    }
}