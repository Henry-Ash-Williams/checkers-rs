pub const KING_MOVES: [&[usize]; 64] = [
    &[0, 9, 18, 27, 36, 45, 54, 63],
    &[8, 10, 19, 28, 37, 46, 55],
    &[9, 16, 11, 20, 29, 38, 47],
    &[10, 17, 24, 12, 21, 30, 39],
    &[11, 18, 25, 32, 13, 22, 31],
    &[12, 19, 26, 33, 40, 14, 23],
    &[13, 20, 27, 34, 41, 48, 15],
    &[14, 21, 28, 35, 42, 49, 56],
    &[1, 17, 26, 35, 44, 53, 62],
    &[0, 18, 27, 36, 45, 54, 63, 2, 16],
    &[1, 19, 28, 37, 46, 55, 64, 3, 17, 24],
    &[2, 20, 29, 38, 47, 56, 4, 18, 25, 32],
    &[3, 21, 30, 39, 48, 57, 5, 19, 26, 33, 40],
    &[4, 22, 31, 40, 49, 58, 6, 20, 27, 34, 41, 48],
    &[5, 23, 32, 41, 50, 59, 7, 21, 28, 35, 42, 49, 56],
    &[6, 22, 29, 36, 43, 50, 57],
    &[9, 2, 25, 34, 43, 52, 61],
    &[8, 26, 35, 44, 53, 62, 10, 3, 24],
    &[9, 0, 27, 36, 45, 54, 63, 11, 4, 25, 32],
    &[10, 1, 28, 37, 46, 55, 64, 12, 5, 26, 33, 40],
    &[11, 2, 29, 38, 47, 56, 13, 6, 27, 34, 41, 48],
    &[12, 3, 30, 39, 48, 57, 14, 7, 28, 35, 42, 49, 56],
    &[13, 4, 31, 40, 49, 58, 15, 29, 36, 43, 50, 57, 64],
    &[14, 5, 30, 37, 44, 51, 58],
    &[17, 10, 3, 33, 42, 51, 60],
    &[16, 7, 34, 43, 52, 61, 18, 11, 4, 32],
    &[17, 8, 35, 44, 53, 62, 19, 12, 5, 33, 40],
    &[18, 9, 0, 36, 45, 54, 63, 20, 13, 6, 34, 41, 48],
    &[19, 10, 1, 37, 46, 55, 64, 21, 14, 7, 35, 42, 49, 56],
    &[20, 11, 2, 38, 47, 56, 22, 15, 36, 43, 50, 57, 64],
    &[21, 12, 3, 39, 48, 57, 23, 37, 44, 51, 58, 65],
    &[22, 13, 4, 38, 45, 52, 59],
    &[25, 18, 11, 4, 41, 50, 59],
    &[24, 15, 6, 42, 51, 60, 26, 19, 12, 5, 40],
    &[25, 16, 7, 43, 52, 61, 27, 20, 13, 6, 41, 48],
    &[26, 17, 8, 44, 53, 62, 28, 21, 14, 7, 42, 49, 56],
    &[27, 18, 9, 0, 45, 54, 63, 29, 22, 15, 43, 50, 57, 64],
    &[28, 19, 10, 1, 46, 55, 64, 30, 23, 44, 51, 58, 65],
    &[29, 20, 11, 2, 47, 56, 31, 45, 52, 59, 66],
    &[30, 21, 12, 3, 46, 53, 60],
    &[33, 26, 19, 12, 5, 49, 58],
    &[32, 23, 14, 5, 50, 59, 34, 27, 20, 13, 6, 48],
    &[33, 24, 15, 6, 51, 60, 35, 28, 21, 14, 7, 49, 56],
    &[34, 25, 16, 7, 52, 61, 36, 29, 22, 15, 50, 57, 64],
    &[35, 26, 17, 8, 53, 62, 37, 30, 23, 51, 58, 65],
    &[36, 27, 18, 9, 0, 54, 63, 38, 31, 52, 59, 66],
    &[37, 28, 19, 10, 1, 55, 64, 39, 53, 60, 67],
    &[38, 29, 20, 11, 2, 54, 61],
    &[41, 34, 27, 20, 13, 6, 57],
    &[40, 31, 22, 13, 4, 58, 42, 35, 28, 21, 14, 7, 56],
    &[41, 32, 23, 14, 5, 59, 43, 36, 29, 22, 15, 57, 64],
    &[42, 33, 24, 15, 6, 60, 44, 37, 30, 23, 58, 65],
    &[43, 34, 25, 16, 7, 61, 45, 38, 31, 59, 66],
    &[44, 35, 26, 17, 8, 62, 46, 39, 60, 67],
    &[45, 36, 27, 18, 9, 0, 63, 47, 61, 68],
    &[46, 37, 28, 19, 10, 1, 62],
    &[49, 42, 35, 28, 21, 14, 7],
    &[48, 50, 43, 36, 29, 22, 15],
    &[49, 40, 51, 44, 37, 30, 23],
    &[50, 41, 32, 52, 45, 38, 31],
    &[51, 42, 33, 24, 53, 46, 39],
    &[52, 43, 34, 25, 16, 54, 47],
    &[53, 44, 35, 26, 17, 8, 55],
    &[54, 45, 36, 27, 18, 9, 0],
];
