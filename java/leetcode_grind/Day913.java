package leetcode_grind;

public class Day913 {
    // https://leetcode.com/problems/zero-array-transformation-i/description/?envType=daily-question&envId=2025-05-20
    static class Solution1 {
        public boolean isZeroArray(int[] nums, int[][] queries) {
            var n = nums.length;
            var t = new int[n + 1];
            for (var q : queries) {
                t[q[0]] -= 1;
                t[q[1] + 1] += 1;
            }
            for (int i = 1; i <= n; i++) {
                t[i] += t[i - 1];
            }
            for (int i = 0; i < n; i++) {
                nums[i] = Math.max(nums[i] + t[i], 0);
                if (nums[i] != 0) {
                    return false;
                }
            }
            return true;
        }
    }
}
