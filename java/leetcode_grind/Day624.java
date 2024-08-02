package leetcode_grind;

public class Day624 {
    // https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together-ii/description/?envType=daily-question&envId=2024-08-02
    static class Solution1 {
        public int minSwaps(int[] nums) {
            int total = 0;
            ;
            for (int i = 0; i < nums.length; i++) {
                total += nums[i];
            }
            int j = 0;
            int curr = 0;
            int ans = Integer.MAX_VALUE;
            for (int i = 0; i < nums.length * 2; i++) {
                curr += nums[i % nums.length];
                if (i >= total) {
                    curr -= nums[j % nums.length];
                    j++;
                }
                ans = Math.min(ans, total - curr);
            }
            return ans;
        }
    }
}
