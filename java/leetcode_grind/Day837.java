package leetcode_grind;

public class Day837 {
    // https://leetcode.com/problems/count-total-number-of-colored-cells/description/
    static class Solution1 {
        public long coloredCells(int n) {
            long numBlueCells = 1;
            int addend = 4;
            while (--n > 0) {
                numBlueCells += addend;
                addend += 4;
            }
            return numBlueCells;
        }
    }
}