package leetcode_grind;

public class Day794 {
    // https://leetcode.com/problems/grid-game/description/?envType=daily-question&envId=2025-01-21
    static class Solution1 {
        public long gridGame(int[][] grid) {
            long firstRowSum = 0;
            for (int num : grid[0]) {
                firstRowSum += num;
            }
            long secondRowSum = 0;
            long minimumSum = Long.MAX_VALUE;
            for (int turnIndex = 0; turnIndex < grid[0].length; ++turnIndex) {
                firstRowSum -= grid[0][turnIndex];
                minimumSum = Math.min(minimumSum, Math.max(firstRowSum, secondRowSum));
                secondRowSum += grid[1][turnIndex];
            }
            return minimumSum;
        }
    }

    // https://leetcode.com/problems/minimum-penalty-for-a-shop/description/
    static class Solution2 {
        public int bestClosingTime(String customers) {
            int curPenalty = 0;
            for (int i = 0; i < customers.length(); i++) {
                if (customers.charAt(i) == 'Y') {
                    curPenalty++;
                }
            }

            // Start with closing at hour 0, the penalty equals all 'Y' in closed hours.
            int minPenalty = curPenalty;
            int earliestHour = 0;

            for (int i = 0; i < customers.length(); i++) {
                char ch = customers.charAt(i);

                // If status in hour i is 'Y', moving it to open hours decrement
                // penalty by 1. Otherwise, moving 'N' to open hours increment
                // penatly by 1.
                if (ch == 'Y') {
                    curPenalty--;
                } else {
                    curPenalty++;
                }

                // Update earliestHour if a smaller penatly is encountered.
                if (curPenalty < minPenalty) {
                    earliestHour = i + 1;
                    minPenalty = curPenalty;
                }
            }

            return earliestHour;
        }
    }

    static class Solution3 {
        public int bestClosingTime(String customers) {
            int minPenalty = 0, curPenalty = 0;
            int earliestHour = 0;

            for (int i = 0; i < customers.length(); i++) {
                char ch = customers.charAt(i);

                if (ch == 'Y') {
                    curPenalty--;
                } else {
                    curPenalty++;
                }

                if (curPenalty < minPenalty) {
                    earliestHour = i + 1;
                    minPenalty = curPenalty;
                }
            }
            return earliestHour;
        }
    }
}
