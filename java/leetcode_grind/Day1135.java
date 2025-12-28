package leetcode_grind;

public class Day1135 {
    // https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/description/?envType=daily-question&envId=2025-12-28
    static class Solution1 {
        public int countNegatives(int[][] grid) {
            int count = 0;
            int n = grid[0].length;
            int currRowNegativeIndex = n - 1;
            for (int[] row : grid) {
                while (currRowNegativeIndex >= 0 && row[currRowNegativeIndex] < 0) {
                    currRowNegativeIndex--;
                }

                count += (n - (currRowNegativeIndex + 1));
            }
            return count;
        }
    }
}
