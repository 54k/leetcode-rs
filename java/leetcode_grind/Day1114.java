package leetcode_grind;

public class Day1114 {
    // https://leetcode.com/problems/count-odd-numbers-in-an-interval-range/description/?envType=daily-question&envId=2025-12-07
    static class Solution1 {
        public int countOdds(int low, int high) {
            if ((low & 1) == 0) {
                low++;
            }
            return low > high ? 0 : (high - low) / 2 + 1;
        }
    }
}
