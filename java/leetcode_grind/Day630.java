package leetcode_grind;

public class Day630 {
    // https://leetcode.com/problems/spiral-matrix-iii/description/?envType=daily-question&envId=2024-08-08
    static class Solution1 {
        public int[][] spiralMatrixIII(int rows, int cols, int rStart, int cStart) {
            var d = 0;
            var dir = new int[][] { { 0, 1 }, { 1, 0 }, { 0, -1 }, { -1, 0 } };
            var vis = rows * cols - 1;
            var len = 1;
            var x = rStart;
            var y = cStart;
            var path = new int[rows * cols][2];
            var pidx = 1;
            path[0] = new int[] { rStart, cStart };
            while (vis > 0) {
                for (int $ = 0; $ < 2; $++) {
                    for (int i = 0; i < len; i++) {
                        x += dir[d][0];
                        y += dir[d][1];
                        if (0 <= x && x < rows && 0 <= y && y < cols) {
                            path[pidx++] = new int[] { x, y };
                            vis--;
                        }
                    }
                    d = (d + 1) % dir.length;
                }
                len++;
            }
            return path;
        }
    }

    static class Solution2 {
        public int[][] spiralMatrixIII(int rows, int cols, int rStart, int cStart) {
            int[][] dir = new int[][] { { 0, 1 }, { 1, 0 }, { 0, -1 }, { -1, 0 } };
            int[][] traversed = new int[rows * cols][2];
            int idx = 0;
            for (int step = 1, direction = 0; idx < rows * cols;) {
                for (int i = 0; i < 2; i++) {
                    for (int j = 0; j < step; ++j) {
                        if (rStart >= 0 && rStart < rows && cStart >= 0 && cStart < cols) {
                            traversed[idx][0] = rStart;
                            traversed[idx][1] = cStart;
                            ++idx;
                        }
                        rStart += dir[direction][0];
                        cStart += dir[direction][1];
                    }
                    direction = (direction + 1) % 4;
                }
                ++step;
            }
            return traversed;
        }
    }
}
