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

    // https://leetcode.com/problems/squirrel-simulation/description/
    static class Solution2 {
        public int minDistance(int height, int width, int[] tree, int[] squirrel, int[][] nuts) {
            int tot_dist = 0, d = Integer.MIN_VALUE;        
            for (int[] nut : nuts) {
                tot_dist += (distance(nut, tree) * 2);
                d = Math.max(d, distance(nut, tree) - distance(nut, squirrel));
            }
            return tot_dist - d;
        }

        int distance(int[] a, int[] b) {
            return Math.abs(a[0] - b[0]) + Math.abs(a[1] - b[1]);
        }
    }
}