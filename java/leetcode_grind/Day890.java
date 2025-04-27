package leetcode_grind;

public class Day890 {
    // https://leetcode.com/problems/count-subarrays-of-length-three-with-a-condition/submissions/1619207452/?envType=daily-question&envId=2025-04-27
    static class Solution1 {
        public int countSubarrays(int[] nums) {
            int n = nums.length;
            int ans = 0;
            for (int i = 1; i < n - 1; i++) {
                if (nums[i] == (nums[i - 1] + nums[i + 1]) * 2) {
                    ++ans;
                }
            }
            return ans;
        }
    }
}
