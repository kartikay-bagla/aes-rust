mod constants;
use constants::{MULT_11, MULT_13, MULT_14, MULT_2, MULT_3, MULT_9};

pub fn mix_columns_u8(state: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut output: [[u8; 4]; 4] = [[0; 4]; 4];
    for i in 0..4 {
        output[i][0] =
            MULT_2[state[i][0] as usize] ^ MULT_3[state[i][1] as usize] ^ state[i][2] ^ state[i][3];
        output[i][1] =
            state[i][0] ^ MULT_2[state[i][1] as usize] ^ MULT_3[state[i][2] as usize] ^ state[i][3];
        output[i][2] =
            state[i][0] ^ state[i][1] ^ MULT_2[state[i][2] as usize] ^ MULT_3[state[i][3] as usize];
        output[i][3] =
            MULT_3[state[i][0] as usize] ^ state[i][1] ^ state[i][2] ^ MULT_2[state[i][3] as usize];
    }
    return output;
}

pub fn inv_mix_columns_u8(state: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut output: [[u8; 4]; 4] = [[0; 4]; 4];
    for i in 0..4 {
        output[i][0] = MULT_14[state[i][0] as usize]
            ^ MULT_11[state[i][1] as usize]
            ^ MULT_13[state[i][2] as usize]
            ^ MULT_9[state[i][3] as usize];
        output[i][1] = MULT_9[state[i][0] as usize]
            ^ MULT_14[state[i][1] as usize]
            ^ MULT_11[state[i][2] as usize]
            ^ MULT_13[state[i][3] as usize];
        output[i][2] = MULT_13[state[i][0] as usize]
            ^ MULT_9[state[i][1] as usize]
            ^ MULT_14[state[i][2] as usize]
            ^ MULT_11[state[i][3] as usize];
        output[i][3] = MULT_11[state[i][0] as usize]
            ^ MULT_13[state[i][1] as usize]
            ^ MULT_9[state[i][2] as usize]
            ^ MULT_14[state[i][3] as usize];
    }
    return output;
}

pub fn mix_columns_u32(state: [u32; 4]) -> [u32; 4] {
    let mut output: [u32; 4] = [0; 4];
    for i in 0..4 {
        let a1: u32 = state[i] >> 24;
        let a2: u32 = (state[i] & 0x00FF0000) >> 16;
        let a3: u32 = (state[i] & 0x0000FF00) >> 8;
        let a4: u32 = state[i] & 0xFF;
        output[i] = ((MULT_2[a1 as usize] as u32 ^ MULT_3[a2 as usize] as u32 ^ a3 ^ a4) << 24)
            | ((a1 ^ MULT_2[a2 as usize] as u32 ^ MULT_3[a3 as usize] as u32 ^ a4) << 16)
            | ((a1 ^ a2 ^ MULT_2[a3 as usize] as u32 ^ MULT_3[a4 as usize] as u32) << 08)
            | (MULT_3[a1 as usize] as u32 ^ a2 ^ a3 ^ MULT_2[a4 as usize] as u32);
    }
    return output;
}

pub fn inv_mix_columns_u32(state: [u32; 4]) -> [u32; 4] {
    let mut output: [u32; 4] = [0; 4];
    for i in 0..4 {
        let a1: u32 = state[i] >> 24;
        let a2: u32 = (state[i] & 0x00FF0000) >> 16;
        let a3: u32 = (state[i] & 0x0000FF00) >> 8;
        let a4: u32 = state[i] & 0xFF;
        output[i] = ((MULT_14[a1 as usize] as u32
            ^ MULT_11[a2 as usize] as u32
            ^ MULT_13[a3 as usize] as u32
            ^ MULT_9[a4 as usize] as u32)
            << 24)
            | ((MULT_9[a1 as usize] as u32
                ^ MULT_14[a2 as usize] as u32
                ^ MULT_11[a3 as usize] as u32
                ^ MULT_13[a4 as usize] as u32)
                << 16)
            | ((MULT_13[a1 as usize] as u32
                ^ MULT_9[a2 as usize] as u32
                ^ MULT_14[a3 as usize] as u32
                ^ MULT_11[a4 as usize] as u32)
                << 08)
            | (MULT_11[a1 as usize] as u32
                ^ MULT_13[a2 as usize] as u32
                ^ MULT_9[a3 as usize] as u32
                ^ MULT_14[a4 as usize] as u32);
    }
    return output;
}

pub fn mix_columns_u128(state: u128) -> u128 {
    let s0: u128 = state >> 96;
    let s1: u128 = (state >> 64) & 0xFFFFFFFF;
    let s2: u128 = (state >> 32) & 0xFFFFFFFF;
    let s3: u128 = state & 0xFFFFFFFF;

    let a1: u128 = s0 >> 24;
    let a2: u128 = (s0 >> 16) & 0xFF;
    let a3: u128 = (s0 >> 8) & 0xFF;
    let a4: u128 = s0 & 0xFF;

    let b1: u128 = s1 >> 24;
    let b2: u128 = (s1 >> 16) & 0xFF;
    let b3: u128 = (s1 >> 8) & 0xFF;
    let b4: u128 = s1 & 0xFF;

    let c1: u128 = s2 >> 24;
    let c2: u128 = (s2 >> 16) & 0xFF;
    let c3: u128 = (s2 >> 8) & 0xFF;
    let c4: u128 = s2 & 0xFF;

    let d1: u128 = s3 >> 24;
    let d2: u128 = (s3 >> 16) & 0xFF;
    let d3: u128 = (s3 >> 8) & 0xFF;
    let d4: u128 = s3 & 0xFF;

    return (((MULT_2[a1 as usize] as u128 ^ MULT_3[a2 as usize] as u128 ^ a3 ^ a4) << 24)
        | ((a1 ^ MULT_2[a2 as usize] as u128 ^ MULT_3[a3 as usize] as u128 ^ a4) << 16)
        | ((a1 ^ a2 ^ MULT_2[a3 as usize] as u128 ^ MULT_3[a4 as usize] as u128) << 8)
        | (MULT_3[a1 as usize] as u128 ^ a2 ^ a3 ^ MULT_2[a4 as usize] as u128))
        << 96
        | (((MULT_2[b1 as usize] as u128 ^ MULT_3[b2 as usize] as u128 ^ b3 ^ b4) << 24)
            | ((b1 ^ MULT_2[b2 as usize] as u128 ^ MULT_3[b3 as usize] as u128 ^ b4) << 16)
            | ((b1 ^ b2 ^ MULT_2[b3 as usize] as u128 ^ MULT_3[b4 as usize] as u128) << 8)
            | (MULT_3[b1 as usize] as u128 ^ b2 ^ b3 ^ MULT_2[b4 as usize] as u128))
            << 64
        | (((MULT_2[c1 as usize] as u128 ^ MULT_3[c2 as usize] as u128 ^ c3 ^ c4) << 24)
            | ((c1 ^ MULT_2[c2 as usize] as u128 ^ MULT_3[c3 as usize] as u128 ^ c4) << 16)
            | ((c1 ^ c2 ^ MULT_2[c3 as usize] as u128 ^ MULT_3[c4 as usize] as u128) << 8)
            | (MULT_3[c1 as usize] as u128 ^ c2 ^ c3 ^ MULT_2[c4 as usize] as u128))
            << 32
        | (((MULT_2[d1 as usize] as u128 ^ MULT_3[d2 as usize] as u128 ^ d3 ^ d4) << 24)
            | ((d1 ^ MULT_2[d2 as usize] as u128 ^ MULT_3[d3 as usize] as u128 ^ d4) << 16)
            | ((d1 ^ d2 ^ MULT_2[d3 as usize] as u128 ^ MULT_3[d4 as usize] as u128) << 8)
            | (MULT_3[d1 as usize] as u128 ^ d2 ^ d3 ^ MULT_2[d4 as usize] as u128));
}

pub fn inv_mix_columns_u128(state: u128) -> u128 {
    let s0: u128 = state >> 96;
    let s1: u128 = (state >> 64) & 0xFFFFFFFF;
    let s2: u128 = (state >> 32) & 0xFFFFFFFF;
    let s3: u128 = state & 0xFFFFFFFF;

    let a1: u128 = s0 >> 24;
    let a2: u128 = (s0 >> 16) & 0xFF;
    let a3: u128 = (s0 >> 8) & 0xFF;
    let a4: u128 = s0 & 0xFF;

    let b1: u128 = s1 >> 24;
    let b2: u128 = (s1 >> 16) & 0xFF;
    let b3: u128 = (s1 >> 8) & 0xFF;
    let b4: u128 = s1 & 0xFF;

    let c1: u128 = s2 >> 24;
    let c2: u128 = (s2 >> 16) & 0xFF;
    let c3: u128 = (s2 >> 8) & 0xFF;
    let c4: u128 = s2 & 0xFF;

    let d1: u128 = s3 >> 24;
    let d2: u128 = (s3 >> 16) & 0xFF;
    let d3: u128 = (s3 >> 8) & 0xFF;
    let d4: u128 = s3 & 0xFF;

    return (((MULT_14[a1 as usize] as u128
        ^ MULT_11[a2 as usize] as u128
        ^ MULT_13[a3 as usize] as u128
        ^ MULT_9[a4 as usize] as u128)
        << 24)
        | ((MULT_9[a1 as usize] as u128
            ^ MULT_14[a2 as usize] as u128
            ^ MULT_11[a3 as usize] as u128
            ^ MULT_13[a4 as usize] as u128)
            << 16)
        | ((MULT_13[a1 as usize] as u128
            ^ MULT_9[a2 as usize] as u128
            ^ MULT_14[a3 as usize] as u128
            ^ MULT_11[a4 as usize] as u128)
            << 8)
        | (MULT_11[a1 as usize] as u128
            ^ MULT_13[a2 as usize] as u128
            ^ MULT_9[a3 as usize] as u128
            ^ MULT_14[a4 as usize] as u128))
        << 96
        | (((MULT_14[b1 as usize] as u128
            ^ MULT_11[b2 as usize] as u128
            ^ MULT_13[b3 as usize] as u128
            ^ MULT_9[b4 as usize] as u128)
            << 24)
            | ((MULT_9[b1 as usize] as u128
                ^ MULT_14[b2 as usize] as u128
                ^ MULT_11[b3 as usize] as u128
                ^ MULT_13[b4 as usize] as u128)
                << 16)
            | ((MULT_13[b1 as usize] as u128
                ^ MULT_9[b2 as usize] as u128
                ^ MULT_14[b3 as usize] as u128
                ^ MULT_11[b4 as usize] as u128)
                << 8)
            | (MULT_11[b1 as usize] as u128
                ^ MULT_13[b2 as usize] as u128
                ^ MULT_9[b3 as usize] as u128
                ^ MULT_14[b4 as usize] as u128))
            << 64
        | (((MULT_14[c1 as usize] as u128
            ^ MULT_11[c2 as usize] as u128
            ^ MULT_13[c3 as usize] as u128
            ^ MULT_9[c4 as usize] as u128)
            << 24)
            | ((MULT_9[c1 as usize] as u128
                ^ MULT_14[c2 as usize] as u128
                ^ MULT_11[c3 as usize] as u128
                ^ MULT_13[c4 as usize] as u128)
                << 16)
            | ((MULT_13[c1 as usize] as u128
                ^ MULT_9[c2 as usize] as u128
                ^ MULT_14[c3 as usize] as u128
                ^ MULT_11[c4 as usize] as u128)
                << 8)
            | (MULT_11[c1 as usize] as u128
                ^ MULT_13[c2 as usize] as u128
                ^ MULT_9[c3 as usize] as u128
                ^ MULT_14[c4 as usize] as u128))
            << 32
        | (((MULT_14[d1 as usize] as u128
            ^ MULT_11[d2 as usize] as u128
            ^ MULT_13[d3 as usize] as u128
            ^ MULT_9[d4 as usize] as u128)
            << 24)
            | ((MULT_9[d1 as usize] as u128
                ^ MULT_14[d2 as usize] as u128
                ^ MULT_11[d3 as usize] as u128
                ^ MULT_13[d4 as usize] as u128)
                << 16)
            | ((MULT_13[d1 as usize] as u128
                ^ MULT_9[d2 as usize] as u128
                ^ MULT_14[d3 as usize] as u128
                ^ MULT_11[d4 as usize] as u128)
                << 8)
            | (MULT_11[d1 as usize] as u128
                ^ MULT_13[d2 as usize] as u128
                ^ MULT_9[d3 as usize] as u128
                ^ MULT_14[d4 as usize] as u128));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix_columns() {
        let input_u8: [[u8; 4]; 4] = [
            [0xdb, 0x13, 0x53, 0x45],
            [0xf2, 0x0a, 0x22, 0x5c],
            [0xd4, 0xd4, 0xd4, 0xd5],
            [0x01, 0x01, 0x01, 0x01],
        ];
        let expected_output_u8: [[u8; 4]; 4] = [
            [0x8e, 0x4d, 0xa1, 0xbc],
            [0x9f, 0xdc, 0x58, 0x9d],
            [0xd5, 0xd5, 0xd7, 0xd6],
            [0x01, 0x01, 0x01, 0x01],
        ];
        assert_eq!(mix_columns_u8(input_u8), expected_output_u8);
        assert_eq!(inv_mix_columns_u8(expected_output_u8), input_u8);

        let input_u32: [u32; 4] = [0xdb135345, 0xf20a225c, 0xd4d4d4d5, 0x01010101];
        let expected_output_u32: [u32; 4] = [0x8e4da1bc, 0x9fdc589d, 0xd5d5d7d6, 0x01010101];
        assert_eq!(mix_columns_u32(input_u32), expected_output_u32);
        assert_eq!(inv_mix_columns_u32(expected_output_u32), input_u32);

        let input_u128: u128 =
            (0xdb135345 << 96) | (0xf20a225c << 64) | (0xd4d4d4d5 << 32) | 0x01010101;
        let expected_output_u128: u128 =
            (0x8e4da1bc << 96) | (0x9fdc589d << 64) | (0xd5d5d7d6 << 32) | 0x01010101;
        assert_eq!(mix_columns_u128(input_u128), expected_output_u128);
        assert_eq!(inv_mix_columns_u128(expected_output_u128), input_u128);
    }
}
