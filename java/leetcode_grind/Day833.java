package leetcode_grind;

public class Day833 {
    // https://leetcode.com/problems/apply-operations-to-an-array/description/?envType=daily-question&envId=2025-03-01
    static class Solution1 {
        public int[] applyOperations(int[] nums) {
            int n = nums.length;
            for (int i = 0; i < n - 1; i++) {
                if (nums[i] == nums[i + 1]) {
                    nums[i] *= 2;
                    nums[i + 1] = 0;
                }
            }

            for (int i = 0, j = 0; i < n; i++) {
                if (nums[i] != 0) {
                    int t = nums[i];
                    nums[i] = nums[j];
                    nums[j++] = t;
                }
            }

            return nums;
        }
    }

    
}
