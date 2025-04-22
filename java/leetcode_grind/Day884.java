package leetcode_grind;

public class Day884 {
    // https://leetcode.com/problems/count-the-hidden-sequences/description/
    static class Solution1 {
        public int numberOfArrays(int[] differences, int lower, int upper) {
            int x = 0, y = 0, cur = 0;
            for (int d : differences) {
                cur += d;
                x = Math.min(x, cur);
                y = Math.max(y, cur);
                if (y - x > upper - lower) {
                    return 0;
                }
            }
            return (upper - lower) - (y - x) + 1;
        }
    
}
