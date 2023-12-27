package leetcode_grind;

public class Day410 {
    // https://leetcode.com/problems/minimum-time-to-make-rope-colorful/description
    static class Solution1 {
        public int minCost(String colors, int[] neededTime) {
            int totalTime = 0;
            int i = 0, j = 0;

            while (i < neededTime.length && j < neededTime.length) {
                int currTotal = 0, currMax = 0;

                while (j < neededTime.length && colors.charAt(i) == colors.charAt(j)) {
                    currTotal += neededTime[j];
                    currMax = Math.max(currMax, neededTime[j]);
                    j++;
                }

                totalTime += currTotal - currMax;
                i = j;
            }

            return totalTime;
        }
    }

    // https://leetcode.com/problems/house-robber-ii/description/
    static class Solution2 {
        public int rob(int[] nums) {
            if (nums.length == 0) {
                return 0;
            }

            if (nums.length == 1) {
                return nums[0];
            }

            int max1 = robSimple(nums, 0, nums.length - 2);
            int max2 = robSimple(nums, 1, nums.length - 1);

            return Math.max(max1, max2);
        }

        int robSimple(int[] nums, int start, int end) {
            int t1 = 0;
            int t2 = 0;

            for (int i = start; i <= end; i++) {
                int temp = t1;
                int current = nums[i];
                t1 = Math.max(current + t2, t1);
                t2 = temp;
            }

            return t1;
        }
    }
}
