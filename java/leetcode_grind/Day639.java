package leetcode_grind;

public class Day639 {
    // https://leetcode.com/problems/maximum-number-of-points-with-cost/description/?envType=daily-question&envId=2024-08-17
    static class Solution {
        public long maxPoints(int[][] points) {
            int rows = points.length, cols = points[0].length;
            long[] previousRow = new long[cols];
            for (int col = 0; col < cols; ++col) {
                previousRow[col] = (long) points[0][col];
            }

            for (int row = 0; row < rows - 1; ++row) {
                long[] leftMax = new long[cols];
                long[] rightMax = new long[cols];
                long[] currentRow = new long[cols];

                leftMax[0] = previousRow[0];
                for (int col = 1; col < cols; ++col) {
                    leftMax[col] = Math.max(leftMax[col - 1] - 1, previousRow[col]);
                }

                rightMax[cols - 1] = previousRow[cols - 1];
                for (int col = cols - 2; col >= 0; --col) {
                    rightMax[col] = Math.max(
                            rightMax[col + 1] - 1,
                            previousRow[col]);
                }

                for (int col = 0; col < cols; ++col) {
                    currentRow[col] = points[row + 1][col] + Math.max(leftMax[col], rightMax[col]);
                }

                previousRow = currentRow;
            }

            long maxPoints = 0;
            for (int col = 0; col < cols; ++col) {
                maxPoints = Math.max(maxPoints, previousRow[col]);
            }
            return maxPoints;
        }
    }
}