package leetcode_grind;

public class Day1076 {
    // https://leetcode.com/problems/minimum-number-of-increments-on-subarrays-to-form-a-target-array/description/?envType=daily-question&envId=2025-10-30
    static class Solution1 {
        public int minNumberOperations(int[] target) {
            int ans = target[0], n = target.length;
            for (int i = 1; i < n; i++) {
                ans += Math.max(0, target[i] - target[i - 1]);
            }
            return ans;
        }
    }
}
