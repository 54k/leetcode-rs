package leetcode_grind;

public class Day1102 {
    // https://leetcode.com/problems/smallest-integer-divisible-by-k/description/?envType=daily-question&envId=2025-11-25
    static class Solution1 {
        public int smallestRepunitDivByK(int K) {
            int remainder = 0;
            for (int length_N = 1; length_N <= K; length_N++) {
                remainder = (remainder * 10 + 1) % K;
                if (remainder == 0) {
                    return length_N;
                }
            }
            return -1;
        }
    }
}
