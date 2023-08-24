pub fn add_round_key_u8(state: [[u8; 4]; 4], round_key: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut output: [[u8; 4]; 4] = [[0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            output[i][j] = state[i][j] ^ round_key[i][j];
        }
    }
    return output;
}

pub fn add_round_key_u32(state: [u32; 4], round_key: [u32; 4]) -> [u32; 4] {
    let mut output: [u32; 4] = [0; 4];
    for i in 0..4 {
        output[i] = state[i] ^ round_key[i];
    }
    return output;
}

pub fn add_round_key_u128(state: u128, round_key: u128) -> u128 {
    return state ^ round_key;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_round_key() {
        let input_u8: [[u8; 4]; 4] = [
            [0x01, 0x02, 0x03, 0x04],
            [0x05, 0x06, 0x07, 0x08],
            [0x09, 0x0A, 0x0B, 0x0C],
            [0x0D, 0x0E, 0x0F, 0x10],
        ];
        let round_key_u8: [[u8; 4]; 4] = [
            [0x11, 0x12, 0x13, 0x14],
            [0x15, 0x16, 0x17, 0x18],
            [0x19, 0x1A, 0x1B, 0x1C],
            [0x1D, 0x1E, 0x1F, 0x00],
        ];
        let expected_output_u8: [[u8; 4]; 4] = [
            [0x10, 0x10, 0x10, 0x10],
            [0x10, 0x10, 0x10, 0x10],
            [0x10, 0x10, 0x10, 0x10],
            [0x10, 0x10, 0x10, 0x10],
        ];
        assert_eq!(add_round_key_u8(input_u8, round_key_u8), expected_output_u8);
        assert_eq!(add_round_key_u8(expected_output_u8, round_key_u8), input_u8);

        let input_u32: [u32; 4] = [0x01020304, 0x05060708, 0x090A0B0C, 0x0D0E0F10];
        let round_key_u32: [u32; 4] = [0x11121314, 0x15161718, 0x191A1B1C, 0x1D1E1F00];
        let expected_output_u32: [u32; 4] = [0x10101010, 0x10101010, 0x10101010, 0x10101010];
        assert_eq!(add_round_key_u32(input_u32, round_key_u32), expected_output_u32);
        assert_eq!(add_round_key_u32(expected_output_u32, round_key_u32), input_u32);

        let input_u128: u128 =
            (0x01020304 << 96) | (0x05060708 << 64) | (0x090A0B0C << 32) | 0x0D0E0F10;
        let round_key_u128: u128 =
            (0x11121314 << 96) | (0x15161718 << 64) | (0x191A1B1C << 32) | 0x1D1E1F00;
        let expected_output_u128: u128 =
            (0x10101010 << 96) | (0x10101010 << 64) | (0x10101010 << 32) | 0x10101010;
        assert_eq!(add_round_key_u128(input_u128, round_key_u128), expected_output_u128);
        assert_eq!(add_round_key_u128(expected_output_u128, round_key_u128), input_u128);
    }
}
