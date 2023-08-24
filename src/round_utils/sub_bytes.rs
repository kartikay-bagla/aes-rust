pub const S_BOX_MAPPING: [u8; 256] = [
    99, 124, 119, 123, 242, 107, 111, 197, 48, 1, 103, 43, 254, 215, 171, 118, 202, 130, 201, 125,
    250, 89, 71, 240, 173, 212, 162, 175, 156, 164, 114, 192, 183, 253, 147, 38, 54, 63, 247, 204,
    52, 165, 229, 241, 113, 216, 49, 21, 4, 199, 35, 195, 24, 150, 5, 154, 7, 18, 128, 226, 235,
    39, 178, 117, 9, 131, 44, 26, 27, 110, 90, 160, 82, 59, 214, 179, 41, 227, 47, 132, 83, 209, 0,
    237, 32, 252, 177, 91, 106, 203, 190, 57, 74, 76, 88, 207, 208, 239, 170, 251, 67, 77, 51, 133,
    69, 249, 2, 127, 80, 60, 159, 168, 81, 163, 64, 143, 146, 157, 56, 245, 188, 182, 218, 33, 16,
    255, 243, 210, 205, 12, 19, 236, 95, 151, 68, 23, 196, 167, 126, 61, 100, 93, 25, 115, 96, 129,
    79, 220, 34, 42, 144, 136, 70, 238, 184, 20, 222, 94, 11, 219, 224, 50, 58, 10, 73, 6, 36, 92,
    194, 211, 172, 98, 145, 149, 228, 121, 231, 200, 55, 109, 141, 213, 78, 169, 108, 86, 244, 234,
    101, 122, 174, 8, 186, 120, 37, 46, 28, 166, 180, 198, 232, 221, 116, 31, 75, 189, 139, 138,
    112, 62, 181, 102, 72, 3, 246, 14, 97, 53, 87, 185, 134, 193, 29, 158, 225, 248, 152, 17, 105,
    217, 142, 148, 155, 30, 135, 233, 206, 85, 40, 223, 140, 161, 137, 13, 191, 230, 66, 104, 65,
    153, 45, 15, 176, 84, 187, 22,
];

pub const INV_S_BOX_MAPPING: [u8; 256] = [
    82, 9, 106, 213, 48, 54, 165, 56, 191, 64, 163, 158, 129, 243, 215, 251, 124, 227, 57, 130,
    155, 47, 255, 135, 52, 142, 67, 68, 196, 222, 233, 203, 84, 123, 148, 50, 166, 194, 35, 61,
    238, 76, 149, 11, 66, 250, 195, 78, 8, 46, 161, 102, 40, 217, 36, 178, 118, 91, 162, 73, 109,
    139, 209, 37, 114, 248, 246, 100, 134, 104, 152, 22, 212, 164, 92, 204, 93, 101, 182, 146, 108,
    112, 72, 80, 253, 237, 185, 218, 94, 21, 70, 87, 167, 141, 157, 132, 144, 216, 171, 0, 140,
    188, 211, 10, 247, 228, 88, 5, 184, 179, 69, 6, 208, 44, 30, 143, 202, 63, 15, 2, 193, 175,
    189, 3, 1, 19, 138, 107, 58, 145, 17, 65, 79, 103, 220, 234, 151, 242, 207, 206, 240, 180, 230,
    115, 150, 172, 116, 34, 231, 173, 53, 133, 226, 249, 55, 232, 28, 117, 223, 110, 71, 241, 26,
    113, 29, 41, 197, 137, 111, 183, 98, 14, 170, 24, 190, 27, 252, 86, 62, 75, 198, 210, 121, 32,
    154, 219, 192, 254, 120, 205, 90, 244, 31, 221, 168, 51, 136, 7, 199, 49, 177, 18, 16, 89, 39,
    128, 236, 95, 96, 81, 127, 169, 25, 181, 74, 13, 45, 229, 122, 159, 147, 201, 156, 239, 160,
    224, 59, 77, 174, 42, 245, 176, 200, 235, 187, 60, 131, 83, 153, 97, 23, 43, 4, 126, 186, 119,
    214, 38, 225, 105, 20, 99, 85, 33, 12, 125,
];

pub fn sub_bytes_u8(state: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut output: [[u8; 4]; 4] = [[0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            output[i][j] = S_BOX_MAPPING[state[i][j] as usize];
        }
    }
    return output;
}

pub fn inv_sub_bytes_u8(state: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut output: [[u8; 4]; 4] = [[0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            output[i][j] = INV_S_BOX_MAPPING[state[i][j] as usize];
        }
    }
    return output;
}

pub fn sub_bytes_u32(state: [u32; 4]) -> [u32; 4] {
    let mut output: [u32; 4] = [0; 4];
    // split each element into 4 parts, perform lookup and then combine them
    for i in 0..4 {
        output[i] = (S_BOX_MAPPING[(state[i] & 0xFF) as usize] as u32)
            | ((S_BOX_MAPPING[(state[i] >> 8 & 0xFF) as usize] as u32) << 8)
            | ((S_BOX_MAPPING[(state[i] >> 16 & 0xFF) as usize] as u32) << 16)
            | ((S_BOX_MAPPING[(state[i] >> 24 & 0xFF) as usize] as u32) << 24)
    }
    return output;
}

pub fn sub_bytes_single_u32(state: u32) -> u32 {
    return (S_BOX_MAPPING[(state & 0xFF) as usize] as u32)
        | ((S_BOX_MAPPING[(state >> 8 & 0xFF) as usize] as u32) << 8)
        | ((S_BOX_MAPPING[(state >> 16 & 0xFF) as usize] as u32) << 16)
        | ((S_BOX_MAPPING[(state >> 24 & 0xFF) as usize] as u32) << 24);
}

pub fn inv_sub_bytes_u32(state: [u32; 4]) -> [u32; 4] {
    let mut output: [u32; 4] = [0; 4];
    // split each element into 4 parts, perform lookup and then combine them
    for i in 0..4 {
        output[i] = (INV_S_BOX_MAPPING[(state[i] & 0xFF) as usize] as u32)
            | ((INV_S_BOX_MAPPING[(state[i] >> 8 & 0xFF) as usize] as u32) << 8)
            | ((INV_S_BOX_MAPPING[(state[i] >> 16 & 0xFF) as usize] as u32) << 16)
            | ((INV_S_BOX_MAPPING[(state[i] >> 24 & 0xFF) as usize] as u32) << 24)
    }
    return output;
}

pub fn sub_bytes_u128(state: u128) -> u128 {
    return (S_BOX_MAPPING[(state & 0xFF) as usize] as u128)
        | ((S_BOX_MAPPING[(state >> 8 & 0xFF) as usize] as u128) << 8)
        | ((S_BOX_MAPPING[(state >> 16 & 0xFF) as usize] as u128) << 16)
        | ((S_BOX_MAPPING[(state >> 24 & 0xFF) as usize] as u128) << 24)
        | ((S_BOX_MAPPING[(state >> 32 & 0xFF) as usize] as u128) << 32)
        | ((S_BOX_MAPPING[(state >> 40 & 0xFF) as usize] as u128) << 40)
        | ((S_BOX_MAPPING[(state >> 48 & 0xFF) as usize] as u128) << 48)
        | ((S_BOX_MAPPING[(state >> 56 & 0xFF) as usize] as u128) << 56)
        | ((S_BOX_MAPPING[(state >> 64 & 0xFF) as usize] as u128) << 64)
        | ((S_BOX_MAPPING[(state >> 72 & 0xFF) as usize] as u128) << 72)
        | ((S_BOX_MAPPING[(state >> 80 & 0xFF) as usize] as u128) << 80)
        | ((S_BOX_MAPPING[(state >> 88 & 0xFF) as usize] as u128) << 88)
        | ((S_BOX_MAPPING[(state >> 96 & 0xFF) as usize] as u128) << 96)
        | ((S_BOX_MAPPING[(state >> 104 & 0xFF) as usize] as u128) << 104)
        | ((S_BOX_MAPPING[(state >> 112 & 0xFF) as usize] as u128) << 112)
        | ((S_BOX_MAPPING[(state >> 120 & 0xFF) as usize] as u128) << 120);
}

pub fn inv_sub_bytes_u128(state: u128) -> u128 {
    return (INV_S_BOX_MAPPING[(state & 0xFF) as usize] as u128)
        | ((INV_S_BOX_MAPPING[(state >> 8 & 0xFF) as usize] as u128) << 8)
        | ((INV_S_BOX_MAPPING[(state >> 16 & 0xFF) as usize] as u128) << 16)
        | ((INV_S_BOX_MAPPING[(state >> 24 & 0xFF) as usize] as u128) << 24)
        | ((INV_S_BOX_MAPPING[(state >> 32 & 0xFF) as usize] as u128) << 32)
        | ((INV_S_BOX_MAPPING[(state >> 40 & 0xFF) as usize] as u128) << 40)
        | ((INV_S_BOX_MAPPING[(state >> 48 & 0xFF) as usize] as u128) << 48)
        | ((INV_S_BOX_MAPPING[(state >> 56 & 0xFF) as usize] as u128) << 56)
        | ((INV_S_BOX_MAPPING[(state >> 64 & 0xFF) as usize] as u128) << 64)
        | ((INV_S_BOX_MAPPING[(state >> 72 & 0xFF) as usize] as u128) << 72)
        | ((INV_S_BOX_MAPPING[(state >> 80 & 0xFF) as usize] as u128) << 80)
        | ((INV_S_BOX_MAPPING[(state >> 88 & 0xFF) as usize] as u128) << 88)
        | ((INV_S_BOX_MAPPING[(state >> 96 & 0xFF) as usize] as u128) << 96)
        | ((INV_S_BOX_MAPPING[(state >> 104 & 0xFF) as usize] as u128) << 104)
        | ((INV_S_BOX_MAPPING[(state >> 112 & 0xFF) as usize] as u128) << 112)
        | ((INV_S_BOX_MAPPING[(state >> 120 & 0xFF) as usize] as u128) << 120);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_bytes() {
        let input_u8: [[u8; 4]; 4] = [
            [0x01, 0x02, 0x03, 0x04],
            [0x05, 0x06, 0x07, 0x08],
            [0x09, 0x0A, 0x0B, 0x0C],
            [0x0D, 0x0E, 0x0F, 0x10],
        ];
        let expected_output_u8: [[u8; 4]; 4] = [
            [0x7c, 0x77, 0x7b, 0xf2],
            [0x6b, 0x6f, 0xc5, 0x30],
            [0x01, 0x67, 0x2b, 0xfe],
            [0xd7, 0xab, 0x76, 0xca],
        ];
        assert_eq!(sub_bytes_u8(input_u8), expected_output_u8);
        assert_eq!(inv_sub_bytes_u8(expected_output_u8), input_u8);

        let input_u32: [u32; 4] = [0x01020304, 0x05060708, 0x090A0B0C, 0x0D0E0F10];
        let expected_output_u32: [u32; 4] = [0x7c777bf2, 0x6b6fc530, 0x01672bfe, 0xd7ab76ca];
        assert_eq!(sub_bytes_u32(input_u32), expected_output_u32);
        assert_eq!(sub_bytes_single_u32(input_u32[0]), expected_output_u32[0]);
        assert_eq!(inv_sub_bytes_u32(expected_output_u32), input_u32);

        let input_u128: u128 =
            (0x01020304 << 96) | (0x05060708 << 64) | (0x090A0B0C << 32) | 0x0D0E0F10;
        let expected_output_u128: u128 =
            (0x7c777bf2 << 96) | (0x6b6fc530 << 64) | (0x01672bfe << 32) | 0xd7ab76ca;
        assert_eq!(sub_bytes_u128(input_u128), expected_output_u128);
        assert_eq!(inv_sub_bytes_u128(expected_output_u128), input_u128);
    }
}
