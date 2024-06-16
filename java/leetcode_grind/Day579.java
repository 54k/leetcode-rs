package leetcode_grind;

public class Day579 {
    // https://leetcode.com/problems/patching-array/description/
    static class Solution1 {
        public int minPatches(int[] nums, int n) {
            int patches = 0, i = 0;
            long miss = 1;
            while (miss <= n) {
                if (i < nums.length && nums[i] <= miss) {
                    miss += nums[i++];
                } else {
                    miss += miss;
                    patches++;
                }
            }
            return patches;
        }
    }
}
