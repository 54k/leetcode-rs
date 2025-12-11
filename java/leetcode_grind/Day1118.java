package leetcode_grind;

import java.util.Arrays;

public class Day1118 {
    // https://leetcode.com/problems/count-covered-buildings/description/?envType=daily-question&envId=2025-12-11
    static class Solution1 {
        public int countCoveredBuildings(int n, int[][] buildings) {
            int[] rowMax = new int[n + 1];
            int[] rowMin = new int[n + 1];
            int[] colMax = new int[n + 1];
            int[] colMin = new int[n + 1];
            Arrays.fill(rowMin, n + 1);
            Arrays.fill(colMin, n + 1);

            for (int[] b : buildings) {
                int x = b[0];
                int y = b[1];
                rowMax[y] = Math.max(rowMax[y], x);
                rowMin[y] = Math.min(rowMin[y], x);
                colMax[x] = Math.max(colMax[x], y);
                colMin[x] = Math.min(colMin[x], y);
            }

            int res = 0;
            for (int[] b : buildings) {
                int x = b[0];
                int y = b[1];

                if (x > rowMin[y] && x < rowMax[y] && y > colMin[x] && y < colMax[x]) {
                    res++;
                }
            }
            return res;
        }
    }
}
