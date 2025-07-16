package leetcode_grind;

public class Day970 {
    // https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-i/description/?envType=daily-question&envId=2025-07-16
    static class Solution1 {
        public int maximumLength(int[] nums) {
            int res = 0;
            int[][] patterns = { { 0, 0 }, { 0, 1 }, { 1, 0 }, { 1, 1 } };
            for (int[] pattern : patterns) {
                int cnt = 0;
                for (int num : nums) {
                    if (num % 2 == pattern[cnt % 2]) {
                        cnt++;
                    }
                }
                res = Math.max(res, cnt);
            }
            return res;
        }
    }
}
