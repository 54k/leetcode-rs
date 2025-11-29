package leetcode_grind;

public class Day1106 {
    // https://leetcode.com/problems/minimum-operations-to-make-array-sum-divisible-by-k/description/?envType=daily-question&envId=2025-11-29
    static class Solution1 {
        public int minOperations(int[] nums, int k) {
            int sum = 0;
            for (int num : nums) {
                sum += num;
            }
            return sum % k;
        }
    }

    // https://leetcode.com/problems/range-addition/description/
    static class Solution2 {
        public int[] getModifiedArray(int length, int[][] updates) {
            int[] result = new int[length];
            for (int[] update : updates) {
                result[update[0]] += update[2];
                if (update[1] + 1 <= (length - 1)) {
                    result[update[1] + 1] -= update[2];
                }
            }
            for (int i = 1; i < length; i++) {
                result[i] += result[i - 1];
            }
            return result;
        }
    }
}
