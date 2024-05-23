package leetcode_grind;

public class Day555 {
    // https://leetcode.com/problems/the-number-of-beautiful-subsets/description/
    static class Solution1 {
        public int beautifulSubsets(int[] nums, int k) {
            return countBeautifulSubsets(nums, k, 0, 0);
        }

        private int countBeautifulSubsets(int[] nums, int difference, int index, int mask) {
            if (index == nums.length) {
                return mask > 0 ? 1 : 0;
            }

            boolean isBeautiful = true;

            for (int j = 0; j < index && isBeautiful; ++j) {
                isBeautiful = ((1 << j) & mask) == 0 || Math.abs(nums[j] - nums[index]) != difference;
            }

            int skip = countBeautifulSubsets(nums, difference, index + 1, mask);
            int take;
            if (isBeautiful) {
                take = countBeautifulSubsets(nums, difference, index + 1, mask + (1 << index));
            } else {
                take = 0;
            }
            return skip + take;
        }
    }
}
