package leetcode_grind;

public class Day730 {
    // https://leetcode.com/problems/find-the-power-of-k-size-subarrays-i/description/?envType=daily-question&envId=2024-11-16
    static class Solution1 {
        public int[] resultsArray(int[] nums, int k) {
            int length = nums.length;
            int[] result = new int[length - k + 1];

            for (int start = 0; start <= length - k; start++) {
                boolean isConsecutiveAndSorted = true;

                for (int index = start; index < start + k - 1; index++) {
                    if (nums[index + 1] != nums[index] + 1) {
                        isConsecutiveAndSorted = false;
                        break;
                    }
                }

                if (isConsecutiveAndSorted) {
                    result[start] = nums[start + k - 1];
                } else {
                    result[start] = -1;
                }
            }

            return result;
        }
    }

}
