package leetcode_grind;

public class Day1094 {
    // https://leetcode.com/problems/check-if-all-1s-are-at-least-length-k-places-away/description/?envType=daily-question&envId=2025-11-17
    static class Solution1 {
        public boolean kLengthApart(int[] nums, int k) {
            int count = k;
            for (int num : nums) {
                if (num == 1) {
                    if (count < k) {
                        return false;
                    }
                    count = 0;
                } else {
                    ++count;
                }
            }
            return true;
        }
    }
}
