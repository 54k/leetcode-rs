package leetcode_grind;

public class Day880 {
    // https://leetcode.com/problems/count-equal-and-divisible-pairs-in-an-array/description/?envType=daily-question&envId=2025-04-17
    static class Solution1 {
        public int countPairs(int[] nums, int k) {
            int n = nums.length;
            int res = 0;
            for (int i = 0; i < n - 1; i++) {
                for (int j = i + 1; j < n; ++j) {
                    if ((i * j) % k == 0 && nums[i] == nums[j]) {
                        ++res;
                    }
                }
            }
            return res;
        }
    }
}
