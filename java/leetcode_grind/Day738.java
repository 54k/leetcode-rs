package leetcode_grind;

public class Day738 {
    // https://leetcode.com/problems/maximum-matrix-sum/description/?envType=daily-question&envId=2024-11-24
    static class Solution1 {
        public long maxMatrixSum(int[][] matrix) {
            long totalSum = 0;
            int minAbsVal = Integer.MAX_VALUE;
            int negativeCount = 0;

            for (int[] row : matrix) {
                for (int val : row) {
                    totalSum += Math.abs(val);
                    if (val < 0) {
                        negativeCount++;
                    }
                    minAbsVal = Math.min(minAbsVal, Math.abs(val));
                }
            }

            if (negativeCount % 2 != 0) {
                totalSum -= 2 * minAbsVal;
            }

            return totalSum;
        }
    }
}
