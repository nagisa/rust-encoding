// AUTOGENERATED FROM index-iso-8859-14.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-iso-8859-14.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
    143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157,
    158, 159, 160, 7682, 7683, 163, 266, 267, 7690, 167, 7808, 169, 7810, 7691,
    7922, 173, 174, 376, 7710, 7711, 288, 289, 7744, 7745, 182, 7766, 7809,
    7767, 7811, 7776, 7923, 7812, 7813, 7777, 192, 193, 194, 195, 196, 197,
    198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 372, 209, 210, 211, 212,
    213, 214, 7786, 216, 217, 218, 219, 220, 221, 374, 223, 224, 225, 226, 227,
    228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 373, 241, 242,
    243, 244, 245, 246, 7787, 248, 249, 250, 251, 252, 253, 375, 255,
];

#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[code as uint]
}

pub fn backward(code: u16) -> u8 {
    match code {
        128 => 0, 129 => 1, 130 => 2, 131 => 3, 132 => 4, 133 => 5, 134 => 6,
        135 => 7, 136 => 8, 137 => 9, 138 => 10, 139 => 11, 140 => 12,
        141 => 13, 142 => 14, 143 => 15, 144 => 16, 145 => 17, 146 => 18,
        147 => 19, 148 => 20, 149 => 21, 150 => 22, 151 => 23, 152 => 24,
        153 => 25, 154 => 26, 155 => 27, 156 => 28, 157 => 29, 158 => 30,
        159 => 31, 160 => 32, 7682 => 33, 7683 => 34, 163 => 35, 266 => 36,
        267 => 37, 7690 => 38, 167 => 39, 7808 => 40, 169 => 41, 7810 => 42,
        7691 => 43, 7922 => 44, 173 => 45, 174 => 46, 376 => 47, 7710 => 48,
        7711 => 49, 288 => 50, 289 => 51, 7744 => 52, 7745 => 53, 182 => 54,
        7766 => 55, 7809 => 56, 7767 => 57, 7811 => 58, 7776 => 59, 7923 => 60,
        7812 => 61, 7813 => 62, 7777 => 63, 192 => 64, 193 => 65, 194 => 66,
        195 => 67, 196 => 68, 197 => 69, 198 => 70, 199 => 71, 200 => 72,
        201 => 73, 202 => 74, 203 => 75, 204 => 76, 205 => 77, 206 => 78,
        207 => 79, 372 => 80, 209 => 81, 210 => 82, 211 => 83, 212 => 84,
        213 => 85, 214 => 86, 7786 => 87, 216 => 88, 217 => 89, 218 => 90,
        219 => 91, 220 => 92, 221 => 93, 374 => 94, 223 => 95, 224 => 96,
        225 => 97, 226 => 98, 227 => 99, 228 => 100, 229 => 101, 230 => 102,
        231 => 103, 232 => 104, 233 => 105, 234 => 106, 235 => 107, 236 => 108,
        237 => 109, 238 => 110, 239 => 111, 373 => 112, 241 => 113, 242 => 114,
        243 => 115, 244 => 116, 245 => 117, 246 => 118, 7787 => 119,
        248 => 120, 249 => 121, 250 => 122, 251 => 123, 252 => 124, 253 => 125,
        375 => 126, 255 => 127, _ => 255
    }
}

#[cfg(test)]
mod tests {
    use super::{forward, backward};

    #[test]
    fn test_correct_table() {
        for i in range(0u8, 128) {
            let j = forward(i);
            if j != 0xffff { assert_eq!(backward(j), i); }
        }
    }
}
