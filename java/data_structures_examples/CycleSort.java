package data_structures_examples;

public class CycleSort {
    // https://leetcode.com/problems/first-missing-positive/description
    static class Solution1 {
        public int firstMissingPositive(int[] nums) {
            int n = nums.length;

            int i = 0;
            while (i < n) {
                int correctIdx = nums[i] - 1;
                if (nums[i] > 0 && nums[i] <= n && nums[i] != nums[correctIdx]) {
                    int t = nums[i];
                    nums[i] = nums[correctIdx];
                    nums[correctIdx] = t;
                } else {
                    i++;
                }
            }

            for (i = 0; i < n; i++) {
                if (nums[i] != i + 1) {
                    return i + 1;
                }
            }

            return n + 1;
        }
    }
}
