package leetcode_grind;

public class Day721 {
    // https://leetcode.com/problems/largest-combination-with-bitwise-and-greater-than-zero/description/?envType=daily-question&envId=2024-11-07
    static class Solution1 {
        public int largestCombination(int[] candidates) {
            int[] bitCount = new int[24];
            for (int i = 0; i < 24; i++) {
                for (int num : candidates) {
                    if ((num & (1 << i)) != 0) {
                        bitCount[i]++;
                    }
                }
            }

            int max = 0;
            for (int count : bitCount) {
                if (count > max) {
                    max = count;
                }
            }
            return max;
        }
    }

}
