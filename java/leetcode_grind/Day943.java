package leetcode_grind;

import java.util.Arrays;

public class Day943 {
    // https://leetcode.com/problems/partition-array-such-that-maximum-difference-is-k/description/?envType=daily-question&envId=2025-06-21
    static class Solution1 {
        public int partitionArray(int[] nums, int k) {
            Arrays.sort(nums);
            int ans = 1;
            int rec = nums[0];
            for (int i = 0; i < nums.length; i++) {
                if (nums[i] - rec > k) {
                    ans++;
                    rec = nums[i];
                }
            }
            return ans;
        }
    }
}
