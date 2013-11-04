// AUTOGENERATED FROM index-ibm866.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-ibm866.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    1040, 1041, 1042, 1043, 1044, 1045, 1046, 1047, 1048, 1049, 1050, 1051,
    1052, 1053, 1054, 1055, 1056, 1057, 1058, 1059, 1060, 1061, 1062, 1063,
    1064, 1065, 1066, 1067, 1068, 1069, 1070, 1071, 1072, 1073, 1074, 1075,
    1076, 1077, 1078, 1079, 1080, 1081, 1082, 1083, 1084, 1085, 1086, 1087,
    9617, 9618, 9619, 9474, 9508, 9569, 9570, 9558, 9557, 9571, 9553, 9559,
    9565, 9564, 9563, 9488, 9492, 9524, 9516, 9500, 9472, 9532, 9566, 9567,
    9562, 9556, 9577, 9574, 9568, 9552, 9580, 9575, 9576, 9572, 9573, 9561,
    9560, 9554, 9555, 9579, 9578, 9496, 9484, 9608, 9604, 9612, 9616, 9600,
    1088, 1089, 1090, 1091, 1092, 1093, 1094, 1095, 1096, 1097, 1098, 1099,
    1100, 1101, 1102, 1103, 1025, 1105, 1028, 1108, 1031, 1111, 1038, 1118,
    176, 8729, 183, 8730, 8470, 164, 9632, 160,
];

#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[code as uint]
}

pub fn backward(code: u16) -> u8 {
    match code {
        1040 => 0, 1041 => 1, 1042 => 2, 1043 => 3, 1044 => 4, 1045 => 5,
        1046 => 6, 1047 => 7, 1048 => 8, 1049 => 9, 1050 => 10, 1051 => 11,
        1052 => 12, 1053 => 13, 1054 => 14, 1055 => 15, 1056 => 16, 1057 => 17,
        1058 => 18, 1059 => 19, 1060 => 20, 1061 => 21, 1062 => 22, 1063 => 23,
        1064 => 24, 1065 => 25, 1066 => 26, 1067 => 27, 1068 => 28, 1069 => 29,
        1070 => 30, 1071 => 31, 1072 => 32, 1073 => 33, 1074 => 34, 1075 => 35,
        1076 => 36, 1077 => 37, 1078 => 38, 1079 => 39, 1080 => 40, 1081 => 41,
        1082 => 42, 1083 => 43, 1084 => 44, 1085 => 45, 1086 => 46, 1087 => 47,
        9617 => 48, 9618 => 49, 9619 => 50, 9474 => 51, 9508 => 52, 9569 => 53,
        9570 => 54, 9558 => 55, 9557 => 56, 9571 => 57, 9553 => 58, 9559 => 59,
        9565 => 60, 9564 => 61, 9563 => 62, 9488 => 63, 9492 => 64, 9524 => 65,
        9516 => 66, 9500 => 67, 9472 => 68, 9532 => 69, 9566 => 70, 9567 => 71,
        9562 => 72, 9556 => 73, 9577 => 74, 9574 => 75, 9568 => 76, 9552 => 77,
        9580 => 78, 9575 => 79, 9576 => 80, 9572 => 81, 9573 => 82, 9561 => 83,
        9560 => 84, 9554 => 85, 9555 => 86, 9579 => 87, 9578 => 88, 9496 => 89,
        9484 => 90, 9608 => 91, 9604 => 92, 9612 => 93, 9616 => 94, 9600 => 95,
        1088 => 96, 1089 => 97, 1090 => 98, 1091 => 99, 1092 => 100,
        1093 => 101, 1094 => 102, 1095 => 103, 1096 => 104, 1097 => 105,
        1098 => 106, 1099 => 107, 1100 => 108, 1101 => 109, 1102 => 110,
        1103 => 111, 1025 => 112, 1105 => 113, 1028 => 114, 1108 => 115,
        1031 => 116, 1111 => 117, 1038 => 118, 1118 => 119, 176 => 120,
        8729 => 121, 183 => 122, 8730 => 123, 8470 => 124, 164 => 125,
        9632 => 126, 160 => 127, _ => 255
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
