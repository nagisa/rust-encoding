// AUTOGENERATED FROM index-iso-8859-16.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-iso-8859-16.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
    143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157,
    158, 159, 160, 260, 261, 321, 8364, 8222, 352, 167, 353, 169, 536, 171,
    377, 173, 378, 379, 176, 177, 268, 322, 381, 8221, 182, 183, 382, 269, 537,
    187, 338, 339, 376, 380, 192, 193, 194, 258, 196, 262, 198, 199, 200, 201,
    202, 203, 204, 205, 206, 207, 272, 323, 210, 211, 212, 336, 214, 346, 368,
    217, 218, 219, 220, 280, 538, 223, 224, 225, 226, 259, 228, 263, 230, 231,
    232, 233, 234, 235, 236, 237, 238, 239, 273, 324, 242, 243, 244, 337, 246,
    347, 369, 249, 250, 251, 252, 281, 539, 255,
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
        159 => 31, 160 => 32, 260 => 33, 261 => 34, 321 => 35, 8364 => 36,
        8222 => 37, 352 => 38, 167 => 39, 353 => 40, 169 => 41, 536 => 42,
        171 => 43, 377 => 44, 173 => 45, 378 => 46, 379 => 47, 176 => 48,
        177 => 49, 268 => 50, 322 => 51, 381 => 52, 8221 => 53, 182 => 54,
        183 => 55, 382 => 56, 269 => 57, 537 => 58, 187 => 59, 338 => 60,
        339 => 61, 376 => 62, 380 => 63, 192 => 64, 193 => 65, 194 => 66,
        258 => 67, 196 => 68, 262 => 69, 198 => 70, 199 => 71, 200 => 72,
        201 => 73, 202 => 74, 203 => 75, 204 => 76, 205 => 77, 206 => 78,
        207 => 79, 272 => 80, 323 => 81, 210 => 82, 211 => 83, 212 => 84,
        336 => 85, 214 => 86, 346 => 87, 368 => 88, 217 => 89, 218 => 90,
        219 => 91, 220 => 92, 280 => 93, 538 => 94, 223 => 95, 224 => 96,
        225 => 97, 226 => 98, 259 => 99, 228 => 100, 263 => 101, 230 => 102,
        231 => 103, 232 => 104, 233 => 105, 234 => 106, 235 => 107, 236 => 108,
        237 => 109, 238 => 110, 239 => 111, 273 => 112, 324 => 113, 242 => 114,
        243 => 115, 244 => 116, 337 => 117, 246 => 118, 347 => 119, 369 => 120,
        249 => 121, 250 => 122, 251 => 123, 252 => 124, 281 => 125, 539 => 126,
        255 => 127, _ => 255
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
