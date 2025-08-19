package leetcode_grind;

public class Day1004 {
    // https://leetcode.com/problems/number-of-zero-filled-subarrays/description/?envType=daily-question&envId=2025-08-19
    static class Solution1 {
        public long zeroFilledSubarray(int[] nums) {
            long ans = 0, numSubarray = 0;

            for (int num : nums) {
                if (num == 0) {
                    numSubarray++;
                } else {
                    numSubarray = 0;
                }
                ans += numSubarray;
            }

            return ans;
        }
    }
}
