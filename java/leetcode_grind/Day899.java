package leetcode_grind;

public class Day899 {
    // https://leetcode.com/problems/build-array-from-permutation/description/?envType=daily-question&envId=2025-05-06
    static class Solution1 {
        public int[] buildArray(int[] nums) {
            int n = nums.length;       
            for (int i = 0; i < n; i++) {
                nums[i] += 1000 * (nums[nums[i]] % 1000);
            }
            for (int i = 0; i < n; i++) {
                nums[i] /= 1000;
            }
            return nums;
        }
    }
}