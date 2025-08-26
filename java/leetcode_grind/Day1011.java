package leetcode_grind;

public class Day1011 {
    // https://leetcode.com/problems/maximum-area-of-longest-diagonal-rectangle/description/?envType=daily-question&envId=2025-08-26
    static class Solution1 {
        public int areaOfMaxDiagonal(int[][] dimensions) {
            int maxDiagSq = 0;
            int maxArea = 0;

            for (int[] dim : dimensions) {
                int l = dim[0];
                int w = dim[1];
                int diagSq = l * l + w * w;
                int area = l * w;

                if (diagSq > maxDiagSq) {
                    maxDiagSq = diagSq;
                    maxArea = area;
                } else if (diagSq == maxDiagSq) {
                    maxArea = Math.max(maxArea, area);
                }
            }
            return maxArea;
        }
    }
}