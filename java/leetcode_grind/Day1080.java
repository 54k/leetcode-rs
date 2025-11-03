package leetcode_grind;

public class Day1080 {
    // https://leetcode.com/problems/minimum-time-to-make-rope-colorful/description/?envType=daily-question&envId=2025-11-03
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

    static class Solution2 {
        public int minCost(String colors, int[] neededTime) {
            int totalTime = 0, currMaxTime = 0;
            for (int i = 0; i < colors.length(); i++) {
                if (i > 0 && colors.charAt(i) != colors.charAt(i - 1)) {
                    currMaxTime = 0;
                }
                totalTime += Math.min(currMaxTime, neededTime[i]);
                currMaxTime = Math.max(currMaxTime, neededTime[i]);
            }
            return totalTime;
        }
    }
}
