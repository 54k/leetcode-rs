package leetcode_grind;

public class Day865 {
    // https://leetcode.com/problems/maximum-value-of-an-ordered-triplet-i/description/?envType=daily-question&envId=2025-04-02
    static class Solution1 {
        public long maximumTripletValue(int[] nums) {
            int n = nums.length;
            long res = 0;
            for (int k = 2; k < n; k++) {
                int maxPrefix = nums[0];
                for (int j = 1; j < k; j++) {
                    res = Math.max(res, (long) (maxPrefix - nums[j]) * nums[k]);
                    maxPrefix = Math.max(maxPrefix, nums[j]);
                }
            }
            return res;
        }
    }

    static class Solution2 {
        public long maximumTripletValue(int[] nums) {
            int n = nums.length;
            int[] leftMax = new int[n];
            int[] rightMax = new int[n];

            for (int i = 1; i < n; i++) {
                leftMax[i] = Math.max(leftMax[i - 1], nums[i - 1]);
                rightMax[n - i - 1] = Math.max(rightMax[n - i], nums[n - i]);
            }

            long res = 0;
            for (int j = 1; j < n - 1; j++) {
                res = Math.max(res, (long) (leftMax[j] - nums[j]) * rightMax[j]);
            }
            return res;
        }
    }

    static class Solution3 {
        public long maximumTripletValue(int[] nums) {
            int n = nums.length;
            long res = 0, imax = 0, dmax = 0;
            for (int k = 0; k < n; k++) {
                res = Math.max(res, dmax * nums[k]);
                dmax = Math.max(dmax, imax - nums[k]);
                imax = Math.max(imax, nums[k]);
            }
            return res;
        }
    }
}
