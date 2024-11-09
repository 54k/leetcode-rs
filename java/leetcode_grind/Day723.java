package leetcode_grind;

public class Day723 {
    // https://leetcode.com/problems/minimum-array-end/description/?envType=daily-question&envId=2024-11-09
    static class Solution1 {
        public long minEnd(int n, int x) {
            long result = 0, bit;
            --n;

            int[] binaryX = new int[64];
            int[] binaryN = new int[64];

            long longX = x;
            long longN = n;

            for (int i = 0; i < 64; i++) {
                bit = (longX >> i) & 1;
                binaryX[i] = (int) bit;
                bit = (longN >> i) & 1;
                binaryN[i] = (int) bit;
            }

            int posX = 0, posN = 0;

            while (posX < 64) {
                while (binaryX[posX] != 0 && posX < 64) {
                    ++posX;
                }
                binaryX[posX] = binaryN[posN];
                ++posX;
                ++posN;
            }

            for (int i = 0; i < 64; i++) {
                if (binaryX[i] == 1) {
                    result += Math.pow(2, i);
                }
            }

            return result;
        }
    }

}
