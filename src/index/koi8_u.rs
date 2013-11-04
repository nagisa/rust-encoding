// AUTOGENERATED FROM index-koi8-u.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-koi8-u.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    9472, 9474, 9484, 9488, 9492, 9496, 9500, 9508, 9516, 9524, 9532, 9600,
    9604, 9608, 9612, 9616, 9617, 9618, 9619, 8992, 9632, 8729, 8730, 8776,
    8804, 8805, 160, 8993, 176, 178, 183, 247, 9552, 9553, 9554, 1105, 1108,
    9556, 1110, 1111, 9559, 9560, 9561, 9562, 9563, 1169, 9565, 9566, 9567,
    9568, 9569, 1025, 1028, 9571, 1030, 1031, 9574, 9575, 9576, 9577, 9578,
    1168, 9580, 169, 1102, 1072, 1073, 1094, 1076, 1077, 1092, 1075, 1093,
    1080, 1081, 1082, 1083, 1084, 1085, 1086, 1087, 1103, 1088, 1089, 1090,
    1091, 1078, 1074, 1100, 1099, 1079, 1096, 1101, 1097, 1095, 1098, 1070,
    1040, 1041, 1062, 1044, 1045, 1060, 1043, 1061, 1048, 1049, 1050, 1051,
    1052, 1053, 1054, 1055, 1071, 1056, 1057, 1058, 1059, 1046, 1042, 1068,
    1067, 1047, 1064, 1069, 1065, 1063, 1066,
];

#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[code as uint]
}

pub fn backward(code: u16) -> u8 {
    match code {
        9472 => 0, 9474 => 1, 9484 => 2, 9488 => 3, 9492 => 4, 9496 => 5,
        9500 => 6, 9508 => 7, 9516 => 8, 9524 => 9, 9532 => 10, 9600 => 11,
        9604 => 12, 9608 => 13, 9612 => 14, 9616 => 15, 9617 => 16, 9618 => 17,
        9619 => 18, 8992 => 19, 9632 => 20, 8729 => 21, 8730 => 22, 8776 => 23,
        8804 => 24, 8805 => 25, 160 => 26, 8993 => 27, 176 => 28, 178 => 29,
        183 => 30, 247 => 31, 9552 => 32, 9553 => 33, 9554 => 34, 1105 => 35,
        1108 => 36, 9556 => 37, 1110 => 38, 1111 => 39, 9559 => 40, 9560 => 41,
        9561 => 42, 9562 => 43, 9563 => 44, 1169 => 45, 9565 => 46, 9566 => 47,
        9567 => 48, 9568 => 49, 9569 => 50, 1025 => 51, 1028 => 52, 9571 => 53,
        1030 => 54, 1031 => 55, 9574 => 56, 9575 => 57, 9576 => 58, 9577 => 59,
        9578 => 60, 1168 => 61, 9580 => 62, 169 => 63, 1102 => 64, 1072 => 65,
        1073 => 66, 1094 => 67, 1076 => 68, 1077 => 69, 1092 => 70, 1075 => 71,
        1093 => 72, 1080 => 73, 1081 => 74, 1082 => 75, 1083 => 76, 1084 => 77,
        1085 => 78, 1086 => 79, 1087 => 80, 1103 => 81, 1088 => 82, 1089 => 83,
        1090 => 84, 1091 => 85, 1078 => 86, 1074 => 87, 1100 => 88, 1099 => 89,
        1079 => 90, 1096 => 91, 1101 => 92, 1097 => 93, 1095 => 94, 1098 => 95,
        1070 => 96, 1040 => 97, 1041 => 98, 1062 => 99, 1044 => 100,
        1045 => 101, 1060 => 102, 1043 => 103, 1061 => 104, 1048 => 105,
        1049 => 106, 1050 => 107, 1051 => 108, 1052 => 109, 1053 => 110,
        1054 => 111, 1055 => 112, 1071 => 113, 1056 => 114, 1057 => 115,
        1058 => 116, 1059 => 117, 1046 => 118, 1042 => 119, 1068 => 120,
        1067 => 121, 1047 => 122, 1064 => 123, 1069 => 124, 1065 => 125,
        1063 => 126, 1066 => 127, _ => 255
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
