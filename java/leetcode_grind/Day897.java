package leetcode_grind;

public class Day897 {
    // https://leetcode.com/problems/number-of-equivalent-domino-pairs/description/?envType=daily-question&envId=2025-05-04
    static class Solution1 {
        public int numEquivDominoPairs(int[][] dominoes) {
            int[] num = new int[100];        
            int ret = 0;
            for (int[] domino : dominoes) {
                int val = domino[0] < domino[1] ? domino[0] * 10 + domino[1] : domino[1] * 10 + domino[0];
                ret += num[val];
                num[val]++;
            }
            return ret;
        }
    }

    // https://leetcode.com/problems/factorial-trailing-zeroes/description/
    static class Solution2 {
        public int trailingZeroes(int n) {
            int zeroCount = 0;        
            long currentMultiple = 5;
            while (n >= currentMultiple) {
                zeroCount += (n / currentMultiple);
                currentMultiple *= 5;
            }
            return zeroCount;
        }
    }
}