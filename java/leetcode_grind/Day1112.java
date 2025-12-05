package leetcode_grind;

public class Day1112 {
    // https://leetcode.com/problems/count-partitions-with-even-sum-difference/description/?envType=daily-question&envId=2025-12-05
    static class Solution1 {
        public int countPartitions(int[] nums) {
            int totalSum = 0;
            for (int x : nums) {
                totalSum += x;
            }
            return totalSum % 2 == 0 ? nums.length - 1 : 0;
        }
    }
}
