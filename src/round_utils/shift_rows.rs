pub fn shift_rows_u8(state: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut output: [[u8; 4]; 4] = [[0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            output[i][j] = state[i][(j + i) % 4];
        }
    }
    output
}

pub fn inv_shift_rows_u8(state: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut output: [[u8; 4]; 4] = [[0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            output[i][j] = state[i][(j + 4 - i) % 4];
        }
    }
    output
}

pub fn shift_rows_u32(state: [u32; 4]) -> [u32; 4] {
    let mut output: [u32; 4] = [0; 4];
    output[0] = state[0];
    output[1] = state[1] >> 8 | state[1] << 24;
    output[2] = state[2] >> 16 | state[2] << 16;
    output[3] = state[3] >> 24 | state[3] << 8;
    return output;
}

pub fn inv_shift_rows_u32(state: [u32; 4]) -> [u32; 4] {
    let mut output: [u32; 4] = [0; 4];
    output[0] = state[0];
    output[1] = state[1] << 8 | state[1] >> 24;
    output[2] = state[2] << 16 | state[2] >> 16;
    output[3] = state[3] << 24 | state[3] >> 8;
    return output;
}

pub fn shift_rows_u128(state: u128) -> u128 {
    return (state & 0xFFFFFFFF_00000000_00000000_00000000)
        | (state >> 8 & 0x00000000_00FFFFFF_00000000_00000000
            | state << 24 & 0x00000000_FF000000_00000000_00000000)
        | (state >> 16 & 0x00000000_00000000_0000FFFF_00000000
            | state << 16 & 0x00000000_00000000_FFFF0000_00000000)
        | (state >> 24 & 0x00000000_00000000_00000000_000000FF
            | state << 8 & 0x00000000_00000000_00000000_FFFFFF00);
}

pub fn inv_shift_rows_u128(state: u128) -> u128 {
    return (state & 0xFFFFFFFF_00000000_00000000_00000000)
        | (state << 8 & 0x00000000_FFFFFF00_00000000_00000000
            | state >> 24 & 0x00000000_000000FF_00000000_00000000)
        | (state << 16 & 0x00000000_00000000_FFFF0000_00000000
            | state >> 16 & 0x00000000_00000000_0000FFFF_00000000)
        | (state << 24 & 0x00000000_00000000_00000000_FFFFFF00
            | state >> 8 & 0x00000000_00000000_00000000_00FFFFFF);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_rows() {
        let input_u8: [[u8; 4]; 4] = [
            [0x01, 0x02, 0x03, 0x04],
            [0x05, 0x06, 0x07, 0x08],
            [0x09, 0x0A, 0x0B, 0x0C],
            [0x0D, 0x0E, 0x0F, 0x10],
        ];
        let expected_output_u8: [[u8; 4]; 4] = [
            [0x01, 0x02, 0x03, 0x04],
            [0x06, 0x07, 0x08, 0x05],
            [0x0B, 0x0C, 0x09, 0x0A],
            [0x10, 0x0D, 0x0E, 0x0F],
        ];
        assert_eq!(shift_rows_u8(input_u8), expected_output_u8);
        assert_eq!(inv_shift_rows_u8(expected_output_u8), input_u8);

        let input_u32: [u32; 4] = [0x01020304, 0x05060708, 0x090A0B0C, 0x0D0E0F10];
        let expected_output_u32: [u32; 4] = [0x01020304, 0x08050607, 0x0B0C090A, 0x0E0F100D];
        assert_eq!(shift_rows_u32(input_u32), expected_output_u32);
        assert_eq!(inv_shift_rows_u32(expected_output_u32), input_u32);

        let input_u128: u128 =
            (0x01020304 << 96) | (0x05060708 << 64) | (0x090A0B0C << 32) | 0x0D0E0F10;
        let expected_output_u128: u128 =
            (0x01020304 << 96) | (0x08050607 << 64) | (0x0B0C090A << 32) | 0x0E0F100D;
        assert_eq!(shift_rows_u128(input_u128), expected_output_u128);
        assert_eq!(inv_shift_rows_u128(expected_output_u128), input_u128);
    }
}
