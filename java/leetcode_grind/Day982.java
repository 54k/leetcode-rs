package leetcode_grind;

public class Day982 {
    // https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/description/?envType=daily-question&envId=2025-07-28
    static class Solution1 {
        public int countMaxOrSubsets(int[] nums) {
            int maxOrValue = 0;
            for (int num : nums) {
                maxOrValue |= num;
            }
            return countSubsets(nums, 0, 0, maxOrValue);
        }

        int countSubsets(int[] nums, int index, int currentOr, int targetOr) {
            if (index == nums.length) {
                return (currentOr == targetOr) ? 1 : 0;
            }

            int countWithout = countSubsets(nums, index + 1, currentOr, targetOr);
            int countWith = countSubsets(nums, index + 1, currentOr | nums[index], targetOr);

            return countWith + countWithout;
        }
    }

}
