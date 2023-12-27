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
}
