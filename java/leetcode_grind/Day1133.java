package leetcode_grind;

public class Day1133 {
    // https://leetcode.com/problems/minimum-penalty-for-a-shop/description/?envType=daily-question&envId=2025-12-26
    static class Solution1 {
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
