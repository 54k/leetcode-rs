package leetcode_grind;

public class Day459 {
    // https://leetcode.com/problems/rearrange-array-elements-by-sign/description/
    static class Solution1 {
        public int[] rearrangeArray(int[] nums) {
            var n = nums.length;
            var ans = new int[n];
            var pos = 0;
            var neg = 1;
            for (int i = 0; i < nums.length; i++) {
                if (nums[i] > 0) {
                    ans[pos] = nums[i];
                    pos += 2;
                } else {
                    ans[neg] = nums[i];
                    neg += 2;
                }
            }
            return ans;
        }
    }
}
